use gihex_widget::gihex_gauge::GihexGauge;
use gtk4::{
    glib,
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
    Application, ApplicationWindow,
};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.example.gihexgauge")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let gauge = GihexGauge::new();
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Example Gihex Gauge")
        .width_request(480)
        .height_request(320)
        .child(&gauge)
        .build();

    win.present();
}
