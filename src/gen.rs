//! Typed grammar generated by pestle from src/grammar.pest

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

// silent rule Sentence generates no code

/// sequence rule Sequence
#[derive(Debug)]
pub struct Sequence<'i> {
    _span: Span<'i>,
    pub atom1: &'i Atom1<'i>,
    pub choice: &'i Vec<'i, &'i Choice<'i>>,
    pub atom3: Option<&'i Atom3<'i>>,
}

impl<'i> TypedRule<'i> for Sequence<'i> {
    const UNTYPED_RULE: Rule = Rule::Sequence;
    fn span(&self) -> &Span<'i> {
        &self._span
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        let _span = pair.as_span();
        let mut _tmp_atom1 = Vec::<&'i Atom1>::new_in(alloc);
        let mut _tmp_choice = Vec::<&'i Choice>::new_in(alloc);
        let mut _tmp_atom3 = Vec::<&'i Atom3>::new_in(alloc);

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::Atom1 => _tmp_atom1.push(Atom1::build(child, alloc)),
                Rule::Choice => _tmp_choice.push(Choice::build(child, alloc)),
                Rule::Atom3 => _tmp_atom3.push(Atom3::build(child, alloc)),

                rule => panic!("unexpected rule {rule:?} within {:?}", Self::UNTYPED_RULE),
            }
        }
        alloc.alloc(Self {
            _span,
            atom1: to_singleton(_tmp_atom1),
            choice: alloc.alloc(_tmp_choice),
            atom3: to_option(_tmp_atom3),
        })
    }
}

/// enum rule Choice
#[derive(Debug)]
pub enum Choice<'i> {
    Atom2(&'i Atom2<'i>),
    Sequence(&'i Sequence<'i>),
}

impl<'i> TypedRule<'i> for Choice<'i> {
    const UNTYPED_RULE: Rule = Rule::Choice;
    fn span(&self) -> &Span<'i> {
        match self {
            Self::Atom2(x) => x.span(),
            Self::Sequence(x) => x.span(),
        }
    }
    fn build(pair: Pair<'i, Rule>, alloc: &'i Bump) -> &'i Self {
        assert_eq!(Self::UNTYPED_RULE, pair.as_rule());
        let inner = pair.into_inner().next().unwrap();
        alloc.alloc(match inner.as_rule() {
            Rule::Atom2 => Choice::Atom2(Atom2::build(inner, alloc)),
            Rule::Sequence => Choice::Sequence(Sequence::build(inner, alloc)),

            rule => panic!("unexpected rule {rule:?} within {:?}", Self::UNTYPED_RULE),
        })
    }
}

/// atomic rule Atom1
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

/// atomic rule Atom2
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

/// atomic rule Atom3
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
