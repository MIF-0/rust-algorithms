use crate::AlgoError;
use std::fmt::Display;

pub trait UnionFind: Display {
    fn add(&mut self, object: usize) -> Result<(), AlgoError>;
    fn union(&mut self, first: &usize, second: &usize) -> Result<(), AlgoError>;
    fn connected(&self, first: &usize, second: &usize) -> Result<bool, AlgoError>;
}
