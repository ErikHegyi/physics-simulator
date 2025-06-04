use crate::*;
use rayon::prelude::*;
use std::collections::HashMap;

const RADIUS_MULTIPLIER: f64 = 100.0;
const REDRAW_AMOUNT: u16 = 2500;


/* ----- TEXTURES ----- */
const STAR_TEXTURE: &str = "textures/star_grayscale.jpg";
const EARTH_TEXTURE: &str = "textures/earth.jpeg";
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
    drag: [f64; 2],
    button_held_down: bool,
    mouse_pos: [f64; 2],
    rotation_sensitivity: f32,
    
    font: Font,

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
            celestials,
            name,
            move_speed,
            multi_processor,
            ..Self::default()
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
        self.textures.insert(
            String::from("earth"), Window::load_texture(EARTH_TEXTURE)
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
            
            self.window.draw_text(
                coordinates[0],
                coordinates[1],
                coordinates[2],
                celestial.get_name(),
                self.font.clone(),
                24.0,
                [1.0, 1.0, 1.0]
            );
            
            if celestial.is_star() {
                Window::add_light_source(color, coordinates);
                Window::bind_texture(
                    unsafe { *self.textures.get("star").unwrap_unchecked() }
                )
            } else if celestial.is_planet() {
                Window::cancel_emission();
                Window::react_to_light(color);
                Window::bind_texture(
                    unsafe { 
                        *self.textures.get(
                            if celestial.get_name().to_lowercase().as_str() == "earth" { "earth" }
                            else { "terrestrial" }
                        ).unwrap_unchecked()
                    }
                )
            }
            
            let radius: f64 = (celestial.get_radius() / map_size).value * RADIUS_MULTIPLIER;
            
            Window::enable_light();
            Window::enable_texture();
            
            Window::draw_sphere(coordinates, radius, color, 64);
        }
    }
    
    fn move_forward(&mut self, amount: f32) {
        let rot_x: Degree = Degree::from_float(self.window.camera_rotation[1] as f64);
        let rot_y: Degree = Degree::from_float(self.window.camera_rotation[0] as f64);
        self.window.camera_location = [
            self.window.camera_location[0] - amount * (rot_x.sin() * rot_y.cos()) as f32,
            self.window.camera_location[1] + amount * rot_y.sin() as f32,
            self.window.camera_location[2] + amount * (rot_x.cos() * rot_y.cos()) as f32
        ];
    }
    
    fn move_backward(&mut self, amount: f32) {
        self.move_forward(-amount);
    }

    fn move_left(&mut self, amount: f32) {
        let rot_x: Degree = Degree::from_float(self.window.camera_rotation[1] as f64);
        let rot_y: Degree = Degree::from_float(self.window.camera_rotation[0] as f64);
        self.window.camera_location = [
            self.window.camera_location[0] + amount * (rot_x.cos() * rot_y.cos()) as f32,
            self.window.camera_location[1] + amount * rot_y.sin() as f32,
            self.window.camera_location[2] + amount * (rot_x.sin() * rot_y.cos()) as f32
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
    
    fn rotate_camera(&mut self, roll: f32, pitch: f32, yaw: f32) {
        self.window.camera_rotation = [
            self.window.camera_rotation[0] + roll,
            self.window.camera_rotation[1] + pitch,
            self.window.camera_rotation[2] + yaw
        ]
    }

    fn handle_events(&mut self, event: WindowEvent) {
        let (x, y) = self.window.get_cursor_pos();
        if self.button_held_down {
            let dx: f64 = x - self.mouse_pos[0];
            let dy: f64 = y - self.mouse_pos[1];
            self.drag = [dx, dy];
            self.rotate_camera(self.drag[1] as f32 * self.rotation_sensitivity, self.drag[0] as f32 * self.rotation_sensitivity, 0.0);
        }
        self.mouse_pos = [x, y];

        match event {
            WindowEvent::Key(Key::W, _, Action::Press, _) | WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                self.move_forward(self.move_speed);
            },
            WindowEvent::Key(Key::S, _, Action::Press, _) | WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                self.move_backward(self.move_speed);
            },
            WindowEvent::Key(Key::D, _, Action::Press, _) | WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                self.move_right(self.move_speed);
            },
            WindowEvent::Key(Key::A, _, Action::Press, _) | WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                self.move_left(self.move_speed);
            },
            WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => {
                self.move_speed *= 5.0;
            },
            WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => {
                self.move_speed /= 5.0;
            },
            WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                self.time_stopped = !self.time_stopped;
            },
            WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                self.button_held_down = true;
                self.drag = [0.0; 2];
            },
            WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
                self.button_held_down = false;
                self.drag = [0.0; 2];
            }
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


impl Default for AstronomicalSimulation {
    fn default() -> Self {
        Self {
            dt: scalar!(10),
            time: scalar!(0),
            celestials: Vec::new(),
            name: String::from("Astronomical Simulation"),
            time_stopped: false,
            move_speed: 0.2,
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
            drag: [0.0; 2],
            mouse_pos: [0.0; 2],
            button_held_down: false,
            rotation_sensitivity: 0.05,
            font: load_font!("fonts\\arial.ttf"),
            calculation_amount: 0,
            multi_processor: false
        }
    }
}


#[macro_export]
macro_rules! astronomical_simulation {
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