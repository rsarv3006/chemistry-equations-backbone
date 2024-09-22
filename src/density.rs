use crate::{
    types::{Equation, EquationSection},
    EquationIdsStatic,
};

pub fn get_density_equations() -> EquationSection {
    let equation_ids = EquationIdsStatic::new();

    return EquationSection {
        id: "density".to_string(),
        title: "Density".to_string(),
        equations: vec![
            Equation {
                id: equation_ids.density1,
                title: "ρ = m / V".to_string(),
                description: DENSITY_EQUATION_DESCRIPTION.to_string(),
                filters: vec!["advanced_equations".to_string()],
                field_labels: vec![
                    "Mass:".to_string(),
                    "Volume:".to_string(),
                    "Density:".to_string(),
                ],
            },
            Equation {
                id: equation_ids.density2,
                title: "m = ρ * V".to_string(),
                description: DENSITY_EQUATION_DESCRIPTION.to_string(),
                filters: vec!["advanced_equations".to_string()],
                field_labels: vec![
                    "Density".to_string(),
                    "Volume:".to_string(),
                    "Mass:".to_string(),
                ],
            },
            Equation {
                id: equation_ids.density3,
                title: "V = ρ / m".to_string(),
                description: DENSITY_EQUATION_DESCRIPTION.to_string(),
                filters: vec!["advanced_equations".to_string()],
                field_labels: vec![
                    "Density".to_string(),
                    "Mass:".to_string(),
                    "Volume:".to_string(),
                ],
            },
        ],
    };
}

const DENSITY_EQUATION_DESCRIPTION: &str =
    "The density of a substance is the mass of a substance divided by the volume of the substance.";

pub fn calculate_density_from_mass_and_volume(x: Vec<f64>) -> f64 {
    x[0] / x[1]
}

pub fn calculate_mass_from_density_and_volume(x: Vec<f64>) -> f64 {
    x[0] * x[1]
}

pub fn calculate_volume_from_density_and_mass(x: Vec<f64>) -> f64 {
    x[0] / x[1]
}
