use gtk::prelude::*;
use gtk::{Builder, Button, DrawingArea, Image, Window};
use gio::prelude::*;
use gio::{File, FileChooserAction, FileFilter, FileChooserDialog};

fn open_file_dialog(window: &Window) -> Option<File> {
    let file_chooser = FileChooserDialog::new(
        Some("Select an Image"),
        Some(window),
        FileChooserAction::Open,
    );
    file_chooser.add_buttons(&[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);

    let filter = FileFilter::new();
    filter.add_mime_type("image/*");
    file_chooser.add_filter(&filter);

    match file_chooser.run() {
        gtk::ResponseType::Ok => Some(file_chooser.get_file().unwrap()),
        _ => None,
    }
}

fn main() {
    // Initialize GTK
    gtk::init().unwrap();

    // Load the UI from a glade file
    let builder = Builder::new_from_string(include_str!("main.glade"));

    // Get the widgets from the builder
    let window: Window = builder.get_object("window").unwrap();
    let image: Image = builder.get_object("image").unwrap();
    let import_button: Button = builder.get_object("import_button").unwrap();
    let canvas: DrawingArea = builder.get_object("canvas").unwrap();

    // Set up the import button
    import_button.connect_clicked(move |_| {
        if let Some(file) = open_file_dialog(&window) {
            let pixbuf = gdk_pixbuf::Pixbuf::new_from_file(&file.get_path().unwrap()).unwrap();
            image.set_from_pixbuf(Some(&pixbuf));
        }
    });

    /*
    // Set up the canvas
    canvas.connect_draw(move |_, context| {
        // Draw the current image onto the canvas
        let pixbuf = image.get_pixbuf().unwrap();
        context.set_source_pixbuf(&pixbuf, 0.0, 0.0);
        context.paint();

        // TODO: Implement the brush tool here
    });*/
}