use gtk4::glib::{self, Object};

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
        prelude::*,
        subclass::widget::{WidgetClassExt, WidgetImpl},
        Widget,
    };

    #[derive(Properties, Default)]
    #[properties(wrapper_type=super::GihexCircleBar)]
    pub struct GihexCircleBar {
        #[property(get,set=Self::set_value,type=f32, default = 30f32)]
        value: Cell<f32>,
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
                value: Cell::new(0.0),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GihexCircleBar {}

    impl WidgetImpl for GihexCircleBar {
        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            
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
