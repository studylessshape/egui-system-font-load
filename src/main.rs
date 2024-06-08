#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, FontFamily};
use font_kit::{font::Font, handle::Handle, source::SystemSource};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Font select",
        options,
        Box::new(|cc| Box::new(FontSelector::new(&cc.egui_ctx))),
    )
}

struct FontSelector {
    font: String,
    font_source: SystemSource,
}

impl Default for FontSelector {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_source: SystemSource::new(),
        }
    }
}

impl FontSelector {
    fn new(ctx: &egui::Context) -> Self {
        let selector = Self::default();

        let mut font_define = egui::FontDefinitions::default();
        // Load all system fonts' families
        for font_family in selector.font_source.all_families().unwrap() {
            // Get fonts in current family
            let handle = selector
                .font_source
                .select_family_by_name(&font_family)
                .unwrap();

            if !handle.is_empty() {
                for (idx, font) in handle.fonts().iter().enumerate() {
                    match font {
                        Handle::Path { path, font_index } => {
                            // Load font by path
                            let load_font = Font::from_path(path, *font_index).unwrap();
                            // Get font name
                            let font_name = load_font.full_name();
                            font_define.font_data.insert(
                                font_name.clone(),
                                egui::FontData::from_owned(
                                    // font data
                                    load_font.copy_font_data().unwrap().to_vec(),
                                ),
                            );

                            font_define
                                .families
                                .entry(egui::FontFamily::Name(font_family.to_owned().into()))
                                .or_default()
                                // add font to its family
                                .push(font_name.to_owned());
                        }
                        Handle::Memory { bytes, .. } => {
                            font_define.font_data.insert(
                                // raw data can't get font's name, so use format with index
                                format!("{}.{}", font_family.clone(), idx),
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

            // read all default font of egui and add to current font family.
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

            default_fonts.clone_from(font_define.families.get(&FontFamily::Proportional).unwrap());
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

impl eframe::App for FontSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Which font family will be displayed
            let show_font_family = if self.font.is_empty() {
                FontFamily::Proportional
            } else {
                FontFamily::Name(self.font.to_owned().into())
            };

            ui.heading(
                egui::RichText::new("(en) Select to change the font.\n(cn) 选择以改变字体。\n(jp) フォントを変更する場合に選択します。")
                    .family(show_font_family.to_owned()),
            );

            egui::ComboBox::new("font_use", "Select font.")
                .selected_text(egui::RichText::new(self.font.to_owned()).family(show_font_family))
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
    }
}
