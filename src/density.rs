use crate::types::{Equation, EquationIds, EquationSection};

pub fn get_density_equations() -> EquationSection {
    return EquationSection {
        title: "Density".to_string(),
        equations: vec![
            Equation {
                id: EquationIds::Density1,
                title: "ρ = m / V".to_string(),
                description: DENSITY_EQUATION_DESCRIPTION.to_string(),
                filters: vec![],
                field_labels: vec!["Density".to_string()],
            },
            Equation {
                id: EquationIds::Density2,
                title: "m = ρ * V".to_string(),
                description: DENSITY_EQUATION_DESCRIPTION.to_string(),
                filters: vec![],
                field_labels: vec!["Density".to_string()],
            },
            Equation {
                id: EquationIds::Density3,
                title: "V = ρ / m".to_string(),
                description: DENSITY_EQUATION_DESCRIPTION.to_string(),
                filters: vec![],
                field_labels: vec!["Density".to_string()],
            },
        ],
    };
}

const DENSITY_EQUATION_DESCRIPTION: &str = "DENSITY DESCRIPTION";

pub fn calculate_density_from_mass_and_volume(x: Vec<f64>) -> f64 {
    x[0] / x[1]
}

pub fn calculate_mass_from_density_and_volume(x: Vec<f64>) -> f64 {
    x[0] * x[1]
}

pub fn calculate_volume_from_density_and_mass(x: Vec<f64>) -> f64 {
    x[0] / x[1]
}
