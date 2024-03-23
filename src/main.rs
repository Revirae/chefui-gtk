#![allow(dead_code)]
mod collection;
mod cuisine;
mod food;
mod window;
mod action;

use adw::prelude::*;
use adw::Application;
use gtk::{gio, glib};
// use tracing::info;
use window::ChefApp;

const APP_ID: &str = "org.gtk_rs.chef";

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();

    gio::resources_register_include!(
        "chef_1.gresource"
    )
    .expect("faied to register resources");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(main_ui);
    app.run()
}

fn main_ui(app: &Application) {
    let window = ChefApp::new(app);
    window.present();
}
