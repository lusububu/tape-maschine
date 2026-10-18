// egui editor for the Tape Machine plugin: dropdowns for machine/tape/speed, grouped sliders for
// the rest, mirroring the Deck/Record/Playback/Transport sections of the web app's UI.

use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, resizable_window::ResizableWindow, widgets, EguiState};

use crate::{MachineChoice, SpeedChoice, TapeChoice, TapeMachineParams};

pub fn default_state() -> Arc<EguiState> {
    EguiState::from_size(620, 520)
}

fn enum_combo<T>(ui: &mut egui::Ui, id: &str, label: &str, param: &EnumParam<T>, setter: &ParamSetter)
where
    T: Enum + PartialEq + Copy + 'static,
{
    let current = param.value();
    let variants = T::variants();
    let current_idx = current.to_index();
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(id)
            .selected_text(variants[current_idx])
            .show_ui(ui, |ui| {
                for (i, name) in variants.iter().enumerate() {
                    let selected = i == current_idx;
                    if ui.selectable_label(selected, *name).clicked() && !selected {
                        setter.begin_set_parameter(param);
                        setter.set_parameter(param, T::from_index(i));
                        setter.end_set_parameter(param);
                    }
                }
            });
    });
}

fn slider(ui: &mut egui::Ui, label: &str, param: &FloatParam, setter: &ParamSetter) {
    ui.vertical(|ui| {
        ui.label(label);
        ui.add(widgets::ParamSlider::for_param(param, setter));
    });
}

pub fn create(params: Arc<TapeMachineParams>, editor_state: Arc<EguiState>) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state.clone(),
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            ResizableWindow::new("tape-machine-editor").min_size(egui::Vec2::new(420.0, 380.0)).show(egui_ctx, editor_state.as_ref(), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Tape Machine");
                    ui.add_space(6.0);

                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Deck").strong());
                        enum_combo::<MachineChoice>(ui, "machine", "Machine", &params.machine, setter);
                        enum_combo::<TapeChoice>(ui, "tape", "Tape", &params.tape, setter);
                        enum_combo::<SpeedChoice>(ui, "speed", "Speed", &params.speed, setter);
                    });
                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Record").strong());
                        ui.columns(3, |cols| {
                            slider(&mut cols[0], "Input", &params.input, setter);
                            slider(&mut cols[1], "Bias", &params.bias, setter);
                            slider(&mut cols[2], "Ceiling", &params.ceiling, setter);
                        });
                        ui.columns(3, |cols| {
                            slider(&mut cols[0], "Knee", &params.knee, setter);
                            slider(&mut cols[1], "Grip", &params.grip, setter);
                            slider(&mut cols[2], "Give", &params.give, setter);
                        });
                    });
                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Playback").strong());
                        ui.columns(4, |cols| {
                            slider(&mut cols[0], "Spacing", &params.spacing, setter);
                            slider(&mut cols[1], "Coating", &params.thickness, setter);
                            slider(&mut cols[2], "Gap", &params.gap, setter);
                            slider(&mut cols[3], "Output", &params.output, setter);
                        });
                    });
                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Transport").strong());
                        ui.columns(3, |cols| {
                            slider(&mut cols[0], "Wow", &params.wow, setter);
                            slider(&mut cols[1], "Flutter", &params.flutter, setter);
                            slider(&mut cols[2], "Hiss", &params.hiss, setter);
                        });
                    });
                });
            });
        },
    )
}
