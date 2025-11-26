/// This script uses enums and structs to 
/// construct an arbitrarily long arithmetic expression
/// such as (1 + 1) * 2
/// and returns the final value.
/// Operators are treated as new branching paths in the expression.
/// We will use recursion to solve this problem.

/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    /// 'Box' is a smart pointer that allows us to
    /// inject another instance of enum Expression from within Expression.
    /// So the left and right parts of an operation can either be a value
    /// or a branching subexpression with its own operators and subexpressions.
    /// This allows us to create an infinitely long arithmetic expression.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value.
    /// The simplest thing an expression can be.
    /// The base case is where the expression tree is just one value.
    /// Subexpressions can also be set to a single value.
    /// This terminates that subexpression branch.
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    // First determine if Expression is just a value
    // or if it contains an operation.
    // This is the base case of our recursion.
    match e {
        Expression::Value(v) => {  // Just a value, return that value.
            return v;
        }
        Expression::Op { op: o, left: l, right: r } => {  // Has operator.
            // Determine which operator is contained in Expression.
            // Then unbox the left and right Expression enums.
            // Call the eval function with the unboxed Expression enums
            // which continues our recursion down to the base case.
            match o {
                Operation::Add => {
                    return eval(*l) + eval(*r);
                }
                Operation::Sub => {
                    return eval(*l) - eval(*r);
                }
                Operation::Mul => {
                    return eval(*l) * eval(*r);
                }
                Operation::Div => {
                    return eval(*l) / eval(*r);
                }
            }
        }
    }
}

//fn main() {
//    println!("{}", eval(Expression::Value(19)));
//}

// Below we have all of our tests
// which have been verified to pass!

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}

#[test]
fn test_div() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(2)),
        }),
        5
    )
}
