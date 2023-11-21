use gtk4::glib;
use gtk4::glib::Object;

mod imp {
    use std::{
        cell::{Cell, RefCell},
        f64::consts::PI,
    };

    use gtk4::{
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

    #[derive(Properties, Default)]
    #[properties(wrapper_type=super::GihexGauge)]
    pub struct GihexGauge {
        #[property(get, set,type=f32, default = 30f32)]
        value: Cell<f32>,
        #[property(get, set,type=f32, name="min-value", default = 0f32)]
        min_value: Cell<f32>,
        #[property(get, set,type=f32, name="max-value", default = 100f32)]
        max_value: Cell<f32>,
        #[property(get, set,type=f32, name="unit-font-size", default = 12f32)]
        unit_font_size: Cell<f32>,
        #[property(get, set,type=String, name="unit-value", default = "%")]
        unit: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GihexGauge {
        const NAME: &'static str = "GihexGauge";
        type Type = super::GihexGauge;
        type ParentType = Widget;
        const ABSTRACT: bool = false;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("gx_gauge");
        }

        fn new() -> Self {
            GihexGauge {
                value: Cell::new(30f32),
                min_value: Cell::new(0f32),
                max_value: Cell::new(100f32),
                unit_font_size: Cell::new(12.0),
                unit: RefCell::new("%".to_owned()),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GihexGauge {}

    impl WidgetImpl for GihexGauge {
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

            let mut value_text = value.to_string();
            value_text.push_str(self.unit.borrow().as_str());

            //draw base gauge
            let cr = snapshot.append_cairo(&graphene::Rect::new(0f32, 0f32, w as f32, h as f32));
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
            cr.set_source_rgb(0.0, 0.5, 0.5);
            cr.fill_preserve().unwrap();
            cr.set_source_rgb(0.0, 0.45, 0.45);
            cr.stroke().unwrap();
            cr.set_line_width(1.0);

            //draw value
            let cr = snapshot.append_cairo(&graphene::Rect::new(0f32, 0f32, w as f32, h as f32));
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
            cr.set_source_rgb(0.0, 0.65, 0.8);
            cr.fill().unwrap();

            //draw text value
            let cr = snapshot.append_cairo(&graphene::Rect::new(0f32, 0f32, w as f32, h as f32));
            cr.set_font_size(self.unit_font_size.get() as f64 * size / 75.0);
            let text_extends = cr.text_extents(value_text.as_str()).unwrap();
            cr.move_to(
                (size / 2.0) - ((text_extends.width() / 2.0) + text_extends.x_bearing()),
                (size / 2.0) - ((text_extends.height() / 2.0) + text_extends.y_bearing()),
            );
            cr.text_path(value_text.as_str());
            cr.set_source_rgb(0.0, 0.5, 0.5);
            cr.fill().unwrap();
        }

        // fn request_mode(&self) -> gtk4::SizeRequestMode {
        //     gtk4::SizeRequestMode::ConstantSize
        // }

        fn measure(&self, _orientation: gtk4::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            (120, 300, -1, -1)
        }
    }
}

glib::wrapper! {
    pub struct GihexGauge(ObjectSubclass<imp::GihexGauge>)
    @extends gtk4::Widget,
    @implements gtk4::Accessible;
}

impl GihexGauge {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
