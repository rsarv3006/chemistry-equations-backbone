uniffi::include_scaffolding!("lib");

mod density;
mod types;

pub use density::*;
pub use types::*;

pub fn get_equations() -> Vec<EquationSection> {
    return [density::get_density_equations()].to_vec();
}

pub fn calculate_equation(equation_id: EquationIds, input: Vec<f64>) -> f64 {
    return match equation_id {
        EquationIds::Density1 => density::calculate_density_from_mass_and_volume(input),
        EquationIds::Density2 => density::calculate_mass_from_density_and_volume(input),
        EquationIds::Density3 => density::calculate_volume_from_density_and_mass(input),
    };
}
