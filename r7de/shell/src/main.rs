use std::process::Command;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Label, MenuButton,
    Orientation, Popover, STYLE_PROVIDER_PRIORITY_APPLICATION, gdk,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const PANEL_HEIGHT: i32 = 44;
const APPLICATION_ID: &str = "org.retro7.Shell";

fn main() {
    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    application.connect_activate(build_panel);
    application.run();
}

fn build_panel(application: &Application) {
    install_styles();

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Retro7 Desktop Panel")
        .default_width(1280)
        .default_height(PANEL_HEIGHT)
        .decorated(false)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_namespace(Some("r7-panel"));
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.auto_exclusive_zone_enable();

    let panel = GtkBox::new(Orientation::Horizontal, 8);
    panel.add_css_class("r7-panel");

    let start_menu = create_start_menu();
    panel.append(&start_menu);

    let separator = GtkBox::new(Orientation::Vertical, 0);
    separator.set_size_request(1, 24);
    separator.add_css_class("panel-separator");
    panel.append(&separator);

    let task_group = GtkBox::new(Orientation::Horizontal, 4);
    task_group.add_css_class("task-group");

    let terminal = Button::with_label("Terminal");
    terminal.add_css_class("task-button");
    connect_launch(&terminal, "foot", "Terminal");
    task_group.append(&terminal);

    let settings = Button::with_label("Settings");
    settings.add_css_class("task-button");
    connect_launch(&settings, "r7-settings", "Settings");
    task_group.append(&settings);

    panel.append(&task_group);

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    panel.append(&spacer);

    let status = Label::new(Some("Retro7"));
    status.add_css_class("panel-status");
    panel.append(&status);

    let clock = Label::new(None);
    clock.add_css_class("panel-clock");
    update_clock(&clock);
    panel.append(&clock);

    gtk::glib::timeout_add_seconds_local(1, {
        let clock = clock.clone();
        move || {
            update_clock(&clock);
            gtk::glib::ControlFlow::Continue
        }
    });

    window.set_child(Some(&panel));
    window.present();
}

fn create_start_menu() -> MenuButton {
    let button = MenuButton::new();
    button.set_label("Retro7");
    button.add_css_class("start-button");

    let popover = Popover::new();
    popover.add_css_class("start-menu");
    let menu = GtkBox::new(Orientation::Vertical, 6);
    menu.set_margin_top(10);
    menu.set_margin_bottom(10);
    menu.set_margin_start(10);
    menu.set_margin_end(10);

    let heading = Label::new(Some("Retro7"));
    heading.set_xalign(0.0);
    heading.add_css_class("menu-heading");
    menu.append(&heading);

    let status = Label::new(Some("Choose an application"));
    status.set_xalign(0.0);
    status.add_css_class("menu-status");
    menu.append(&status);

    add_app_button(&menu, &status, "Terminal", "foot");
    add_app_button(&menu, &status, "Files", "r7-explorer");
    add_app_button(&menu, &status, "System Settings", "r7-settings");

    popover.set_child(Some(&menu));
    button.set_popover(Some(&popover));
    button
}

fn add_app_button(menu: &GtkBox, status: &Label, label: &str, program: &'static str) {
    let button = Button::with_label(label);
    button.set_halign(gtk::Align::Fill);
    button.add_css_class("menu-item");

    let status = status.clone();
    button.connect_clicked(move |_| match Command::new(program).spawn() {
        Ok(_) => status.set_text(&format!("Started {program}")),
        Err(error) => status.set_text(&format!("{program} is not available: {error}")),
    });

    menu.append(&button);
}

fn connect_launch(button: &Button, program: &'static str, label: &'static str) {
    button.connect_clicked(move |_| {
        if let Err(error) = Command::new(program).spawn() {
            eprintln!("Could not start {label} ({program}): {error}");
        }
    });
}

fn update_clock(clock: &Label) {
    let value = gtk::glib::DateTime::now_local()
        .and_then(|now| now.format("%H:%M"))
        .map(|formatted| formatted.to_string())
        .unwrap_or_else(|_| "--:--".to_string());
    clock.set_label(&value);
}

fn install_styles() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../assets/r7-panel.css"));

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
