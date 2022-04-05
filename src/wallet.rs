pub mod wall{
use egui;

	pub fn wallet_screen(ui: &mut egui::Ui) {

		egui::Grid::new("Wallet").show(ui, |ui| {
			ui.label("This is the wallet");
		});
	}
}