use gihex_widget::gihex_circle_bar::GihexCircleBar;
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
    let bar = GihexCircleBar::new();
    let bar1 = GihexCircleBar::new();
    let bar2 = GihexCircleBar::new();

    container.append(&bar);
    container.append(&bar1);
    container.append(&bar2);
    bar.set_valign(gtk4::Align::Center);
    bar.set_halign(gtk4::Align::Center);
    bar1.set_valign(gtk4::Align::Center);
    bar1.set_halign(gtk4::Align::Center);
    bar2.set_valign(gtk4::Align::Center);
    bar2.set_halign(gtk4::Align::Center);
    bar1.set_width_request(300);
    bar1.set_height_request(300);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Example Gihex Circle")
        // .width_request(480) 
        // .height_request(320)
        .child(&container)
        .build();

    win.present();
}
