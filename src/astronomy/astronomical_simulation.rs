use crate::*;
use rayon::prelude::*;
use std::collections::HashMap;

const RADIUS_MULTIPLIER: f64 = 100.0;
const REDRAW_AMOUNT: u16 = 2500;


/* ----- TEXTURES ----- */
const STAR_TEXTURE: &str = "textures/star_grayscale.jpg";
const TERRESTRIAL_TEXTURE: &str = "textures/terrestrial.jpg";


/* ----- BACKGROUND IMAGE ----- */
const BACKGROUND_IMAGE: &str = "images/astronomy_bg_small.jpg";


pub trait Celestial {
    fn point_body(&self) -> &PointBody;
    fn point_body_mut(&mut self) -> &mut PointBody;
    fn get_radius(&self) -> Scalar;
    fn get_name(&self) -> String;
    fn is_star(&self) -> bool;
    fn is_planet(&self) -> bool;
    fn planet_type(&self) -> Option<PlanetType>;
    fn get_color(&self) -> [f64; 4];
}


pub struct AstronomicalSimulation {
    dt: Scalar,
    time: Scalar,
    celestials: Vec<Box<dyn Celestial + Sync>>,
    name: String,

    time_stopped: bool,
    move_speed: f32,

    window: Window,
    textures: HashMap<String, u32>,

    calculation_amount: u16,

    multi_processor: bool,

}


impl AstronomicalSimulation {
    pub fn new(
        dt: Scalar,
        celestials: Vec<Box<dyn Celestial + Sync>>,
        name: String,
        move_speed: f32,
        multi_processor: bool
    ) -> Self {
        let mut s: Self = Self {
            dt,
            time: Scalar::new(0.0),
            celestials,
            name,
            time_stopped: false,
            move_speed,
            window: Window::new(
                    600,
                    600,
                    [0.0, 0.0, 0.0, 1.0],
                    "Astronomical Simulation",
                    45,
                    [0.0, 0.0, -5.0],
                    Some(BACKGROUND_IMAGE)
                ).unwrap(),
            textures: HashMap::new(),
            calculation_amount: 0,
            multi_processor
        };
        
        s.load_textures();
        s        
    }
    
    fn load_textures(&mut self) {
        self.textures.insert(
            String::from("star"), Window::load_texture(STAR_TEXTURE) 
        );
        self.textures.insert(
            String::from("terrestrial"), Window::load_texture(TERRESTRIAL_TEXTURE)
        );
    }

    pub fn add_celestial(&mut self, celestial: impl Celestial + Sync + 'static) {
        self.celestials.push(Box::new(celestial))
    }

    /* ----- TIME ----- */
    pub fn stop_time(&mut self) { self.time_stopped = true; }
    pub fn resume_time(&mut self) { self.time_stopped = false; }
    pub fn toggle_time(&mut self) { self.time_stopped = !self.time_stopped; }
    
    /* ----- CELESTIALS ----- */
    pub fn get_celestial_from_index(&self, n: usize) -> &Box<dyn Celestial + Sync> {
        self.celestials.iter().nth(n).unwrap()
    }

    /* ----- GRAPHICS ----- */
    pub fn draw_objects(&self) {
        let map_size: Scalar = AU;  // TODO: Calculate map size
        for celestial in self.celestials.iter() {
            // Calculate the coordinates in the <-1; +1> range
            let coordinates: [f32; 3] = [
                (celestial.point_body().coordinates.x / map_size).value as f32,
                (celestial.point_body().coordinates.y / map_size).value as f32,
                (celestial.point_body().coordinates.z / map_size).value as f32
            ];
            
            let colors: [f64; 4] = celestial.get_color();
            let color: [f32; 3] = [colors[0] as f32, colors[1] as f32, colors[2] as f32];
            
            if celestial.is_star() {
                Window::add_light_source(color, coordinates);
                Window::bind_texture(
                    unsafe { *self.textures.get("star").unwrap_unchecked() }
                )
            } else if celestial.is_planet() {
                Window::cancel_emission();
                Window::react_to_light(color);
                Window::bind_texture(
                    unsafe { *self.textures.get("terrestrial").unwrap_unchecked() }
                )
            }
            
            let radius: f64 = (celestial.get_radius() / map_size).value * RADIUS_MULTIPLIER;
            
            Window::enable_light();
            Window::enable_texture();
            
            Window::draw_sphere(coordinates, radius, color, 64);
        }
    }
    
    fn move_forward(&mut self, amount: f32) {
        self.window.camera_location = [
            self.window.camera_location[0],
            self.window.camera_location[1],
            self.window.camera_location[2] + amount
        ];
    }
    
    fn move_backward(&mut self, amount: f32) {
        self.move_forward(-amount);
    }

    fn move_left(&mut self, amount: f32) {
        self.window.camera_location = [
            self.window.camera_location[0] + amount,
            self.window.camera_location[1],
            self.window.camera_location[2]
        ];
    }
    
    fn move_right(&mut self, amount: f32) {
        self.move_left(-amount);
    }

    fn move_up(&mut self, amount: f32) {
        self.window.camera_location = [
            self.window.camera_location[0],
            self.window.camera_location[1] - amount,
            self.window.camera_location[2]
        ];
    }
    
    fn move_down(&mut self, amount: f32) {
        self.move_up(-amount);
    }
    
    fn rotate_camera(&mut self) {
        
    }

    fn handle_events(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::W, _, Action::Press, _) | WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                self.move_forward(self.move_speed);
            },
            WindowEvent::Key(Key::S, _, Action::Press, _) | WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                self.move_backward(self.move_speed);
            },
            WindowEvent::Key(Key::A, _, Action::Press, _) | WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                self.move_left(self.move_speed);
            },
            WindowEvent::Key(Key::D, _, Action::Press, _) | WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                self.move_right(self.move_speed);
            },
            WindowEvent::Key(Key::Space, _, Action::Press, _) | WindowEvent::Key(Key::Space, _, Action::Repeat, _) => {
                self.move_up(self.move_speed);
            },
            WindowEvent::Key(Key::LeftShift, _, Action::Press, _) | WindowEvent::Key(Key::LeftShift, _, Action::Repeat, _) => {
                self.move_down(self.move_speed);
            },
            WindowEvent::Key(Key::Tab, _, Action::Press, _) => {
                self.time_stopped = !self.time_stopped;
            },
            _ => ()
        }
    }
    
    pub fn run(&mut self) {
        let this: *mut Self = self as *mut Self;
        
        self.window.start_render_loop(move |_w, e| {
            unsafe { 
                if !(*this).time_stopped {
                    (*this).time += (*this).dt;
                    (*this).calculate();
                }
                
                if (*this).calculation_amount < REDRAW_AMOUNT {
                    (*this).calculation_amount += 1;
                    return;
                }

                (*this).calculation_amount = 0;
                
                Window::load_identity_matrix();
                Window::clear_screen();

                for (_, event) in flush_messages(e) {
                    (*this).handle_events(event);
                }

                (*this).window.move_camera();
                
                (*this).window.load_background_image();
                (*this).draw_objects();

                (*this).window.swap_buffers();
                (*this).window.poll_events();
            }
        }
        );
    }

    /* ----- CALCULATIONS ----- */
    fn calculate(&mut self) {
        let celestials: &Vec<Box<dyn Celestial + Sync>> = &self.celestials;
        
        // Calculate the forces applied to each object
        let mut forces: Vec<Vector> = Vec::with_capacity(self.celestials.len());

        if self.multi_processor {
            (0..celestials.len()).into_par_iter().map(|a| {
                let mut force: Vector = NULL_VECTOR;
                let ca: &PointBody = celestials[a].point_body();

                for b in 0..celestials.len() {
                    if a != b {
                        let cb: &PointBody = celestials[b].point_body();
                        force += ca.gravitational_force(cb);
                    }
                }
                force
            }).collect_into_vec(&mut forces);
        } else {
            for a in 0..celestials.len() {
                let mut force: Vector = NULL_VECTOR;
                let ca: &PointBody = celestials[a].point_body();

                for b in 0..celestials.len() {
                    if a != b {
                        let cb: &PointBody = celestials[b].point_body();
                        force += ca.gravitational_force(cb);
                    }
                }

                forces.push(force);
            }
        }


        // Move the object based on the force applied to it and its initial velocity
        for (i, object) in self.celestials.iter_mut().enumerate() {
            let body: &mut PointBody = object.point_body_mut();
            body.advance(self.dt);
            body.velocity += body.acceleration(forces[i]) * self.dt;
        }
    }
}


#[macro_export]
macro_rules! astronomical_simulation {
    (
        $( $celestial: expr ),* $(,)?
    ) => {
        AstronomicalSimulation::new(
            scalar!(1),
            {
                let mut vector: Vec<Box<dyn Celestial + Sync>> = Vec::new();
                $(
                    vector.push(Box::new($celestial));
                )*
                vector
            },
            String::from("Simulation"),
            0.02,
            false
        )
    };
    (
        $dt: expr,
        $( $celestial: expr ),* $(,)?
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            {
                let mut vector: Vec<Box<dyn Celestial + Sync>> = Vec::new();
                $(
                    vector.push(Box::new($celestial));
                )*
                vector
            },
            String::from("Simulation"),
            0.02,
            false
        )
    };
    (
        $dt: expr,
        $name: expr,
        $( $celestial: expr ),* $(,)?
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            {
                let mut vector: Vec<Box<dyn Celestial + Sync>> = Vec::new();
                $(
                    vector.push(Box::new($celestial));
                )*
                vector
            },
            String::from($name),
            0.02,
            false
        )
    };
    (
        $dt: expr,
        $name: expr,
        $move_speed: expr,
        $( $celestial: expr ),* $(,)?
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            {
                let mut vector: Vec<Box<dyn Celestial + Sync>> = Vec::new();
                $(
                    vector.push(Box::new($celestial));
                )*
                vector
            },
            String::from($name),
            f32::from($move_speed),
            false
        )
    };
    (
        $dt: expr,
        $name: expr,
        $move_speed: expr,
        $multi_processor: expr, $( $celestial: expr ),* $(,)?
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            {
                let mut vector: Vec<Box<dyn Celestial + Sync>> = Vec::new();
                $(
                    vector.push(Box::new($celestial));
                )*
                vector
            },
            String::from($name),
            f32::from($move_speed),
            $multi_processor
        )
    };
    (
        $celestials: expr
    ) => {
        AstronomicalSimulation::new(
            scalar!(1),
            $celestials,
            String::from("Simulation"),
            0.02,
            false
        )
    };
    (
        $dt: expr,
        $celestials: expr
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            $celestials,
            String::from("Simulation"),
            0.02,
            false
        )
    };
    (
        $dt: expr,
        $celestials: expr,
        $name: expr
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            $celestials,
            String::from($name),
            0.02,
            false
        )
    };
    (
        $dt: expr,
        $celestials: expr,
        $name: expr,
        $move_speed: expr
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            $celestials,
            String::from($name),
            f32::from($move_speed),
            false
        )
    };
    (
        $dt: expr,
        $celestials: expr,
        $name: expr,
        $move_speed: expr,
        $multi_processor: expr
    ) => {
        AstronomicalSimulation::new(
            scalar!($dt),
            $celestials,
            String::from($name),
            f32::from($move_speed),
            $multi_processor
        )
    };
}