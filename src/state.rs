use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Technique {
    #[default]
    Gradient,
    Dla,
}

#[derive(Resource)]
pub struct TerrainParams {
    pub technique: Technique,
    pub seed: u32,
    pub octaves: usize,
    pub gradient_falloff: f64,
    pub dla_walkers: usize,
    pub blur_passes: usize,
    pub dirty: bool,
}

impl Default for TerrainParams {
    fn default() -> Self {
        Self {
            technique: Technique::Gradient,
            seed: 42,
            octaves: 6,
            gradient_falloff: 1.0,
            dla_walkers: 8000,
            blur_passes: 3,
            dirty: true,
        }
    }
}
