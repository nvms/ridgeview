use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

use crate::state::{TerrainParams, Technique};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, ui_panel);
    }
}

fn ui_panel(mut contexts: EguiContexts, mut params: ResMut<TerrainParams>) {
    let ctx = contexts.ctx_mut();

    bevy_egui::egui::SidePanel::left("controls")
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.heading("ridgeview");
            ui.separator();

            ui.label("technique");
            let mut changed = false;

            if ui.radio_value(&mut params.technique, Technique::Gradient, "gradient trick").changed() {
                changed = true;
            }
            if ui.radio_value(&mut params.technique, Technique::Dla, "DLA").changed() {
                changed = true;
            }

            ui.separator();
            ui.label("seed");
            let mut seed_i32 = params.seed as i32;
            if ui.add(bevy_egui::egui::DragValue::new(&mut seed_i32).range(0..=999999)).changed() {
                params.seed = seed_i32.max(0) as u32;
                changed = true;
            }
            if ui.button("randomize").clicked() {
                params.seed = rand::random::<u32>() % 999999;
                changed = true;
            }

            ui.separator();

            match params.technique {
                Technique::Gradient => {
                    ui.label("octaves");
                    let mut octaves_i32 = params.octaves as i32;
                    if ui.add(bevy_egui::egui::Slider::new(&mut octaves_i32, 1..=10)).changed() {
                        params.octaves = octaves_i32 as usize;
                        changed = true;
                    }

                    ui.label("gradient falloff");
                    if ui.add(bevy_egui::egui::Slider::new(&mut params.gradient_falloff, 0.0..=5.0)).changed() {
                        changed = true;
                    }
                }
                Technique::Dla => {
                    ui.label("walkers");
                    let mut walkers_i32 = params.dla_walkers as i32;
                    if ui.add(bevy_egui::egui::Slider::new(&mut walkers_i32, 500..=20000)).changed() {
                        params.dla_walkers = walkers_i32 as usize;
                        changed = true;
                    }

                    ui.label("blur passes");
                    let mut blur_i32 = params.blur_passes as i32;
                    if ui.add(bevy_egui::egui::Slider::new(&mut blur_i32, 0..=20)).changed() {
                        params.blur_passes = blur_i32 as usize;
                        changed = true;
                    }
                }
            }

            if changed {
                params.dirty = true;
            }
        });
}
