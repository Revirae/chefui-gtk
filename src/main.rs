mod window;

use adw::prelude::*;
use adw::Application;
use gtk::{gio, glib};
use window::ChefApp;

const APP_ID: &str = "org.gtk_rs.chef";

fn main() -> glib::ExitCode {
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
