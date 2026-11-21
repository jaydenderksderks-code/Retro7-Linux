use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, ScrolledWindow};
use vte4::prelude::*;

const APPLICATION_ID: &str = "org.retro7.Terminal";

fn main() {
    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();
    application.connect_activate(build_window);
    application.run();
}

fn build_window(application: &Application) {
    let terminal = vte4::Terminal::new();
    terminal.set_scrollback_lines(10_000);
    terminal.set_bold_is_bright(true);
    terminal.set_hexpand(true);
    terminal.set_vexpand(true);

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Retro7 Terminal")
        .default_width(900)
        .default_height(560)
        .build();

    let layout = GtkBox::new(Orientation::Vertical, 6);
    let toolbar = GtkBox::new(Orientation::Horizontal, 6);
    toolbar.set_margin_top(8);
    toolbar.set_margin_start(8);
    toolbar.set_margin_end(8);

    let heading = Label::new(Some("Retro7 Terminal"));
    heading.set_xalign(0.0);
    heading.set_hexpand(true);
    heading.add_css_class("heading");
    toolbar.append(&heading);

    let clear = Button::with_label("Clear");
    toolbar.append(&clear);
    layout.append(&toolbar);

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .child(&terminal)
        .build();
    layout.append(&scroll);
    window.set_child(Some(&layout));

    clear.connect_clicked({
        let terminal = terminal.clone();
        move |_| terminal.feed_child(b"\x0c")
    });

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"));
    let home_string = home.to_string_lossy().to_string();
    let status_window = window.clone();

    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        Some(&home_string),
        &[shell.as_str()],
        &[],
        gtk::glib::SpawnFlags::DEFAULT,
        || {},
        -1,
        None::<&gtk::gio::Cancellable>,
        move |result| {
            if let Err(error) = result {
                status_window.set_title(Some(&format!("Retro7 Terminal — {error}")));
            }
        },
    );

    window.present();
    terminal.grab_focus();
}
