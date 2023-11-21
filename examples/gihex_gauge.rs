use gihex_widget::gihex_gauge::GihexGauge;
use gtk4::{
    glib,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt},
    Application, ApplicationWindow, Box,
};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.example.gihexgauge")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let container = Box::new(gtk4::Orientation::Horizontal, 20);
    let gauge = GihexGauge::new();
    let gauge_1 = GihexGauge::new();
    gauge_1.set_unit_value("\u{2103}");
    gauge_1.set_value(60.0);
    let gauge_2 = GihexGauge::new();
    gauge_2.set_unit_value("\u{2109}");
    container.append(&gauge);
    container.append(&gauge_1);
    container.append(&gauge_2);
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Example Gihex Gauge")
        // .width_request(480)
        // .height_request(320)
        .child(&container)
        .build();

    win.present();
}
