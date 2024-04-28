// use crate::task2::controller::controller::Controller;
// use std::io;
//
// pub struct ConsoleView {
//     controller: Controller,
//     logged_user: Option<String>
// }
//
// impl ConsoleView {
//     pub fn new() -> Self {
//         let controller: Controller = Controller::new();
//         let logged_user: Option<String> = None;
//         Self { controller, logged_user }
//     }
//
//     fn display_logged(&mut self) {
//         match &self.logged_user {
//             None => { println!("There is currently no logged user."); }
//             Some(name) => { println!("Logged in as {}", name); }
//         }
//     }
//
//     fn display_options(&mut self) {
//         println!("1. Change logged user");
//         println!("exit. Exit the program");
//     }
//
//     pub fn execute(&mut self) -> i16 {
//         self.logged_user = Some("student".to_string());
//         let mut option: String = String::new();
//         loop {
//             self.display_logged();
//             option.clear();
//             match io::stdin().read_line(&mut option) {
//                 Ok(_) => {}
//                 Err(error) => {
//                     eprintln!("Error when reading from stdio: {}", error.to_string());
//                     option = "exit".to_string();
//                 }
//             }
//
//             if option.trim() == "exit" {
//                 break
//             }
//         }
//         0i16
//     }
// }