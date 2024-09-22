uniffi::include_scaffolding!("lib");

mod density;
mod types;

pub use density::*;
pub use types::*;

pub fn get_equations() -> Vec<EquationSection> {
    return [density::get_density_equations()].to_vec();
}

pub fn calculate_equation(equation_id: String, input: Vec<f64>) -> f64 {
    let equation_id_enum = convert_equation_id_string_to_enum(equation_id);

    return match equation_id_enum {
        Some(EquationIds::Density1) => density::calculate_density_from_mass_and_volume(input),
        Some(EquationIds::Density2) => density::calculate_mass_from_density_and_volume(input),
        Some(EquationIds::Density3) => density::calculate_volume_from_density_and_mass(input),
        None => 0.0,
    };
}

fn convert_equation_id_string_to_enum(id: String) -> Option<EquationIds> {
    return match id.as_str() {
        "density1" => Some(EquationIds::Density1),
        "density2" => Some(EquationIds::Density2),
        "density3" => Some(EquationIds::Density3),
        _ => None,
    };
}
