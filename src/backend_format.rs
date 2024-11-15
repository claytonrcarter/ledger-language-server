use ledger::anon_unions::Account_Amount_BalanceAssertion_Note_Price_Status as PostingFields;
use ledger::anon_unions::AutomatedXact_PeriodicXact_PlainXact as Transactions;
use ledger::anon_unions::BlockComment_Comment_Directive_Test_Xact as JournalItems;
use ledger::anon_unions::Code_Date_EffectiveDate_Note_Payee_Posting_Status as XactFields;
use ledger::anon_unions::Commodity_NegativeQuantity_Quantity as AmountFields;
use ledger::anon_unions::Interval_Note_Posting as PeriodicXactFields;
use ledger::anon_unions::Note_Posting_Query as AutomatedXactFields;
use type_sitter::{Node, Parser, Range, TreeCursor};

use std::cmp::Ordering;
use std::fmt::Display;
use std::io::Write;

mod ledger {
    include!("./type_sitter/ledger/nodes.rs");
}

pub fn format(content: &str) -> Result<String, std::io::Error> {
    //
    // parse with tree sitter
    //
    let mut parser = Parser::<ledger::SourceFile>::new(&tree_sitter_ledger::LANGUAGE.into())
        .expect("loading Ledger tree-sitter grammar");
    let tree = parser.parse(content, None).unwrap();
    let mut cursor = tree.walk();

    //
    // convert from tree sitter to internal types (easier to use)
    //
    let journal_items: Vec<JournalItem> =
        tree.root_node()
            .unwrap()
            .journal_items(&mut cursor)
            .map(|journal_item| {
                match journal_item.unwrap().child().unwrap() {
                    JournalItems::Comment(comment) => {
                        JournalItem::Comment(substring(content, comment.range()))
                    }
                    JournalItems::Directive(directive) => {
                        JournalItem::Directive(substring(content, directive.range()))
                    }
                    JournalItems::Xact(xact) => {
                        match xact.child().unwrap() {
                            Transactions::AutomatedXact(xact) => JournalItem::AutomatedXact(
                                AutomatedXact::from_ts_xact(xact, content, || tree.walk()),
                            ),
                            Transactions::PeriodicXact(xact) => JournalItem::PeriodicXact(
                                PeriodicXact::from_ts_xact(xact, content, || tree.walk()),
                            ),
                            Transactions::PlainXact(xact) => JournalItem::PlainXact(
                                PlainXact::from_ts_xact(xact, content, || tree.walk()),
                            ),
                        }
                    }
                    JournalItems::BlockComment(comment) => {
                        // TODO
                        JournalItem::Comment(substring(content, comment.range()))
                    }
                    JournalItems::Test(test) => {
                        // TODO
                        JournalItem::Other(substring(content, test.range()))
                    }
                }
            })
            .collect();

    //
    // sort, attempting to keep comments that are interspersed with transactions
    // together with those transactions, while pushing all other things to the
    // start of the journal
    //
    let mut chunks = Vec::new();
    let mut chunk = SortableChunk::new();
    for journal_item in journal_items.iter() {
        chunk.items.push(&journal_item);
        match journal_item {
            JournalItem::PlainXact(ref t) => {
                chunk.date = t.date.clone();
                chunks.push(chunk);
                chunk = SortableChunk::new();
            }
            JournalItem::AutomatedXact(_)
            | JournalItem::PeriodicXact(_)
            | JournalItem::Directive(_) => {
                chunks.push(chunk);
                chunk = SortableChunk::new();
            }
            JournalItem::Comment(_) | JournalItem::Other(_) => {}
        }
    }
    // in case of trailing comments
    if !chunk.items.is_empty() {
        chunks.push(chunk);
    }

    chunks.sort();

    let journal_items = chunks
        .iter()
        .flat_map(|chunk| chunk.items.clone())
        .cloned()
        .collect::<Vec<JournalItem>>();

    //
    // print/format
    //
    let mut previous_item = None;
    let mut buf = Vec::new();
    for journal_item in journal_items {
        let formatted_item = match &journal_item {
            JournalItem::PlainXact(xact) => xact.to_string(),
            JournalItem::PeriodicXact(xact) => xact.to_string(),
            JournalItem::AutomatedXact(xact) => xact.to_string(),
            JournalItem::Comment(s) | JournalItem::Directive(s) | JournalItem::Other(s) => {
                s.clone()
            }
        };

        // group similar items together into blocks, but separate all xacts w/ a
        // blank line
        match (previous_item, &journal_item) {
            (None, _)
            | (Some(JournalItem::Comment(_)), JournalItem::Comment(_))
            | (Some(JournalItem::Directive(_)), JournalItem::Directive(_))
            | (Some(JournalItem::Other(_)), JournalItem::Other(_)) => {}
            (_, JournalItem::PlainXact(_))
            | (_, JournalItem::PeriodicXact(_))
            | (_, JournalItem::AutomatedXact(_))
            | (Some(_), _) => {
                writeln!(buf, "")?;
            }
        }

        writeln!(buf, "{}", formatted_item.trim())?;

        previous_item = Some(journal_item);
    }

    Ok(String::from_utf8(buf).expect("TODO"))
}

fn substring(content: &str, range: Range) -> String {
    content[range.start_byte..range.end_byte].to_string()
}

#[derive(Clone, Eq, PartialEq)]
enum JournalItem {
    PlainXact(PlainXact),
    PeriodicXact(PeriodicXact),
    AutomatedXact(AutomatedXact),
    Comment(String),
    Directive(String),
    Other(String),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PlainXact {
    /// the row on which the transaction starts
    row: usize,

    date: Option<String>,
    effective_date: Option<String>,
    status: Option<String>,
    code: Option<String>,
    payee: Option<String>,
    postings: Vec<Posting>,

    payee_note: Option<String>,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PeriodicXact {
    /// the row on which the transaction starts
    row: usize,

    interval: String,
    postings: Vec<Posting>,

    interval_note: Option<String>,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct AutomatedXact {
    /// the row on which the transaction starts
    row: usize,

    query: String,
    postings: Vec<Posting>,

    query_note: Option<String>,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum CommodityPosition {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Posting {
    account: String,
    commodity_position: CommodityPosition,
    commodity: Option<String>,
    quantity: Option<String>,
    balance_assertion: Option<String>,

    inline_note: Option<String>,
    trailing_notes: Vec<String>,
}

impl<'tree> PlainXact {
    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        xact: ledger::PlainXact<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Self {
        let mut x = PlainXact::default();
        x.row = xact.range().start_point.row;

        let mut cursor = cursor_fn();
        for child in xact.children(&mut cursor) {
            match child.unwrap() {
                XactFields::Date(date) => {
                    x.date = Some(substring(content, date.range()));
                }
                XactFields::Code(code) => {
                    x.code = Some(substring(content, code.range()).trim().to_string());
                }
                XactFields::EffectiveDate(effective_date) => {
                    x.effective_date = Some(substring(content, effective_date.range()));
                }
                XactFields::Note(note) => {
                    if note.range().start_point.row == x.row {
                        x.payee_note = Some(substring(content, note.range()));
                    } else if !x.postings.is_empty() {
                        // FIXME can we use in-place manipulation of the last() slice element
                        let mut posting = x.postings.pop().unwrap();
                        posting
                            .trailing_notes
                            .push(substring(content, note.range()));
                        x.postings.push(posting);
                    } else {
                        x.notes.push(substring(content, note.range()));
                    }
                }
                XactFields::Payee(payee) => {
                    x.payee = Some(substring(content, payee.range()).trim().to_string());
                }
                XactFields::Posting(posting) => {
                    x.postings
                        .push(Posting::from_ts_posting(posting, content, &cursor_fn));
                }
                XactFields::Status(status) => {
                    x.status = Some(substring(content, status.range()));
                }
            };
        }

        x
    }
}

impl Display for PlainXact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref date) = self.date {
            write!(f, "{date}")?;
        }
        if let Some(ref date) = self.effective_date {
            // includes leading =
            write!(f, "{date}")?;
        }
        if let Some(ref status) = self.status {
            write!(f, " {status}")?;
        }
        if let Some(ref code) = self.code {
            write!(f, " {code}")?;
        }
        if let Some(ref payee) = self.payee {
            write!(f, " {payee}")?;
        }
        if let Some(ref payee_note) = self.payee_note {
            write!(f, " {payee_note}")?;
        }
        writeln!(f, "")?;

        for note in self.notes.iter() {
            writeln!(f, "    {note}")?;
        }

        for posting in self.postings.iter() {
            write!(f, "{posting}")?;
        }

        Ok(())
    }
}

impl<'tree> PeriodicXact {
    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        xact: ledger::PeriodicXact<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Self {
        let mut x = PeriodicXact::default();
        x.row = xact.range().start_point.row;

        let mut cursor = cursor_fn();
        for child in xact.children(&mut cursor) {
            match child.unwrap() {
                PeriodicXactFields::Interval(interval) => {
                    x.interval = substring(content, interval.range());
                }
                PeriodicXactFields::Note(note) => {
                    if note.range().start_point.row == x.row {
                        x.interval_note = Some(substring(content, note.range()));
                    } else {
                        x.notes.push(substring(content, note.range()));
                    }
                }
                PeriodicXactFields::Posting(posting) => {
                    x.postings
                        .push(Posting::from_ts_posting(posting, content, &cursor_fn));
                }
            }
        }

        x
    }
}

impl Display for PeriodicXact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "~ {}", self.interval)?;
        if let Some(ref interval_note) = self.interval_note {
            write!(f, " {interval_note}")?;
        }
        writeln!(f, "")?;

        for note in self.notes.iter() {
            writeln!(f, "    {note}")?;
        }

        for posting in self.postings.iter() {
            write!(f, "{posting}")?;
        }
        Ok(())
    }
}

impl<'tree> AutomatedXact {
    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        xact: ledger::AutomatedXact<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Self {
        let mut x = AutomatedXact::default();
        x.row = xact.range().start_point.row;

        let mut cursor = cursor_fn();
        for child in xact.children(&mut cursor) {
            match child.unwrap() {
                AutomatedXactFields::Query(query) => {
                    x.query = substring(content, query.range()).trim().to_string();
                }
                AutomatedXactFields::Note(note) => {
                    if note.range().start_point.row == x.row {
                        x.query_note = Some(substring(content, note.range()));
                    } else {
                        x.notes.push(substring(content, note.range()));
                    }
                }
                AutomatedXactFields::Posting(posting) => {
                    x.postings
                        .push(Posting::from_ts_posting(posting, content, &cursor_fn));
                }
            }
        }

        x
    }
}

impl Display for AutomatedXact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "= {}", self.query)?;
        if let Some(ref query_note) = self.query_note {
            write!(f, " {query_note}")?;
        }
        writeln!(f, "")?;

        for note in self.notes.iter() {
            writeln!(f, "    {note}")?;
        }

        for posting in self.postings.iter() {
            write!(f, "{posting}")?;
        }
        Ok(())
    }
}

impl<'tree> Posting {
    fn from_ts_posting<'a, T: Fn() -> TreeCursor<'tree>>(
        posting: ledger::Posting<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Self {
        let mut p = Posting::default();

        let mut cursor = cursor_fn();
        for p_child in posting.children(&mut cursor) {
            match p_child.unwrap() {
                PostingFields::Account(account) => {
                    p.account = substring(content, account.range());
                }
                PostingFields::Amount(amount) => {
                    let mut cursor = cursor_fn();
                    for a_child in amount.children(&mut cursor) {
                        match a_child.unwrap() {
                            AmountFields::Commodity(commodity) => {
                                p.commodity = Some(substring(content, commodity.range()));
                                p.commodity_position = match p.quantity {
                                    None => CommodityPosition::Left,
                                    Some(_) => CommodityPosition::Right,
                                };
                            }
                            AmountFields::NegativeQuantity(quantity) => {
                                p.quantity = Some(substring(content, quantity.range()));
                            }
                            AmountFields::Quantity(quantity) => {
                                p.quantity = Some(substring(content, quantity.range()));
                            }
                        }
                    }
                }
                PostingFields::BalanceAssertion(ba) => {
                    p.balance_assertion = Some(substring(content, ba.range()));
                }
                PostingFields::Note(note) => {
                    p.inline_note = Some(substring(content, note.range()));
                }
                PostingFields::Price(_) => todo!(),
                PostingFields::Status(_) => todo!(),
            }
        }
        p
    }
}

impl Display for Posting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amount = match (&self.commodity, &self.quantity) {
            (Some(commodity), Some(quantity)) => match self.commodity_position {
                CommodityPosition::Left => format!("{commodity}{quantity}"),
                CommodityPosition::Right => format!("{quantity}{commodity}"),
            },
            (None, Some(quantity)) => quantity.clone(),
            (Some(_), None) | (None, None) => String::new(),
        };
        let amount = if let Some(ref assertion) = self.balance_assertion {
            format!("{amount} {assertion}")
        } else {
            amount
        };

        let amount_width = if amount.is_empty() {
            // no amount on this line => leave no trailing spaces at all
            0
        } else {
            // try to align to 48 chars, unless account name is too long
            let width = 48usize.checked_sub(self.account.len()).unwrap_or(0);
            width.max(2 + amount.len())
        };

        write!(
            f,
            "    {}{amount:>width$}",
            self.account,
            width = amount_width
        )?;
        if let Some(ref note) = self.inline_note {
            write!(f, "{}{note}", if amount.is_empty() { "  " } else { " " })?;
        }
        writeln!(f, "",)?;

        for note in self.trailing_notes.iter() {
            writeln!(f, "    {note}")?;
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq)]
struct SortableChunk<'a> {
    date: Option<String>,
    items: Vec<&'a JournalItem>,
    // items: Vec<&'a dyn Node<'a>>,
    // date: Option<&'a NaiveDate>,
    // items: Vec<&'a LedgerItem>,
}

impl<'a> SortableChunk<'a> {
    fn new() -> Self {
        Self {
            date: None,
            items: Vec::new(),
        }
    }
}

impl<'a> Ord for SortableChunk<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.date, &other.date) {
            (Some(self_date), Some(other_date)) => self_date.cmp(other_date),

            // sort transactions below other items
            (Some(_), _) => Ordering::Greater,
            (_, Some(_)) => Ordering::Less,

            // don't alter order of other items
            (_, _) => Ordering::Equal,
        }
    }
}

impl<'a> PartialOrd for SortableChunk<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn format_transaction() {
    let source = textwrap::dedent(
        "
        2018/10/01    (123)     Payee 123
          TEST:ABC 123        $1.20
           TEST:DEF 123
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        2018/10/01 (123) Payee 123
            TEST:ABC 123                               $1.20
            TEST:DEF 123
        "
    );
}

#[test]
fn format_transaction_with_looooong_account_name() {
    let source = textwrap::dedent(
        "
        2018/10/01       Payee 123
          TEST:LoremIpsumDolorSitAmetConsecteturAdipiscingElit    $1.20
           TEST:DEF 123
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        2018/10/01 Payee 123
            TEST:LoremIpsumDolorSitAmetConsecteturAdipiscingElit  $1.20
            TEST:DEF 123
        "
    );
}

#[test]
fn format_periodic_transaction() {
    let source = textwrap::dedent(
        r#"~   Monthly
  Account 1        $1.20
    Account 2"#,
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r#"
        ~ Monthly
            Account 1                                  $1.20
            Account 2
        "#
    );
}

#[test]
fn format_automated_transaction() {
    let source = textwrap::dedent(
        r#"=   Expenses:.*
            (Account:Foo)                 0.67
              (Account:Bar)  0.33
        "#,
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r#"
        = Expenses:.*
            (Account:Foo)                               0.67
            (Account:Bar)                               0.33
        "#
    );
}

#[test]
fn format_transaction_notes() {
    let source = textwrap::dedent(
        "
        2018/10/01 Payee    ; note 1
               ; note 2
              ; note 3
            TEST:ABC 123                               $1.20       ; note 4
          ; note 5
            TEST:DEF 123       ; note 6
             ; note 7
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        2018/10/01 Payee ; note 1
            ; note 2
            ; note 3
            TEST:ABC 123                               $1.20 ; note 4
            ; note 5
            TEST:DEF 123  ; note 6
            ; note 7
        "
    );
}

#[test]
fn format_payee_with_special_chars() {
    let source = textwrap::dedent(
        "
        2018/10/01  *  (123)     Payee* !123
          TEST:ABC 123        $1.20
           TEST:DEF 123
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        2018/10/01 * (123) Payee* !123
            TEST:ABC 123                               $1.20
            TEST:DEF 123
        "
    );
}

#[test]
fn format_effective_dates() {
    let source = textwrap::dedent(
        "
        2018/10/01=2011/02/03 Payee
          TEST:ABC 123        $1.20
           TEST:DEF 123
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        2018/10/01=2011/02/03 Payee
            TEST:ABC 123                               $1.20
            TEST:DEF 123
        "
    );
}

#[test]
fn format_balance_assertions() {
    let source = textwrap::dedent(
        "
        2018/10/01 Payee
          TEST:ABC 123        $1.20 = $123
           TEST:DEF 123  = $456
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        2018/10/01 Payee
            TEST:ABC 123                        $1.20 = $123
            TEST:DEF 123                              = $456
        "
    );
}

#[test]
fn format_grouping_journal_items() {
    let source = textwrap::dedent(
        "
        ; comment 1
        ; comment 2
        include one.ledger
        include two.ledger
        2018/10/01 Payee
            TEST:ABC 123                               $1.20
            TEST:DEF 123
        2018/11/22 Payee
            TEST:ABC 123                               $3.40
            TEST:DEF 123
        ",
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r"
        ; comment 1
        ; comment 2

        include one.ledger
        include two.ledger

        2018/10/01 Payee
            TEST:ABC 123                               $1.20
            TEST:DEF 123

        2018/11/22 Payee
            TEST:ABC 123                               $3.40
            TEST:DEF 123
        "
    );
}

#[test]
fn format_journal() {
    let source = textwrap::dedent(
        r#"
        ; comment 1
        include foo.ledger
        ; comment 3
        2023/12/21 Name
            Account1:Foo  -10
          Account2
        ; comment 2
        2023/12/22 Name
          Account1:Foo   -10
            Account2
        "#,
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r#"
        ; comment 1

        include foo.ledger

        ; comment 3

        2023/12/21 Name
            Account1:Foo                                 -10
            Account2

        ; comment 2

        2023/12/22 Name
            Account1:Foo                                 -10
            Account2
        "#
    );
}

// TODO fn serialize_with_custom_date_format() {

#[test]
fn format_sorted_transactions() {
    let source = textwrap::dedent(
        r#"
        ; first comment
        2018-01-02 Payee 1
          Account1  $1.23
          Account2
        ; xact 3 comment
        2018-01-03 Payee 3
          Account1  $1.23
          Account2
        ; foo comment
        include foo
        ; xact 2 comment
        2018-01-01 Payee 2
          Account3  $4.56
          Account4
        "#,
    );

    insta::assert_snapshot!(
        format(&source).unwrap(),
        @r#"
        ; foo comment

        include foo

        ; xact 2 comment

        2018-01-01 Payee 2
            Account3                                   $4.56
            Account4

        ; first comment

        2018-01-02 Payee 1
            Account1                                   $1.23
            Account2

        ; xact 3 comment

        2018-01-03 Payee 3
            Account1                                   $1.23
            Account2
        "#
    );
}