use gtk4::glib;
use gtk4::glib::Object;

mod imp {
    use std::cell::Cell;

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
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GihexGauge {
        const NAME: &'static str = "GihexGauge";
        type Type = super::GihexGauge;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("gx_gauge");
        }

        fn new() -> Self {
            GihexGauge {
                value: Cell::new(30f32),
                min_value: Cell::new(0f32),
                max_value: Cell::new(100f32),
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

            let cr = snapshot.append_cairo(&graphene::Rect::new(0f32, 0f32, w as f32, h as f32));

            // cr.arc(xc, yc, radius, angle1, angle2)
            cr.set_source_rgb(0.0, 0.5, 0.5);
            cr.arc(w as f64 / 2.0, h as f64 / 2.0, h as f64 / 2.0, 0.0, 360.0);
            cr.fill().unwrap();
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
