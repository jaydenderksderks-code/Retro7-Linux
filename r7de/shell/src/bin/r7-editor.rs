use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, FileChooserAction,
    FileChooserNative, Label, Orientation, ResponseType, ScrolledWindow, TextView,
};

const APPLICATION_ID: &str = "org.retro7.Editor";

fn main() {
    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();
    application.connect_activate(build_window);
    application.run();
}

fn build_window(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Retro7 Editor")
        .default_width(900)
        .default_height(640)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 8);
    root.set_margin_top(10);
    root.set_margin_bottom(10);
    root.set_margin_start(10);
    root.set_margin_end(10);

    let toolbar = GtkBox::new(Orientation::Horizontal, 6);
    let open = Button::with_label("Open");
    let save = Button::with_label("Save");
    let save_as = Button::with_label("Save As");
    toolbar.append(&open);
    toolbar.append(&save);
    toolbar.append(&save_as);
    root.append(&toolbar);

    let editor = TextView::new();
    editor.set_monospace(true);
    editor.set_wrap_mode(gtk::WrapMode::None);
    editor.set_vexpand(true);
    editor.set_hexpand(true);
    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&editor)
        .build();
    root.append(&scroll);

    let status = Label::new(Some("Untitled document"));
    status.set_xalign(0.0);
    root.append(&status);
    window.set_child(Some(&root));

    let current_file: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

    {
        let window = window.clone();
        let editor = editor.clone();
        let status = status.clone();
        let current_file = current_file.clone();
        open.connect_clicked(move |_| {
            let dialog = FileChooserNative::new(
                Some("Open text file"),
                Some(&window),
                FileChooserAction::Open,
                Some("Open"),
                Some("Cancel"),
            );
            let editor = editor.clone();
            let status = status.clone();
            let current_file = current_file.clone();
            let window = window.clone();
            dialog.connect_response(move |dialog, response| {
                if response != ResponseType::Accept {
                    return;
                }
                let Some(path) = dialog.file().and_then(|file| file.path()) else {
                    status.set_text("The selected file has no local path.");
                    return;
                };
                match fs::read_to_string(&path) {
                    Ok(text) => {
                        editor.buffer().set_text(&text);
                        *current_file.borrow_mut() = Some(path.clone());
                        window.set_title(Some(&format!("Retro7 Editor — {}", path.display())));
                        status.set_text(&format!("Opened {}", path.display()));
                    }
                    Err(error) => status.set_text(&format!("Could not open file: {error}")),
                }
            });
            dialog.show();
        });
    }

    {
        let window = window.clone();
        let editor = editor.clone();
        let status = status.clone();
        let current_file = current_file.clone();
        save.connect_clicked(move |_| {
            if let Some(path) = current_file.borrow().clone() {
                save_file(&editor, &status, &window, &path);
            } else {
                choose_save_path(&window, &editor, &status, &current_file);
            }
        });
    }

    {
        let window = window.clone();
        let editor = editor.clone();
        let status = status.clone();
        let current_file = current_file.clone();
        save_as.connect_clicked(move |_| {
            choose_save_path(&window, &editor, &status, &current_file);
        });
    }

    if let Some(path) = std::env::args_os().nth(1).map(PathBuf::from) {
        match fs::read_to_string(&path) {
            Ok(text) => {
                editor.buffer().set_text(&text);
                *current_file.borrow_mut() = Some(path.clone());
                window.set_title(Some(&format!("Retro7 Editor — {}", path.display())));
                status.set_text(&format!("Opened {}", path.display()));
            }
            Err(error) => status.set_text(&format!("Could not open {}: {error}", path.display())),
        }
    }

    window.present();
}

fn choose_save_path(
    window: &ApplicationWindow,
    editor: &TextView,
    status: &Label,
    current_file: &Rc<RefCell<Option<PathBuf>>>,
) {
    let dialog = FileChooserNative::new(
        Some("Save text file"),
        Some(window),
        FileChooserAction::Save,
        Some("Save"),
        Some("Cancel"),
    );
    dialog.set_current_name("untitled.txt");

    let window = window.clone();
    let editor = editor.clone();
    let status = status.clone();
    let current_file = current_file.clone();
    dialog.connect_response(move |dialog, response| {
        if response != ResponseType::Accept {
            return;
        }
        let Some(path) = dialog.file().and_then(|file| file.path()) else {
            status.set_text("Choose a local file location.");
            return;
        };
        save_file(&editor, &status, &window, &path);
        if status.text().starts_with("Saved ") {
            *current_file.borrow_mut() = Some(path);
        }
    });
    dialog.show();
}

fn save_file(editor: &TextView, status: &Label, window: &ApplicationWindow, path: &PathBuf) {
    let buffer = editor.buffer();
    let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
    match fs::write(path, text.as_str()) {
        Ok(()) => {
            window.set_title(Some(&format!("Retro7 Editor — {}", path.display())));
            status.set_text(&format!("Saved {}", path.display()));
        }
        Err(error) => status.set_text(&format!("Could not save file: {error}")),
    }
}
