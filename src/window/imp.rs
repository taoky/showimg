use std::cell::Cell;

use gtk::cairo::{RectangleInt, Region};
use gtk::glib;
use gtk::prelude::{GestureSingleExt, GtkWindowExt, NativeExt, SurfaceExt, WidgetExt};
use gtk::subclass::prelude::*;

use crate::MouseBehavior;

// Object holding the state
#[derive(Default)]
pub struct Window {
    pub behavior: Cell<MouseBehavior>,
    pub ratio: Cell<f64>,
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
        let Some(surface) = self.obj().surface() else {
            return;
        };
        if let Some(child) = self.obj().child() {
            // Workaround for mysterious GtkImage allocation...
            let window_ratio = width as f64 / height as f64;
            if window_ratio >= self.ratio.get() {
                // window is wider than image
                // println!("window is wider than image");
                child.set_valign(gtk::Align::Fill);
                child.set_halign(gtk::Align::Center);
            } else {
                // window is taller than image
                // println!("window is taller than image");
                child.set_valign(gtk::Align::Center);
                child.set_halign(gtk::Align::Fill);
            }

            if self.behavior.get() != MouseBehavior::Passthrough {
                // When valign/halign is set, the allocation is not updated
                // and it seems impossible to force it to update...
                // so we have to calculate the real allocation ourselves
                // and set the right input region

                if window_ratio >= self.ratio.get() {
                    // window is wider than image
                    let new_width = (height as f64 * self.ratio.get()) as i32;
                    let x = (width - new_width) / 2;
                    let region = Region::create_rectangle(&RectangleInt::new(x, 0, new_width, height));
                    surface.set_input_region(&region);
                } else {
                    // window is taller than image
                    let new_height = (width as f64 / self.ratio.get()) as i32;
                    let y = (height - new_height) / 2;
                    let region = Region::create_rectangle(&RectangleInt::new(0, y, width, new_height));
                    surface.set_input_region(&region);
                }
            }
        }

        if self.behavior.get() == MouseBehavior::Passthrough {
            surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
        }
    }
}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
