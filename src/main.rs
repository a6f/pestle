use pest_meta::ast::{Expr, RuleType};
use pest_meta::parser::{consume_rules, parse, Rule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grammar_filename = std::env::args().nth(1).unwrap();
    let grammar_source = std::fs::read_to_string(grammar_filename).unwrap();
    let grammar_tree = parse(Rule::grammar_rules, &grammar_source).unwrap();
    let grammar_rules = consume_rules(grammar_tree).unwrap();
    for rule in grammar_rules {
        if rule.ty == RuleType::Silent {
            continue;
        }
        let name = &rule.name;
        if matches!(rule.ty, RuleType::Atomic) || matches!(rule.expr, Expr::Str(_)) {
            println!("atomic rule {name}");
        } else if let Some(f) = choice_items(&rule.expr) {
            println!("enum rule {name}: {f:?}");
        } else {
            let f = sequence_items(&rule.expr);
            println!("struct rule {name}: {f:?}");
        }
    }
    Ok(())
}

fn choice_items(expr: &Expr) -> Option<Vec<String>> {
    use core::borrow::Borrow;
    match expr {
        Expr::Choice(a, b) => {
            let mut r = vec![];
            for c in [a.borrow(), b.borrow()] {
                match c {
                    Expr::Ident(n) => r.push(n.clone()),
                    Expr::Choice(..) => r.extend(choice_items(a)?),
                    _ => panic!(
                        "choices must be in dedicated rules (to guide enum variant names): {c:?}"
                    ),
                }
            }
            assert!(r.len() > 1);
            Some(r)
        }
        _ => None,
    }
}

fn sequence_items(expr: &Expr) -> Vec<(String, usize)> {
    type H = Vec<(String, usize)>;
    fn both(mut a: H, b: H) -> H {
        for (n, i) in b {
            if let Some(x) = a.iter_mut().find(|x| x.0 == n) {
                x.1 = 2;
            } else {
                a.push((n, i));
            }
        }
        a
    }
    fn optional(mut a: H) -> H {
        for (_, i) in &mut a {
            *i &= 2; // reduce 1 to 0
        }
        a
    }
    fn repeated(mut a: H) -> H {
        for (_, i) in &mut a {
            *i = 2;
        }
        a
    }
    match expr {
        Expr::Ident(n) => vec![(n.clone(), 1)],
        Expr::Seq(a, b) => both(sequence_items(a), sequence_items(b)),
        Expr::Choice(..) => {
            panic!("choices must be in dedicated rules (to guide enum variant names): {expr:?}")
        }
        Expr::Opt(e) => optional(sequence_items(e)),
        Expr::Rep(e) => repeated(sequence_items(e)),
        e => unimplemented!("can't handle grammar expression {e:?}"),
    }
}
