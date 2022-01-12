use std::{fmt::Display, iter::repeat};

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
pub struct SolvedSimplex {
    coefs: Vec<Rational>,
}

#[derive(Debug, Clone)]
pub struct Constraint {
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
                value: 0.into(),
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

    pub fn get_tableau(mut self) -> Tableau {
        self.constraints
            .iter_mut()
            .for_each(|c| c.coefficients.push(0.into()));
        self.target_function.coefficients.push(1.into());
        Tableau {
            constraints: self.constraints,
            target_function: self.target_function,
        }
    }
}

impl Tableau {
    pub fn apply_simplex(mut self) -> SolvedSimplex {
        while !self.is_optimal() {
            self.pivot();
        }
        self.into_solved()
    }

    fn pivot(&mut self) {
        let column = self.find_pivot_column();
        let row = self.find_pivot_row(column);
        self.constraints[row].make_pivot_element_unit(column);
        let pivot_row = self.constraints[row].clone();
        self.constraints
            .iter_mut()
            .enumerate()
            .filter(|(idx, _)| *idx != row)
            .for_each(|(_, constraint)| constraint.cancel_from_pivot(column, &pivot_row));
        self.target_function.cancel_from_pivot(column, &pivot_row);
    }

    fn is_optimal(&self) -> bool {
        self.target_function
            .coefficients
            .iter()
            .all(|c| !c.is_neg())
    }

    fn find_pivot_column(&self) -> usize {
        self.target_function
            .coefficients
            .iter()
            .enumerate()
            .fold(
                (None, 0.into()),
                |(idx, acc): (Option<usize>, Rational), (i, coef)| {
                    if coef < &acc {
                        (Some(i), *coef)
                    } else {
                        (idx, acc)
                    }
                },
            )
            .0
            .unwrap()
    }

    fn find_pivot_row(&self, column: usize) -> usize {
        self.constraints
            .iter()
            .enumerate()
            .filter_map(|(row, c)| {
                if c.coefficients[column] == 0.into()
                    || (c.value / c.coefficients[column]) < 0.into()
                {
                    None
                } else {
                    Some((row, c.value / c.coefficients[column]))
                }
            })
            .min_by_key(|(_, v)| *v)
            .unwrap()
            .0
    }

    fn into_solved(mut self) -> SolvedSimplex {
        self.constraints.push(self.target_function);
        let len = &self.constraints[0].coefficients.len();
        let mut coefs = vec![];
        for i in 0..*len {
            let units = self
                .constraints
                .iter()
                .enumerate()
                .filter(|(_, c)| c.coefficients[i] == 1.into())
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();
            if units.len() == 1
                && self
                    .constraints
                    .iter()
                    .filter(|c| c.coefficients[i] != 0.into())
                    .count()
                    == 1
            {
                coefs.push(self.constraints[units[0]].value);
            } else {
                coefs.push(0.into());
            }
        }
        SolvedSimplex { coefs }
    }
}

impl Display for Tableau {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.constraints {
            writeln!(f, "{}", c)?;
        }

        writeln!(f, "{}", self.target_function)
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .coefficients
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{} | {}", s, self.value)
    }
}

impl Constraint {
    fn make_pivot_element_unit(&mut self, column: usize) {
        let element = self.coefficients[column];
        self.coefficients
            .iter_mut()
            .for_each(|coef| *coef /= element);
        self.value /= element;
    }

    fn cancel_from_pivot(&mut self, column: usize, pivot_row: &Constraint) {
        let term = self.coefficients[column];
        if term == 0.into() {
            return;
        }
        self.coefficients
            .iter_mut()
            .enumerate()
            .for_each(|(idx, coef)| *coef = *coef - term * pivot_row.coefficients[idx]);
        self.value = self.value - term * pivot_row.value;
    }
}

impl SolvedSimplex {
    pub fn get_coef(&self, idx: usize) -> f64 {
        self.coefs[idx].into()
    }

    pub fn get_max(&self) -> f64 {
        (*self.coefs.last().unwrap()).into()
    }
}
