use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use glib::clone;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut title = "Discord Quest Completer".to_string();
    let mut start_hidden = false;
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                if i + 1 < args.len() {
                    title = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--tray" => {
                start_hidden = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    let app = Application::builder()
        .application_id("me.markterence.discordquestcompleter.runner")
        .build();

    let title_clone = title.clone();
    let start_hidden_clone = start_hidden;

    app.connect_activate(move |app| {
        build_ui(app, &title_clone, start_hidden_clone);
    });

    app.run();
}

fn build_ui(app: &Application, title_text: &str, start_hidden: bool) {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title(title_text)
        .default_width(400)
        .default_height(400)
        .build();

    // Create vertical box layout
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    // App title label
    let app_label = Label::new(Some("Discord Quest Completer"));
    app_label.add_css_class("title-1");
    vbox.append(&app_label);

    // Game title label
    let title_label = Label::new(Some(title_text));
    title_label.add_css_class("title-2");
    title_label.set_margin_top(20);
    vbox.append(&title_label);

    // Description label
    let desc_label = Label::new(Some("This program is part of the Discord Quest Completer"));
    desc_label.set_margin_top(20);
    vbox.append(&desc_label);

    // GitHub link button
    let link_button = Button::with_label("Source on Github");
    link_button.set_margin_top(40);
    
    link_button.connect_clicked(|_| {
        let _ = open::that("https://github.com/markterence/discord-quest-completer");
    });
    
    vbox.append(&link_button);

    // Quit button
    let quit_button = Button::with_label("Quit");
    quit_button.set_margin_top(10);
    
    quit_button.connect_clicked(clone!(@weak window => move |_| {
        window.close();
    }));
    
    vbox.append(&quit_button);

    window.set_child(Some(&vbox));

    if !start_hidden {
        window.present();
    }
}