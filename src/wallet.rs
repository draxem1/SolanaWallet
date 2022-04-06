pub mod wall{
use egui::{RichText, Color32};

	#[derive(Debug)]
	enum Amount{
		Sol,
	}

	fn get(amount: Amount) -> f32{
		match amount {
			Amount::Sol => 0.0,
		}
	}

	pub fn wallet_screen(ui: &mut egui::Ui) {

		egui::Grid::new("Wallet").show(ui, |ui| {
			ui.label(RichText::new("Sol....:").size(40.0).color(Color32::from_rgb(56,250,82)));
			ui.label(RichText::new(format!("{}", get(Amount::Sol))).size(40.0));
			ui.end_row();

			if ui.button("Send").clicked() {

			}

			if ui.button("Recieve").clicked() {

			}
		});
	}
}