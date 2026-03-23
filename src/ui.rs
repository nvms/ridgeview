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
            let mut regenerate = false;

            if ui.radio_value(&mut params.technique, Technique::Gradient, "gradient trick").changed() {
                regenerate = true;
            }
            if ui.radio_value(&mut params.technique, Technique::Dla, "DLA").changed() {
                regenerate = true;
            }

            ui.separator();
            ui.label("seed");
            let mut seed_i32 = params.seed as i32;
            let seed_resp = ui.add(bevy_egui::egui::DragValue::new(&mut seed_i32).range(0..=999999));
            if seed_resp.changed() {
                params.seed = seed_i32.max(0) as u32;
            }
            if seed_resp.drag_stopped() || seed_resp.lost_focus() {
                regenerate = true;
            }
            if ui.button("randomize").clicked() {
                params.seed = rand::random::<u32>() % 999999;
                regenerate = true;
            }

            ui.separator();

            match params.technique {
                Technique::Gradient => {
                    ui.label("octaves");
                    let mut octaves_i32 = params.octaves as i32;
                    let resp = ui.add(bevy_egui::egui::Slider::new(&mut octaves_i32, 1..=10));
                    if resp.changed() {
                        params.octaves = octaves_i32 as usize;
                    }
                    if resp.drag_stopped() {
                        regenerate = true;
                    }

                    ui.label("gradient falloff");
                    let resp = ui.add(bevy_egui::egui::Slider::new(&mut params.gradient_falloff, 0.0..=5.0));
                    if resp.drag_stopped() {
                        regenerate = true;
                    }
                }
                Technique::Dla => {
                    ui.label("walkers");
                    let mut walkers_i32 = params.dla_walkers as i32;
                    let resp = ui.add(bevy_egui::egui::Slider::new(&mut walkers_i32, 500..=20000));
                    if resp.changed() {
                        params.dla_walkers = walkers_i32 as usize;
                    }
                    if resp.drag_stopped() {
                        regenerate = true;
                    }

                    ui.label("blur passes");
                    let mut blur_i32 = params.blur_passes as i32;
                    let resp = ui.add(bevy_egui::egui::Slider::new(&mut blur_i32, 0..=20));
                    if resp.changed() {
                        params.blur_passes = blur_i32 as usize;
                    }
                    if resp.drag_stopped() {
                        regenerate = true;
                    }
                }
            }

            ui.separator();
            ui.label("height");
            let resp = ui.add(bevy_egui::egui::Slider::new(&mut params.height_scale, 0.1..=3.0));
            if resp.drag_stopped() {
                regenerate = true;
            }

            if regenerate {
                params.dirty = true;
            }
        });
}
