/*

generate a typed AST from an untyped pest parse result

all we need to know for each rule is:
  is it atomic/literal, else
  what are the subrules
  is it an enum
  how many times can each appear (1, ?, *)

then we can generate types.

the types are mutually recursive, so store them all in an arena and embed references.

consume_rules() / consume_expr() in pest-meta are good examples of parsing to enums
https://docs.rs/pest_meta/2.7.15/pest_meta/parser/fn.consume_rules.html

// TODO: hand impl Debug/Display?

*/

use super::Rule;
use bumpalo::collections::Vec;
use bumpalo::Bump;
use pest::iterators::Pair;
use pest::Span;

pub trait TypedRule<'i> {
    const UNTYPED_RULE: Rule;
    fn span(&self) -> &Span<'i>;
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self;
}

#[derive(Debug)]
pub struct Atom1<'i>(Span<'i>);

impl<'i> TypedRule<'i> for Atom1<'i> {
    const UNTYPED_RULE: Rule = Rule::Atom1;
    fn span(&self) -> &Span<'i> {
        &self.0
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        alloc.alloc(Self(pair.as_span()))
    }
}

#[derive(Debug)]
pub struct Atom2<'i>(Span<'i>);

impl<'i> TypedRule<'i> for Atom2<'i> {
    const UNTYPED_RULE: Rule = Rule::Atom2;
    fn span(&self) -> &Span<'i> {
        &self.0
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        alloc.alloc(Self(pair.as_span()))
    }
}

#[derive(Debug)]
pub struct Atom3<'i>(Span<'i>);

impl<'i> TypedRule<'i> for Atom3<'i> {
    const UNTYPED_RULE: Rule = Rule::Atom3;
    fn span(&self) -> &Span<'i> {
        &self.0
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        alloc.alloc(Self(pair.as_span()))
    }
}

#[derive(Debug)]
pub enum Choice<'i> {
    Atom1(&'i Atom1<'i>),
    Atom2(&'i Atom2<'i>),
    Sequence(&'i Sequence<'i>),
}

impl<'i> TypedRule<'i> for Choice<'i> {
    const UNTYPED_RULE: Rule = Rule::Choice;
    fn span(&self) -> &Span<'i> {
        match self {
            Self::Atom1(x) => x.span(),
            Self::Atom2(x) => x.span(),
            Self::Sequence(x) => x.span(),
        }
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        let inner = pair.into_inner().next().unwrap();
        alloc.alloc(match inner.as_rule() {
            Rule::Atom1 => Choice::Atom1(Atom1::build(inner, alloc)),
            Rule::Atom2 => Choice::Atom2(Atom2::build(inner, alloc)),
            Rule::Sequence => Choice::Sequence(Sequence::build(inner, alloc)),
            rule => panic!("unexpected rule {rule:?} within {:?}", Self::UNTYPED_RULE),
        })
    }
}

#[derive(Debug)]
pub struct Sequence<'i> {
    _span: Span<'i>,
    pub atom1: &'i Atom1<'i>,
    pub atom2: Option<&'i Atom2<'i>>,
    pub choice: &'i Vec<'i, &'i Choice<'i>>,
    pub atom3: &'i Vec<'i, &'i Atom3<'i>>,
}

impl<'i> TypedRule<'i> for Sequence<'i> {
    const UNTYPED_RULE: Rule = Rule::Sequence;
    fn span(&self) -> &Span<'i> {
        &self._span
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        let _span = pair.as_span();
        let mut atom1: Vec<&'i Atom1> = Vec::new_in(&alloc);
        let mut atom2: Vec<&'i Atom2> = Vec::new_in(&alloc);
        let mut choice: Vec<&'i Choice> = Vec::new_in(&alloc);
        let mut atom3: Vec<&'i Atom3> = Vec::new_in(&alloc);
        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::Atom1 => atom1.push(Atom1::build(child, alloc)),
                Rule::Atom2 => atom2.push(Atom2::build(child, alloc)),
                Rule::Choice => choice.push(Choice::build(child, alloc)),
                Rule::Atom3 => atom3.push(Atom3::build(child, alloc)),
                rule => panic!("unexpected rule {rule:?} within {:?}", Self::UNTYPED_RULE),
            }
        }
        let atom1 = to_singleton(atom1);
        let atom2 = to_option(atom2);
        let choice = alloc.alloc(choice);
        let atom3 = alloc.alloc(atom3);
        alloc.alloc(Sequence {
            _span,
            atom1,
            atom2,
            choice,
            atom3,
        })
    }
}

fn to_singleton<T>(mut v: Vec<T>) -> T {
    match v.len() {
        1 => v.pop().unwrap(),
        n => panic!("expected exactly one item, got {n}"),
    }
}

fn to_option<T>(mut v: Vec<T>) -> Option<T> {
    match v.len() {
        0 => None,
        1 => v.pop(),
        n => panic!("expected at most one item, got {n}"),
    }
}
