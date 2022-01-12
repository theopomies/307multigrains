use std::iter::repeat;

use crate::rational::Rational;

#[derive(Debug, Clone)]
pub struct TableauBuilder {
    constraints: Vec<Constraint>,
    target_function: Constraint,
}

#[derive(Debug, Clone)]
pub struct Tableau {
    constraints: Vec<Constraint>,
    target_function: Constraint,
}

#[derive(Debug, Clone)]
struct Constraint {
    coefficients: Vec<Rational>,
    value: Rational,
}

impl TableauBuilder {
    pub fn new(target_function: &[u64]) -> Self {
        TableauBuilder {
            constraints: vec![],
            target_function: Constraint {
                coefficients: target_function
                    .iter()
                    .map(|c| -Rational::from(*c))
                    .collect(),
                value: 1.into(),
            },
        }
    }

    pub fn add_constraint(mut self, coefficients: &[u64], value: u64) -> Result<Self, String> {
        let len = coefficients.len() + self.constraints.len();

        if self.constraints.iter().any(|c| c.coefficients.len() != len) {
            return Err("Invalid number of coefficients.".into());
        }

        if self.target_function.coefficients.len() != len {
            return Err("Invalid number of coefficients.".into());
        }

        self.constraints.push(Constraint {
            coefficients: coefficients
                .iter()
                .copied()
                .map(Rational::from)
                .chain(repeat(0.into()).take(self.constraints.len()))
                .collect(),
            value: value.into(),
        });

        self.constraints
            .iter_mut()
            .for_each(|c| c.coefficients.push(0.into()));

        *self
            .constraints
            .last_mut()
            .unwrap()
            .coefficients
            .last_mut()
            .unwrap() = 1.into();

        self.target_function.coefficients.push(0.into());
        Ok(self)
    }

    pub fn get_tableau(self) -> Tableau {
        Tableau {
            constraints: self.constraints,
            target_function: self.target_function,
        }
    }
}
