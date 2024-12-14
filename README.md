## pestle: typed AST facade for the pest parser

A code generator which produces ergonomic Rust structs and enums from a pest
grammar.  These types are meant for examining a parsed AST&#8212;not modifying
it or constructing a new one.

The grammar must obey one restriction: the choice operator (`|`) may appear
only in rules which use no other operators (these become Rust enums), and all
choices must be named rules (these become the variant names).

Atomic rules become tuple structs containing only a
[Span](https://docs.rs/pest/latest/pest/struct.Span.html).

Other rules become ordinary structs with fields named according to the types of
their children.  All children of the same type are accumulated into one field;
depending on cardinality, this may be a value, an `Option`, or a
[Vec](https://docs.rs/bumpalo/latest/bumpalo/collections/vec/struct.Vec.html).
Each child type appears in parse order, but if there are multiple types,
relative ordering is lost.  (Many rules will be unambiguous, but consider e.g.
`{ A* ~ B ~ A* }`.  The original order could be deduced from spans, however.)

All children are stored as references; this allows representing recursive or
mutually recursive rules.  Since `Span` already infects all the types with a
lifetime parameter (representing the input string), the child references are
given the same lifetime, and an
[allocator](https://docs.rs/bumpalo/latest/bumpalo/struct.Bump.html) is
required during construction.

The generated types, in addition to their public fields, have a `span()`
accessor, and a static function `build()` which can construct them from a
[Pair](https://docs.rs/pest/latest/pest/iterators/struct.Pair.html) and an
allocator.


### Example

```
#[derive(pest_derive::Parser, pestle::TypedRules)]
#[grammar = "src/expr.pest"]
#[typed_mod = "ast"]
pub struct Parser;

pub fn parse<'i>(source: &'i str, arena: &'i Bump) -> &'i ast::Expr<i'> {
    let pair = Parser::parse(Rule::Expr, source).unwrap().next().unwrap();
    ast::Expr::build(pair, arena)
}
```


##### Why

Written out of frustration after examining all the alternatives, implementing a
complete system atop
[pest_typed_derive](https://crates.io/crates/pest_typed_derive), and
discovering it adds quadratic overhead to parsing.
