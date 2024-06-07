#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, FontFamily};
use font_kit::{font::Font, handle::Handle, source::SystemSource};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Font select",
        options,
        Box::new(|cc| {
            let selector = FontSelector::new(&cc.egui_ctx);

            Box::new(selector)
        }),
    )
}

struct FontSelector {
    font: String,
    font_source: SystemSource,
}

impl FontSelector {
    fn new(ctx: &egui::Context) -> Self {
        let selector = Self::default();

        let mut font_define = egui::FontDefinitions::default();
        for font_family in selector.font_source.all_families().unwrap() {
            // get font
            let handle = selector
                .font_source
                .select_family_by_name(&font_family)
                .unwrap();

            if !handle.is_empty() {
                for font in handle.fonts() {
                    match font {
                        Handle::Path { path, font_index } => {
                            let load_font = Font::from_path(path, *font_index).unwrap();
                            let font_name = load_font.full_name();
                            font_define.font_data.insert(
                                font_name.clone(),
                                egui::FontData::from_owned(
                                    load_font.copy_font_data().unwrap().to_vec(),
                                ),
                            );

                            font_define
                                .families
                                .entry(egui::FontFamily::Name(font_family.to_owned().into()))
                                .or_default()
                                .push(font_name.to_owned());
                        }
                        Handle::Memory { bytes, .. } => {
                            font_define.font_data.insert(
                                font_family.clone(),
                                egui::FontData::from_owned(bytes.to_vec()),
                            );

                            font_define
                                .families
                                .entry(egui::FontFamily::Name(font_family.to_owned().into()))
                                .or_default()
                                .push(font_family.to_owned());
                        }
                    };
                }
            }

            let mut default_fonts = font_define
                .families
                .get(&FontFamily::Monospace)
                .unwrap()
                .clone();
            font_define
                .families
                .entry(FontFamily::Name(font_family.to_owned().into()))
                .or_default()
                .append(&mut default_fonts);

            default_fonts = font_define
                .families
                .get(&FontFamily::Proportional)
                .unwrap()
                .clone();
            font_define
                .families
                .entry(FontFamily::Name(font_family.to_owned().into()))
                .or_default()
                .append(&mut default_fonts);
        }

        ctx.set_fonts(font_define);

        selector
    }
}

impl Default for FontSelector {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_source: SystemSource::new(),
        }
    }
}

impl eframe::App for FontSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let show_font_family = if self.font.len() == 0 {
                        FontFamily::Proportional
                    } else {
                        FontFamily::Name(self.font.to_owned().into())
                    };

                    egui::ComboBox::new("font_use", "font")
                        .selected_text(
                            egui::RichText::new(self.font.to_owned())
                                .family(show_font_family),
                        )
                        .show_ui(ui, |ui| {
                            let all_font_families = self.font_source.all_families().unwrap();
                            for font_families in all_font_families {
                                ui.selectable_value(
                                    &mut self.font,
                                    font_families.clone(),
                                    egui::RichText::new(font_families.to_owned())
                                        .family(FontFamily::Name(font_families.to_owned().into())),
                                );
                            }
                        });
                });
            });
        });
    }
}
