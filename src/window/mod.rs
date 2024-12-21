mod imp;

use glib::Object;
use gtk::{
    gdk, gio, glib,
    prelude::{
        Cast, EventControllerExt, GestureDragExt, GestureExt, GestureSingleExt, GtkWindowExt,
        NativeExt, ToplevelExt, WidgetExt,
    },
    subclass::prelude::ObjectSubclassIsExt,
    Application,
};

use crate::{Args, MouseBehavior};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application, title: &str, args: Args) -> Self {
        let window: Self = Object::builder().build();
        let imp = window.imp();
        imp.behavior.set(args.mouse);
        if args.mouse == MouseBehavior::Drag {
            imp.gesturedrag.connect_drag_update(glib::clone!(
                #[weak]
                window,
                move |g, offset_x, offset_y| {
                    let Some((start_x, start_y)) = g.start_point() else {
                        return;
                    };
                    let Some(surface) = window.surface() else {
                        return;
                    };
                    let Some(device) = g.device() else {
                        return;
                    };
                    let button = g.current_button();
                    let Some(event) = g.current_event() else {
                        return;
                    };
                    let event_time = event.time();
                    if let Some(toplevel) = surface.downcast_ref::<gdk::Toplevel>() {
                        let x = start_x + offset_x;
                        let y = start_y + offset_y;
                        toplevel.begin_move(&device, button.try_into().unwrap(), x, y, event_time);
                    }
                }
            ));
        }

        imp.gestureclick.connect_pressed(glib::clone!(
            #[weak]
            window,
            move |g, n_press, _x, _y| {
                let Some(surface) = window.surface() else {
                    return;
                };
                let Some(toplevel) = surface.downcast_ref::<gdk::Toplevel>() else {
                    return;
                };
                let Some(event) = g.current_event() else {
                    return;
                };
                let button = g.current_button();
                match button {
                    1 => {
                        // primary
                        if n_press == 2 && !args.no_maximize {
                            let _ = window.activate_action("window.toggle-maximized", None);
                        }
                    }
                    3 => {
                        // right
                        if n_press == 1 && !args.no_context_menu {
                            toplevel.show_window_menu(event);
                        }
                    }
                    _ => {}
                }
            }
        ));
        window.set_application(Some(app));
        window.set_title(Some(title));
        window
    }
}
