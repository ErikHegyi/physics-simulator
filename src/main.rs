mod point;
mod vector;
mod degree;
mod scalar;
mod radiation;
mod point_body;
mod constants;
mod astronomy;
mod graphics;

use crate::{
    constants::*,
    degree::*,
    point::*,
    point_body::*,
    radiation::*,
    scalar::*,
    vector::*,
};

use astronomy::astronomical_simulation::*;
use astronomy::planet::*;
use astronomy::star::*;
use graphics::window::*;



fn main() {
    let mut sim = astronomical_simulation!(
        star!("Sun", vector!(0), (0, 0, 0), 2e30, 696_340_000),
        planet!("Mercury", vector!(0, 47_400, 0), point!(58e9, 0, 0), 3.3e22, 2_440_000),
        planet!("Venus", (0, 35e3, 0), (0, 0, -108e9), 4.87e24, 6052e3),
        planet!("Earth", (-29780, 0, 0), (0, 0, AU), 5.97e24, 6378e3),
        planet!("Mars", (0, 0, -24100), (228e9, 0, 0), 6.42e23, 3390e3),
    );
    sim.run();
}