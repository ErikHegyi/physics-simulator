use std::path::Path;
pub use glfw::*;
pub use glu_sys::*;
use gl;
use image::GenericImageView;


pub struct Window {
    width: u32,
    height: u32,
    background_color: [f32; 4],
    title: String,
    fov: u8,
    pub camera_location: [f32; 3],
    pub camera_rotation: [f32; 3],
    background_texture_id: Option<u32>,

    glfw_instance: Glfw,
    window: PWindow,
    glfw_receiver: GlfwReceiver<(f64, WindowEvent)>
}


impl Window {
    pub fn new(
        width: u32,
        height: u32,
        background_color: [f32; 4],
        title: &str,
        fov: u8,
        camera_location: [f32; 3],
        background_image: Option<impl AsRef<Path>>
    ) -> Result<Self, InitError> {
        // Initialize GLFW
        let mut glfw_instance: Glfw = init_no_callbacks()?;

        // Create the window
        let (mut window, receiver) = match glfw_instance
            .create_window(
                width, height, title, WindowMode::Windowed
            ) {
            Some(x) => x,
            None => return Err(InitError::Internal)
        };
        
        window.make_current();  // Make the context current
        window.set_framebuffer_size_polling(true);  // Check for window resizes
        window.set_key_polling(true);  // Record key presses
        window.set_mouse_button_polling(true);  // Record mouse button presses
        window.set_cursor_pos_polling(true);  // Record the position of the cursor
        
        // Load OpenGL functions
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        
        // Load the background image (if it exists)
        let background_texture_id: Option<u32> = match background_image {
            Some(image) => Some(Self::load_texture(image)),
            None => None
        };
        
        // Create the object
        let mut s: Self = Self {
            width,
            height,
            background_color,
            title: String::from(title),
            fov,
            camera_location,
            camera_rotation: [0.0; 3],
            background_texture_id,
            glfw_instance,
            window,
            glfw_receiver: receiver,
        };
        
        s.set_size_callback();  // Set resize functions

        // Initialize OpenGL
        unsafe {
            gl::ClearColor(
                s.background_color[0],
                s.background_color[1],
                s.background_color[2],
                s.background_color[3]
            );
            glEnable(GL_DEPTH_TEST);
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            gluPerspective(s.fov as GLdouble, (s.width / s.height) as GLdouble, 0.1, 50.0);
            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
            glTranslatef(s.camera_location[0], s.camera_location[1], s.camera_location[2]);
            gl::Viewport(0, 0, s.width as i32, s.height as i32);

            glEnable(GL_TEXTURE_2D);
        }

        Ok(s)
    }
    
    pub fn move_camera(&self) {
        unsafe {
            glRotatef(self.camera_rotation[0], 1.0, 0.0, 0.0);
            glRotatef(self.camera_rotation[1], 0.0, 1.0, 0.0);
            glRotatef(self.camera_rotation[2], 0.0, 0.0, 1.0);
            glTranslatef(self.camera_location[0], self.camera_location[1], self.camera_location[2]);
        }
    }

    fn set_size_callback(&mut self) {
        let this: *mut Self = self as *mut Self;
        let fov: u8 = self.fov;
        self.window.set_size_callback(move |_win, width, mut height| {
            if height == 0 { height = 1; }
            unsafe {
                glViewport(0, 0, width, height);
                glMatrixMode(GL_PROJECTION);
                glLoadIdentity();
                gluPerspective(fov as f64, width as f64 / height as f64, 0.1, 50.0);
                glMatrixMode(GL_MODELVIEW);
                glLoadIdentity();
            }

            unsafe {
                (*this).width = width as u32;
                (*this).height = height as u32;
            }
        });
    }

    pub fn load_background_image(&self) {
        if let Some(texture_id) = self.background_texture_id {
            unsafe {
                glDisable(GL_DEPTH_TEST);
                glDepthMask(GL_FALSE as GLboolean);

                glMatrixMode(GL_PROJECTION);
                glPushMatrix();
                glLoadIdentity();
                glOrtho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
                
                glMatrixMode(GL_MODELVIEW);
                glPushMatrix();
                glLoadIdentity();
                
                glEnable(GL_TEXTURE_2D);
                glBindTexture(GL_TEXTURE_2D, texture_id);

                glBegin(GL_QUADS);
                glTexCoord2f(0.0, 0.0);
                glVertex2f(-1.0, -1.0);
                glTexCoord2f(1.0, 0.0);
                glVertex2f(1.0, -1.0);
                glTexCoord2f(1.0, 1.0);
                glVertex2f(1.0, 1.0);
                glTexCoord2f(0.0, 1.0);
                glVertex2f(-1.0, 1.0);
                glEnd();

                glDisable(GL_TEXTURE_2D);

                glPopMatrix();
                glMatrixMode(GL_PROJECTION);
                glPopMatrix();

                glMatrixMode(GL_MODELVIEW);

                glEnable(GL_DEPTH_TEST);
                glDepthMask(GL_TRUE as GLboolean);
            }
        }
    }
    
    /// Draw a sphere at the given coordinates with the given radius and color.
    pub fn draw_sphere(
        coordinates: [f32; 3],
        radius: f64,
        color: [f32; 3],
        subdivisions: i32
    ) {
        unsafe {
            glPushMatrix();
            glTranslatef(coordinates[0], coordinates[1], coordinates[2]);
            glColor4f(color[0], color[1], color[2], 1.0);

            let quad: *mut GLUquadric = gluNewQuadric();
            gluQuadricTexture(quad, GL_TRUE as GLboolean);
            gluSphere(quad, radius, subdivisions, subdivisions);
            glPopMatrix();
        }
    }

    pub fn draw_text(
        coordinates: [f32; 3],
        color: [f32; 4],
        _text: String
    ) {
        unsafe {
            glColor4f(color[0], color[1], color[2], color[3]);
            glRasterPos3f(coordinates[0], coordinates[1], coordinates[2]);
            // TODO: Write out a text
        }
    }

    /// Load a texture and return its ID
    /// ## Important
    /// This only loads the texture into memory.\
    /// The user still has to bind it if they want to use it.\
    /// For this, use
    /// ```rust
    /// Window::bind_texture(texture_id: u32) -> ()
    /// ```
    /// or
    /// ```rust
    /// Window::load_and_bind_texture(file: impl AsRef<Path>) -> u32
    /// ```
    pub fn load_texture(file: impl AsRef<Path>) -> u32 {
        let img: image::DynamicImage = image::open(file).unwrap();
        let (width, height) = img.dimensions();
        let format: GLenum = match img {
            image::DynamicImage::ImageRgb8(_) | image::DynamicImage::ImageRgb16(_) | image::DynamicImage::ImageRgb32F(_) => GL_RGB,
            _ => GL_RGBA
        };
        let data: Vec<u8> = img.into_bytes();

        let mut texture_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);

            glBindTexture(GL_TEXTURE_2D, texture_id);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLfloat);
            glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLfloat);
            glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as GLfloat);
            glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as GLfloat);
            glTexEnvf(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE as GLfloat);

            glTexImage2D(GL_TEXTURE_2D, 0, format as GLint, width as GLsizei, height as GLsizei, 0, format, GL_UNSIGNED_BYTE, data.as_ptr() as *const _);
        }
        texture_id
    }

    /// Bind an already loaded texture.
    /// For loading the texture, use:
    /// ```rust
    /// Window::load_texture(file: impl AsRef<Path>) -> u32
    /// ```
    /// or use the combined function:
    /// ```rust
    /// Window::load_and_bind_texture(file: impl AsRef<Path>) -> u32
    /// ```
    pub fn bind_texture(texture_id: u32) {
        unsafe { glBindTexture(GL_TEXTURE_2D, texture_id) }
    }

    /// Load a texture based into the memory, bind it and return its ID.
    /// Combination of:
    /// ```rust
    /// Window::load_texture(file: impl AsRef<Path>) -> u32
    /// ```
    /// and
    /// ```rust
    /// Window::bind_texture(texture_id: u32) -> ()
    /// ```
    pub fn load_and_bind_texture(file: impl AsRef<Path>) -> u32 {
        let id: u32 = Self::load_texture(file);
        Self::bind_texture(id);
        id
    }

    /// Get the ID of the currently bound texture
    pub fn get_bound_texture() -> i32 {
        let mut bound_texture: i32 = 0;
        unsafe {
            gl::GetIntegerv(GL_TEXTURE_BINDING_2D, &mut bound_texture)
        };
        bound_texture
    }

    /// Add a light source
    pub fn add_light_source(color: [f32; 3], coordinates: [f32; 3]) {
        unsafe {
            let light: u32 = GL_LIGHT0;
            glMaterialfv(GL_FRONT, GL_EMISSION, [color[0], color[1], color[2], 1.0].as_ptr() as *const _);

            glEnable(light);
            glLightfv(light, GL_POSITION, [coordinates[0], coordinates[1], coordinates[2], 1.0].as_ptr() as *const _);
            glLightfv(light, GL_DIFFUSE, [
                0.7 + 0.3 * color[0],
                0.7 + 0.3 * color[1],
                0.7 + 0.3 * color[2],
                1.0,
            ].as_ptr() as *const _);
        }
    }

    /// Cancel emission\
    /// Wrapper for:
    /// ```rust
    /// unsafe {
    ///     glMaterialfv(GL_FRONT, GL_EMISSION, [0.0, 0.0, 0.0, 1.0].as_ptr());
    /// }
    /// ```
    pub fn cancel_emission() {
        unsafe {
            glMaterialfv(GL_FRONT, GL_EMISSION, [0.0, 0.0, 0.0, 1.0].as_ptr());
        }
    }

    /// Set the material to react to light
    pub fn react_to_light(color: [f32; 3]) {
        unsafe {
            glMaterialfv(GL_FRONT, GL_AMBIENT, [color[0] / 10., color[1] / 10.0, color[2] / 10.0, 1.0].as_ptr());
            glMaterialfv(GL_FRONT, GL_DIFFUSE, [color[0], color[1], color[2], 1.0].as_ptr())
        }
    }

    /// Enable lighting
    pub fn enable_light() {
        unsafe { glEnable(GL_LIGHTING); }
    }

    /// Disable lighting
    pub fn disable_light() {
        unsafe { glDisable(GL_LIGHTING); }
    }

    pub fn enable_texture() {
        unsafe { glEnable(GL_TEXTURE_2D); }
    }

    pub fn disable_texture() {
        unsafe { glDisable(GL_TEXTURE_2D); }
    }
    
    /// Clear the screen
    pub fn clear_screen() {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }
    
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
    
    pub fn poll_events(&mut self) {
        self.glfw_instance.poll_events();
    }
    
    pub fn load_identity_matrix() {
        unsafe { glLoadIdentity(); }
    }
    
    /// Get the current state for the given key
    /// ## Returns
    /// The state of the key:
    /// - `Action::Release` - The key was just released
    /// - `Action::Press` - The key was just pressed
    /// - `Action::Repeat` - The key is being held down
    pub fn get_key(&self, key: Key) -> Action {
        match unsafe { ffi::glfwGetKey(self.window.window_ptr(), key as i32) } {
            0 => Action::Release,
            1 => Action::Press,
            2 => Action::Repeat,
            _ => panic!("Some unknown action occurred.")
        }
    }
    
    /// Get the current cursor position
    pub fn get_cursor_pos(&self) -> (f64, f64) {
        self.window.get_cursor_pos()
    }
    
    /// Get the width of the window
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    /// Get the height of the window
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Run the render loop with the given function
    /// The render loop looks like this:
    /// ```rust
    /// while !window.should_close() {
    ///     function();  // The user-written function to process events
    ///     window.swap_buffers();
    ///     glfw.poll_events();
    /// }
    /// ```
    pub fn start_render_loop<F>(&mut self, mut function: F)
    where
        F: FnMut(&mut glfw::Window, &GlfwReceiver<(f64, WindowEvent)>)
    {
        while !self.window.should_close() {
            function(&mut self.window, &self.glfw_receiver);
        }
    }
}