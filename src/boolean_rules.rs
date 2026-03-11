use crate::{Rule, Term};

/// Boolean rewrite rules:
/// - [or1] or(true, true) = true
/// - [or2] or(true, false) = true
/// - [or3] or(false, true) = true
/// - [or4] or(false, false) = false
/// - [and1] and(true, true) = true
/// - [and2] and(true, false) = false
/// - [and3] and(false, true) = false
/// - [and4] and(false, false) = false
/// - [not1] not(true) = false
/// - [not2] not(false) = true
pub fn boolean_rules() -> Vec<Rule<'static>> {
    // [or1] or(true, true) = true
    let or_1 = Rule {
        lhs: Term::Node {
            operator: "or",
            children: vec![Term::Leaf("true"), Term::Leaf("true")],
        },
        rhs: Term::Leaf("true"),
    };

    // [or2] or(true, false) = true
    let or_2 = Rule {
        lhs: Term::Node {
            operator: "or",
            children: vec![Term::Leaf("true"), Term::Leaf("false")],
        },
        rhs: Term::Leaf("true"),
    };

    // [or3] or(false, true) = true
    let or_3 = Rule {
        lhs: Term::Node {
            operator: "or",
            children: vec![Term::Leaf("false"), Term::Leaf("true")],
        },
        rhs: Term::Leaf("true"),
    };

    // [or4] or(false, false) = false
    let or_4 = Rule {
        lhs: Term::Node {
            operator: "or",
            children: vec![Term::Leaf("false"), Term::Leaf("false")],
        },
        rhs: Term::Leaf("false"),
    };

    // [and1] and(true, true) = true
    let and_1 = Rule {
        lhs: Term::Node {
            operator: "and",
            children: vec![Term::Leaf("true"), Term::Leaf("true")],
        },
        rhs: Term::Leaf("true"),
    };

    // [and2] and(true, false) = false
    let and_2 = Rule {
        lhs: Term::Node {
            operator: "and",
            children: vec![Term::Leaf("true"), Term::Leaf("false")],
        },
        rhs: Term::Leaf("false"),
    };

    // [and3] and(false, true) = false
    let and_3 = Rule {
        lhs: Term::Node {
            operator: "and",
            children: vec![Term::Leaf("false"), Term::Leaf("true")],
        },
        rhs: Term::Leaf("false"),
    };

    // [and4] and(false, false) = false
    let and_4 = Rule {
        lhs: Term::Node {
            operator: "and",
            children: vec![Term::Leaf("false"), Term::Leaf("false")],
        },
        rhs: Term::Leaf("false"),
    };

    // [not1] not(true) = false
    let not_1 = Rule {
        lhs: Term::Node {
            operator: "not",
            children: vec![Term::Leaf("true")],
        },
        rhs: Term::Leaf("false"),
    };

    // [not2] not(false) = true
    let not_2 = Rule {
        lhs: Term::Node {
            operator: "not",
            children: vec![Term::Leaf("false")],
        },
        rhs: Term::Leaf("true"),
    };

    vec![
        or_1, or_2, or_3, or_4, and_1, and_2, and_3, and_4, not_1, not_2,
    ]
}
