use crate::AlgoError::MissingElement;
use std::fmt::Display;
use AlgoError::ElementAlreadyExist;

pub mod dynamic_connectivity;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub enum AlgoError {
    MissingElement(String),
    ElementAlreadyExist(String),
}

impl AlgoError {
    fn missing_element(element_name: &str, element_value: &dyn Display) -> AlgoError {
        return MissingElement(format!(
            "{} with value {} doesn't exist",
            element_name, element_value
        ));
    }

    fn missing_elements(
        first_element_name: &str,
        first_element_value: &dyn Display,
        second_element_name: &str,
        second_element_value: &dyn Display,
    ) -> AlgoError {
        return MissingElement(format!(
            "{} with value {} doesn't exist \n {} with value {} doesn't exist",
            first_element_name, first_element_value, second_element_name, second_element_value
        ));
    }

    fn element_already_exist(element_name: &str, element_value: &dyn Display) -> AlgoError {
        return ElementAlreadyExist(format!(
            "{} with value {} already exist",
            element_name, element_value
        ));
    }

    pub fn to_readable_string(&self) -> String {
        return match self {
            MissingElement(value) => value.clone(),
            ElementAlreadyExist(value) => value.clone(),
        };
    }
}
