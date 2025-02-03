//! Dice notation parsing.
//!
//! At the moment, I'm only going to support the very basic
//! XdY notation, with + - * / operators.
//!
//! A more long-term goal could be feature-parity with the
//! [d20](https://d20.readthedocs.io/en/latest/start.html#dice-syntax) dice parser,
//! as seen in Avrae.
//!

use std::{str::FromStr, sync::LazyLock};

use chumsky::{error::Simple, BoxedParser, Parser};
use serde::Deserialize;

use crate::core::dice::{expr::QDie, DExpr, Die};

thread_local! {
    static EXPR_PARSER: LazyLock<BoxedParser<'static, char, DExpr, Simple<char>>> = LazyLock::new(|| {
        use chumsky::prelude::*;
        use chumsky::text::int;

        let constant = choice((just("+"), just("-"), empty().to("")))
            .then(int::<_, Simple<char>>(10).from_str::<i32>())
            .try_map(|(s, i), span| {
                let err = |_| Simple::custom(span, "Expected an integer here.");
                match s {
                    "-" => i.map(|int| -int).map_err(err),
                    _ => i.map_err(err),
                }
            })
            .map(DExpr::Constant);

        let die_sides = int::<_, Simple<char>>(10)
            .from_str::<u32>()
            .try_map(|s, span| s.map_err(|_| Simple::custom(span, "Expected an integer here.")));

        let die = die_sides
            .or_not()
            .then_ignore(just("d"))
            .then(die_sides.map(|sides| Die(sides as i32)))
            .map(|(qty, die)| (qty.unwrap_or(1), die))
            .then(choice((just("kh1"), just("kl1"))).or_not())
            .validate(|((qty, die), m), span, emit| {
                let qty_check = || {
                    if qty != 2 {
                        emit(Simple::custom(
                            span,
                            "Expected 2dX for an advantage or disadvantage expression.",
                        ))
                    }
                };

                match m {
                    Some("kh1") => {
                        qty_check();
                        DExpr::Advantage(die)
                    }
                    Some("kl1") => {
                        qty_check();
                        DExpr::Disadvantage(die)
                    }
                    None => DExpr::Die {
                        die: QDie(qty, die),
                        both_adv_dis: false,
                    },
                    _ => unreachable!(),
                }
            });

        recursive::<char, DExpr, _, _, Simple<char>>(|expr| {
            let unary = choice((
                die,
                constant,
                expr.padded().delimited_by(just("("), just(")")),
            ));

            #[derive(Debug, Clone, Copy)]
            enum Op {
                Add,
                Sub,
                Mul,
                Div,
            }

            impl Op {
                fn op(self, lhs: DExpr, rhs: Box<DExpr>) -> DExpr {
                    let lhs = Box::new(lhs);
                    match self {
                        Op::Add => DExpr::Add(lhs, rhs),
                        Op::Sub => DExpr::Sub(lhs, rhs),
                        Op::Mul => DExpr::Mul(lhs, rhs),
                        Op::Div => DExpr::Div(lhs, rhs),
                    }
                }
            }

            let mul_div = unary
                .clone()
                .then(
                    choice((
                        just("*").padded().to(Op::Mul),
                        just("/").padded().to(Op::Div),
                    ))
                    .then(unary.map(Box::new))
                    .repeated(),
                )
                .map(|(unary, md): (DExpr, Vec<(Op, Box<DExpr>)>)| {
                    md.into_iter().fold(unary, |lhs, (op, rhs)| op.op(lhs, rhs))
                });

            mul_div
                .clone()
                .then(
                    choice((
                        just("+").padded().to(Op::Add),
                        just("-").padded().to(Op::Sub),
                    ))
                    .then(mul_div.map(Box::new))
                    .repeated(),
                )
                .map(|(unary, md): (DExpr, Vec<(Op, Box<DExpr>)>)| {
                    md.into_iter().fold(unary, |lhs, (op, rhs)| op.op(lhs, rhs))
                })
        })
        .boxed()
    });
}

impl FromStr for DExpr {
    type Err = DParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EXPR_PARSER
            .with(|parser| parser.parse(s))
            .map_err(DParseError)
    }
}

impl TryFrom<&str> for DExpr {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DExprRaw {
    Str(String),
    Const(i32),
}

#[derive(Debug, Clone)]
pub struct DParseError(Vec<Simple<char>>);

impl std::fmt::Display for DParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: use ariadne.
        for err in &self.0 {
            writeln!(f, "{err}")?;
        }

        Ok(())
    }
}

impl TryFrom<DExprRaw> for DExpr {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: DExprRaw) -> Result<Self, Self::Error> {
        match value {
            DExprRaw::Str(ref s) => DExpr::from_str(s),
            DExprRaw::Const(modifier) => Ok(Self::Constant(modifier)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::dice::{self, DExpr};

    #[test]
    fn test_parse() {
        dice::set_seed(0);
        let p: DExpr = "5d20 + 3".parse::<DExpr>().unwrap();
        let r = p.evaluate();
        print!("{p} => {r} => {}", r.result());
    }
}
