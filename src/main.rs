use clap::{ArgEnum, Parser};
use gtk::gio::File;
use gtk::Application;
use gtk::{
    gdk, prelude::*, style_context_add_provider_for_display, CssProvider, FileChooserNative,
    Picture, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

mod window;

const APP_ID: &str = "moe.taoky.showimg";
const CSS: &str = ".background {
    background-color: rgba(0, 0, 0, 0);
}";

#[derive(Clone, Copy, ArgEnum, Debug, PartialEq, Default)]
pub enum MouseBehavior {
    #[default]
    None,
    Drag,
    Passthrough,
}

#[derive(Parser, Debug, Clone)]
#[clap(about, version)]
pub struct Args {
    /// The image file to open. Empty value would open a file chooser dialog
    #[clap(value_parser)]
    file: Option<String>,

    /// The accelerator to quit the application, "none" to disable.
    /// Syntax: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/fn.accelerator_parse.html
    #[clap(short, long, value_parser, default_value = "q")]
    quit_with: String,

    /// Controls how window reacts to mouse events
    #[clap(short, long, value_enum, default_value_t = MouseBehavior::None)]
    mouse: MouseBehavior,

    /// Disable right-click context menu
    #[clap(long)]
    no_context_menu: bool,

    /// Disable double-click to maximize
    #[clap(long)]
    no_maximize: bool,
}

fn main() {
    // Parse args to get image
    let args = Args::parse();

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app: &Application| build_ui(app, &args));

    // Run the application (bypass GTK command line handling)
    app.run_with_args(&[""]);
}

fn message_dialog(title: &str, message: &str, parent: &impl IsA<gtk::Window>) {
    gtk::glib::MainContext::default().block_on(async {
        let dialog = gtk::MessageDialog::builder()
            .message_type(gtk::MessageType::Error)
            .text(title)
            .secondary_text(message)
            .buttons(gtk::ButtonsType::Close)
            .transient_for(parent)
            .modal(true)
            .build();
        dialog.run_future().await;
    });
}

fn build_ui(app: &Application, args: &Args) {
    let window = window::Window::new(app, "Show Img", args.clone());
    let filename = match &args.file {
        None => gtk::glib::MainContext::default().block_on(async {
            let dialog = FileChooserNative::builder()
                .title("Open Image")
                .action(gtk::FileChooserAction::Open)
                .modal(true)
                .transient_for(&window)
                .build();
            let filename = if dialog.run_future().await == gtk::ResponseType::Accept {
                dialog
                    .file()
                    .and_then(|f| f.path())
                    .map(|p| p.to_string_lossy().to_string())
            } else {
                None
            };
            match filename {
                Some(f) => f,
                None => {
                    println!("No file selected, exiting...");
                    std::process::exit(1);
                }
            }
        }),
        Some(filename) => filename.to_string(),
    };
    if args.quit_with != "none" {
        app.set_accels_for_action("window.close", &[&args.quit_with]);
    }

    // Image
    let texture = match gdk::Texture::from_file(&File::for_path(filename)) {
        Ok(t) => t,
        Err(e) => {
            message_dialog("Failed to load image", &e.to_string(), &window);
            std::process::exit(1);
        }
    };
    let image = Picture::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Fill)
        .paintable(&texture)
        .build();
    let image_height = texture.height();
    let image_width = texture.width();
    if image_height == 0 || image_width == 0 {
        message_dialog("Failed to load image", "Image has zero size", &window);
        std::process::exit(1);
    }
    window.set_ratio(image_width as f64 / image_height as f64);

    // CSS style
    let css_provider = CssProvider::new();
    css_provider.load_from_data(CSS);
    style_context_add_provider_for_display(
        &gdk::Display::default().expect("Error initializing gdk default display"),
        &css_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_decorated(false);
    window.set_child(Some(&image));

    if args.no_context_menu {
        println!("Hint: Press Alt+Space to set window always on top");
    }
    // Present window
    window.present();
}
