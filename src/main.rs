#![allow(dead_code)]
#![allow(unused_imports)]
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit0, digit1},
    combinator::opt,
    multi::many0,
    sequence::{delimited, pair, tuple},
    IResult, Parser,
};
use pretty_assertions::assert_eq;

fn main() {
    process("2 + 5 - 4 / 2");
}

fn process(input: &str) -> String {
    let clean = input.replace(" ", "");
    let (_remaining, result) = equation_parser(&clean).unwrap();

    result.solve().to_string()
}

fn test_process(input: &str) -> Equation {
    let clean = input.replace(" ", "");
    let (_remaining, result) = equation_parser(&clean).unwrap();

    result
}

#[derive(Debug, PartialEq)]
enum Operator {
    Addition,
    Substract,
    Multiple,
    Divide,
}

impl Operator {
    fn solve(&self, left: f32, right: f32) -> f32 {
        match self {
            Operator::Addition => left + right,
            Operator::Substract => left - right,
            Operator::Multiple => left * right,
            Operator::Divide => left / right,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Sign {
    POSITIVE,
    NIGATIVE,
}

#[derive(Debug, PartialEq)]
struct Number {
    sign: Vec<Sign>,
    value: f32,
}
impl Number {
    fn solve(&self) -> f32 {
        let signer: f32 = if self.sign.iter().filter(|x| x == &&Sign::NIGATIVE).count() % 2 != 0 {
            -1.0
        } else {
            1.0
        };
        self.value as f32 * signer
    }
}

#[derive(Debug, PartialEq)]
enum Equation {
    Number(Number),
    Part(Box<Operation>),
}

impl Equation {
    fn solve(&self) -> f32 {
        match self {
            Equation::Number(number) => number.solve(),
            Equation::Part(parent) => parent.solve(),
        }
    }
}
#[derive(Debug, PartialEq)]
struct Operation {
    left: Equation,
    operator: Operator,
    right: Equation,
}

impl Operation {
    fn solve(&self) -> f32 {
        self.operator.solve(self.left.solve(), self.right.solve())
    }
}

fn sign_parser(input: &str) -> IResult<&str, Sign> {
    alt((
        tag("+").map(|_x| Sign::POSITIVE),
        tag("-").map(|_x| Sign::NIGATIVE),
    ))
    .parse(input)
}

fn number_parser(input: &str) -> IResult<&str, Number> {
    pair(many0(sign_parser), digit1)
        .map(|(signs, number)| Number {
            sign: signs,
            value: number.parse::<f32>().unwrap(),
        })
        .parse(input)
}

fn add_sub_operator_parser(input: &str) -> IResult<&str, Operator> {
    alt((
        tag("+").map(|_x| Operator::Addition),
        tag("-").map(|_x| Operator::Substract),
    ))
    .parse(input)
}
fn multi_div_operator_parser(input: &str) -> IResult<&str, Operator> {
    alt((
        tag("*").map(|_x| Operator::Multiple),
        tag("/").map(|_x| Operator::Divide),
    ))
    .parse(input)
}

fn equation_parser(input: &str) -> IResult<&str, Equation> {
    pair(
        multi_div_equation_parser,
        many0(pair(add_sub_operator_parser, multi_div_equation_parser)),
    )
    .map(|(left, list)| {
        list.into_iter().fold(left, |acc, (operator, right)| {
            Equation::Part(Box::new(Operation {
                left: acc,
                operator,
                right,
            }))
        })
    })
    .parse(input)
}
fn branching_parser(input: &str) -> IResult<&str, Equation> {
    alt((
        number_parser.map(|x| Equation::Number(x)),
        pair(
            many0(sign_parser),
            delimited(tag("("), equation_parser, tag(")")),
        )
        .map(|(signs, equation)| {
            let number = Number {
                value: 1.0,
                sign: signs,
            };

            if number.solve() == 1.0 {
                return equation;
            }

            Equation::Part(Box::new(Operation {
                left: Equation::Number(number),
                operator: Operator::Multiple,
                right: equation,
            }))
        }),
    ))
    .parse(input)
}

fn multi_div_equation_parser(input: &str) -> IResult<&str, Equation> {
    pair(
        branching_parser,
        many0(pair(multi_div_operator_parser, branching_parser)),
    )
    .map(|(left, list)| {
        list.into_iter().fold(left, |acc, (operator, right)| {
            Equation::Part(Box::new(Operation {
                left: acc,
                operator,
                right,
            }))
        })
    })
    .parse(input)
}

#[test]
fn milestone_1() {
    assert_eq!(
        test_process("5"),
        Equation::Number(Number {
            value: 5.0,
            sign: vec![]
        })
    );
    assert_eq!(
        test_process("(5)"),
        Equation::Number(Number {
            value: 5.0,
            sign: vec![]
        })
    );
    assert_eq!(
        test_process("5 + 7"),
        Equation::Part(Box::new(Operation {
            left: Equation::Number(Number {
                value: 5.0,
                sign: vec![]
            }),
            operator: Operator::Addition,
            right: Equation::Number(Number {
                value: 7.0,
                sign: vec![]
            })
        }))
    );
    assert_eq!(
        test_process("(5 + 7)"),
        Equation::Part(Box::new(Operation {
            left: Equation::Number(Number {
                value: 5.0,
                sign: vec![]
            }),
            operator: Operator::Addition,
            right: Equation::Number(Number {
                value: 7.0,
                sign: vec![]
            })
        }))
    );
    assert_eq!(
        test_process("5 + 7 - 3"),
        Equation::Part(Box::new(Operation {
            left: Equation::Part(Box::new(Operation {
                left: Equation::Number(Number {
                    value: 5.0,
                    sign: vec![]
                }),
                operator: Operator::Addition,
                right: Equation::Number(Number {
                    value: 7.0,
                    sign: vec![]
                })
            })),
            operator: Operator::Substract,
            right: Equation::Number(Number {
                value: 3.0,
                sign: vec![]
            })
        }))
    );
    assert_eq!(
        test_process("(5 + 7) - 3"),
        Equation::Part(Box::new(Operation {
            left: Equation::Part(Box::new(Operation {
                left: Equation::Number(Number {
                    value: 5.0,
                    sign: vec![]
                }),
                operator: Operator::Addition,
                right: Equation::Number(Number {
                    value: 7.0,
                    sign: vec![]
                })
            })),
            operator: Operator::Substract,
            right: Equation::Number(Number {
                value: 3.0,
                sign: vec![]
            })
        }))
    );
    assert_eq!(
        test_process("5 + (7 - 3)"),
        Equation::Part(Box::new(Operation {
            left: Equation::Number(Number {
                value: 5.0,
                sign: vec![]
            }),
            operator: Operator::Addition,
            right: Equation::Part(Box::new(Operation {
                left: Equation::Number(Number {
                    value: 7.0,
                    sign: vec![]
                }),
                operator: Operator::Substract,
                right: Equation::Number(Number {
                    value: 3.0,
                    sign: vec![]
                })
            })),
        }))
    );
    assert_eq!(
        test_process("5 + ((7 + 1) - 3)"),
        Equation::Part(Box::new(Operation {
            left: Equation::Number(Number {
                value: 5.0,
                sign: vec![]
            }),
            operator: Operator::Addition,
            right: Equation::Part(Box::new(Operation {
                left: Equation::Part(Box::new(Operation {
                    left: Equation::Number(Number {
                        value: 7.0,
                        sign: vec![]
                    }),
                    operator: Operator::Addition,
                    right: Equation::Number(Number {
                        value: 1.0,
                        sign: vec![]
                    })
                })),
                operator: Operator::Substract,
                right: Equation::Number(Number {
                    value: 3.0,
                    sign: vec![]
                })
            })),
        }))
    );
    assert_eq!(
        test_process("5 * (7 - 3)"),
        Equation::Part(Box::new(Operation {
            left: Equation::Number(Number {
                value: 5.0,
                sign: vec![]
            }),
            operator: Operator::Multiple,
            right: Equation::Part(Box::new(Operation {
                left: Equation::Number(Number {
                    value: 7.0,
                    sign: vec![]
                }),
                operator: Operator::Substract,
                right: Equation::Number(Number {
                    value: 3.0,
                    sign: vec![]
                })
            })),
        }))
    );
    assert_eq!(
        test_process("5 + 7 - 3 + 2"),
        Equation::Part(Box::new(Operation {
            left: Equation::Part(Box::new(Operation {
                left: Equation::Part(Box::new(Operation {
                    left: Equation::Number(Number {
                        sign: vec![],
                        value: 5.0
                    }),
                    operator: Operator::Addition,
                    right: Equation::Number(Number {
                        sign: vec![],
                        value: 7.0
                    })
                })),
                operator: Operator::Substract,
                right: Equation::Number(Number {
                    sign: vec![],
                    value: 3.0
                })
            })),
            operator: Operator::Addition,
            right: Equation::Number(Number {
                sign: vec![],
                value: 2.0
            })
        }))
    );
    assert_eq!(process("5 + 5 + 5 + 5"), "20".to_string());
    assert_eq!(process("7 - 5 + 1 - 3"), "0".to_string());
    assert_eq!(process("5 - 7"), "-2".to_string());
    assert_eq!(process("2 * 3"), "6".to_string());
    assert_eq!(process("2 * 3 + 1"), "7".to_string());
    assert_eq!(process("2 * 2 * 3 + 1"), "13".to_string());
    assert_eq!(process("2 + 1 * 3 / 3"), "3".to_string());
    assert_eq!(process("8 / 2"), "4".to_string());
    assert_eq!(process("-(3 - 2)"), "-1".to_string());
    assert_eq!(process("1 - -(5)"), "6".to_string());
    assert_eq!(process("5 + (3 + 7) * 99"), "995".to_string());
    assert_eq!(process("2 / 4"), "0.5".to_string());
}
