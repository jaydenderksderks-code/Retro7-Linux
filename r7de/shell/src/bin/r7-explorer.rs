use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Entry, Label, ListBox, Orientation, ScrolledWindow};

const APPLICATION_ID: &str = "org.retro7.Explorer";

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
        .title("Retro7 Explorer")
        .default_width(900)
        .default_height(620)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 8);
    root.set_margin_top(12);
    root.set_margin_bottom(12);
    root.set_margin_start(12);
    root.set_margin_end(12);

    let toolbar = GtkBox::new(Orientation::Horizontal, 6);
    let home = Button::with_label("Home");
    let up = Button::with_label("Up");
    let location = Entry::new();
    location.set_hexpand(true);
    toolbar.append(&home);
    toolbar.append(&up);
    toolbar.append(&location);
    root.append(&toolbar);

    let list = ListBox::new();
    list.set_selection_mode(gtk::SelectionMode::None);
    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .child(&list)
        .build();
    root.append(&scroll);

    let status = Label::new(None);
    status.set_xalign(0.0);
    root.append(&status);
    window.set_child(Some(&root));

    let current = Rc::new(RefCell::new(home_directory()));
    show_directory(&current.borrow(), &location, &list, &status);

    {
        let current = current.clone();
        let location = location.clone();
        let list = list.clone();
        let status = status.clone();
        home.connect_clicked(move |_| {
            let path = home_directory();
            *current.borrow_mut() = path.clone();
            show_directory(&path, &location, &list, &status);
        });
    }

    {
        let current = current.clone();
        let location = location.clone();
        let list = list.clone();
        let status = status.clone();
        up.connect_clicked(move |_| {
            let parent = current.borrow().parent().map(Path::to_path_buf);
            if let Some(parent) = parent {
                *current.borrow_mut() = parent.clone();
                show_directory(&parent, &location, &list, &status);
            }
        });
    }

    {
        let current = current.clone();
        let location = location.clone();
        let list = list.clone();
        let status = status.clone();
        location.connect_activate(move |entry| {
            let requested = PathBuf::from(entry.text().as_str());
            if requested.is_dir() {
                *current.borrow_mut() = requested.clone();
                show_directory(&requested, &location, &list, &status);
            } else {
                status.set_text("That location is not a folder.");
            }
        });
    }

    window.present();
}

fn home_directory() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .unwrap_or_else(|| PathBuf::from("/"))
}

fn show_directory(path: &Path, location: &Entry, list: &ListBox, status: &Label) {
    location.set_text(&path.display().to_string());
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }

    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            status.set_text(&format!("Cannot read folder: {error}"));
            return;
        }
    };

    let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
    entries.sort_by_key(|entry| {
        let is_directory = entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false);
        (!is_directory, entry.file_name().to_string_lossy().to_lowercase())
    });

    let count = entries.len();
    for entry in entries {
        let path = entry.path();
        let is_directory = entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false);
        let name = entry.file_name().to_string_lossy().to_string();
        let label = if is_directory {
            format!("📁  {name}")
        } else {
            format!("📄  {name}")
        };
        let button = Button::with_label(&label);
        button.set_halign(gtk::Align::Fill);
        button.set_tooltip_text(Some(&path.display().to_string()));

        let current_for_action = current.clone();
        let list_for_action = list.clone();
        let location_for_action = location.clone();
        let status_for_action = status.clone();
        button.connect_clicked(move |_| {
            if path.is_dir() {
                *current_for_action.borrow_mut() = path.clone();
                show_directory(&path, &location_for_action, &list_for_action, &status_for_action);
            } else {
                open_file(&path, &status_for_action);
            }
        });
        list.append(&button);
    }
    status.set_text(&format!("{count} items — {}", path.display()));
}

fn open_file(path: &Path, status: &Label) {
    let file = gtk::gio::File::for_path(path);
    let uri = file.uri();
    if let Err(error) = gtk::gio::AppInfo::launch_default_for_uri(
        uri.as_str(),
        None::<&gtk::gio::AppLaunchContext>,
    ) {
        let fallback = Command::new("xdg-open").arg(path).spawn();
        if let Err(fallback_error) = fallback {
            status.set_text(&format!("Cannot open file: {error}; {fallback_error}"));
            return;
        }
    }
    status.set_text(&format!("Opened {}", path.display()));
}
