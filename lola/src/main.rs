struct Identifier(String);
struct Function {
    name: Identifier,
    parameters: Vec<Identifier>,
    expression: Expr,
}

enum Expr {
    Const(bool),
    Var(Identifier),
    Not(Box<Expr>),
    Func { ident: Identifier, param: Vec<Expr> },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

// a | b
// a & b
// !a
// func(a)

fn main() {
    println!("Hello, world!");
}
