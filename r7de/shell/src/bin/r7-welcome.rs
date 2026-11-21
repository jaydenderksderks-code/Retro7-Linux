use std::process::Command;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Label, Orientation,
    STYLE_PROVIDER_PRIORITY_APPLICATION, gdk,
};

const APP_ID: &str = "org.retro7.Welcome";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_window);
    app.run();
}

fn build_window(app: &Application) {
    install_styles();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Welcome to Retro7")
        .default_width(760)
        .default_height(560)
        .resizable(false)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 18);
    root.add_css_class("welcome-root");
    root.set_margin_top(28);
    root.set_margin_bottom(24);
    root.set_margin_start(32);
    root.set_margin_end(32);

    let eyebrow = Label::new(Some("COMPUTING, REIMAGINED."));
    eyebrow.set_xalign(0.0);
    eyebrow.add_css_class("welcome-eyebrow");
    root.append(&eyebrow);

    let title = Label::new(Some("Welcome to Retro7"));
    title.set_xalign(0.0);
    title.add_css_class("welcome-title");
    root.append(&title);

    let subtitle = Label::new(Some(
        "A familiar desktop with a fresh perspective. Take a look around, make yourself at home, and explore what is included in this preview.",
    ));
    subtitle.set_xalign(0.0);
    subtitle.set_wrap(true);
    subtitle.add_css_class("welcome-subtitle");
    root.append(&subtitle);

    let divider = GtkBox::new(Orientation::Horizontal, 0);
    divider.add_css_class("welcome-divider");
    divider.set_size_request(-1, 1);
    root.append(&divider);

    let cards = GtkBox::new(Orientation::Horizontal, 12);
    cards.set_vexpand(true);
    cards.append(&action_card(
        "Install Retro7",
        "Open the current installer entry point. This preview may use the advanced Arch installer.",
        "Open installer",
        "r7-installer",
    ));
    cards.append(&action_card(
        "Explore the desktop",
        "Open your files, terminal, and system settings to try the live environment.",
        "Open settings",
        "r7-settings",
    ));
    root.append(&cards);

    let footer = GtkBox::new(Orientation::Horizontal, 10);
    footer.add_css_class("welcome-footer");
    let note = Label::new(Some(
        "Development preview • Changes are not installed until you install a system",
    ));
    note.set_xalign(0.0);
    note.set_wrap(true);
    note.set_hexpand(true);
    note.add_css_class("welcome-note");
    footer.append(&note);

    let close = Button::with_label("Got it");
    close.add_css_class("welcome-primary");
    let window_for_close = window.clone();
    close.connect_clicked(move |_| {
        mark_seen();
        window_for_close.close();
    });
    footer.append(&close);
    root.append(&footer);

    window.set_child(Some(&root));
    window.present();
}

fn action_card(title: &str, description: &str, button_label: &str, program: &'static str) -> GtkBox {
    let card = GtkBox::new(Orientation::Vertical, 12);
    card.add_css_class("welcome-card");
    card.set_hexpand(true);
    card.set_margin_top(4);
    card.set_margin_bottom(4);
    card.set_margin_start(2);
    card.set_margin_end(2);

    let heading = Label::new(Some(title));
    heading.set_xalign(0.0);
    heading.add_css_class("welcome-card-title");
    card.append(&heading);

    let body = Label::new(Some(description));
    body.set_xalign(0.0);
    body.set_yalign(0.0);
    body.set_wrap(true);
    body.set_hexpand(true);
    body.add_css_class("welcome-card-body");
    card.append(&body);

    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    card.append(&spacer);

    let button = Button::with_label(button_label);
    button.add_css_class("welcome-action");
    button.set_halign(gtk::Align::Start);
    button.connect_clicked(move |_| {
        if let Err(error) = Command::new(program).spawn() {
            eprintln!("Retro7 could not start {program}: {error}");
        }
    });
    card.append(&button);
    card
}

fn install_styles() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../../assets/r7-welcome.css"));
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn mark_seen() {
    if let Some(home) = std::env::var_os("HOME") {
        let config = std::path::PathBuf::from(home).join(".config/retro7");
        if std::fs::create_dir_all(&config).is_ok() {
            let _ = std::fs::write(config.join("welcome-seen"), "1\n");
        }
    }
}
