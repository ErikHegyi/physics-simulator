use glu_sys::*;
use rusttype;


const PIXEL_HEIGHT: f32 = 48.0;


#[derive(Debug, Copy, Clone)]
pub struct Character {
    id: u32,
    size: [i32; 2],
    bearing: [i32; 2]
}

impl Character {
    /// Draw the character onto the screen
    pub fn draw(&self, x: f32, y: f32, z: f32, scale: f32) {
        unsafe {            
            let width: f32 = self.width() as f32 * scale;
            let height: f32 = self.height() as f32 * scale;

            glRotatef(180.0, 1.0, 0.0, 0.0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            glBegin(gl::QUADS);

            glTexCoord2f(0.0, 0.0);
            glVertex3f(x, y, z);

            glTexCoord2f(0.0, 1.0);
            glVertex3f(x, y + height, z);

            glTexCoord2f(1.0, 1.0);
            glVertex3f(x + width, y + height, z);

            glTexCoord2f(1.0, 0.0);
            glVertex3f(x + width, y, z);

            glEnd();
            glRotatef(-180.0, 1.0, 0.0, 0.0);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    pub fn width(&self) -> i32 {
        self.size[0]
    }
    pub fn height(&self) -> i32 {
        self.size[1]
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    characters: std::collections::HashMap<char, Character>
}


impl Font {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let mut font: Self = Self { characters: std::collections::HashMap::new() };

        // Load font from file
        let font_data: Vec<u8> = std::fs::read(path)
            .expect("Could not read in font.");
        let rusttype_font: rusttype::Font = rusttype::Font::try_from_vec(font_data)
            .expect("Error while constructing font");
        let scale: rusttype::Scale = rusttype::Scale::uniform(PIXEL_HEIGHT);

        for c in 0u8..128u8 {
            let glyph: rusttype::ScaledGlyph = rusttype_font.glyph(c as char).scaled(scale);
            let positioned_glyph: rusttype::PositionedGlyph = glyph.clone().positioned(
                rusttype::point(0.0, 0.0)
            );

            if let Some(bb) = positioned_glyph.pixel_bounding_box() {
                let width: i32 = bb.width();
                let height: i32 = bb.height();
                let mut pixel_data: Vec<u8> = vec![0u8; (width * height) as usize];

                positioned_glyph.draw(|x, y, v| {
                    pixel_data[(x + y * width as u32) as usize] = (v * 255.0) as u8;
                });

                unsafe {
                    let mut texture: u32 = 0;
                    gl::GenTextures(1, &mut texture);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::ALPHA as GLint,
                        width,
                        height,
                        0,
                        gl::ALPHA,
                        gl::UNSIGNED_BYTE,
                        pixel_data.as_ptr() as *const _
                    );
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                    font.characters.insert(
                        c as char,
                        Character {
                            id: texture,
                            size: [width, height],
                            bearing: [glyph.h_metrics().left_side_bearing.round() as i32, -bb.min.y]
                        }
                    );
                }
            } else {
                font.characters.insert(
                    c as char,
                    Character {
                        id: 0,
                        size: [0, 0],
                        bearing: [0, 0]
                    }
                );
            }
        }
        font
    }

    pub fn get_character(&self, c: char) -> Option<Character> {
        match self.characters.get(&c) {
            Some(c) => Some(*c),
            None => None
        }
    }
}


#[macro_export]
macro_rules! load_font {
    ($font: literal) => {
        {
            use crate::graphics::Font;
            Font::new($font)
        }
    };
}