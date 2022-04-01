//Renders a registration window after create new in login.rs is clicked
#[path = "./pasgen.rs"]
mod pasgen;

pub mod register {

use super::list;
use serde::{Serialize, Deserialize};
use super::pasgen::generator;
use egui::{FontId, RichText, Color32};
use std::fs;
use std::fs::File;
use std::collections::BTreeMap;

	#[derive(Serialize, Deserialize, Debug, Default)]
	pub struct User{
		first_name: String,
		last_name: String,
		email: String,

		#[serde(skip_serializing, skip_deserializing)]
		username: String,
		password: String,

		#[serde(skip_serializing, skip_deserializing)]
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
            	.spacing([0.0,20.0])
                .show(ui, |ui| {
                ui.add_space(220.0);

                if ui.button("Generate Password").clicked() {
                	pass = generator::call_gen();
                	let tmp = &pass;
                	ch = tmp.to_string();
                }
                ui.end_row();
                ui.add_space(220.0);
          
                if ui.button("Submit").clicked() {
                	self.save();
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

	fn save(&self){

		let list;
		loop{
			list = match list::open_file("list.txt") {
			Ok(s) => String::from(s),
			Err(_) => {File::create("list.txt").expect("Failed to create file!!!");
						fs::write("list.txt", b"none").expect("Failed to write none to new file!!");
						continue},	
			};
			break;
		}

		let valid_list: BTreeMap<String, String>;
		loop {
				valid_list = match list::get_list(&list) {
				Ok(l) => l,
				Err(_) => {list::default(); continue},
			};
			break;
		}

		let valid_user = list::search(valid_list, &self.username);

		if valid_user {
			println!("An account under this user name already exist!!");
		}
		else {
			
	    }
	}
}}

pub mod list {

use serde::{Serialize, Deserialize};
pub use super::register::User;
use std::collections::BTreeMap;
use std::collections::BTreeMap as Map;
use std::io::Read;
use std::fs::File;
use std::fs;
use std::error::Error;

		#[derive(Serialize, Deserialize, Debug, Default)]
		struct Copy{
			first_name: String,
			last_name: String,
			email: String,

			#[serde(skip_serializing, skip_deserializing)]
			username: String,
			password: String,

			#[serde(skip_serializing, skip_deserializing)]
			check: String,
			phone: String,
	}

	impl Copy {
		fn new() -> Self {
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
	}
	pub fn open_file(file: &str) -> Result<String, Box<dyn Error>> {
	    let mut file = File::open(file)?;
	    let mut contents = String::new();

	    file.read_to_string(&mut contents).expect("No Value to Return!!!");

	    Ok(contents)
	}
    
    pub fn search(list: BTreeMap<String, String>, username: &str) -> bool{

		let result: bool = match list.get(username) {
			Some(_u) => true,
			_ => false,
		};
    	result
    }

	pub fn get_list(list: &str) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
		let test: BTreeMap<String, String> = serde_json::from_str(&list)?;

		Ok(test)
	}

	pub fn get_user(list: BTreeMap<String, String>, username: &str) -> User {

		let result: User = match list.get(username) {
			Some(u) => serde_json::from_str(&u).unwrap(),
			_ => panic!("user exist but can't get info!!!"),
		};

    	result
	}


	pub fn add_user(mut list: BTreeMap<String, String>, username: &str, data: &User){
		let serialized = serde_json::to_string(&data).unwrap();
		list.insert(username.to_string(), serialized);
		let json = serde_json::to_string(&list).unwrap();

		fs::write("list.txt", json).expect("Failed to add to the list!!");
	}
	pub fn default(){
		let user = Copy::new();

		let mut metadata = Map::new();
		let serialized = serde_json::to_string(&user).unwrap();
		metadata.insert(&user.username, serialized);
		let json = serde_json::to_string(&metadata).unwrap();

		fs::write("list.txt", json).expect("Failed to write none to new file!!");
	}
}