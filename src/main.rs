mod login;
mod newuser;

#[path = "./subs/styles.rs"]
mod styles;
use styles::styling;

#[path = "./subs/file.rs"]
mod file;
use file::serve;

use newuser::register::User;
use login::credentials::Login;
use eframe::{epi, egui};


#[derive(Default)]
struct State{
    state1: Login,
    state2: User,
}

impl State{
    fn new() -> Self{
        Self {
            state1: Login::new(),
            state2: User::new(),
        }
    }
}


//**********EGUI TRAIT ADDED ON TO (OBJECT) *********\
impl epi::App for State{
   fn name(&self) -> &str {
       "D-SOL"
   }

#[allow(unused_variables)]
   fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let my_frame = styling::set_main_style();
        let state = serve::get_state();

        match state {
            1 => {egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
                        self.state1.login_screen(ui); });},
            2 => {egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
                        self.state2.user_screen(ui); });},
                _ => (),
            }
   }

   fn on_exit(&mut self){
        serve::send_state("1");
   }

}

fn main() {
    let app = State::new();

//Window style and action options
    let native_options = styling::set_native_options();
    eframe::run_native(Box::new(app), native_options);
}