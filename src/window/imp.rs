use std::cell::Cell;

use gtk::cairo::{RectangleInt, Region};
use gtk::glib;
use gtk::prelude::{GestureSingleExt, NativeExt, SurfaceExt, WidgetExt};
use gtk::subclass::prelude::*;

use crate::MouseBehavior;

// Object holding the state
#[derive(Default)]
pub struct Window {
    pub behavior: Cell<MouseBehavior>,
    pub gesturedrag: gtk::GestureDrag,
    pub gestureclick: gtk::GestureClick,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "ShowImgWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().add_controller(self.gesturedrag.clone());
        self.gestureclick.set_button(0);
        self.obj().add_controller(self.gestureclick.clone());
    }
}

impl WidgetImpl for Window {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
        if self.behavior.get() == MouseBehavior::Passthrough {
            let Some(surface) = self.obj().surface() else {
                return;
            };
            surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
        }
    }
}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
