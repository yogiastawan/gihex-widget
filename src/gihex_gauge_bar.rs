//! # Introduction
//! This module used to visualization data as `Gauge Bar`.
//!
//! # CSS Node
//! CSS node name for `GihexGaugeBar` is *`gihex_gauge_bar`*. Currently CSS style only affected to property `margin`.
//! ```css
//! gihex_gauge_bar{
//!     padding-top: 10px;
//! }
//! ```
//!
//! # Widget Properties
//! There are any properties used in this module:
//!
//! | Name | XML Name | Setter| Getter | Type | Read | Write | Default Value|
//! |------|----------|-------|--------|------|:----:|:-----:|--------------|
//! | Value | value | set_value| value | f32 | yes | yes | 3 |
//! | Min value | min-value | set_min_value | min_value| f32 | yes | yes | 0.0 |
//! | Max value | max-value | set_max_value | max_value| f32 | yes | yes | 100.0 |
//! | Unit font size | unit-font-size | set_unit_font_size | unit_font_size | f32 | yes | yes | 12.0 |
//! | Unit | unit | set_unit | unit | String | yes | yes | % |
//! | Background color | background-color | set_background_color | background_color | [GihexColor] | yes | yes | (0, 51, 51, 255) |
//! | Track color | track-color | set_track_color | track_color | [GihexColor] | yes | yes | (0, 127, 127, 255) |
//! | Bar color | bar-color | set_bar_color | barcolor | [GihexColor] | yes | yes | (0, 166, 204, 255) |
//! | Stroke color | stroke-color | set_stroke_color | stroke_color | [GihexColor] | yes | yes | (0, 145, 145, 255) |
//! | Text color | text-color | set_text_color | text_color | [GihexColor] | yes | yes | (0, 127, 127, 255) |
//!
//! # Example
//! Example of usage:
//! ```rust
//! use gihex_widget::gihex_gauge_bar::{GihexGaugeBar, GihexColor};
//! use gtk4::{
//!     glib,
//!     prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, WidgetExt},
//!     Application, ApplicationWindow, Box,
//! };
//!
//! fn main() -> glib::ExitCode {
//!     let app = Application::builder()
//!         .application_id("com.example.gihexgauge")
//!         .build();
//!     app.connect_activate(build_ui);
//!
//!     app.run()
//! }
//!
//! fn build_ui(app: &Application) {
//!     let container = Box::new(gtk4::Orientation::Horizontal, 20);
//!     let gauge = GihexGaugeBar::new(0.0, 100.0);
//!     gauge.set_min_value(-30.0);
//!     gauge.set_background_color(GihexColor::new(0, 50, 100, 255));
//!     let gauge_1 = GihexGaugeBar::new(-20.0, 160.0);
//!     gauge_1.set_unit("\u{2103}");
//!     gauge_1.set_value(60.0);
//!     let gauge_2 = GihexGaugeBar::new(10.0, 120.0);
//!     gauge_2.set_unit("\u{2109}");
//!     container.append(&gauge);
//!     container.append(&gauge_1);
//!     container.append(&gauge_2);
//!     gauge.set_valign(gtk4::Align::Center);
//!     gauge_1.set_valign(gtk4::Align::Center);
//!     gauge_2.set_valign(gtk4::Align::Center);
//!     gauge.set_halign(gtk4::Align::Center);
//!     gauge_1.set_halign(gtk4::Align::Center);
//!     gauge_2.set_halign(gtk4::Align::Center);
//!     gauge_1.set_width_request(300);
//!     gauge_1.set_height_request(300);
//!     let win = ApplicationWindow::builder()
//!         .application(app)
//!         .title("Example Gihex Gauge")
//!         // .width_request(480)
//!         // .height_request(320)
//!         .child(&container)
//!         .build();
//!
//!     win.present();
//! }
//! ```

use gtk4::gdk::RGBA;
use gtk4::glib::{self, Object, ValueDelegate};

mod imp {

    use std::{
        cell::{Cell, RefCell},
        f64::consts::PI,
    };

    use gtk4::{
        gdk::{Rectangle, RGBA},
        glib::{
            self,
            subclass::{
                object::ObjectImpl,
                prelude::*,
                types::{ObjectSubclass, ObjectSubclassExt},
            },
            Properties,
        },
        graphene,
        prelude::*,
        subclass::widget::{WidgetClassExt, WidgetImpl},
        Widget,
    };

    use super::GihexColor;

    #[derive(Properties, Default)]
    #[properties(wrapper_type=super::GihexGaugeBar)]
    pub struct GihexGaugeBar {
        #[property(get,set=Self::set_value,type=f32, default = 30f32)]
        value: Cell<f32>,
        #[property(get, set=Self::set_min_value,type=f32, name="min-value", default = 0f32)]
        min_value: Cell<f32>,
        #[property(get, set=Self::set_max_value,type=f32, name="max-value", default = 100f32)]
        max_value: Cell<f32>,
        #[property(get, set=Self::set_font_size,type=f32, name="unit-font-size", default = 12f32)]
        unit_font_size: Cell<f32>,
        #[property(get, set=Self::set_unit,type=String, name="unit", default = "%")]
        unit: RefCell<String>,
        #[property(get, set=Self::set_background_color,type=GihexColor, name="background-color")]
        backgorund_color: RefCell<GihexColor>,
        #[property(get, set=Self::set_track_color,type=GihexColor, name="track-color")]
        track_color: RefCell<GihexColor>,
        #[property(get, set=Self::set_bar_color,type=GihexColor, name="bar-color")]
        bar_color: RefCell<GihexColor>,
        #[property(get, set=Self::set_stroke_color,type=GihexColor, name="stroke-color")]
        stroke_color: RefCell<GihexColor>,
        #[property(get, set=Self::set_text_color,type=GihexColor, name="text-color")]
        text_color: RefCell<GihexColor>,
    }

    impl GihexGaugeBar {
        /**
         * Set value.
         * Parameter `value` is value to be set.
         */
        fn set_value(&self, value: f32) {
            self.value.set(value);
            self.obj().queue_draw();
        }

        fn set_min_value(&self, value: f32) {
            self.min_value.set(value);
            self.obj().queue_draw();
        }
        fn set_max_value(&self, value: f32) {
            self.max_value.set(value);
            self.obj().queue_draw();
        }
        fn set_font_size(&self, value: f32) {
            self.unit_font_size.set(value);
            self.obj().queue_draw();
        }

        fn set_unit(&self, unit: String) {
            *self.unit.borrow_mut() = unit;
            self.obj().queue_draw();
        }

        fn set_background_color(&self, color: GihexColor) {
            *self.backgorund_color.borrow_mut() = color;
            self.obj().queue_draw();
        }

        fn set_track_color(&self, color: RGBA) {
            *self.track_color.borrow_mut() = GihexColor(color);
            self.obj().queue_draw();
        }

        fn set_bar_color(&self, color: RGBA) {
            *self.bar_color.borrow_mut() = GihexColor(color);
            self.obj().queue_draw();
        }

        fn set_stroke_color(&self, color: RGBA) {
            *self.stroke_color.borrow_mut() = GihexColor(color);
            self.obj().queue_draw();
        }

        fn set_text_color(&self, color: RGBA) {
            *self.text_color.borrow_mut() = GihexColor(color);
            self.obj().queue_draw();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GihexGaugeBar {
        const NAME: &'static str = "GihexGaugeBar";
        type Type = super::GihexGaugeBar;
        type ParentType = Widget;
        const ABSTRACT: bool = false;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("gihex_gauge_bar");
        }

        fn new() -> Self {
            GihexGaugeBar {
                value: Cell::new(30f32),
                min_value: Cell::new(0f32),
                max_value: Cell::new(100f32),
                unit_font_size: Cell::new(12.0),
                unit: RefCell::new("%".to_owned()),
                backgorund_color: RefCell::new(GihexColor::new(0, 51, 51, 255)),
                track_color: RefCell::new(GihexColor::new(0, 127, 127, 255)),
                bar_color: RefCell::new(GihexColor::new(0, 166, 204, 255)),
                stroke_color: RefCell::new(GihexColor::new(0, 145, 145, 255)),
                text_color: RefCell::new(GihexColor::new(0, 127, 127, 255)),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GihexGaugeBar {}

    impl WidgetImpl for GihexGaugeBar {
        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            let widget = self.obj();

            let w = widget.allocated_width();
            let h = widget.allocated_height();
            let size = match w > h {
                true => h as f64,
                false => w as f64,
            };

            let ro = (size - 2.0) / 2.0;
            let ri = (size - 2.0) / 3.0;

            let max_value = self.max_value.get() as f64;
            let min_value = self.min_value.get() as f64;
            let value = self.value.get() as f64;
            let value = if value > max_value {
                max_value
            } else if value < min_value {
                min_value
            } else {
                value
            };

            let value_text = format!("{:.2}{}", value, self.unit.borrow());

            // let stroke_color=
            //draw background
            let cr = snapshot.append_cairo(&graphene::Rect::new(0.0, 0.0, w as f32, h as f32));

            cr.add_rectangle(&Rectangle::new(0, 0, size as i32, size as i32));
            cr.set_source_rgba(
                self.backgorund_color.borrow().get_red().into(),
                self.backgorund_color.borrow().get_green().into(),
                self.backgorund_color.borrow().get_blue().into(),
                self.backgorund_color.borrow().get_alpha().into(),
            );
            cr.fill().unwrap();

            //draw base gauge
            let cr =
                snapshot.append_cairo(&graphene::Rect::new(0.0, 0.0, size as f32, size as f32));
            cr.arc(size / 2.0, size / 2.0, ro, 2.0 * PI / 3.0, 7.0 * PI / 3.0);
            cr.arc(
                (size / 2.0) - (((ro + ri) / 2.0) * (11.0 * PI / 6.0).sin()),
                (size / 2.0) + (((ro + ri) / 2.0) * (11.0 * PI / 6.0).cos()),
                (ro - ri) / 2.0,
                PI / 3.0,
                4.0 * PI / 3.0,
            );
            cr.arc_negative(size / 2.0, size / 2.0, ri, PI / 3.0, 2.0 * PI / 3.0);
            cr.arc(
                (size / 2.0) - (((ro + ri) / 2.0) * (PI / 6.0).sin()),
                (size / 2.0) + (((ro + ri) / 2.0) * (PI / 6.0).cos()),
                (ro - ri) / 2.0,
                5.0 * PI / 3.0,
                2.0 * PI / 3.0,
            );
            cr.close_path();
            cr.set_source_rgba(
                self.track_color.borrow().get_red().into(),
                self.track_color.borrow().get_green().into(),
                self.track_color.borrow().get_blue().into(),
                self.track_color.borrow().get_alpha().into(),
            );
            cr.fill_preserve().unwrap();
            cr.set_source_rgba(
                self.stroke_color.borrow().get_red().into(),
                self.stroke_color.borrow().get_green().into(),
                self.stroke_color.borrow().get_blue().into(),
                self.stroke_color.borrow().get_alpha().into(),
            );
            cr.stroke().unwrap();
            cr.set_line_width(1.0);

            //draw value
            let cr =
                snapshot.append_cairo(&graphene::Rect::new(0.0, 0.0, size as f32, size as f32));
            cr.arc(
                size / 2.0,
                size / 2.0,
                ro - 2.0,
                2.0 * PI / 3.0,
                ((300.0 * (value - min_value) / (max_value - min_value)) + 120.0) * PI / 180.0,
            );
            cr.arc(
                (size / 2.0)
                    - (((ro + ri) / 2.0)
                        * (((300.0 * (value - min_value) / (max_value - min_value)) + 30.0) * PI
                            / 180.0)
                            .sin()),
                (size / 2.0)
                    + (((ro + ri) / 2.0)
                        * (((300.0 * (value - min_value) / (max_value - min_value)) + 30.0) * PI
                            / 180.0)
                            .cos()),
                (ro - ri - 4.0) / 2.0,
                (120.0 + (300.0 * (value - min_value) / (max_value - min_value))) * PI / 180.0,
                (300.0 * (1.0 + ((value - min_value) / (max_value - min_value)))) * PI / 180.0,
            );
            cr.arc_negative(
                size / 2.0,
                size / 2.0,
                ri + 2.0,
                ((300.0 * (value - min_value) / (max_value - min_value)) + 120.0) * PI / 180.0,
                2.0 * PI / 3.0,
            );
            cr.arc(
                (size / 2.0) - (((ro + ri) / 2.0) * (PI / 6.).sin()),
                (size / 2.0) + (((ro + ri) / 2.0) * (PI / 6.0).cos()),
                (ro - ri - 4.0) / 2.0,
                5.0 * PI / 3.0,
                2.0 * PI / 3.0,
            );
            cr.close_path();
            cr.set_source_rgba(
                self.bar_color.borrow().get_red().into(),
                self.bar_color.borrow().get_green().into(),
                self.bar_color.borrow().get_blue().into(),
                self.bar_color.borrow().get_alpha().into(),
            );
            cr.fill().unwrap();

            //draw text value
            let cr =
                snapshot.append_cairo(&graphene::Rect::new(0.0, 0.0, size as f32, size as f32));
            cr.set_font_size(self.unit_font_size.get() as f64 * size / 75.0);
            let text_extends = cr.text_extents(value_text.as_str()).unwrap();
            cr.move_to(
                (size / 2.0) - ((text_extends.width() / 2.0) + text_extends.x_bearing()),
                (size / 2.0) - ((text_extends.height() / 2.0) + text_extends.y_bearing()),
            );
            cr.text_path(value_text.as_str());
            cr.set_source_rgba(
                self.text_color.borrow().get_red().into(),
                self.text_color.borrow().get_green().into(),
                self.text_color.borrow().get_blue().into(),
                self.text_color.borrow().get_alpha().into(),
            );
            cr.fill().unwrap();
        }

        // fn request_mode(&self) -> gtk4::SizeRequestMode {
        //     gtk4::SizeRequestMode::ConstantSize
        // }

        fn measure(&self, _orientation: gtk4::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let size = match for_size < 150 {
                true => 150,
                false => for_size,
            };
            (150, size, -1, -1)
        }
    }
}
glib::wrapper! {
    pub struct GihexGaugeBar(ObjectSubclass<imp::GihexGaugeBar>)
    @extends gtk4::Widget,
    @implements gtk4::Accessible;
}

impl Default for GihexGaugeBar {
    fn default() -> Self {
        Object::builder().build()
    }
}

impl GihexGaugeBar {
    /// Create new `GihexGaugeBar`.
    /// * Parameter `min_value` is minimum value of gauge.
    /// * And parameter `max_value` is maximum value of gauge.
    pub fn new(min_value: f32, max_value: f32) -> Self {
        Object::builder()
            .property("max-value", max_value)
            .property("min-value", min_value)
            .build()
    }
}

/// Object wrapper for `gtk4::gdk::RGBA`. It Used to pass `color property` into custom widget.
#[derive(ValueDelegate, Clone)]
pub struct GihexColor(RGBA);

impl GihexColor {
    /// Create new `GihexColor`.
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        GihexColor(RGBA::new(
            red as f32 / 255.0,
            green as f32 / 255.0,
            blue as f32 / 255.0,
            alpha as f32 / 255.0,
        ))
    }

    /// Get [gtk4::gdk::RGBA] color.
    pub fn get_rgba(&self) -> RGBA {
        RGBA::new(self.0.red(), self.0.green(), self.0.blue(), self.0.alpha())
    }

    /// Get `red` part of color. Return value type `f32`.
    pub fn get_red(&self) -> f32 {
        self.0.red()
    }

    /// Get `green` part of color. Return value type `f32`.
    pub fn get_green(&self) -> f32 {
        self.0.green()
    }

    /// Get `blue` part of color. Return value type `f32`.
    pub fn get_blue(&self) -> f32 {
        self.0.blue()
    }

    /// Get `alpha` part of color. Return value type `f32`.
    pub fn get_alpha(&self) -> f32 {
        self.0.alpha()
    }
}

impl Default for GihexColor {
    fn default() -> Self {
        Self(RGBA::new(0.0, 0.0, 0.0, 1.0))
    }
}
