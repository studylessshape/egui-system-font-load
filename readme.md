# system fonts load for egui
This repo show how to load system fonts for egui.

## Logic
Read all font families, and load them to `egui::FontDefinitions` by family name. 

After load, append egui default font.

## Problems
Memory usage is the biggest problem.

After loading 1000 fonts, the memory usage will reach about 2 GB.

## Dependence
- [eframe](https://docs.rs/eframe): egui framework.
- [font-kit](https://docs.rs/font-kit): load system font.