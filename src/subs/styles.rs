//Sets main style for Central panel in main.rs

pub mod styling{
use eframe::egui;
use egui::vec2;
use egui::Color32;
	
	pub fn set_main_style() -> egui::containers::Frame{
		let my_frame = egui::containers::Frame {
              margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
              rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
              shadow: eframe::epaint::Shadow { extrusion: 1.0, color: Color32::BLACK },
              fill: Color32::from_rgb(187,82,232),
              stroke: egui::Stroke::new(2.0, Color32::BLACK),
          };
          my_frame
	}

	pub fn set_native_options() -> eframe::NativeOptions {
		let native_options = eframe::NativeOptions {
	        resizable: false,
	        initial_window_size: Some(vec2(360.0, 520.0)),
	        decorated: true,
	        always_on_top: false, 
	        ..Default::default()
    	};
    	native_options
	}
}