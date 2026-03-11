use std::collections::HashMap;

mod boolean_rules;
mod playground;

/*
    A term rewriting system is just: pattern match, substitute, repeat.


    d(u + v)/dx -> du/dx + dv/dx
    There is ^ a complex left-hand side that can be simplified into the expression
    appearing at the right-hand side. These expressions are called terms.

    A rule matches subexpressions:
    Rule: add(succ(X), Y) -> succ(add(X, Y))
    Subexpression: add(succ(succ(0)), succ(succ(0)))  <<--  Note: rule matches the entire expression
    Subexpression: succ(add(succ(0), succ(succ(0))))

    Term:
    - A single variable is a term, e.g. X, Y or Z.
    - A function name applied to zero or more arguments is a term, e.g., add(X,Y).

    Note:
    add(a, b), (+ a b), (a + b)  ->>  functions are equivalent

    Rewriting is the mechanism (apply a rule, get a new term). Reduction implies directionality toward simpler forms.


    Three things that define validity: the signature (function names + their arities), arity checking, and unknown symbols.


    Example:
    a(b + c) -> ab + ac
    10(x + y) -> 10x + 10y
    Matching produces the substitution {a → 10, b → x, c → y}, then substitution applies it to
    the right-hand side. Those two steps together — that's one rewrite step.


    Normal form:
    The normal form is whatever rules can't reduce further.

    According to pipeline of the peano axioms rewriting, that expression:
    succ(succ(succ(succ0))))
    would be a normal form

    Bonus, zippers:
    Zipper = "Breadcrumbs" for graph traversal
*/

// TODO -> how to handle same variable appears twice? like Or(X, X)

// peano numbers - are basically encoding
// TODO -> how to implement that encoding from natural number to succ function?

// variables can match eanything except predefine rule names (like succ, add etc.)

// TODO -> how variables bindings for rewriting different from variable bindings of hash-map
// for tree-walk interpreter?

/* -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- */

// Initial term:
// succ(add(succ(0), succ(succ(0))))

// Rules:
// add(succ(X), Y) = succ(add(X, Y))
// add(0, X) = X

// enum Term<'a> {
//     Leaf(&'a str),
//     Node {
//         operator: &'a str,
//         children: Vec<Term<'a>>,
//     },
// }

// struct Rule<'a> {
//     lhs: Term<'a>,
//     rhs: Term<'a>,
// }

// TODO -> how does rules lookup happens?

// Each successful rewrite step gets its own temporary bindings (substitution map),
// produced by matching that step’s redex against that rule’s LHS. Next rewrite step starts fresh
// and computes a new binding map. That is a key difference from an interpreter environment/hashmap,
// which usually persists across evaluation steps.

// NOTE about comparision two trees: need to match operator (plus it's arity?)
// variable: bind x to subterm

// NOTE: comparison -> fails fast on first mismatch. but where exactly?

// TODO -> how to try to lookup different rules?
// TODO -> is it correct to express rule_lhs and rule_rhs as Term?
fn do_rewrite_step(term: Term, rule_lhs: Term, rule_rhs: Term) {
    // Algorithm: substitute subterm with rule_rhs with putted vairables from original term in it

    match (term, rule_lhs) {
        (Term::Leaf(constant), Term::Leaf(variable)) => todo!(),
        (Term::Leaf(_), Term::Node { operator, children }) => todo!(),
        (Term::Node { operator, children }, Term::Leaf(_)) => todo!(),
        (
            Term::Node {
                operator: term_op,
                children: term_children,
            },
            Term::Node {
                operator: rule_op,
                children: rule_children,
            },
        ) => {
            if term_op == rule_op {
                //
            } else {
                // TODO -> how to descend? recrusively
            }
        }
    }
}

// Define key as root + first argument
// type Bucket<'a> = (Term<'a>, Option<Term<'a>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Term<'a> {
    Leaf(&'a str),
    Node {
        operator: &'a str,
        children: Vec<Term<'a>>,
    },
}

impl<'a> Term<'a> {
    fn arity(&self) -> u8 {
        match self {
            Term::Leaf(_) => 1,
            Term::Node { children, .. } => children.len() as u8,
        }
    }

    fn equivalent(&self, another: &Self) -> bool {
        match (self, another) {
            (Term::Leaf(val), Term::Leaf(another_val)) => val == another_val,
            (Term::Leaf(_), Term::Node { .. }) => false,
            (Term::Node { .. }, Term::Leaf(_)) => false,

            // TODO
            (Term::Node { .. }, Term::Node { .. }) => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    lhs: Term<'a>,
    rhs: Term<'a>,
}

impl<'a> Rule<'a> {
    fn extract_key(&self) -> &'a str {
        match &self.lhs {
            Term::Node { operator, .. } => *operator,
            Term::Leaf(_) => panic!("Rule is expected to define operation"),
        }
    }
}

struct Rewriter<'a> {
    term: Term<'a>,

    // TODO -> critical optimization step would assume having key based on arity and/or first argument
    // (as it was drafted at Bucket). Right now two or more rules with same name would imply linear search
    // against that name
    rules: HashMap<&'a str, Vec<Rule<'a>>>,

    // Unlike traditional interpretation, bindings during term rewriting assumed
    // to be newly created for each rewriting step
    bindings: HashMap<&'a str, Term<'a>>,
}

impl<'a> Rewriter<'a> {
    fn new(initial_term: Term<'a>, rules: Vec<Rule<'a>>) -> Self {
        let rules_hm =
            rules
                .into_iter()
                .fold(HashMap::<&'a str, Vec<Rule<'a>>>::new(), |mut acc, rule| {
                    acc.entry(rule.extract_key()).or_default().push(rule);
                    acc
                });

        Rewriter {
            term: initial_term,
            rules: rules_hm,
            bindings: HashMap::new(),
        }
    }

    fn do_traversal(&mut self) {
        while let Some(rewrited_term) = self.try_rewrite(self.term.clone()) {
            self.term = rewrited_term;
        }

        dbg!(&self.term);
    }

    /// Picking redex candidate is based on the operator match. Everything else -
    /// two trees traversal in attempt to fully match this candidate
    fn try_rewrite(&self, mut term: Term<'a>) -> Option<Term<'a>> {
        match &mut term {
            Term::Leaf(_) => {
                // Early return: term/subterm is already normalized
                return None;
            }
            Term::Node {
                operator,
                children: term_children,
            } => {
                if let Some(rule_candidates) = &self.rules.get(operator) {
                    //

                    for rule_index in 0..rule_candidates.len() {
                        let rule_candidate = &rule_candidates[rule_index];

                        // TODO -> tracing

                        match &rule_candidate.lhs {
                            Term::Node {
                                children: sub_rules,
                                ..
                            } => {
                                // Arity expected to be match since operator matched
                                let arity = &term_children.len();

                                let mut match_counter = 0;

                                for k in 0..*arity {
                                    let subterm = term_children[k].clone();
                                    let sub_rule = &sub_rules[k];

                                    if subterm.equivalent(sub_rule) {
                                        match_counter += 1;
                                    }
                                }

                                if match_counter == *arity {
                                    // Return substitution:
                                    return Some(rule_candidate.rhs.clone());
                                } else {
                                    // Check if tried all rules
                                    if rule_index == &rule_candidates.len() - 1 {
                                        // Continue descending with attempt to find other matches
                                        for j in 0..term_children.len() {
                                            let subterm = &term_children[j];

                                            if let Term::Node { .. } = subterm {
                                                if let Some(rewrited_subterm) =
                                                    self.try_rewrite(subterm.clone())
                                                {
                                                    term_children[j] = rewrited_subterm;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Term::Leaf(_) => panic!("Unexpected rule leaf"),
                        }
                    }

                    return Some(term);
                    // return None;
                } else {
                    // TODO -> no rules find.

                    todo!()
                }
            }
        }
    }
}

/// Redex selection:
/// from the top (root) to the bottom and left-to right order
/// although left-most innermost (leaves-to-root) is also possible

/// Two distinct steps:
/// - find where in the term a rule applies -> redex selection
/// - check if a specific subterm matches a rule's LHS -> traverses two trees

// QUESTIONS:
// - Variables definition and substitution for the boolean expressions

// Impl plan:
// - Rewriting simple boolean rules (no variables)
// - Boolean rules with variables
// - Arithmetic (peano axiom) rules rewriting

fn main() {
    playground::playground();

    //
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boolean_rules::boolean_rules;

    fn assert_rewrites_to(initial_term: Term<'static>, expected: Term<'static>) {
        let rules = boolean_rules();
        let mut rewriter = Rewriter::new(initial_term, rules);
        rewriter.do_traversal();
        assert_eq!(rewriter.term, expected);
    }

    #[test]
    fn boolean_rules_initialization() {
        // // not(or(false, and(true,not(false))))  ->>  false
        let initial_term = Term::Node {
            operator: "not",
            children: vec![Term::Node {
                operator: "or",
                children: vec![
                    Term::Leaf("false"),
                    Term::Node {
                        operator: "and",
                        children: vec![
                            Term::Leaf("true"),
                            Term::Node {
                                operator: "not",
                                children: vec![Term::Leaf("false")],
                            },
                        ],
                    },
                ],
            }],
        };

        let expected = Term::Leaf("false");
        assert_rewrites_to(initial_term, expected);
    }

    #[test]
    fn boolean_rules_and_false_not_true() {
        // and(false, not(true))
        let initial_term = Term::Node {
            operator: "and",
            children: vec![
                Term::Leaf("false"),
                Term::Node {
                    operator: "not",
                    children: vec![Term::Leaf("true")],
                },
            ],
        };

        let expected = Term::Leaf("false");
        assert_rewrites_to(initial_term, expected);
    }

    #[test]
    fn boolean_rules_simple_or_and_not_cases() {
        // or(false, true) = true
        let or_ft = Term::Node {
            operator: "or",
            children: vec![Term::Leaf("false"), Term::Leaf("true")],
        };
        assert_rewrites_to(or_ft, Term::Leaf("true"));

        // and(true, false) = false
        let and_tf = Term::Node {
            operator: "and",
            children: vec![Term::Leaf("true"), Term::Leaf("false")],
        };
        assert_rewrites_to(and_tf, Term::Leaf("false"));

        // not(false) = true
        let not_f = Term::Node {
            operator: "not",
            children: vec![Term::Leaf("false")],
        };
        assert_rewrites_to(not_f, Term::Leaf("true"));
    }

    #[test]
    fn boolean_rules_nested_or_and_not_1() {
        // or(and(true, false), not(false))  ==>  or(false, true)  ==>  true
        let initial_term = Term::Node {
            operator: "or",
            children: vec![
                Term::Node {
                    operator: "and",
                    children: vec![Term::Leaf("true"), Term::Leaf("false")],
                },
                Term::Node {
                    operator: "not",
                    children: vec![Term::Leaf("false")],
                },
            ],
        };

        assert_rewrites_to(initial_term, Term::Leaf("true"));
    }

    #[test]
    fn boolean_rules_nested_or_and_not_2() {
        // not(and(or(true, false), not(false)))
        // or(true, false)  ==>  true
        // not(false)       ==>  true
        // and(true, true)  ==>  true
        // not(true)        ==>  false
        let initial_term = Term::Node {
            operator: "not",
            children: vec![Term::Node {
                operator: "and",
                children: vec![
                    Term::Node {
                        operator: "or",
                        children: vec![Term::Leaf("true"), Term::Leaf("false")],
                    },
                    Term::Node {
                        operator: "not",
                        children: vec![Term::Leaf("false")],
                    },
                ],
            }],
        };

        assert_rewrites_to(initial_term, Term::Leaf("false"));
    }

    #[test]
    fn boolean_rules_nested_or_and_not_3() {
        // and(or(false, false), not(or(false, false)))
        // or(false, false)      ==> false
        // not(or(false, false)) ==> not(false) ==> true
        // and(false, true)      ==> false
        let initial_term = Term::Node {
            operator: "and",
            children: vec![
                Term::Node {
                    operator: "or",
                    children: vec![Term::Leaf("false"), Term::Leaf("false")],
                },
                Term::Node {
                    operator: "not",
                    children: vec![Term::Node {
                        operator: "or",
                        children: vec![Term::Leaf("false"), Term::Leaf("false")],
                    }],
                },
            ],
        };

        assert_rewrites_to(initial_term, Term::Leaf("false"));
    }

    #[test]
    fn arithmetic_rules_initialization() {
        // add(S(0), S(S(0)))
        let initial_term = Term::Node {
            operator: "add",
            children: vec![
                Term::Node {
                    operator: "succ",
                    children: vec![Term::Leaf("0")],
                },
                Term::Node {
                    operator: "succ",
                    children: vec![Term::Node {
                        operator: "succ",
                        children: vec![Term::Leaf("0")],
                    }],
                },
            ],
        };

        // add(S(X), Y) -> S(add(X, Y))
        let rule_1 = Rule {
            lhs: Term::Node {
                operator: "add",
                children: vec![
                    Term::Node {
                        operator: "succ",
                        children: vec![Term::Leaf("$X")],
                    },
                    Term::Leaf("Y"),
                ],
            },
            rhs: Term::Node {
                operator: "succ",
                children: vec![Term::Node {
                    operator: "add",
                    children: vec![Term::Leaf("$X"), Term::Leaf("$Y")],
                }],
            },
        };

        // add(0, X) -> X
        let rule_2 = Rule {
            lhs: Term::Node {
                operator: "add",
                // NOTE: "0" - is a constant here
                children: vec![Term::Leaf("0"), Term::Leaf("$X")],
            },
            rhs: Term::Leaf("X"),
        };

        let rules = vec![rule_1, rule_2];

        let _rewriter = Rewriter::new(initial_term, rules);
    }
}
