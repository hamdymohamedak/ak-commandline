use ak_macros::*;
use gdk::Display;
use gtk::prelude::*;
use gtk::{
    gdk, glib, Application, ApplicationWindow, Box, Button, CssProvider, Entry, Label,
    ScrolledWindow,
};

const APP_ID: &str = "org.gtk_rs.hamdymohamedak";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("./style.css"));

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let submit_btn = Button::builder().label("Go").margin_bottom(12).build();
    let entry_command = Entry::new();
    entry_command.set_placeholder_text(Some("AK>>>"));
    let text_label = Label::new(None);

    let box_container = Box::new(gtk::Orientation::Vertical, 0);
    box_container.append(&entry_command);
    box_container.append(&submit_btn);
    box_container.append(&text_label);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrollbar
        .vscrollbar_policy(gtk::PolicyType::Automatic) // Enable vertical scrollbar
        .min_content_width(500) // Set minimum content width
        .min_content_height(300) // Set minimum content height
        .child(&box_container)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("AK-Commandline")
        .child(&box_container)
        .child(&scrolled_window)
        .build();

    let result_label_clone = text_label.clone();
    let entry_command_clone = entry_command.clone();
    let submit_btn_clone = submit_btn.clone();
    submit_btn.connect_clicked(move |_| {
        let command = entry_command_clone.text();
        let command_output = if !command.is_empty() {
            if command.starts_with("cd ") {
                // Extract the directory from the command
                let new_dir = command.trim_start_matches("cd ");
                // Attempt to change the directory
                if std::env::set_current_dir(new_dir).is_err() {
                    "Failed to change directory".to_string()
                } else {
                    // Optionally update UI to show the new directory
                    format!("Changed directory to {}", new_dir)
                }
            } else {
                // Execute non-directory navigation commands
                terminal!("sh", &command)
            }
        } else {
            String::new()
        };
        result_label_clone.set_text(&command_output);
    });

    // press Enter
    entry_command.connect_activate(move |_| {
        submit_btn_clone.activate();
    });
    window.set_height_request(50);
    window.set_width_request(250);
    window.set_opacity(0.9);

    // Present window
    window.present();
}
