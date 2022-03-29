pub mod register {

use egui::{FontId, RichText, Color32};

	#[derive(Default)]
	pub struct User{
		first_name: String,
		last_name: String,
		email: String,
		username: String,
		password: String,
		check: String,
		phone: String,
	}

	impl User {
		pub fn new() -> Self {
			Self {
				first_name: String::new(),
				last_name: String::new(),
				email: String::new(),
				username: String::new(),
				password: String::new(),
				check: String::new(),
				phone: String::new(),
			}
		}

		pub fn user_screen(&mut self, ui: &mut egui::Ui){
			let mut first = String::from(&self.first_name);
			let mut last = String::from(&self.last_name);
			let mut em = String::from(&self.email);
			let mut user = String::from(&self.username);
			let mut phn = String::from(&self.phone);
			let mut pass = String::from(&self.password);
			let mut ch = String::from(&self.check);

			egui::Grid::new("Registration")
            .num_columns(2)
            .spacing([10.0, 20.0])
            .max_col_width(170.0)
            .show(ui, |ui| {

            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("First Name:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut first)
            		.font(FontId::proportional(15.0)));
            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("Last Name:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut last)
            		.font(FontId::proportional(15.0)));
            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("Email:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut em)
            		.font(FontId::proportional(15.0))
            		.hint_text("Ex: \"name@address\""));
            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("Username:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut user)
            		.font(FontId::proportional(15.0)));
            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("Phone #:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut phn)
            		.font(FontId::proportional(15.0))
            		.hint_text("(xxx) xxx - xxxx"));
            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("Password:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut pass)
            		.font(FontId::proportional(15.0)));
            	ui.end_row();
            	ui.add_space(10.0);

            	ui.label(RichText::new("Re-type Password:").size(20.0).color(Color32::BLACK));
            	ui.add(egui::TextEdit::singleline(&mut ch)
            		.font(FontId::proportional(15.0)));
            	ui.end_row();
            });

            egui::Grid::new("Generate")
                .show(ui, |ui| {
                ui.add_space(220.0);

                if ui.button("Generate Password").clicked() {

                }
                ui.end_row();
                ui.add_space(220.0);
          
                if ui.button("Submit").clicked() {

                }
            });
            self.first_name = first;
            self.last_name = last;
            self.email = em;
            self.username = user;
            self.phone = phn;
            self.password = pass;
            self.check = ch;
		}
	}
}