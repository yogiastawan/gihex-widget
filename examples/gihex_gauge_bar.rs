use gihex_widget::gihex_gauge_bar::{GihexGaugeBar, GihexColor};
use gtk4::{
    glib,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, WidgetExt},
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
    let gauge = GihexGaugeBar::new(0.0, 100.0);
    gauge.set_min_value(-30.0);
    gauge.set_background_color(GihexColor::new(0, 50, 100, 255));
    let gauge_1 = GihexGaugeBar::new(-20.0, 160.0);
    gauge_1.set_unit("\u{2103}");
    gauge_1.set_value(60.0);
    let gauge_2 = GihexGaugeBar::new(10.0, 120.0);
    gauge_2.set_unit("\u{2109}");
    container.append(&gauge);
    container.append(&gauge_1);
    container.append(&gauge_2);
    gauge.set_valign(gtk4::Align::Center);
    gauge_1.set_valign(gtk4::Align::Center);
    gauge_2.set_valign(gtk4::Align::Center);
    gauge.set_halign(gtk4::Align::Center);
    gauge_1.set_halign(gtk4::Align::Center);
    gauge_2.set_halign(gtk4::Align::Center);
    gauge_1.set_width_request(300);
    gauge_1.set_height_request(300);
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Example Gihex Gauge")
        // .width_request(480)
        // .height_request(320)
        .child(&container)
        .build();

    win.present();
}
