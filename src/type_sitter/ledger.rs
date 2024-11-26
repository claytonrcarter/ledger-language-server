#[doc = "Typed node `account`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Account<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Account<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Account<'tree> {
    type WithLifetime<'a> = Account<'a>;
    const KIND: &'static str = "account";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "account" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "account");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `account_directive`\n\nThis node has named children of type `{account | account_subdirective | comment}+`:\n\n- [`Account`]\n- [`AccountSubdirective`]\n- [`Comment`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AccountDirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AccountDirective<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{account | account_subdirective | comment}+`:\n\n- [`Account`]\n- [`AccountSubdirective`]\n- [`Comment`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Account_AccountSubdirective_Comment<'tree>,
        >,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(
                <anon_unions::Account_AccountSubdirective_Comment<'tree> as ::type_sitter::Node<
                    'tree,
                >>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AccountDirective<'tree> {
    type WithLifetime<'a> = AccountDirective<'a>;
    const KIND: &'static str = "account_directive";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "account_directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "account_directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `account_subdirective`\n\nThis node has an optional named child of type `{alias_subdirective | assert_subdirective | check_subdirective | default_subdirective | note_subdirective}?`:\n\n- [`AliasSubdirective`]\n- [`AssertSubdirective`]\n- [`CheckSubdirective`]\n- [`DefaultSubdirective`]\n- [`NoteSubdirective`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AccountSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AccountSubdirective<'tree> {
    #[doc = "Get the node's only not-extra named child, if it has one.\n\nThis child has type `{alias_subdirective | assert_subdirective | check_subdirective | default_subdirective | note_subdirective}?`:\n\n- [`AliasSubdirective`]\n- [`AssertSubdirective`]\n- [`CheckSubdirective`]\n- [`DefaultSubdirective`]\n- [`NoteSubdirective`]\n"]
    #[inline]    pub fn child (& self) -> :: std :: option :: Option < :: type_sitter :: NodeResult < 'tree , anon_unions :: AliasSubdirective_AssertSubdirective_CheckSubdirective_DefaultSubdirective_NoteSubdirective < 'tree > > >{
        (0 .. :: type_sitter :: Node :: raw (self) . named_child_count ()) . map (| i | :: type_sitter :: Node :: raw (self) . named_child (i) . unwrap ()) . filter (| n | ! n . is_extra ()) . next () . map (< anon_unions :: AliasSubdirective_AssertSubdirective_CheckSubdirective_DefaultSubdirective_NoteSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AccountSubdirective<'tree> {
    type WithLifetime<'a> = AccountSubdirective<'a>;
    const KIND: &'static str = "account_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "account_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "account_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `alias_subdirective`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AliasSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AliasSubdirective<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AliasSubdirective<'tree> {
    type WithLifetime<'a> = AliasSubdirective<'a>;
    const KIND: &'static str = "alias_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "alias_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "alias_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `amount`\n\nThis node has named children of type `{commodity | negative_quantity | quantity}+`:\n\n- [`Commodity`]\n- [`NegativeQuantity`]\n- [`Quantity`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Amount<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Amount<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{commodity | negative_quantity | quantity}+`:\n\n- [`Commodity`]\n- [`NegativeQuantity`]\n- [`Quantity`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Commodity_NegativeQuantity_Quantity<'tree>,
        >,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(
                <anon_unions::Commodity_NegativeQuantity_Quantity<'tree> as ::type_sitter::Node<
                    'tree,
                >>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Amount<'tree> {
    type WithLifetime<'a> = Amount<'a>;
    const KIND: &'static str = "amount";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "amount" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "amount");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `assert_subdirective`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AssertSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AssertSubdirective<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AssertSubdirective<'tree> {
    type WithLifetime<'a> = AssertSubdirective<'a>;
    const KIND: &'static str = "assert_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "assert_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "assert_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `automated_xact`\n\nThis node has named children of type `{note | posting | query}+`:\n\n- [`Note`]\n- [`Posting`]\n- [`Query`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AutomatedXact<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AutomatedXact<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{note | posting | query}+`:\n\n- [`Note`]\n- [`Posting`]\n- [`Query`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Note_Posting_Query<'tree>>,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Note_Posting_Query < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AutomatedXact<'tree> {
    type WithLifetime<'a> = AutomatedXact<'a>;
    const KIND: &'static str = "automated_xact";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "automated_xact" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "automated_xact");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `balance_assertion`\n\nThis node has a named child of type `amount` ([`Amount`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BalanceAssertion<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BalanceAssertion<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `amount` ([`Amount`])"]
    #[inline]
    pub fn amount(&self) -> ::type_sitter::NodeResult<'tree, Amount<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Amount<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BalanceAssertion<'tree> {
    type WithLifetime<'a> = BalanceAssertion<'a>;
    const KIND: &'static str = "balance_assertion";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "balance_assertion" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "balance_assertion");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `block_comment`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BlockComment<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BlockComment<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BlockComment<'tree> {
    type WithLifetime<'a> = BlockComment<'a>;
    const KIND: &'static str = "block_comment";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "block_comment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "block_comment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `char_directive`\n\nThis node has named children of type `{account | amount | check_in | check_out | commodity | date}*`:\n\n- [`Account`]\n- [`Amount`]\n- [`CheckIn`]\n- [`CheckOut`]\n- [`Commodity`]\n- [`Date`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CharDirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CharDirective<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{account | amount | check_in | check_out | commodity | date}*`:\n\n- [`Account`]\n- [`Amount`]\n- [`CheckIn`]\n- [`CheckOut`]\n- [`Commodity`]\n- [`Date`]\n"]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Account_Amount_CheckIn_CheckOut_Commodity_Date<'tree>,
        >,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Account_Amount_CheckIn_CheckOut_Commodity_Date < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CharDirective<'tree> {
    type WithLifetime<'a> = CharDirective<'a>;
    const KIND: &'static str = "char_directive";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "char_directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "char_directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `check_in`\n\nThis node has named children of type `{account | date | payee | time}+`:\n\n- [`Account`]\n- [`Date`]\n- [`Payee`]\n- [`Time`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CheckIn<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CheckIn<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{account | date | payee | time}+`:\n\n- [`Account`]\n- [`Date`]\n- [`Payee`]\n- [`Time`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Account_Date_Payee_Time<'tree>>,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Account_Date_Payee_Time < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CheckIn<'tree> {
    type WithLifetime<'a> = CheckIn<'a>;
    const KIND: &'static str = "check_in";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "check_in" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "check_in");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `check_out`\n\nThis node has named children of type `{date | time}+`:\n\n- [`Date`]\n- [`Time`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CheckOut<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CheckOut<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{date | time}+`:\n\n- [`Date`]\n- [`Time`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Date_Time<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<anon_unions::Date_Time<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CheckOut<'tree> {
    type WithLifetime<'a> = CheckOut<'a>;
    const KIND: &'static str = "check_out";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "check_out" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "check_out");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `check_subdirective`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CheckSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CheckSubdirective<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CheckSubdirective<'tree> {
    type WithLifetime<'a> = CheckSubdirective<'a>;
    const KIND: &'static str = "check_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "check_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "check_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `code`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Code<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Code<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Code<'tree> {
    type WithLifetime<'a> = Code<'a>;
    const KIND: &'static str = "code";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "code" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "code");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `comment`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Comment<'tree> {
    type WithLifetime<'a> = Comment<'a>;
    const KIND: &'static str = "comment";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `commodity`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Commodity<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Commodity<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Commodity<'tree> {
    type WithLifetime<'a> = Commodity<'a>;
    const KIND: &'static str = "commodity";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "commodity" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "commodity");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `commodity_directive`\n\nThis node has named children of type `{comment | commodity | commodity_subdirective}+`:\n\n- [`Comment`]\n- [`Commodity`]\n- [`CommoditySubdirective`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CommodityDirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CommodityDirective<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{comment | commodity | commodity_subdirective}+`:\n\n- [`Comment`]\n- [`Commodity`]\n- [`CommoditySubdirective`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Comment_Commodity_CommoditySubdirective<'tree>,
        >,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Comment_Commodity_CommoditySubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CommodityDirective<'tree> {
    type WithLifetime<'a> = CommodityDirective<'a>;
    const KIND: &'static str = "commodity_directive";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "commodity_directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "commodity_directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `commodity_subdirective`\n\nThis node has an optional named child of type `{alias_subdirective | default_subdirective | format_subdirective | note_subdirective}?`:\n\n- [`AliasSubdirective`]\n- [`DefaultSubdirective`]\n- [`FormatSubdirective`]\n- [`NoteSubdirective`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CommoditySubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CommoditySubdirective<'tree> {
    #[doc = "Get the node's only not-extra named child, if it has one.\n\nThis child has type `{alias_subdirective | default_subdirective | format_subdirective | note_subdirective}?`:\n\n- [`AliasSubdirective`]\n- [`DefaultSubdirective`]\n- [`FormatSubdirective`]\n- [`NoteSubdirective`]\n"]
    #[inline]
    pub fn child(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<
            'tree,
            anon_unions::AliasSubdirective_DefaultSubdirective_FormatSubdirective_NoteSubdirective<
                'tree,
            >,
        >,
    > {
        (0 .. :: type_sitter :: Node :: raw (self) . named_child_count ()) . map (| i | :: type_sitter :: Node :: raw (self) . named_child (i) . unwrap ()) . filter (| n | ! n . is_extra ()) . next () . map (< anon_unions :: AliasSubdirective_DefaultSubdirective_FormatSubdirective_NoteSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CommoditySubdirective<'tree> {
    type WithLifetime<'a> = CommoditySubdirective<'a>;
    const KIND: &'static str = "commodity_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "commodity_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "commodity_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `date`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Date<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Date<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Date<'tree> {
    type WithLifetime<'a> = Date<'a>;
    const KIND: &'static str = "date";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "date" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "date");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `date_spec`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DateSpec<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DateSpec<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DateSpec<'tree> {
    type WithLifetime<'a> = DateSpec<'a>;
    const KIND: &'static str = "date_spec";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "date_spec" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "date_spec");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `default_subdirective`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DefaultSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DefaultSubdirective<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DefaultSubdirective<'tree> {
    type WithLifetime<'a> = DefaultSubdirective<'a>;
    const KIND: &'static str = "default_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "default_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "default_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `directive`\n\nThis node has a named child of type `{account_directive | char_directive | commodity_directive | option | tag_directive | word_directive}`:\n\n- [`AccountDirective`]\n- [`CharDirective`]\n- [`CommodityDirective`]\n- [`Option`]\n- [`TagDirective`]\n- [`WordDirective`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Directive<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Directive<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `{account_directive | char_directive | commodity_directive | option | tag_directive | word_directive}`:\n\n- [`AccountDirective`]\n- [`CharDirective`]\n- [`CommodityDirective`]\n- [`Option`]\n- [`TagDirective`]\n- [`WordDirective`]\n"]
    #[inline]    pub fn child (& self) -> :: type_sitter :: NodeResult < 'tree , anon_unions :: AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective < 'tree > >{
        (0 .. :: type_sitter :: Node :: raw (self) . named_child_count ()) . map (| i | :: type_sitter :: Node :: raw (self) . named_child (i) . unwrap ()) . filter (| n | ! n . is_extra ()) . next () . map (< anon_unions :: AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw) . expect ("required child not present, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Directive<'tree> {
    type WithLifetime<'a> = Directive<'a>;
    const KIND: &'static str = "directive";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `effective_date`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EffectiveDate<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> EffectiveDate<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EffectiveDate<'tree> {
    type WithLifetime<'a> = EffectiveDate<'a>;
    const KIND: &'static str = "effective_date";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "effective_date" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "effective_date");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `filename`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Filename<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Filename<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Filename<'tree> {
    type WithLifetime<'a> = Filename<'a>;
    const KIND: &'static str = "filename";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "filename" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "filename");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `format_subdirective`\n\nThis node has a named child of type `amount` ([`Amount`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FormatSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FormatSubdirective<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `amount` ([`Amount`])"]
    #[inline]
    pub fn amount(&self) -> ::type_sitter::NodeResult<'tree, Amount<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Amount<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for FormatSubdirective<'tree> {
    type WithLifetime<'a> = FormatSubdirective<'a>;
    const KIND: &'static str = "format_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "format_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "format_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `interval`\n\nThis node has named children of type `date_spec*` ([`DateSpec`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Interval<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Interval<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `date_spec*` ([`DateSpec`])"]
    #[inline]
    pub fn date_specs<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, DateSpec<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<DateSpec<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Interval<'tree> {
    type WithLifetime<'a> = Interval<'a>;
    const KIND: &'static str = "interval";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "interval" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "interval");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `journal_item`\n\nThis node has a named child of type `{block_comment | comment | directive | test | xact}`:\n\n- [`BlockComment`]\n- [`Comment`]\n- [`Directive`]\n- [`Test`]\n- [`Xact`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct JournalItem<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> JournalItem<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `{block_comment | comment | directive | test | xact}`:\n\n- [`BlockComment`]\n- [`Comment`]\n- [`Directive`]\n- [`Test`]\n- [`Xact`]\n"]
    #[inline]
    pub fn child(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::BlockComment_Comment_Directive_Test_Xact<'tree>,
    > {
        (0 .. :: type_sitter :: Node :: raw (self) . named_child_count ()) . map (| i | :: type_sitter :: Node :: raw (self) . named_child (i) . unwrap ()) . filter (| n | ! n . is_extra ()) . next () . map (< anon_unions :: BlockComment_Comment_Directive_Test_Xact < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw) . expect ("required child not present, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for JournalItem<'tree> {
    type WithLifetime<'a> = JournalItem<'a>;
    const KIND: &'static str = "journal_item";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "journal_item" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "journal_item");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `lot_price`\n\nThis node has a named child of type `amount` ([`Amount`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct LotPrice<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> LotPrice<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `amount` ([`Amount`])"]
    #[inline]
    pub fn amount(&self) -> ::type_sitter::NodeResult<'tree, Amount<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Amount<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for LotPrice<'tree> {
    type WithLifetime<'a> = LotPrice<'a>;
    const KIND: &'static str = "lot_price";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "lot_price" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "lot_price");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `negative_quantity`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NegativeQuantity<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NegativeQuantity<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NegativeQuantity<'tree> {
    type WithLifetime<'a> = NegativeQuantity<'a>;
    const KIND: &'static str = "negative_quantity";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "negative_quantity" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "negative_quantity");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `note`\n\nThis node has an optional named child of type `effective_date?` ([`EffectiveDate`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Note<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Note<'tree> {
    #[doc = "Get the node's only not-extra named child, if it has one.\n\nThis child has type `effective_date?` ([`EffectiveDate`])"]
    #[inline]
    pub fn effective_date(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, EffectiveDate<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<EffectiveDate<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Note<'tree> {
    type WithLifetime<'a> = Note<'a>;
    const KIND: &'static str = "note";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "note" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "note");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `note_subdirective`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NoteSubdirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NoteSubdirective<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NoteSubdirective<'tree> {
    type WithLifetime<'a> = NoteSubdirective<'a>;
    const KIND: &'static str = "note_subdirective";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "note_subdirective" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "note_subdirective");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `option`\n\nThis node has an optional named child of type `option_value?` ([`OptionValue`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Option<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Option<'tree> {
    #[doc = "Get the node's only not-extra named child, if it has one.\n\nThis child has type `option_value?` ([`OptionValue`])"]
    #[inline]
    pub fn option_value(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, OptionValue<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<OptionValue<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Option<'tree> {
    type WithLifetime<'a> = Option<'a>;
    const KIND: &'static str = "option";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "option" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "option");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `option_value`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct OptionValue<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> OptionValue<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for OptionValue<'tree> {
    type WithLifetime<'a> = OptionValue<'a>;
    const KIND: &'static str = "option_value";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "option_value" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "option_value");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `payee`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Payee<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Payee<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Payee<'tree> {
    type WithLifetime<'a> = Payee<'a>;
    const KIND: &'static str = "payee";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "payee" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "payee");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `periodic_xact`\n\nThis node has named children of type `{interval | note | posting}+`:\n\n- [`Interval`]\n- [`Note`]\n- [`Posting`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PeriodicXact<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PeriodicXact<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{interval | note | posting}+`:\n\n- [`Interval`]\n- [`Note`]\n- [`Posting`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Interval_Note_Posting<'tree>>,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Interval_Note_Posting < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PeriodicXact<'tree> {
    type WithLifetime<'a> = PeriodicXact<'a>;
    const KIND: &'static str = "periodic_xact";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "periodic_xact" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "periodic_xact");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `plain_xact`\n\nThis node has named children of type `{code | date | effective_date | note | payee | posting | status}+`:\n\n- [`Code`]\n- [`Date`]\n- [`EffectiveDate`]\n- [`Note`]\n- [`Payee`]\n- [`Posting`]\n- [`Status`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PlainXact<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PlainXact<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{code | date | effective_date | note | payee | posting | status}+`:\n\n- [`Code`]\n- [`Date`]\n- [`EffectiveDate`]\n- [`Note`]\n- [`Payee`]\n- [`Posting`]\n- [`Status`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Code_Date_EffectiveDate_Note_Payee_Posting_Status<'tree>,
        >,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Code_Date_EffectiveDate_Note_Payee_Posting_Status < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PlainXact<'tree> {
    type WithLifetime<'a> = PlainXact<'a>;
    const KIND: &'static str = "plain_xact";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "plain_xact" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "plain_xact");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `posting`\n\nThis node has named children of type `{account | amount | balance_assertion | lot_price | note | price | status}+`:\n\n- [`Account`]\n- [`Amount`]\n- [`BalanceAssertion`]\n- [`LotPrice`]\n- [`Note`]\n- [`Price`]\n- [`Status`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Posting<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Posting<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{account | amount | balance_assertion | lot_price | note | price | status}+`:\n\n- [`Account`]\n- [`Amount`]\n- [`BalanceAssertion`]\n- [`LotPrice`]\n- [`Note`]\n- [`Price`]\n- [`Status`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status<'tree>,
        >,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Posting<'tree> {
    type WithLifetime<'a> = Posting<'a>;
    const KIND: &'static str = "posting";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "posting" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "posting");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `price`\n\nThis node has a named child of type `amount` ([`Amount`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Price<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Price<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `amount` ([`Amount`])"]
    #[inline]
    pub fn amount(&self) -> ::type_sitter::NodeResult<'tree, Amount<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Amount<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Price<'tree> {
    type WithLifetime<'a> = Price<'a>;
    const KIND: &'static str = "price";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "price" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "price");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `quantity`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Quantity<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Quantity<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Quantity<'tree> {
    type WithLifetime<'a> = Quantity<'a>;
    const KIND: &'static str = "quantity";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "quantity" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "quantity");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `query`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Query<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Query<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Query<'tree> {
    type WithLifetime<'a> = Query<'a>;
    const KIND: &'static str = "query";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "query" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "query");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `source_file`\n\nThis node has named children of type `journal_item*` ([`JournalItem`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SourceFile<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SourceFile<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `journal_item*` ([`JournalItem`])"]
    #[inline]
    pub fn journal_items<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, JournalItem<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<JournalItem<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SourceFile<'tree> {
    type WithLifetime<'a> = SourceFile<'a>;
    const KIND: &'static str = "source_file";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "source_file" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "source_file");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `status`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Status<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Status<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Status<'tree> {
    type WithLifetime<'a> = Status<'a>;
    const KIND: &'static str = "status";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "status" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "status");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `tag_directive`\n\nThis node has named children of type `{assert_subdirective | check_subdirective | comment}*`:\n\n- [`AssertSubdirective`]\n- [`CheckSubdirective`]\n- [`Comment`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TagDirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TagDirective<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `{assert_subdirective | check_subdirective | comment}*`:\n\n- [`AssertSubdirective`]\n- [`CheckSubdirective`]\n- [`Comment`]\n"]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::AssertSubdirective_CheckSubdirective_Comment<'tree>,
        >,
    > + 'a {
        :: type_sitter :: Node :: raw (self) . named_children (& mut c . 0) . filter (| n | ! n . is_extra ()) . map (< anon_unions :: AssertSubdirective_CheckSubdirective_Comment < 'tree > as :: type_sitter :: Node < 'tree >> :: try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TagDirective<'tree> {
    type WithLifetime<'a> = TagDirective<'a>;
    const KIND: &'static str = "tag_directive";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "tag_directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "tag_directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `test`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Test<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Test<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Test<'tree> {
    type WithLifetime<'a> = Test<'a>;
    const KIND: &'static str = "test";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "test" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "test");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `time`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Time<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Time<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Time<'tree> {
    type WithLifetime<'a> = Time<'a>;
    const KIND: &'static str = "time";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "time" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "time");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `word_directive`\n\nThis node has an optional named child of type `{account | filename}?`:\n\n- [`Account`]\n- [`Filename`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WordDirective<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> WordDirective<'tree> {
    #[doc = "Get the node's only not-extra named child, if it has one.\n\nThis child has type `{account | filename}?`:\n\n- [`Account`]\n- [`Filename`]\n"]
    #[inline]
    pub fn child(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, anon_unions::Account_Filename<'tree>>>
    {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<anon_unions::Account_Filename<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for WordDirective<'tree> {
    type WithLifetime<'a> = WordDirective<'a>;
    const KIND: &'static str = "word_directive";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "word_directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "word_directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `xact`\n\nThis node has a named child of type `{automated_xact | periodic_xact | plain_xact}`:\n\n- [`AutomatedXact`]\n- [`PeriodicXact`]\n- [`PlainXact`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Xact<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Xact<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `{automated_xact | periodic_xact | plain_xact}`:\n\n- [`AutomatedXact`]\n- [`PeriodicXact`]\n- [`PlainXact`]\n"]
    #[inline]
    pub fn child(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::AutomatedXact_PeriodicXact_PlainXact<'tree>>
    {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <anon_unions::AutomatedXact_PeriodicXact_PlainXact<'tree> as ::type_sitter::Node<
                    'tree,
                >>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Xact<'tree> {
    type WithLifetime<'a> = Xact<'a>;
    const KIND: &'static str = "xact";
    #[inline]
    fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
        if node.kind() == "xact" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "xact");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node ``\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct __<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> __<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for __<'tree> {
        type WithLifetime<'a> = __<'a>;
        const KIND: &'static str = "";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `A`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct A<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> A<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for A<'tree> {
        type WithLifetime<'a> = A<'a>;
        const KIND: &'static str = "A";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "A" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "A");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `C`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct C<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> C<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for C<'tree> {
        type WithLifetime<'a> = C<'a>;
        const KIND: &'static str = "C";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "C" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "C");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `D`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct D<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> D<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for D<'tree> {
        type WithLifetime<'a> = D<'a>;
        const KIND: &'static str = "D";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "D" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "D");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `I`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct I<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> I<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for I<'tree> {
        type WithLifetime<'a> = I<'a>;
        const KIND: &'static str = "I";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "I" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "I");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `N`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct N<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> N<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for N<'tree> {
        type WithLifetime<'a> = N<'a>;
        const KIND: &'static str = "N";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "N" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "N");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `O`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct O<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> O<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for O<'tree> {
        type WithLifetime<'a> = O<'a>;
        const KIND: &'static str = "O";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "O" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "O");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `P`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct P<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> P<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for P<'tree> {
        type WithLifetime<'a> = P<'a>;
        const KIND: &'static str = "P";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "P" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "P");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `Y`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Y<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Y<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Y<'tree> {
        type WithLifetime<'a> = Y<'a>;
        const KIND: &'static str = "Y";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "Y" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "Y");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `account`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Account<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Account<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Account<'tree> {
        type WithLifetime<'a> = Account<'a>;
        const KIND: &'static str = "account";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "account" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "account");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `alias`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Alias<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Alias<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Alias<'tree> {
        type WithLifetime<'a> = Alias<'a>;
        const KIND: &'static str = "alias";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "alias" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "alias");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `assert`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Assert<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Assert<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Assert<'tree> {
        type WithLifetime<'a> = Assert<'a>;
        const KIND: &'static str = "assert";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "assert" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "assert");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `bucket`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Bucket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Bucket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Bucket<'tree> {
        type WithLifetime<'a> = Bucket<'a>;
        const KIND: &'static str = "bucket";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "bucket" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "bucket");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `check`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Check<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Check<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Check<'tree> {
        type WithLifetime<'a> = Check<'a>;
        const KIND: &'static str = "check";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "check" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "check");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `comment`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Comment<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comment<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Comment<'tree> {
        type WithLifetime<'a> = Comment<'a>;
        const KIND: &'static str = "comment";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "comment" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "comment");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `commodity`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Commodity<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Commodity<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Commodity<'tree> {
        type WithLifetime<'a> = Commodity<'a>;
        const KIND: &'static str = "commodity";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "commodity" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "commodity");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `day`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Day<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Day<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Day<'tree> {
        type WithLifetime<'a> = Day<'a>;
        const KIND: &'static str = "day";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "day" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "day");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `def`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Def<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Def<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Def<'tree> {
        type WithLifetime<'a> = Def<'a>;
        const KIND: &'static str = "def";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "def" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "def");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `default`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Default<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Default<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Default<'tree> {
        type WithLifetime<'a> = Default<'a>;
        const KIND: &'static str = "default";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "default" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "default");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `end`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct End<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> End<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for End<'tree> {
        type WithLifetime<'a> = End<'a>;
        const KIND: &'static str = "end";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "end" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "end");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `eval`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Eval<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Eval<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Eval<'tree> {
        type WithLifetime<'a> = Eval<'a>;
        const KIND: &'static str = "eval";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "eval" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "eval");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `format`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Format<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Format<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Format<'tree> {
        type WithLifetime<'a> = Format<'a>;
        const KIND: &'static str = "format";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "format" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "format");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `i`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct I_<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> I_<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for I_<'tree> {
        type WithLifetime<'a> = I_<'a>;
        const KIND: &'static str = "i";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "i" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "i");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `include`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Include<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Include<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Include<'tree> {
        type WithLifetime<'a> = Include<'a>;
        const KIND: &'static str = "include";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "include" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "include");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `last`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Last<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Last<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Last<'tree> {
        type WithLifetime<'a> = Last<'a>;
        const KIND: &'static str = "last";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "last" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "last");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `month`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Month<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Month<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Month<'tree> {
        type WithLifetime<'a> = Month<'a>;
        const KIND: &'static str = "month";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "month" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "month");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `next`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Next<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Next<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Next<'tree> {
        type WithLifetime<'a> = Next<'a>;
        const KIND: &'static str = "next";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "next" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "next");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `nomarket`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Nomarket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Nomarket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Nomarket<'tree> {
        type WithLifetime<'a> = Nomarket<'a>;
        const KIND: &'static str = "nomarket";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "nomarket" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "nomarket");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `note`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Note<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Note<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Note<'tree> {
        type WithLifetime<'a> = Note<'a>;
        const KIND: &'static str = "note";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "note" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "note");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `o`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct O_<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> O_<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for O_<'tree> {
        type WithLifetime<'a> = O_<'a>;
        const KIND: &'static str = "o";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "o" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "o");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `payee`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Payee<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Payee<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Payee<'tree> {
        type WithLifetime<'a> = Payee<'a>;
        const KIND: &'static str = "payee";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "payee" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "payee");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `quarter`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Quarter<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Quarter<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Quarter<'tree> {
        type WithLifetime<'a> = Quarter<'a>;
        const KIND: &'static str = "quarter";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "quarter" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "quarter");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `tag`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Tag<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Tag<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Tag<'tree> {
        type WithLifetime<'a> = Tag<'a>;
        const KIND: &'static str = "tag";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "tag" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "tag");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `test`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Test<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Test<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Test<'tree> {
        type WithLifetime<'a> = Test<'a>;
        const KIND: &'static str = "test";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "test" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "test");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `this`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct This<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> This<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for This<'tree> {
        type WithLifetime<'a> = This<'a>;
        const KIND: &'static str = "this";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "this" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "this");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `week`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Week<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Week<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Week<'tree> {
        type WithLifetime<'a> = Week<'a>;
        const KIND: &'static str = "week";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "week" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "week");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `year`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Year<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Year<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Year<'tree> {
        type WithLifetime<'a> = Year<'a>;
        const KIND: &'static str = "year";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "year" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "year");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `\t`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Tab<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Tab<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Tab<'tree> {
        type WithLifetime<'a> = Tab<'a>;
        const KIND: &'static str = "\t";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "\t" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\t");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `\t\t`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct TabTab<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> TabTab<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for TabTab<'tree> {
        type WithLifetime<'a> = TabTab<'a>;
        const KIND: &'static str = "\t\t";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "\t\t" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\t\t");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `\t `\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct TabSpace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> TabSpace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for TabSpace<'tree> {
        type WithLifetime<'a> = TabSpace<'a>;
        const KIND: &'static str = "\t ";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "\t " {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\t ");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `\n`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Newline<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Newline<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Newline<'tree> {
        type WithLifetime<'a> = Newline<'a>;
        const KIND: &'static str = "\n";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "\n" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\n");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node ` `\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Space<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Space<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Space<'tree> {
        type WithLifetime<'a> = Space<'a>;
        const KIND: &'static str = " ";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == " " {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), " ");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node ` \t`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SpaceTab<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SpaceTab<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SpaceTab<'tree> {
        type WithLifetime<'a> = SpaceTab<'a>;
        const KIND: &'static str = " \t";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == " \t" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), " \t");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `  `\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SpaceSpace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SpaceSpace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SpaceSpace<'tree> {
        type WithLifetime<'a> = SpaceSpace<'a>;
        const KIND: &'static str = "  ";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "  " {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "  ");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `!`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Not<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Not<'tree> {
        type WithLifetime<'a> = Not<'a>;
        const KIND: &'static str = "!";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "!" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `(`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LParen<'tree> {
        type WithLifetime<'a> = LParen<'a>;
        const KIND: &'static str = "(";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "(");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `)`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RParen<'tree> {
        type WithLifetime<'a> = RParen<'a>;
        const KIND: &'static str = ")";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ")");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `*`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mul<'tree> {
        type WithLifetime<'a> = Mul<'a>;
        const KIND: &'static str = "*";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `+`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Add<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Add<'tree> {
        type WithLifetime<'a> = Add<'a>;
        const KIND: &'static str = "+";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `-`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Sub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Sub<'tree> {
        type WithLifetime<'a> = Sub<'a>;
        const KIND: &'static str = "-";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `--`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubSub<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubSub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubSub<'tree> {
        type WithLifetime<'a> = SubSub<'a>;
        const KIND: &'static str = "--";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "--" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "--");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `;`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Semicolon<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Semicolon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Semicolon<'tree> {
        type WithLifetime<'a> = Semicolon<'a>;
        const KIND: &'static str = ";";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == ";" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ";");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Eq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Eq<'tree> {
        type WithLifetime<'a> = Eq<'a>;
        const KIND: &'static str = "=";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `@`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct At<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> At<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for At<'tree> {
        type WithLifetime<'a> = At<'a>;
        const KIND: &'static str = "@";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "@" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "@");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `@@`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AtAt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AtAt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AtAt<'tree> {
        type WithLifetime<'a> = AtAt<'a>;
        const KIND: &'static str = "@@";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "@@" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "@@");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `[`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBracket<'tree> {
        type WithLifetime<'a> = LBracket<'a>;
        const KIND: &'static str = "[";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "[");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `]`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBracket<'tree> {
        type WithLifetime<'a> = RBracket<'a>;
        const KIND: &'static str = "]";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "]");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `end comment`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EndSpacecomment<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EndSpacecomment<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EndSpacecomment<'tree> {
        type WithLifetime<'a> = EndSpacecomment<'a>;
        const KIND: &'static str = "end comment";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "end comment" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "end comment");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `end test`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EndSpacetest<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EndSpacetest<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EndSpacetest<'tree> {
        type WithLifetime<'a> = EndSpacetest<'a>;
        const KIND: &'static str = "end test";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "end test" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "end test");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `{`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBrace<'tree> {
        type WithLifetime<'a> = LBrace<'a>;
        const KIND: &'static str = "{";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `{{`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBraceLBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBraceLBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBraceLBrace<'tree> {
        type WithLifetime<'a> = LBraceLBrace<'a>;
        const KIND: &'static str = "{{";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "{{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{{");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `}`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBrace<'tree> {
        type WithLifetime<'a> = RBrace<'a>;
        const KIND: &'static str = "}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `}}`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBraceRBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBraceRBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBraceRBrace<'tree> {
        type WithLifetime<'a> = RBraceRBrace<'a>;
        const KIND: &'static str = "}}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "}}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `~`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitNot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitNot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitNot<'tree> {
        type WithLifetime<'a> = BitNot<'a>;
        const KIND: &'static str = "~";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            if node.kind() == "~" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "~");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "One of `{account_directive | char_directive | commodity_directive | option | tag_directive | word_directive}`:\n- [`AccountDirective`]\n- [`CharDirective`]\n- [`CommodityDirective`]\n- [`Option`]\n- [`TagDirective`]\n- [`WordDirective`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective<
        'tree,
    > {
        AccountDirective(AccountDirective<'tree>),
        CharDirective(CharDirective<'tree>),
        CommodityDirective(CommodityDirective<'tree>),
        Option(Option<'tree>),
        TagDirective(TagDirective<'tree>),
        WordDirective(WordDirective<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree>
        AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective<'tree>
    {
        #[doc = "Returns the node if it is of type `account_directive` ([`AccountDirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account_directive(self) -> ::std::option::Option<AccountDirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AccountDirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `char_directive` ([`CharDirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_char_directive(self) -> ::std::option::Option<CharDirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CharDirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `commodity_directive` ([`CommodityDirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_commodity_directive(self) -> ::std::option::Option<CommodityDirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CommodityDirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `option` ([`Option`]), otherwise returns `None`"]
        #[inline]
        pub fn as_option(self) -> ::std::option::Option<Option<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Option(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `tag_directive` ([`TagDirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_tag_directive(self) -> ::std::option::Option<TagDirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TagDirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `word_directive` ([`WordDirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_word_directive(self) -> ::std::option::Option<WordDirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::WordDirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective<
            'tree,
        >
    {
        type WithLifetime<'a> =
            AccountDirective_CharDirective_CommodityDirective_Option_TagDirective_WordDirective<'a>;
        const KIND : & 'static str = "{account_directive | char_directive | commodity_directive | option | tag_directive | word_directive}" ;
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "account_directive" => Ok(unsafe {
                    Self::AccountDirective(<AccountDirective<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "char_directive" => {
                    Ok(unsafe {
                        Self :: CharDirective (< CharDirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "commodity_directive" => {
                    Ok(unsafe {
                        Self :: CommodityDirective (< CommodityDirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "option" => Ok(unsafe {
                    Self::Option(
                        <Option<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "tag_directive" => Ok(unsafe {
                    Self::TagDirective(
                        <TagDirective<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "word_directive" => {
                    Ok(unsafe {
                        Self :: WordDirective (< WordDirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AccountDirective(x) => ::type_sitter::Node::raw(x),
                Self::CharDirective(x) => ::type_sitter::Node::raw(x),
                Self::CommodityDirective(x) => ::type_sitter::Node::raw(x),
                Self::Option(x) => ::type_sitter::Node::raw(x),
                Self::TagDirective(x) => ::type_sitter::Node::raw(x),
                Self::WordDirective(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AccountDirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::CharDirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::CommodityDirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::Option(x) => ::type_sitter::Node::raw_mut(x),
                Self::TagDirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::WordDirective(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AccountDirective(x) => x.into_raw(),
                Self::CharDirective(x) => x.into_raw(),
                Self::CommodityDirective(x) => x.into_raw(),
                Self::Option(x) => x.into_raw(),
                Self::TagDirective(x) => x.into_raw(),
                Self::WordDirective(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{account | account_subdirective | comment}`:\n- [`Account`]\n- [`AccountSubdirective`]\n- [`Comment`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Account_AccountSubdirective_Comment<'tree> {
        Account(Account<'tree>),
        AccountSubdirective(AccountSubdirective<'tree>),
        Comment(Comment<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Account_AccountSubdirective_Comment<'tree> {
        #[doc = "Returns the node if it is of type `account` ([`Account`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account(self) -> ::std::option::Option<Account<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Account(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `account_subdirective` ([`AccountSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account_subdirective(self) -> ::std::option::Option<AccountSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AccountSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Account_AccountSubdirective_Comment<'tree> {
        type WithLifetime<'a> = Account_AccountSubdirective_Comment<'a>;
        const KIND: &'static str = "{account | account_subdirective | comment}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "account" => Ok(unsafe {
                    Self::Account(
                        <Account<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "account_subdirective" => {
                    Ok(unsafe {
                        Self :: AccountSubdirective (< AccountSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "comment" => Ok(unsafe {
                    Self::Comment(
                        <Comment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw(x),
                Self::AccountSubdirective(x) => ::type_sitter::Node::raw(x),
                Self::Comment(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw_mut(x),
                Self::AccountSubdirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => x.into_raw(),
                Self::AccountSubdirective(x) => x.into_raw(),
                Self::Comment(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{account | amount | balance_assertion | lot_price | note | price | status}`:\n- [`Account`]\n- [`Amount`]\n- [`BalanceAssertion`]\n- [`LotPrice`]\n- [`Note`]\n- [`Price`]\n- [`Status`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status<'tree> {
        Account(Account<'tree>),
        Amount(Amount<'tree>),
        BalanceAssertion(BalanceAssertion<'tree>),
        LotPrice(LotPrice<'tree>),
        Note(Note<'tree>),
        Price(Price<'tree>),
        Status(Status<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status<'tree> {
        #[doc = "Returns the node if it is of type `account` ([`Account`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account(self) -> ::std::option::Option<Account<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Account(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `amount` ([`Amount`]), otherwise returns `None`"]
        #[inline]
        pub fn as_amount(self) -> ::std::option::Option<Amount<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Amount(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `balance_assertion` ([`BalanceAssertion`]), otherwise returns `None`"]
        #[inline]
        pub fn as_balance_assertion(self) -> ::std::option::Option<BalanceAssertion<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BalanceAssertion(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `lot_price` ([`LotPrice`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lot_price(self) -> ::std::option::Option<LotPrice<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LotPrice(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `note` ([`Note`]), otherwise returns `None`"]
        #[inline]
        pub fn as_note(self) -> ::std::option::Option<Note<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Note(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `price` ([`Price`]), otherwise returns `None`"]
        #[inline]
        pub fn as_price(self) -> ::std::option::Option<Price<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Price(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `status` ([`Status`]), otherwise returns `None`"]
        #[inline]
        pub fn as_status(self) -> ::std::option::Option<Status<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Status(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status<'tree>
    {
        type WithLifetime<'a> = Account_Amount_BalanceAssertion_LotPrice_Note_Price_Status<'a>;
        const KIND: &'static str =
            "{account | amount | balance_assertion | lot_price | note | price | status}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "account" => Ok(unsafe {
                    Self::Account(
                        <Account<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "amount" => Ok(unsafe {
                    Self::Amount(
                        <Amount<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "balance_assertion" => Ok(unsafe {
                    Self::BalanceAssertion(<BalanceAssertion<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "lot_price" => Ok(unsafe {
                    Self::LotPrice(
                        <LotPrice<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "note" => Ok(unsafe {
                    Self::Note(
                        <Note<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "price" => Ok(unsafe {
                    Self::Price(
                        <Price<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "status" => Ok(unsafe {
                    Self::Status(
                        <Status<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw(x),
                Self::Amount(x) => ::type_sitter::Node::raw(x),
                Self::BalanceAssertion(x) => ::type_sitter::Node::raw(x),
                Self::LotPrice(x) => ::type_sitter::Node::raw(x),
                Self::Note(x) => ::type_sitter::Node::raw(x),
                Self::Price(x) => ::type_sitter::Node::raw(x),
                Self::Status(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw_mut(x),
                Self::Amount(x) => ::type_sitter::Node::raw_mut(x),
                Self::BalanceAssertion(x) => ::type_sitter::Node::raw_mut(x),
                Self::LotPrice(x) => ::type_sitter::Node::raw_mut(x),
                Self::Note(x) => ::type_sitter::Node::raw_mut(x),
                Self::Price(x) => ::type_sitter::Node::raw_mut(x),
                Self::Status(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => x.into_raw(),
                Self::Amount(x) => x.into_raw(),
                Self::BalanceAssertion(x) => x.into_raw(),
                Self::LotPrice(x) => x.into_raw(),
                Self::Note(x) => x.into_raw(),
                Self::Price(x) => x.into_raw(),
                Self::Status(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{account | amount | check_in | check_out | commodity | date}`:\n- [`Account`]\n- [`Amount`]\n- [`CheckIn`]\n- [`CheckOut`]\n- [`Commodity`]\n- [`Date`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Account_Amount_CheckIn_CheckOut_Commodity_Date<'tree> {
        Account(Account<'tree>),
        Amount(Amount<'tree>),
        CheckIn(CheckIn<'tree>),
        CheckOut(CheckOut<'tree>),
        Commodity(Commodity<'tree>),
        Date(Date<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Account_Amount_CheckIn_CheckOut_Commodity_Date<'tree> {
        #[doc = "Returns the node if it is of type `account` ([`Account`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account(self) -> ::std::option::Option<Account<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Account(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `amount` ([`Amount`]), otherwise returns `None`"]
        #[inline]
        pub fn as_amount(self) -> ::std::option::Option<Amount<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Amount(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `check_in` ([`CheckIn`]), otherwise returns `None`"]
        #[inline]
        pub fn as_check_in(self) -> ::std::option::Option<CheckIn<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CheckIn(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `check_out` ([`CheckOut`]), otherwise returns `None`"]
        #[inline]
        pub fn as_check_out(self) -> ::std::option::Option<CheckOut<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CheckOut(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `commodity` ([`Commodity`]), otherwise returns `None`"]
        #[inline]
        pub fn as_commodity(self) -> ::std::option::Option<Commodity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Commodity(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `date` ([`Date`]), otherwise returns `None`"]
        #[inline]
        pub fn as_date(self) -> ::std::option::Option<Date<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Date(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Account_Amount_CheckIn_CheckOut_Commodity_Date<'tree> {
        type WithLifetime<'a> = Account_Amount_CheckIn_CheckOut_Commodity_Date<'a>;
        const KIND: &'static str = "{account | amount | check_in | check_out | commodity | date}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "account" => Ok(unsafe {
                    Self::Account(
                        <Account<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "amount" => Ok(unsafe {
                    Self::Amount(
                        <Amount<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "check_in" => Ok(unsafe {
                    Self::CheckIn(
                        <CheckIn<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "check_out" => Ok(unsafe {
                    Self::CheckOut(
                        <CheckOut<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "commodity" => Ok(unsafe {
                    Self::Commodity(
                        <Commodity<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "date" => Ok(unsafe {
                    Self::Date(
                        <Date<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw(x),
                Self::Amount(x) => ::type_sitter::Node::raw(x),
                Self::CheckIn(x) => ::type_sitter::Node::raw(x),
                Self::CheckOut(x) => ::type_sitter::Node::raw(x),
                Self::Commodity(x) => ::type_sitter::Node::raw(x),
                Self::Date(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw_mut(x),
                Self::Amount(x) => ::type_sitter::Node::raw_mut(x),
                Self::CheckIn(x) => ::type_sitter::Node::raw_mut(x),
                Self::CheckOut(x) => ::type_sitter::Node::raw_mut(x),
                Self::Commodity(x) => ::type_sitter::Node::raw_mut(x),
                Self::Date(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => x.into_raw(),
                Self::Amount(x) => x.into_raw(),
                Self::CheckIn(x) => x.into_raw(),
                Self::CheckOut(x) => x.into_raw(),
                Self::Commodity(x) => x.into_raw(),
                Self::Date(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{account | date | payee | time}`:\n- [`Account`]\n- [`Date`]\n- [`Payee`]\n- [`Time`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Account_Date_Payee_Time<'tree> {
        Account(Account<'tree>),
        Date(Date<'tree>),
        Payee(Payee<'tree>),
        Time(Time<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Account_Date_Payee_Time<'tree> {
        #[doc = "Returns the node if it is of type `account` ([`Account`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account(self) -> ::std::option::Option<Account<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Account(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `date` ([`Date`]), otherwise returns `None`"]
        #[inline]
        pub fn as_date(self) -> ::std::option::Option<Date<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Date(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `payee` ([`Payee`]), otherwise returns `None`"]
        #[inline]
        pub fn as_payee(self) -> ::std::option::Option<Payee<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Payee(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `time` ([`Time`]), otherwise returns `None`"]
        #[inline]
        pub fn as_time(self) -> ::std::option::Option<Time<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Time(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Account_Date_Payee_Time<'tree> {
        type WithLifetime<'a> = Account_Date_Payee_Time<'a>;
        const KIND: &'static str = "{account | date | payee | time}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "account" => Ok(unsafe {
                    Self::Account(
                        <Account<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "date" => Ok(unsafe {
                    Self::Date(
                        <Date<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "payee" => Ok(unsafe {
                    Self::Payee(
                        <Payee<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "time" => Ok(unsafe {
                    Self::Time(
                        <Time<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw(x),
                Self::Date(x) => ::type_sitter::Node::raw(x),
                Self::Payee(x) => ::type_sitter::Node::raw(x),
                Self::Time(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw_mut(x),
                Self::Date(x) => ::type_sitter::Node::raw_mut(x),
                Self::Payee(x) => ::type_sitter::Node::raw_mut(x),
                Self::Time(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => x.into_raw(),
                Self::Date(x) => x.into_raw(),
                Self::Payee(x) => x.into_raw(),
                Self::Time(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{account | filename}`:\n- [`Account`]\n- [`Filename`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Account_Filename<'tree> {
        Account(Account<'tree>),
        Filename(Filename<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Account_Filename<'tree> {
        #[doc = "Returns the node if it is of type `account` ([`Account`]), otherwise returns `None`"]
        #[inline]
        pub fn as_account(self) -> ::std::option::Option<Account<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Account(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `filename` ([`Filename`]), otherwise returns `None`"]
        #[inline]
        pub fn as_filename(self) -> ::std::option::Option<Filename<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Filename(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Account_Filename<'tree> {
        type WithLifetime<'a> = Account_Filename<'a>;
        const KIND: &'static str = "{account | filename}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "account" => Ok(unsafe {
                    Self::Account(
                        <Account<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "filename" => Ok(unsafe {
                    Self::Filename(
                        <Filename<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw(x),
                Self::Filename(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => ::type_sitter::Node::raw_mut(x),
                Self::Filename(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Account(x) => x.into_raw(),
                Self::Filename(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{alias_subdirective | assert_subdirective | check_subdirective | default_subdirective | note_subdirective}`:\n- [`AliasSubdirective`]\n- [`AssertSubdirective`]\n- [`CheckSubdirective`]\n- [`DefaultSubdirective`]\n- [`NoteSubdirective`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AliasSubdirective_AssertSubdirective_CheckSubdirective_DefaultSubdirective_NoteSubdirective<
        'tree,
    > {
        AliasSubdirective(AliasSubdirective<'tree>),
        AssertSubdirective(AssertSubdirective<'tree>),
        CheckSubdirective(CheckSubdirective<'tree>),
        DefaultSubdirective(DefaultSubdirective<'tree>),
        NoteSubdirective(NoteSubdirective<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree>
        AliasSubdirective_AssertSubdirective_CheckSubdirective_DefaultSubdirective_NoteSubdirective<
            'tree,
        >
    {
        #[doc = "Returns the node if it is of type `alias_subdirective` ([`AliasSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_alias_subdirective(self) -> ::std::option::Option<AliasSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AliasSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `assert_subdirective` ([`AssertSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_assert_subdirective(self) -> ::std::option::Option<AssertSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AssertSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `check_subdirective` ([`CheckSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_check_subdirective(self) -> ::std::option::Option<CheckSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CheckSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `default_subdirective` ([`DefaultSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_default_subdirective(self) -> ::std::option::Option<DefaultSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DefaultSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `note_subdirective` ([`NoteSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_note_subdirective(self) -> ::std::option::Option<NoteSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NoteSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl < 'tree > :: type_sitter :: Node < 'tree > for AliasSubdirective_AssertSubdirective_CheckSubdirective_DefaultSubdirective_NoteSubdirective < 'tree > { type WithLifetime < 'a > = AliasSubdirective_AssertSubdirective_CheckSubdirective_DefaultSubdirective_NoteSubdirective < 'a > ; const KIND : & 'static str = "{alias_subdirective | assert_subdirective | check_subdirective | default_subdirective | note_subdirective}" ; # [inline] fn try_from_raw (node : :: type_sitter :: raw :: Node < 'tree >) -> :: type_sitter :: NodeResult < Self > { match node . kind () { "alias_subdirective" => Ok (unsafe { Self :: AliasSubdirective (< AliasSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node)) }) , "assert_subdirective" => Ok (unsafe { Self :: AssertSubdirective (< AssertSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node)) }) , "check_subdirective" => Ok (unsafe { Self :: CheckSubdirective (< CheckSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node)) }) , "default_subdirective" => Ok (unsafe { Self :: DefaultSubdirective (< DefaultSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node)) }) , "note_subdirective" => Ok (unsafe { Self :: NoteSubdirective (< NoteSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node)) }) , _ => Err (:: type_sitter :: IncorrectKind :: new :: < Self > (node)) } } # [inline] fn raw (& self) -> & :: type_sitter :: raw :: Node < 'tree > { match self { Self :: AliasSubdirective (x) => :: type_sitter :: Node :: raw (x) , Self :: AssertSubdirective (x) => :: type_sitter :: Node :: raw (x) , Self :: CheckSubdirective (x) => :: type_sitter :: Node :: raw (x) , Self :: DefaultSubdirective (x) => :: type_sitter :: Node :: raw (x) , Self :: NoteSubdirective (x) => :: type_sitter :: Node :: raw (x) , } } # [inline] fn raw_mut (& mut self) -> & mut :: type_sitter :: raw :: Node < 'tree > { match self { Self :: AliasSubdirective (x) => :: type_sitter :: Node :: raw_mut (x) , Self :: AssertSubdirective (x) => :: type_sitter :: Node :: raw_mut (x) , Self :: CheckSubdirective (x) => :: type_sitter :: Node :: raw_mut (x) , Self :: DefaultSubdirective (x) => :: type_sitter :: Node :: raw_mut (x) , Self :: NoteSubdirective (x) => :: type_sitter :: Node :: raw_mut (x) , } } # [inline] fn into_raw (self) -> :: type_sitter :: raw :: Node < 'tree > { match self { Self :: AliasSubdirective (x) => x . into_raw () , Self :: AssertSubdirective (x) => x . into_raw () , Self :: CheckSubdirective (x) => x . into_raw () , Self :: DefaultSubdirective (x) => x . into_raw () , Self :: NoteSubdirective (x) => x . into_raw () , } } }
    #[doc = "One of `{alias_subdirective | default_subdirective | format_subdirective | note_subdirective}`:\n- [`AliasSubdirective`]\n- [`DefaultSubdirective`]\n- [`FormatSubdirective`]\n- [`NoteSubdirective`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AliasSubdirective_DefaultSubdirective_FormatSubdirective_NoteSubdirective<'tree> {
        AliasSubdirective(AliasSubdirective<'tree>),
        DefaultSubdirective(DefaultSubdirective<'tree>),
        FormatSubdirective(FormatSubdirective<'tree>),
        NoteSubdirective(NoteSubdirective<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AliasSubdirective_DefaultSubdirective_FormatSubdirective_NoteSubdirective<'tree> {
        #[doc = "Returns the node if it is of type `alias_subdirective` ([`AliasSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_alias_subdirective(self) -> ::std::option::Option<AliasSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AliasSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `default_subdirective` ([`DefaultSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_default_subdirective(self) -> ::std::option::Option<DefaultSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DefaultSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `format_subdirective` ([`FormatSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_format_subdirective(self) -> ::std::option::Option<FormatSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FormatSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `note_subdirective` ([`NoteSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_note_subdirective(self) -> ::std::option::Option<NoteSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NoteSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for AliasSubdirective_DefaultSubdirective_FormatSubdirective_NoteSubdirective<'tree>
    {
        type WithLifetime<'a> =
            AliasSubdirective_DefaultSubdirective_FormatSubdirective_NoteSubdirective<'a>;
        const KIND: &'static str =
            "{alias_subdirective | default_subdirective | format_subdirective | note_subdirective}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "alias_subdirective" => Ok(unsafe {
                    Self::AliasSubdirective(<AliasSubdirective<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "default_subdirective" => {
                    Ok(unsafe {
                        Self :: DefaultSubdirective (< DefaultSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "format_subdirective" => {
                    Ok(unsafe {
                        Self :: FormatSubdirective (< FormatSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "note_subdirective" => Ok(unsafe {
                    Self::NoteSubdirective(<NoteSubdirective<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AliasSubdirective(x) => ::type_sitter::Node::raw(x),
                Self::DefaultSubdirective(x) => ::type_sitter::Node::raw(x),
                Self::FormatSubdirective(x) => ::type_sitter::Node::raw(x),
                Self::NoteSubdirective(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AliasSubdirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::DefaultSubdirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::FormatSubdirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::NoteSubdirective(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AliasSubdirective(x) => x.into_raw(),
                Self::DefaultSubdirective(x) => x.into_raw(),
                Self::FormatSubdirective(x) => x.into_raw(),
                Self::NoteSubdirective(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{assert_subdirective | check_subdirective | comment}`:\n- [`AssertSubdirective`]\n- [`CheckSubdirective`]\n- [`Comment`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AssertSubdirective_CheckSubdirective_Comment<'tree> {
        AssertSubdirective(AssertSubdirective<'tree>),
        CheckSubdirective(CheckSubdirective<'tree>),
        Comment(Comment<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AssertSubdirective_CheckSubdirective_Comment<'tree> {
        #[doc = "Returns the node if it is of type `assert_subdirective` ([`AssertSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_assert_subdirective(self) -> ::std::option::Option<AssertSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AssertSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `check_subdirective` ([`CheckSubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_check_subdirective(self) -> ::std::option::Option<CheckSubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CheckSubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AssertSubdirective_CheckSubdirective_Comment<'tree> {
        type WithLifetime<'a> = AssertSubdirective_CheckSubdirective_Comment<'a>;
        const KIND: &'static str = "{assert_subdirective | check_subdirective | comment}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "assert_subdirective" => {
                    Ok(unsafe {
                        Self :: AssertSubdirective (< AssertSubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "check_subdirective" => Ok(unsafe {
                    Self::CheckSubdirective(<CheckSubdirective<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "comment" => Ok(unsafe {
                    Self::Comment(
                        <Comment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AssertSubdirective(x) => ::type_sitter::Node::raw(x),
                Self::CheckSubdirective(x) => ::type_sitter::Node::raw(x),
                Self::Comment(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AssertSubdirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::CheckSubdirective(x) => ::type_sitter::Node::raw_mut(x),
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AssertSubdirective(x) => x.into_raw(),
                Self::CheckSubdirective(x) => x.into_raw(),
                Self::Comment(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{automated_xact | periodic_xact | plain_xact}`:\n- [`AutomatedXact`]\n- [`PeriodicXact`]\n- [`PlainXact`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AutomatedXact_PeriodicXact_PlainXact<'tree> {
        AutomatedXact(AutomatedXact<'tree>),
        PeriodicXact(PeriodicXact<'tree>),
        PlainXact(PlainXact<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AutomatedXact_PeriodicXact_PlainXact<'tree> {
        #[doc = "Returns the node if it is of type `automated_xact` ([`AutomatedXact`]), otherwise returns `None`"]
        #[inline]
        pub fn as_automated_xact(self) -> ::std::option::Option<AutomatedXact<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AutomatedXact(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `periodic_xact` ([`PeriodicXact`]), otherwise returns `None`"]
        #[inline]
        pub fn as_periodic_xact(self) -> ::std::option::Option<PeriodicXact<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PeriodicXact(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `plain_xact` ([`PlainXact`]), otherwise returns `None`"]
        #[inline]
        pub fn as_plain_xact(self) -> ::std::option::Option<PlainXact<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PlainXact(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AutomatedXact_PeriodicXact_PlainXact<'tree> {
        type WithLifetime<'a> = AutomatedXact_PeriodicXact_PlainXact<'a>;
        const KIND: &'static str = "{automated_xact | periodic_xact | plain_xact}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "automated_xact" => {
                    Ok(unsafe {
                        Self :: AutomatedXact (< AutomatedXact < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "periodic_xact" => Ok(unsafe {
                    Self::PeriodicXact(
                        <PeriodicXact<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "plain_xact" => Ok(unsafe {
                    Self::PlainXact(
                        <PlainXact<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AutomatedXact(x) => ::type_sitter::Node::raw(x),
                Self::PeriodicXact(x) => ::type_sitter::Node::raw(x),
                Self::PlainXact(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AutomatedXact(x) => ::type_sitter::Node::raw_mut(x),
                Self::PeriodicXact(x) => ::type_sitter::Node::raw_mut(x),
                Self::PlainXact(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AutomatedXact(x) => x.into_raw(),
                Self::PeriodicXact(x) => x.into_raw(),
                Self::PlainXact(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{block_comment | comment | directive | test | xact}`:\n- [`BlockComment`]\n- [`Comment`]\n- [`Directive`]\n- [`Test`]\n- [`Xact`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BlockComment_Comment_Directive_Test_Xact<'tree> {
        BlockComment(BlockComment<'tree>),
        Comment(Comment<'tree>),
        Directive(Directive<'tree>),
        Test(Test<'tree>),
        Xact(Xact<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BlockComment_Comment_Directive_Test_Xact<'tree> {
        #[doc = "Returns the node if it is of type `block_comment` ([`BlockComment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_block_comment(self) -> ::std::option::Option<BlockComment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BlockComment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`"]
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `test` ([`Test`]), otherwise returns `None`"]
        #[inline]
        pub fn as_test(self) -> ::std::option::Option<Test<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Test(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `xact` ([`Xact`]), otherwise returns `None`"]
        #[inline]
        pub fn as_xact(self) -> ::std::option::Option<Xact<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Xact(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BlockComment_Comment_Directive_Test_Xact<'tree> {
        type WithLifetime<'a> = BlockComment_Comment_Directive_Test_Xact<'a>;
        const KIND: &'static str = "{block_comment | comment | directive | test | xact}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "block_comment" => Ok(unsafe {
                    Self::BlockComment(
                        <BlockComment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "comment" => Ok(unsafe {
                    Self::Comment(
                        <Comment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "directive" => Ok(unsafe {
                    Self::Directive(
                        <Directive<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "test" => Ok(unsafe {
                    Self::Test(
                        <Test<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "xact" => Ok(unsafe {
                    Self::Xact(
                        <Xact<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => ::type_sitter::Node::raw(x),
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::Test(x) => ::type_sitter::Node::raw(x),
                Self::Xact(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Test(x) => ::type_sitter::Node::raw_mut(x),
                Self::Xact(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.into_raw(),
                Self::Comment(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::Test(x) => x.into_raw(),
                Self::Xact(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{code | date | effective_date | note | payee | posting | status}`:\n- [`Code`]\n- [`Date`]\n- [`EffectiveDate`]\n- [`Note`]\n- [`Payee`]\n- [`Posting`]\n- [`Status`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Code_Date_EffectiveDate_Note_Payee_Posting_Status<'tree> {
        Code(Code<'tree>),
        Date(Date<'tree>),
        EffectiveDate(EffectiveDate<'tree>),
        Note(Note<'tree>),
        Payee(Payee<'tree>),
        Posting(Posting<'tree>),
        Status(Status<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Code_Date_EffectiveDate_Note_Payee_Posting_Status<'tree> {
        #[doc = "Returns the node if it is of type `code` ([`Code`]), otherwise returns `None`"]
        #[inline]
        pub fn as_code(self) -> ::std::option::Option<Code<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Code(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `date` ([`Date`]), otherwise returns `None`"]
        #[inline]
        pub fn as_date(self) -> ::std::option::Option<Date<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Date(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `effective_date` ([`EffectiveDate`]), otherwise returns `None`"]
        #[inline]
        pub fn as_effective_date(self) -> ::std::option::Option<EffectiveDate<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EffectiveDate(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `note` ([`Note`]), otherwise returns `None`"]
        #[inline]
        pub fn as_note(self) -> ::std::option::Option<Note<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Note(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `payee` ([`Payee`]), otherwise returns `None`"]
        #[inline]
        pub fn as_payee(self) -> ::std::option::Option<Payee<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Payee(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `posting` ([`Posting`]), otherwise returns `None`"]
        #[inline]
        pub fn as_posting(self) -> ::std::option::Option<Posting<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Posting(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `status` ([`Status`]), otherwise returns `None`"]
        #[inline]
        pub fn as_status(self) -> ::std::option::Option<Status<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Status(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for Code_Date_EffectiveDate_Note_Payee_Posting_Status<'tree>
    {
        type WithLifetime<'a> = Code_Date_EffectiveDate_Note_Payee_Posting_Status<'a>;
        const KIND: &'static str =
            "{code | date | effective_date | note | payee | posting | status}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "code" => Ok(unsafe {
                    Self::Code(
                        <Code<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "date" => Ok(unsafe {
                    Self::Date(
                        <Date<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "effective_date" => {
                    Ok(unsafe {
                        Self :: EffectiveDate (< EffectiveDate < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                    })
                }
                "note" => Ok(unsafe {
                    Self::Note(
                        <Note<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "payee" => Ok(unsafe {
                    Self::Payee(
                        <Payee<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "posting" => Ok(unsafe {
                    Self::Posting(
                        <Posting<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "status" => Ok(unsafe {
                    Self::Status(
                        <Status<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Code(x) => ::type_sitter::Node::raw(x),
                Self::Date(x) => ::type_sitter::Node::raw(x),
                Self::EffectiveDate(x) => ::type_sitter::Node::raw(x),
                Self::Note(x) => ::type_sitter::Node::raw(x),
                Self::Payee(x) => ::type_sitter::Node::raw(x),
                Self::Posting(x) => ::type_sitter::Node::raw(x),
                Self::Status(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Code(x) => ::type_sitter::Node::raw_mut(x),
                Self::Date(x) => ::type_sitter::Node::raw_mut(x),
                Self::EffectiveDate(x) => ::type_sitter::Node::raw_mut(x),
                Self::Note(x) => ::type_sitter::Node::raw_mut(x),
                Self::Payee(x) => ::type_sitter::Node::raw_mut(x),
                Self::Posting(x) => ::type_sitter::Node::raw_mut(x),
                Self::Status(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Code(x) => x.into_raw(),
                Self::Date(x) => x.into_raw(),
                Self::EffectiveDate(x) => x.into_raw(),
                Self::Note(x) => x.into_raw(),
                Self::Payee(x) => x.into_raw(),
                Self::Posting(x) => x.into_raw(),
                Self::Status(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{comment | commodity | commodity_subdirective}`:\n- [`Comment`]\n- [`Commodity`]\n- [`CommoditySubdirective`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comment_Commodity_CommoditySubdirective<'tree> {
        Comment(Comment<'tree>),
        Commodity(Commodity<'tree>),
        CommoditySubdirective(CommoditySubdirective<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comment_Commodity_CommoditySubdirective<'tree> {
        #[doc = "Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `commodity` ([`Commodity`]), otherwise returns `None`"]
        #[inline]
        pub fn as_commodity(self) -> ::std::option::Option<Commodity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Commodity(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `commodity_subdirective` ([`CommoditySubdirective`]), otherwise returns `None`"]
        #[inline]
        pub fn as_commodity_subdirective(
            self,
        ) -> ::std::option::Option<CommoditySubdirective<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CommoditySubdirective(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Comment_Commodity_CommoditySubdirective<'tree> {
        type WithLifetime<'a> = Comment_Commodity_CommoditySubdirective<'a>;
        const KIND: &'static str = "{comment | commodity | commodity_subdirective}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "comment" => Ok(unsafe {
                    Self::Comment(
                        <Comment<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "commodity" => Ok(unsafe {
                    Self::Commodity(
                        <Commodity<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "commodity_subdirective" => Ok(unsafe {
                    Self :: CommoditySubdirective (< CommoditySubdirective < 'tree > as :: type_sitter :: Node < 'tree >> :: from_raw_unchecked (node))
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Commodity(x) => ::type_sitter::Node::raw(x),
                Self::CommoditySubdirective(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Commodity(x) => ::type_sitter::Node::raw_mut(x),
                Self::CommoditySubdirective(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Commodity(x) => x.into_raw(),
                Self::CommoditySubdirective(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{commodity | negative_quantity | quantity}`:\n- [`Commodity`]\n- [`NegativeQuantity`]\n- [`Quantity`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Commodity_NegativeQuantity_Quantity<'tree> {
        Commodity(Commodity<'tree>),
        NegativeQuantity(NegativeQuantity<'tree>),
        Quantity(Quantity<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Commodity_NegativeQuantity_Quantity<'tree> {
        #[doc = "Returns the node if it is of type `commodity` ([`Commodity`]), otherwise returns `None`"]
        #[inline]
        pub fn as_commodity(self) -> ::std::option::Option<Commodity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Commodity(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `negative_quantity` ([`NegativeQuantity`]), otherwise returns `None`"]
        #[inline]
        pub fn as_negative_quantity(self) -> ::std::option::Option<NegativeQuantity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NegativeQuantity(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `quantity` ([`Quantity`]), otherwise returns `None`"]
        #[inline]
        pub fn as_quantity(self) -> ::std::option::Option<Quantity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Quantity(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Commodity_NegativeQuantity_Quantity<'tree> {
        type WithLifetime<'a> = Commodity_NegativeQuantity_Quantity<'a>;
        const KIND: &'static str = "{commodity | negative_quantity | quantity}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "commodity" => Ok(unsafe {
                    Self::Commodity(
                        <Commodity<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "negative_quantity" => Ok(unsafe {
                    Self::NegativeQuantity(<NegativeQuantity<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "quantity" => Ok(unsafe {
                    Self::Quantity(
                        <Quantity<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Commodity(x) => ::type_sitter::Node::raw(x),
                Self::NegativeQuantity(x) => ::type_sitter::Node::raw(x),
                Self::Quantity(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Commodity(x) => ::type_sitter::Node::raw_mut(x),
                Self::NegativeQuantity(x) => ::type_sitter::Node::raw_mut(x),
                Self::Quantity(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Commodity(x) => x.into_raw(),
                Self::NegativeQuantity(x) => x.into_raw(),
                Self::Quantity(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{date | time}`:\n- [`Date`]\n- [`Time`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Date_Time<'tree> {
        Date(Date<'tree>),
        Time(Time<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Date_Time<'tree> {
        #[doc = "Returns the node if it is of type `date` ([`Date`]), otherwise returns `None`"]
        #[inline]
        pub fn as_date(self) -> ::std::option::Option<Date<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Date(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `time` ([`Time`]), otherwise returns `None`"]
        #[inline]
        pub fn as_time(self) -> ::std::option::Option<Time<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Time(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Date_Time<'tree> {
        type WithLifetime<'a> = Date_Time<'a>;
        const KIND: &'static str = "{date | time}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "date" => Ok(unsafe {
                    Self::Date(
                        <Date<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "time" => Ok(unsafe {
                    Self::Time(
                        <Time<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Date(x) => ::type_sitter::Node::raw(x),
                Self::Time(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Date(x) => ::type_sitter::Node::raw_mut(x),
                Self::Time(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Date(x) => x.into_raw(),
                Self::Time(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{interval | note | posting}`:\n- [`Interval`]\n- [`Note`]\n- [`Posting`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Interval_Note_Posting<'tree> {
        Interval(Interval<'tree>),
        Note(Note<'tree>),
        Posting(Posting<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Interval_Note_Posting<'tree> {
        #[doc = "Returns the node if it is of type `interval` ([`Interval`]), otherwise returns `None`"]
        #[inline]
        pub fn as_interval(self) -> ::std::option::Option<Interval<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Interval(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `note` ([`Note`]), otherwise returns `None`"]
        #[inline]
        pub fn as_note(self) -> ::std::option::Option<Note<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Note(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `posting` ([`Posting`]), otherwise returns `None`"]
        #[inline]
        pub fn as_posting(self) -> ::std::option::Option<Posting<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Posting(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Interval_Note_Posting<'tree> {
        type WithLifetime<'a> = Interval_Note_Posting<'a>;
        const KIND: &'static str = "{interval | note | posting}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "interval" => Ok(unsafe {
                    Self::Interval(
                        <Interval<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "note" => Ok(unsafe {
                    Self::Note(
                        <Note<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "posting" => Ok(unsafe {
                    Self::Posting(
                        <Posting<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Interval(x) => ::type_sitter::Node::raw(x),
                Self::Note(x) => ::type_sitter::Node::raw(x),
                Self::Posting(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Interval(x) => ::type_sitter::Node::raw_mut(x),
                Self::Note(x) => ::type_sitter::Node::raw_mut(x),
                Self::Posting(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Interval(x) => x.into_raw(),
                Self::Note(x) => x.into_raw(),
                Self::Posting(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{note | posting | query}`:\n- [`Note`]\n- [`Posting`]\n- [`Query`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Note_Posting_Query<'tree> {
        Note(Note<'tree>),
        Posting(Posting<'tree>),
        Query(Query<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Note_Posting_Query<'tree> {
        #[doc = "Returns the node if it is of type `note` ([`Note`]), otherwise returns `None`"]
        #[inline]
        pub fn as_note(self) -> ::std::option::Option<Note<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Note(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `posting` ([`Posting`]), otherwise returns `None`"]
        #[inline]
        pub fn as_posting(self) -> ::std::option::Option<Posting<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Posting(x) = self {
                Some(x)
            } else {
                None
            }
        }
        #[doc = "Returns the node if it is of type `query` ([`Query`]), otherwise returns `None`"]
        #[inline]
        pub fn as_query(self) -> ::std::option::Option<Query<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Query(x) = self {
                Some(x)
            } else {
                None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Note_Posting_Query<'tree> {
        type WithLifetime<'a> = Note_Posting_Query<'a>;
        const KIND: &'static str = "{note | posting | query}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) -> ::type_sitter::NodeResult<Self> {
            match node.kind() {
                "note" => Ok(unsafe {
                    Self::Note(
                        <Note<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "posting" => Ok(unsafe {
                    Self::Posting(
                        <Posting<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "query" => Ok(unsafe {
                    Self::Query(
                        <Query<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Note(x) => ::type_sitter::Node::raw(x),
                Self::Posting(x) => ::type_sitter::Node::raw(x),
                Self::Query(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Note(x) => ::type_sitter::Node::raw_mut(x),
                Self::Posting(x) => ::type_sitter::Node::raw_mut(x),
                Self::Query(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Note(x) => x.into_raw(),
                Self::Posting(x) => x.into_raw(),
                Self::Query(x) => x.into_raw(),
            }
        }
    }
}
