#[derive(Clone, Debug)]
pub enum GroundMaterial {
    Dirt,
    Grass,
    Stone,
}

impl GroundMaterial {
    // Returns color of Texture in RGB
    pub fn get_color(&self) -> [f32; 3] {
        match *self {
            GroundMaterial::Dirt => [0.38, 0.13, 0.03],
            GroundMaterial::Grass => [0.0, 0.5, 0.0],
            GroundMaterial::Stone => [0.5, 0.5, 0.5],
        }
    }
}
