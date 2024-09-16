use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EquationSection {
    pub title: String,
    pub equations: Vec<Equation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equation {
    pub id: EquationIds,
    pub title: String,
    pub description: String,

    pub filters: Vec<EquationFilter>,
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
