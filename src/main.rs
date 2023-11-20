use clap::Parser;
use gtk::gio::File;
use gtk::{
    gdk, prelude::*, style_context_add_provider_for_display, CssProvider, Picture,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.taoky.showimg";
const CSS: &str = ".background {
    background-color: rgba(0, 0, 0, 0);
}";

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    // The image file to open
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    // Parse args to get image
    let args = Args::parse();

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app: &Application| build_ui(app, &args.file));

    // Run the application (bypass GTK command line handling)
    app.run_with_args(&[""]);
}

fn build_ui(app: &Application, filename: &str) {
    // CSS style
    let css_provider = CssProvider::new();
    css_provider.load_from_data(CSS);
    style_context_add_provider_for_display(
        &gdk::Display::default().expect("Error initializing gdk default display"),
        &css_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Image
    let texture = gdk::Texture::from_file(&File::for_path(filename)).expect("Cannot open image");
    let image = Picture::builder().paintable(&texture).build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Show Img")
        .child(&image)
        .build();

    window.set_decorated(false);

    println!("Press Alt+Space to set window always on top");
    // Present window
    window.present();
}
