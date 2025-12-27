use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, ComboBoxText, Label, Box as GtkBox, Orientation};
use webbrowser;
use std::process::Command;

mod config;
mod dpi;

fn main() {
    let app = Application::builder()
        .application_id("org.changegoose.duckpx")
        .build();

    app.connect_activate(|app| {
        // Lade Konfiguration
        let config = config::Config::load();

        // Fenster erstellen
        let window = ApplicationWindow::builder()
            .application(app)
            .title("DuckPx")
            .build();

        // Hauptbox (vertikal)
        let vbox = GtkBox::new(Orientation::Vertical, 5);

        // Toolbar (oben oder unten)
        let toolbar = GtkBox::new(Orientation::Horizontal, 5);

        // Update-Knopf
        let update_button = Button::with_label("Update");
        update_button.connect_clicked(move |_| {
            Command::new("sh")
                .arg("-c")
                .arg("/usr/local/share/duckpx/update.sh")
                .spawn()
                .expect("Update fehlgeschlagen!");
        });

        // GitHub-Knopf
        let github_button = Button::with_label("GitHub");
        github_button.connect_clicked(move |_| {
            webbrowser::open("https://github.com/Change-Goose-Open-Surce-Software/duckpx")
                .unwrap();
        });

        // Einstellungen-Knopf (Platzhalter)
        let settings_button = Button::with_label("Einstellungen");
        settings_button.connect_clicked(|_| {
            println!("Einstellungen werden geöffnet...");
        });

        // Dropdown für Einheiten
        let unit_combo = ComboBoxText::new();
        unit_combo.append_text("Pixel (px)");
        unit_combo.append_text("Millimeter (mm)");
        unit_combo.append_text("Inch (in)");
        unit_combo.set_active(Some(0));

        // Eingabefeld
        let input_entry = Entry::builder()
            .placeholder_text("Größe eingeben (z. B. 50)")
            .build();

        // Berechnen-Knopf
        let calculate_button = Button::with_label("Berechnen");
        let result_label = Label::new(None);

        calculate_button.connect_clicked({
            let input_entry = input_entry.clone();
            let unit_combo = unit_combo.clone();
            let result_label = result_label.clone();

            move |_| {
                let input = input_entry.text().parse::<f64>().unwrap_or(0.0);
                let unit = unit_combo.active_text().unwrap().as_str();

                let (px, mm, inch) = match unit {
                    "Pixel (px)" => (input, dpi::px_to_mm(input), dpi::px_to_inch(input)),
                    "Millimeter (mm)" => (dpi::mm_to_px(input), input, dpi::mm_to_inch(input)),
                    "Inch (in)" => (dpi::inch_to_px(input), dpi::inch_to_mm(input), input),
                    _ => (0.0, 0.0, 0.0),
                };

                result_label.set_text(&format!("{}px = {:.2}mm = {:.2}in", px, mm, inch));
            }
        });

        // Toolbar aufbauen
        toolbar.pack_start(&update_button, false, false, 0);
        toolbar.pack_start(&github_button, false, false, 0);
        toolbar.pack_start(&settings_button, false, false, 0);

        // Position der Toolbar (oben oder unten)
        match config.ui.toolbar_position.as_str() {
            "top" => {
                vbox.pack_start(&toolbar, false, false, 0);
                vbox.pack_start(&input_entry, false, false, 0);
                vbox.pack_start(&unit_combo, false, false, 0);
                vbox.pack_start(&calculate_button, false, false, 0);
                vbox.pack_start(&result_label, false, false, 0);
            },
            "bottom" => {
                vbox.pack_start(&input_entry, false, false, 0);
                vbox.pack_start(&unit_combo, false, false, 0);
                vbox.pack_start(&calculate_button, false, false, 0);
                vbox.pack_start(&result_label, false, false, 0);
                vbox.pack_end(&toolbar, false, false, 0);
            },
            _ => (),
        }

        window.add(&vbox);
        window.show_all();
    });

    app.run();
}
