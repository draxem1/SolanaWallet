
pub mod credentials{
use crate::file::serve;
use egui::{RichText, FontId, Color32};

    #[derive(Default)]
    pub struct Login {
        username: String,
        password: String,
    }

    impl Login{
       pub fn new() -> Self{
            Self {
                username: String::new(),
                password: String::new(),
            }
        }

        pub fn login_screen(&mut self, ui: &mut egui::Ui){
            let mut user = String::from(&self.username);
            let mut pass = String::from(&self.password);

            egui::Grid::new("Login")
            .num_columns(1)
            .min_row_height(20.0)
            .min_col_width(100.0).show(ui, |ui| {

            ui.vertical_centered(|ui| {

                ui.add_space(120.0);
                ui.label(RichText::new("Username").size(40.0).color(Color32::from_rgb(56,250,82)));
                ui.end_row();
                ui.add(egui::TextEdit::singleline(&mut user)
                    .desired_width(220.0)
                    .hint_text("Username")
                    .font(FontId::proportional(25.0)));
                    ui.end_row();

                ui.add_space(20.0);
                ui.label(RichText::new("Password").size(40.0).color(Color32::from_rgb(56,250,82)));
                ui.end_row();

                ui.add(egui::TextEdit::singleline(&mut pass)
                    .password(true)
                    .desired_width(220.0)
                    .hint_text("Password")
                    .font(FontId::proportional(25.0)));
                    ui.end_row();
                    ui.add_space(5.0);
                });
            });

            egui::Grid::new("buttons")
                .num_columns(1)
                .show(ui, |ui| {

                ui.columns(5, |columns| {
                if columns[1].button("New User").clicked() {
                    serve::send_state("2");
                }
                if columns[3].button("Login").clicked() {

                }
                });
            });
                self.username = user;
                self.password = pass;
        }
    }
}