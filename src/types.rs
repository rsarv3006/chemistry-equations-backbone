use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EquationSection {
    pub id: String,
    pub title: String,
    pub equations: Vec<Equation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub filters: Vec<String>,
    pub field_labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EquationFilter {
    AdvancedEquations,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EquationIds {
    Density1,
    Density2,
    Density3,
}

pub struct EquationIdsStatic {
    pub density1: String,
    pub density2: String,
    pub density3: String,
}

impl EquationIdsStatic {
    pub fn new() -> Self {
        Self {
            density1: "density1".to_string(),
            density2: "density2".to_string(),
            density3: "density3".to_string(),
        }
    }
}
