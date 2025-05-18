mod astronomy;
mod graphics;
mod general;

use crate::{
    astronomy::*,
    graphics::*,
    general::*
};


fn main() {
    let mut sim = astronomical_simulation!(
        vec![
            Box::new(star!("Sun", vector!(0), (0, 0, 0), 2e30, 696.34e6)),
            Box::new(planet!("Mercury", (0, 47.4e3, 0), (58e9, 0, 0), 3.3e22, 2.44e6)),
            Box::new(planet!("Venus", (0, 35e3, 0), (0, 0, -108e9), 4.87e24, 6052e3)),
            Box::new(planet!("Earth", (-29.78e3, 0, 0), (0, 0, AU), 5.97e24, 6378e3)),
            Box::new(planet!("Mars", (0, 0, -24.1e3), (228e9, 0, 0), 6.42e23, 3390e3))
        ]
    );
    sim.run();
}