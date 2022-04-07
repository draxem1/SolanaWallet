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

		egui::Grid::new("Wallet")
		.num_columns(2)
		.spacing([20.0,8.0])
		.show(ui, |ui| {

			ui.end_row();
			ui.add_space(30.0);
			ui.label(RichText::new("Sol....:").size(60.0).color(Color32::from_rgb(56,250,82)));
			ui.add_space(50.0);
			ui.label(RichText::new(format!("{}", get(Amount::Sol))).size(60.0));
			ui.end_row();

		}); 
	}
}