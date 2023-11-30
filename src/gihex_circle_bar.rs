use gtk4::glib::{self, Object};

mod imp {
    use std::{cell::Cell, f64::consts::PI};

    use gtk4::{
        gdk::RGBA,
        glib::{
            self,
            subclass::{
                object::ObjectImpl,
                prelude::*,
                types::{ObjectSubclass, ObjectSubclassExt},
            },
            Properties,
        },
        graphene::Rect,
        prelude::*,
        subclass::widget::{WidgetClassExt, WidgetImpl},
        Widget,
    };

    #[derive(Properties, Default)]
    #[properties(wrapper_type=super::GihexCircleBar)]
    pub struct GihexCircleBar {
        #[property(get,set=Self::set_value,type=f32, default = 30f32)]
        value: Cell<f32>,
        max_value: Cell<f32>,
        min_value: Cell<f32>,
        numb_step: Cell<u8>,
        current_index: Cell<u8>,
    }

    impl GihexCircleBar {
        fn set_value(&self, value: f32) {
            self.value.set(value);
            self.obj().queue_draw();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GihexCircleBar {
        const NAME: &'static str = "GihexCircleBar";
        type Type = super::GihexCircleBar;
        type ParentType = Widget;
        const ABSTRACT: bool = false;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("gihex_circle_bar");
        }

        fn new() -> Self {
            GihexCircleBar {
                value: Cell::new(30.0),
                max_value: Cell::new(100.0),
                min_value: Cell::new(0.0),
                numb_step: Cell::new(20),
                current_index: Cell::new(0),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GihexCircleBar {}

    impl WidgetImpl for GihexCircleBar {
        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            let widget = self.obj();

            let w = widget.allocated_width();
            let h = widget.allocated_height();
            let size = match w > h {
                true => h as f64,
                false => w as f64,
            };

            let stroke_circle = 4.5 * size / 300.0;
            let r_circle = (size - (stroke_circle * 2.0)) / 2.0; //9.0=>stroke*2; stroke=4.5
            let s_o = 27.0 * size / 300.0;
            let r_o_dash = (size - s_o) / 2.0; //54.0=> (r_circle+18.0)*2
            let s_i = 132.0 * size / 300.0;
            let r_i_dash = (size - s_i) / 2.0; //

            let s_dash = 5.0 * size / 300.0;

            let s_angle = (s_dash / r_o_dash).atan();

            //draw background
            snapshot.append_color(
                &RGBA::new(0.0, 0.2, 0.2, 1.0),
                &Rect::new(0.0, 0.0, w as f32, h as f32),
            );

            //draw outtest circle
            let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, size as f32, size as f32));
            cr.arc(size / 2.0, size / 2.0, r_circle, 0.0, 2.0 * PI);
            cr.set_source_rgba(0.05, 0.627, 0.59, 1.0);
            cr.set_line_width(stroke_circle);
            cr.stroke().unwrap();

            //draw dash
            let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, size as f32, size as f32));
            let id_val = (self.value.get() / (self.max_value.get() - self.min_value.get()))
                * self.numb_step.get() as f32;
            let id_val = id_val as i8 - 1;
            let step_angle = 360 / self.numb_step.get() as i16;
            for i in 0..self.numb_step.get() as i16 {
                cr.save().unwrap();
                cr.arc(
                    size / 2.0,
                    size / 2.0,
                    r_o_dash,
                    (((i * step_angle) as f64 - 90.0) * PI / 180.0) + s_angle,
                    ((((i + 1) * step_angle) as f64 - 90.0) * PI / 180.0) - s_angle,
                );
                cr.line_to(
                    (size / 2.0)
                        + (r_i_dash
                            * (((((i + 1) * step_angle) as f64 - 90.0) * PI / 180.0) - s_angle)
                                .cos()),
                    (size / 2.0)
                        + (r_i_dash
                            * (((((i + 1) * step_angle) as f64 - 90.0) * PI / 180.0) - s_angle)
                                .sin()),
                );
                cr.arc_negative(
                    size / 2.0,
                    size / 2.0,
                    r_i_dash,
                    ((((i + 1) * step_angle) as f64 - 90.0) * PI / 180.0) - s_angle,
                    (((i * step_angle) as f64 - 90.0) * PI / 180.0) + s_angle,
                );
                cr.close_path();
                if i <= id_val.into() {
                    cr.set_source_rgba(0.05, 0.627, 0.59, 1.0);
                } else {
                    cr.set_source_rgba(0.7, 0.827, 0.827, 0.4);
                }
                cr.fill().unwrap();
                cr.restore().unwrap();
            }
        }

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
    pub struct GihexCircleBar(ObjectSubclass<imp::GihexCircleBar>)
    @extends gtk4::Widget,
    @implements gtk4::Accessible;
}

impl Default for GihexCircleBar {
    fn default() -> Self {
        Object::builder().build()
    }
}

impl GihexCircleBar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
