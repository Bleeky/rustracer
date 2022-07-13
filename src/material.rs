use crate::{color::*, patterns::Pattern};

#[derive(Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            reflective: 0.0,
            shininess: 200.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }
}
