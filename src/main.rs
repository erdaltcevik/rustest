use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "com.aruvbala.Rustest";

fn main() -> glib::ExitCode {
    // Create a new app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of app
    app.connect_activate(build_ui);

    // Run the app
    app.run()

}

fn build_ui(app: &Application)  {
    let button = Button::builder()
        .label("Press me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello World!")
    })
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    window.present();
}
