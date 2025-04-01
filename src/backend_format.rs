use anyhow::Result;
use anyhow::{anyhow, bail};
use ledger::anon_unions::AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective as Directives;
use ledger::anon_unions::Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status as PostingFields;
use ledger::anon_unions::AutomatedXact_PeriodicXact_PlainXact as Transactions;
use ledger::anon_unions::BlockComment_Comment_Directive_Test_Xact as JournalItems;
use ledger::anon_unions::Code_Date_EffectiveDate_Note_Payee_Posting_Status as XactFields;
use ledger::anon_unions::Commodity_NegativeQuantity_Quantity as AmountFields;
use ledger::anon_unions::Interval_Note_Posting as PeriodicXactFields;
use ledger::anon_unions::Note_Posting_Query as AutomatedXactFields;
use ledger::JournalItem as TS_JournalItem;
use type_sitter::{HasChild, HasChildren, HasOptionalChild, Node, Parser, Range, TreeCursor};

use std::cmp::Ordering;
use std::fmt::Display;
use std::io::Write;

mod ledger {
    #![allow(clippy::all, clippy::expect_used, clippy::unwrap_used)]
    include!("./type_sitter/ledger.rs");
}

pub fn format(content: &str, sort_transactions: bool) -> Result<String> {
    //
    // parse with tree sitter
    //
    let mut parser = Parser::<ledger::SourceFile>::new(&tree_sitter_ledger::LANGUAGE.into())
        .map_err(|_| anyhow!("loading Ledger tree-sitter grammar"))?;
    let tree = parser
        .parse(content, None)
        .map_err(|_| anyhow!("parsing content"))?;
    let root_node = tree.root_node().map_err(|err| anyhow!("{err}"))?;
    let mut raw_cursor = root_node.raw().walk();

    //
    // convert from tree sitter to internal types (easier to use)
    //
    let journal_items: Vec<JournalItem> = root_node
        .raw()
        .children(&mut raw_cursor)
        .map(|journal_item| {
            match TS_JournalItem::try_from_raw(journal_item) {
                Ok(journal_item) => {
                    let journal_item = match journal_item.child() {
                        Ok(journal_item) => journal_item,
                        Err(_) => {
                            return JournalItem::Error(substring(content, journal_item.range()))
                        }
                    };

                    if journal_item.has_error() {
                        // dbg!(substring(content, journal_item.range()));
                        // dbg!(journal_item.to_sexp());
                        return JournalItem::Error(substring(content, journal_item.range()));
                    }

                    match journal_item {
                        JournalItems::Comment(comment) => JournalItem::Comment(Comment {
                            range: comment.range(),
                            content: substring(content, comment.range()),
                        }),
                        JournalItems::Directive(directive) => {
                            match Directive::from_ts_xact(directive, content, || tree.walk()) {
                                Ok(directive) => JournalItem::Directive(directive),
                                Err(_) => JournalItem::Error(substring(content, directive.range())),
                            }
                        }
                        JournalItems::Xact(xact) => match xact.child() {
                            Ok(Transactions::AutomatedXact(xact)) => {
                                match AutomatedXact::from_ts_xact(xact, content, || tree.walk()) {
                                    Ok(xact) => JournalItem::AutomatedXact(xact),
                                    Err(_) => JournalItem::Error(substring(content, xact.range())),
                                }
                            }
                            Ok(Transactions::PeriodicXact(xact)) => {
                                match PeriodicXact::from_ts_xact(xact, content, || tree.walk()) {
                                    Ok(xact) => JournalItem::PeriodicXact(xact),
                                    Err(_) => JournalItem::Error(substring(content, xact.range())),
                                }
                            }
                            Ok(Transactions::PlainXact(xact)) => {
                                match PlainXact::from_ts_xact(xact, content, || tree.walk()) {
                                    Ok(xact) => JournalItem::PlainXact(xact),
                                    Err(_) => JournalItem::Error(substring(content, xact.range())),
                                }
                            }
                            Err(_) => JournalItem::Error(substring(content, xact.range())),
                        },
                        JournalItems::BlockComment(comment) => {
                            // TODO
                            JournalItem::Comment(Comment {
                                range: comment.range(),
                                content: substring(content, comment.range()),
                            })
                        }
                        JournalItems::Test(test) => {
                            // TODO
                            JournalItem::Other(substring(content, test.range()))
                        }
                    }
                }
                Err(err) => match err.cause() {
                    type_sitter::IncorrectKindCause::Error => {
                        // dbg!(substring(content, err.node.range()));
                        JournalItem::Error(substring(content, err.node.range()))
                    }
                    type_sitter::IncorrectKindCause::Missing
                    | type_sitter::IncorrectKindCause::OtherKind(_) => JournalItem::Skip,
                },
            }
        })
        .collect();

    let journal_items = if !sort_transactions {
        journal_items
    } else {
        //
        // sort, attempting to keep comments that are interspersed with transactions
        // together with those transactions, while pushing all other things to the
        // start of the journal
        //
        let mut chunks = Vec::new();
        let mut chunk = SortableChunk::new();
        for journal_item in journal_items.iter() {
            match journal_item {
                JournalItem::PlainXact(ref t) => {
                    chunk.items.push(journal_item);
                    chunk.date = t.date.clone();
                    chunks.push(chunk);
                    chunk = SortableChunk::new();
                }
                JournalItem::AutomatedXact(_)
                | JournalItem::PeriodicXact(_)
                | JournalItem::Directive(_) => {
                    chunk.items.push(journal_item);
                    chunks.push(chunk);
                    chunk = SortableChunk::new();
                }
                JournalItem::Comment(_)
                | JournalItem::Other(_)
                | JournalItem::Error(_)
                | JournalItem::Skip => {
                    chunk.items.push(journal_item);
                }
            }
        }
        // don't forget any trailing items
        if !chunk.items.is_empty() {
            chunks.push(chunk);
        }

        chunks.sort();

        chunks
            .iter()
            .flat_map(|chunk| chunk.items.clone())
            .cloned()
            .collect()
    };

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
            JournalItem::Comment(comment) => comment.content.clone(),
            JournalItem::Directive(directive) => directive.to_string(),
            JournalItem::Other(s) => s.clone(),
            JournalItem::Error(s) => s.clone(),
            JournalItem::Skip => continue,
        };

        // group similar items together into blocks, but separate all xacts w/ a
        // blank line
        match (previous_item, &journal_item) {
            // don't start w/ a newline, and don't split "other" items
            (None, _) | (Some(JournalItem::Other(_)), JournalItem::Other(_)) => {}

            // don't toucn errors
            (Some(_), JournalItem::Error(_)) | (Some(JournalItem::Error(_)), _) => {}

            // preserve gaps between blocks of comments
            (Some(JournalItem::Comment(prev_comment)), JournalItem::Comment(comment))
                if prev_comment.range.end_point.row != comment.range.start_point.row =>
            {
                writeln!(buf)?
            }

            // preserve gaps between blocks of directives, also group them by
            // directive type
            (Some(JournalItem::Directive(prev_directive)), JournalItem::Directive(directive))
                if prev_directive.range.end_point.row != directive.range.start_point.row
                    || prev_directive.name != directive.name =>
            {
                writeln!(buf)?
            }

            // preserve blocks of comments and directives
            (Some(JournalItem::Comment(_)), JournalItem::Comment(_)) => {}
            (Some(JournalItem::Directive(_)), JournalItem::Directive(_)) => {}

            // don't split comments immediately preceeding directives and xacts
            (Some(JournalItem::Comment(prev_comment)), JournalItem::Directive(directive))
                if prev_comment.range.end_point.row == directive.range.start_point.row => {}
            (Some(JournalItem::Comment(prev_comment)), JournalItem::PlainXact(xact))
                if prev_comment.range.end_point.row == xact.range.start_point.row => {}
            (Some(JournalItem::Comment(prev_comment)), JournalItem::PeriodicXact(xact))
                if prev_comment.range.end_point.row == xact.range.start_point.row => {}
            (Some(JournalItem::Comment(prev_comment)), JournalItem::AutomatedXact(xact))
                if prev_comment.range.end_point.row == xact.range.start_point.row => {}

            // separate transactions by a newline
            (_, JournalItem::PlainXact(_))
            | (_, JournalItem::PeriodicXact(_))
            | (_, JournalItem::AutomatedXact(_))
            | (Some(_), _) => {
                writeln!(buf)?;
            }
        }

        writeln!(buf, "{}", formatted_item.trim())?;

        previous_item = Some(journal_item);
    }

    Ok(String::from_utf8(buf)?)
}

fn substring(content: &str, range: Range) -> String {
    content[range.start_byte..range.end_byte].trim().to_string()
}

#[derive(Clone, Eq, PartialEq)]
enum JournalItem {
    PlainXact(PlainXact),
    PeriodicXact(PeriodicXact),
    AutomatedXact(AutomatedXact),
    Comment(Comment),
    Directive(Directive),
    Other(String),
    Error(String),
    Skip,
}

#[derive(Clone, Eq, PartialEq)]
struct Comment {
    range: Range,
    content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Directive {
    range: Range,
    name: String,
    content: String,
    subdirectives: Vec<Directive>,
    comments: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlainXact {
    range: Range,

    date: Option<String>,
    effective_date: Option<String>,
    status: Option<String>,
    code: Option<String>,
    payee: Option<String>,
    postings: Vec<Posting>,

    payee_note: Option<String>,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PeriodicXact {
    range: Range,

    interval: String,
    postings: Vec<Posting>,

    interval_note: Option<String>,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AutomatedXact {
    range: Range,

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

    status: Option<String>,

    amount: Option<Amount>,
    lot_price: Option<Amount>,
    price: Option<Price>,
    balance_assertion: Option<Amount>,

    inline_note: Option<String>,
    trailing_notes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Price {
    Unit(Amount),
    Total(Amount),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Amount {
    negative: bool,
    commodity_position: CommodityPosition,
    commodity: Option<String>,
    quantity: Option<String>,
}

impl<'tree> Directive {
    fn new(range: Range, directive_content: String) -> Self {
        let (name, content) = directive_content
            .trim()
            .split_once([' ', '\t'])
            .unwrap_or(("", ""));
        Self {
            range,
            name: name.to_string(),
            content: content.trim().to_string(),
            subdirectives: Vec::new(),
            comments: Vec::new(),
        }
    }

    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        directive: ledger::Directive<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Result<Self> {
        use ledger::anon_unions::Account_AccountSubdirective_Comment as AccountDirectiveNodes;

        let mut d;
        let mut cursor = cursor_fn();
        match directive.child().map_err(|err| anyhow!("{err}"))? {
            Directives::AccountDirective(directive) => {
                d = Directive::new(directive.range(), substring(content, directive.range()));

                for child in directive.children(&mut cursor) {
                    match child.map_err(|err| anyhow!("{err}"))? {
                        AccountDirectiveNodes::Account(account) => {
                            d.content = substring(content, account.range());
                        }
                        AccountDirectiveNodes::AccountSubdirective(subdirective) => {
                            match subdirective.child() {
                                Some(Ok(subdirective)) => {
                                    // FIXME consider normalizing/formatting subdirectives depending on thier type?
                                    d.subdirectives.push(Directive::new(
                                        subdirective.range(),
                                        substring(content, subdirective.range()),
                                    ));
                                }
                                Some(Err(err)) => bail!("{err}"),
                                None => {}
                            }
                        }
                        AccountDirectiveNodes::Comment(comment) => {
                            match d.subdirectives.as_mut_slice() {
                                [last_subdirective] | [.., last_subdirective] => {
                                    last_subdirective
                                        .comments
                                        .push(substring(content, comment.range()));
                                }
                                [] => d.comments.push(substring(content, comment.range())),
                            }
                        }
                    }
                }
            }

            Directives::WordDirective(directive) => {
                d = Directive::new(directive.range(), substring(content, directive.range()));

                // unlike other directives, word and char directives do not
                // include the trailing newline; bump the range to fake it so
                // that we can treat all directives the same for grouping
                d.range.end_byte += 1;
                d.range.end_point.row += 1;
                d.range.end_point.column = 0;
            }

            Directives::CharDirective(directive) => {
                d = Directive::new(directive.range(), substring(content, directive.range()));

                // unlike other directives, word and char directives do not
                // include the trailing newline; bump the range to fake it so
                // that we can treat all directives the same for grouping
                d.range.end_byte += 1;
                d.range.end_point.row += 1;
                d.range.end_point.column = 0;
            }

            Directives::CommodityDirective(directive) => {
                d = Directive::new(directive.range(), substring(content, directive.range()));
            }

            Directives::Option(directive) => {
                d = Directive::new(directive.range(), substring(content, directive.range()));
            }

            Directives::TagDirective(directive) => {
                d = Directive::new(directive.range(), substring(content, directive.range()));
            }
        }

        Ok(d)
    }
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.name, self.content)?;

        for comment in self.comments.iter() {
            writeln!(f, "    {comment}")?;
        }

        for subdirective in self.subdirectives.iter() {
            write!(f, "    {subdirective}")?;
        }

        Ok(())
    }
}

impl<'tree> PlainXact {
    fn new(range: Range) -> Self {
        Self {
            range,
            date: None,
            effective_date: None,
            status: None,
            code: None,
            payee: None,
            postings: Vec::new(),
            payee_note: None,
            notes: Vec::new(),
        }
    }

    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        xact: ledger::PlainXact<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Result<Self> {
        let mut x = PlainXact::new(xact.range());

        let mut cursor = cursor_fn();
        for child in xact.children(&mut cursor) {
            match child.map_err(|err| anyhow!("{err}"))? {
                XactFields::Date(date) => {
                    let normalized_date = substring(content, date.range()).replace(['.', '-'], "/");
                    x.date = Some(normalized_date);
                }
                XactFields::Code(code) => {
                    x.code = Some(substring(content, code.range()).trim().to_string());
                }
                XactFields::EffectiveDate(effective_date) => {
                    x.effective_date = Some(substring(content, effective_date.range()));
                }
                XactFields::Note(note) => {
                    if note.range().start_point.row == x.range.start_point.row {
                        x.payee_note = Some(substring(content, note.range()));
                    } else {
                        match x.postings.as_mut_slice() {
                            [last_posting] | [.., last_posting] => last_posting
                                .trailing_notes
                                .push(substring(content, note.range())),
                            [] => x.notes.push(substring(content, note.range())),
                        }
                    }
                }
                XactFields::Payee(payee) => {
                    x.payee = Some(substring(content, payee.range()).trim().to_string());
                }
                XactFields::Posting(posting) => {
                    x.postings
                        .push(Posting::from_ts_posting(posting, content, &cursor_fn)?);
                }
                XactFields::Status(status) => {
                    x.status = Some(substring(content, status.range()));
                }
            };
        }

        Ok(x)
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
        writeln!(f)?;

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
    fn new(range: Range) -> Self {
        Self {
            range,
            interval: String::new(),
            postings: Vec::new(),
            interval_note: None,
            notes: Vec::new(),
        }
    }

    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        xact: ledger::PeriodicXact<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Result<Self> {
        let mut x = PeriodicXact::new(xact.range());

        let mut cursor = cursor_fn();
        for child in xact.children(&mut cursor) {
            match child.map_err(|err| anyhow!("{err}"))? {
                PeriodicXactFields::Interval(interval) => {
                    x.interval = substring(content, interval.range());
                }
                PeriodicXactFields::Note(note) => {
                    if note.range().start_point.row == x.range.start_point.row {
                        x.interval_note = Some(substring(content, note.range()));
                    } else {
                        x.notes.push(substring(content, note.range()));
                    }
                }
                PeriodicXactFields::Posting(posting) => {
                    x.postings
                        .push(Posting::from_ts_posting(posting, content, &cursor_fn)?);
                }
            }
        }

        Ok(x)
    }
}

impl Display for PeriodicXact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "~ {}", self.interval)?;
        if let Some(ref interval_note) = self.interval_note {
            write!(f, " {interval_note}")?;
        }
        writeln!(f)?;

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
    fn new(range: Range) -> Self {
        Self {
            range,
            query: String::new(),
            postings: Vec::new(),
            query_note: None,
            notes: Vec::new(),
        }
    }

    fn from_ts_xact<'a, T: Fn() -> TreeCursor<'tree>>(
        xact: ledger::AutomatedXact<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Result<Self> {
        let mut x = AutomatedXact::new(xact.range());

        let mut cursor = cursor_fn();
        for child in xact.children(&mut cursor) {
            match child.map_err(|err| anyhow!("{err}"))? {
                AutomatedXactFields::Query(query) => {
                    x.query = substring(content, query.range()).trim().to_string();
                }
                AutomatedXactFields::Note(note) => {
                    if note.range().start_point.row == x.range.start_point.row {
                        x.query_note = Some(substring(content, note.range()));
                    } else {
                        x.notes.push(substring(content, note.range()));
                    }
                }
                AutomatedXactFields::Posting(posting) => {
                    x.postings
                        .push(Posting::from_ts_posting(posting, content, &cursor_fn)?);
                }
            }
        }

        Ok(x)
    }
}

impl Display for AutomatedXact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "= {}", self.query)?;
        if let Some(ref query_note) = self.query_note {
            write!(f, " {query_note}")?;
        }
        writeln!(f)?;

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
    ) -> Result<Self> {
        let mut p = Posting::default();

        let mut cursor = cursor_fn();
        for p_child in posting.children(&mut cursor) {
            match p_child.map_err(|err| anyhow!("{err}"))? {
                PostingFields::Account(account) => {
                    p.account = substring(content, account.range());
                }
                PostingFields::Amount(amount) => {
                    p.amount = Some(Amount::from_ts(amount, content, &cursor_fn)?);
                }
                PostingFields::BalanceAssertion(ba) => {
                    p.balance_assertion = Some(Amount::from_ts(
                        ba.amount().map_err(|err| anyhow!("{err}"))?,
                        content,
                        &cursor_fn,
                    )?);
                }
                PostingFields::Note(note) => {
                    p.inline_note = Some(substring(content, note.range()));
                }
                PostingFields::LotPrice(lot_price) => {
                    p.lot_price = Some(Amount::from_ts(
                        lot_price.amount().map_err(|err| anyhow!("{err}"))?,
                        content,
                        &cursor_fn,
                    )?)
                }
                PostingFields::Price(price) => {
                    let amount = Amount::from_ts(
                        price.amount().map_err(|err| anyhow!("{err}"))?,
                        content,
                        &cursor_fn,
                    )?;

                    p.price = if substring(content, price.range()).starts_with("@@") {
                        Some(Price::Total(amount))
                    } else {
                        Some(Price::Unit(amount))
                    };
                }
                PostingFields::Status(status) => {
                    p.status = Some(substring(content, status.range()));
                }
            }
        }

        Ok(p)
    }
}

impl Display for Posting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut amount = match &self.amount {
            Some(amount) => format!("{amount}"),
            None => String::new(),
        };
        if let Some(ref lot_price) = self.lot_price {
            amount.push_str(format!(" {{{lot_price}}}").as_str());
        };
        if let Some(ref price) = self.price {
            amount.push_str(format!(" {price}").as_str());
        };
        if let Some(ref assertion) = self.balance_assertion {
            amount.push_str(format!(" = {assertion}").as_str());
        };

        let status = self.status.clone().map_or("".to_string(), |s| s + " ");

        let amount_width = if amount.is_empty() {
            // no amount on this line => leave no trailing spaces at all
            0
        } else {
            // try to align to 48 chars, unless account name is too long
            let width = 48usize.saturating_sub(self.account.len() + status.len());
            width.max(2 + amount.len())
        };

        write!(
            f,
            "    {}{}{amount:>width$}",
            status,
            self.account,
            width = amount_width
        )?;
        if let Some(ref note) = self.inline_note {
            write!(f, "{}{note}", if amount.is_empty() { "  " } else { " " })?;
        }
        writeln!(f)?;

        for note in self.trailing_notes.iter() {
            writeln!(f, "    {note}")?;
        }

        Ok(())
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Price::Unit(amount) => write!(f, "@ {amount}")?,
            Price::Total(amount) => write!(f, "@@ {amount}")?,
        }

        Ok(())
    }
}

impl<'tree> Amount {
    fn from_ts<'a, T: Fn() -> TreeCursor<'tree>>(
        amount: ledger::Amount<'tree>,
        content: &str,
        cursor_fn: T,
    ) -> Result<Self> {
        let mut a = Amount::default();

        let mut cursor = cursor_fn();
        for a_child in amount.children(&mut cursor) {
            match a_child {
                Ok(AmountFields::Commodity(commodity)) => {
                    a.commodity = Some(substring(content, commodity.range()));
                    a.commodity_position = match a.quantity {
                        None => CommodityPosition::Left,
                        Some(_) => CommodityPosition::Right,
                    };
                }
                Ok(AmountFields::NegativeQuantity(quantity)) => {
                    a.quantity = Some(substring(content, quantity.range()));
                    a.negative = true;
                }
                Ok(AmountFields::Quantity(quantity)) => {
                    a.quantity = Some(substring(content, quantity.range()));
                }
                Err(err) => bail!("{err}"),
            }
        }

        Ok(a)
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let negative = if self.negative { "-" } else { "" };

        match (&self.commodity, &self.quantity) {
            (Some(commodity), Some(quantity)) => match self.commodity_position {
                CommodityPosition::Left => write!(f, "{commodity}{negative}{quantity}")?,
                CommodityPosition::Right => write!(f, "{negative}{quantity}{commodity}")?,
            },
            (None, Some(quantity)) => write!(f, "{negative}{quantity}")?,
            (Some(_), None) | (None, None) => {}
        };

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

impl SortableChunk<'_> {
    fn new() -> Self {
        Self {
            date: None,
            items: Vec::new(),
        }
    }
}

impl Ord for SortableChunk<'_> {
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

impl PartialOrd for SortableChunk<'_> {
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
            !   TEST:DEF 123  $2.30
           TEST:GHI
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        2018/10/01 (123) Payee 123
            TEST:ABC 123                               $1.20
            ! TEST:DEF 123                             $2.30
            TEST:GHI
        "
    );
}

#[test]
fn format_amounts() {
    let source = textwrap::dedent(
        "
        2018/10/01 Payee 123
          ABC  $1.20
          DEF  -$2.30
          GHI  $-4.50
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        2018/10/01 Payee 123
            ABC                                        $1.20
            DEF                                       $-2.30
            GHI                                       $-4.50
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
        format(&source, false).unwrap(),
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
    Account 2

        "#,
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
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
        format(&source, false).unwrap(),
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
        format(&source, false).unwrap(),
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
        format(&source, false).unwrap(),
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
        format(&source, false).unwrap(),
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
        format(&source, false).unwrap(),
        @r"
        2018/10/01 Payee
            TEST:ABC 123                        $1.20 = $123
            TEST:DEF 123                              = $456
        "
    );
}

#[test]
fn format_prices() {
    let source = textwrap::dedent(
        "
        2023/11/21
            Produce:Sweet Potatoes          -80  {$2.40}    @@    $192
            Assets:Accounts Recievable
        2023/11/21
            Produce:Peppers          -80 lb @ $2.40
            Assets:Accounts Recievable
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        2023/11/21
            Produce:Sweet Potatoes       -80 {$2.40} @@ $192
            Assets:Accounts Recievable

        2023/11/21
            Produce:Peppers                    -80lb @ $2.40
            Assets:Accounts Recievable
        "
    );
}

#[test]
fn format_directives() {
    let source = textwrap::dedent(
        "
        include   foo.ledger
        account  Foo
          ; comment 1
          alias  Bar
          ; comment 2
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        include foo.ledger

        account Foo
            ; comment 1
            alias Bar
            ; comment 2
        "
    );
}

#[test]
fn format_directives_grouping() {
    let source = textwrap::dedent(
        "
        account Uncategorized

        tag Memo
        tag memo
        commodity $

        account Equity:Opening Balances

        account Assets:Farm:Accounts Recievable
            alias Assets:Accounts Recievable
        account Assets:Buildings:Barn
            alias Assets:Farm Buildings:Barn
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        account Uncategorized

        tag Memo
        tag memo

        commodity $

        account Equity:Opening Balances

        account Assets:Farm:Accounts Recievable
            alias Assets:Accounts Recievable
        account Assets:Buildings:Barn
            alias Assets:Farm Buildings:Barn
        "
    );
}

#[test]
fn format_grouping_journal_items() {
    let source = textwrap::dedent(
        "
        ; comment 1
        ; comment 2

        ; comment 3
        include one.ledger

        include two.ledger
        include three.ledger
        ; comment 4
        2018/10/01 Payee
            TEST:ABC 123                               $1.20
            TEST:DEF 123
        ; comment 5
        2018/11/22 Payee
            TEST:ABC 123                               $3.40
            TEST:DEF 123
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        ; comment 1
        ; comment 2

        ; comment 3
        include one.ledger

        include two.ledger
        include three.ledger

        ; comment 4
        2018/10/01 Payee
            TEST:ABC 123                               $1.20
            TEST:DEF 123

        ; comment 5
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
        format(&source, false).unwrap(),
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

#[test]
fn format_normalize_dates() {
    let source = textwrap::dedent(
        "
        2018.10.01 Payee
            Account
        2018-10-02 Payee
            Account
        2018/10-03 Payee
            Account
        2018.10/04 Payee
            Account
        ",
    );

    insta::assert_snapshot!(
        format(&source, false).unwrap(),
        @r"
        2018/10/01 Payee
            Account

        2018/10/02 Payee
            Account

        2018/10/03 Payee
            Account

        2018/10/04 Payee
            Account
        "
    );
}

#[test]
fn format_sorted_transactions() {
    let source = textwrap::dedent(
        r#"
        ; first comment
        2018/01/02 Payee 1
          Account1  $1.23
          Account2
        ; xact 3 comment
        2018/01/03 Payee 3
          Account1  $1.23
          Account2
        ; foo comment

        ; bar comment
        include foo
        ; xact 2 comment
        2018/01/01 Payee 2
          Account3  $4.56
          Account4
        "#,
    );

    insta::assert_snapshot!(
        format(&source, true).unwrap(),
        @r#"
        ; foo comment

        ; bar comment
        include foo

        ; xact 2 comment
        2018/01/01 Payee 2
            Account3                                   $4.56
            Account4

        ; first comment
        2018/01/02 Payee 1
            Account1                                   $1.23
            Account2

        ; xact 3 comment
        2018/01/03 Payee 3
            Account1                                   $1.23
            Account2
        "#
    );
}

#[test]
fn format_error_nodes() {
    let source = textwrap::dedent(
        r#"
        invalid   directive
        include foo.ledger
        22/2/2 Payee
          Account  12/3/4 ; invalid qty
        11/1/1 Payee
          Account  $1
        111 Payee
          Does Not Compute  $1
        "#,
    );

    insta::assert_snapshot!(
        format(&source, true).unwrap(),
        @r#"
        invalid   directive
        include foo.ledger
        111 Payee
          Does Not Compute  $1
        22/2/2 Payee
          Account  12/3/4 ; invalid qty
        11/1/1 Payee
            Account                                       $1
        "#
    );
}
