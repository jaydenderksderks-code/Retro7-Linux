use std::fs;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, ComboBoxText, Label, Orientation};

const APPLICATION_ID: &str = "org.retro7.Settings";
const THEMES: [(&str, &str); 6] = [
    ("aero", "Retro7 Aero"),
    ("frutiger-aero", "Frutiger Aero"),
    ("vista", "Vista Glass"),
    ("luna", "Luna"),
    ("classic", "Classic"),
    ("y2k", "Y2K"),
];

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
        .title("Retro7 System Settings")
        .default_width(560)
        .default_height(340)
        .build();

    let content = GtkBox::new(Orientation::Vertical, 14);
    content.set_margin_top(24);
    content.set_margin_bottom(24);
    content.set_margin_start(24);
    content.set_margin_end(24);

    let title = Label::new(Some("Personalization"));
    title.set_xalign(0.0);
    title.add_css_class("title-1");
    content.append(&title);

    let description = Label::new(Some(
        "Choose the appearance used by the Retro7 desktop panel.",
    ));
    description.set_xalign(0.0);
    content.append(&description);

    let theme = ComboBoxText::new();
    for (id, name) in THEMES {
        theme.append(Some(id), name);
    }
    theme.set_active_id(Some(&read_theme()));
    content.append(&theme);

    let apply = Button::with_label("Apply theme");
    apply.set_halign(gtk::Align::Start);
    content.append(&apply);

    let status = Label::new(Some("Theme changes apply to the panel immediately."));
    status.set_xalign(0.0);
    content.append(&status);

    {
        let theme = theme.clone();
        let status = status.clone();
        apply.connect_clicked(move |_| {
            let Some(selected) = theme.active_id() else {
                status.set_text("Choose a theme first.");
                return;
            };
            match write_theme(selected.as_str()) {
                Ok(path) => status.set_text(&format!("Theme applied. Settings saved in {}.", path.display())),
                Err(error) => status.set_text(&format!("Could not save settings: {error}")),
            }
        });
    }

    window.set_child(Some(&content));
    window.present();
}

fn settings_path() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".config/retro7/theme.conf"))
}

fn read_theme() -> String {
    let Some(path) = settings_path() else {
        return "aero".to_string();
    };
    let Ok(value) = fs::read_to_string(path) else {
        return "aero".to_string();
    };
    value.trim().strip_prefix("theme=").unwrap_or("aero").to_string()
}

fn write_theme(theme: &str) -> std::io::Result<PathBuf> {
    if !THEMES.iter().any(|(id, _)| *id == theme) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "unsupported theme",
        ));
    }
    let path = settings_path().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "HOME is not set")
    })?;
    let directory = path.parent().expect("theme settings path has a parent");
    fs::create_dir_all(directory)?;
    let temporary = directory.join("theme.conf.tmp");
    fs::write(&temporary, format!("theme={theme}\n"))?;
    fs::rename(&temporary, &path)?;
    Ok(path)
}
