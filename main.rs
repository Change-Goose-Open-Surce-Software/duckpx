use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, ComboBoxText, Label, DrawingArea, Box as GtkBox, Orientation};
use webbrowser;
use std::process::Command;
use gtk::gdk::RGBA;

mod config;
mod dpi;

fn main() {
    let app = Application::builder()
        .application_id("org.changegoose.duckpx")
        .build();

    app.connect_activate(|app| {
        // Lade Konfiguration
        let config = config::Config::load();

        // Hauptfenster erstellen
        let window = ApplicationWindow::builder()
            .application(app)
            .title("DuckPx")
            .default_width(400)
            .default_height(500)
            .build();

        // Hauptbox (vertikal)
        let vbox = GtkBox::new(Orientation::Vertical, 5);

        // Toolbar (oben oder unten)
        let toolbar = GtkBox::new(Orientation::Horizontal, 5);

        // Update-Knopf mit Statuslabel
        let update_button = Button::with_label("Update");
        let update_label = Label::new(Some("Bereit für Update..."));

        // GitHub-Knopf
        let github_button = Button::with_label("GitHub");

        // Einstellungen-Knopf
        let settings_button = Button::with_label("Einstellungen");

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

        // Ergebnis-Label
        let result_label = Label::new(Some("Ergebnis erscheint hier"));

        // Zeichnungsbereich für Pixel
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(200, 200);

        // Logik für Berechnen-Knopf
        calculate_button.connect_clicked({
            let input_entry = input_entry.clone();
            let unit_combo = unit_combo.clone();
            let result_label = result_label.clone();
            let drawing_area = drawing_area.clone();
            let square_color = config.colors.square.clone(); // Klone die Farbe hier

            move |_| {
                let input = input_entry.text().parse::<f64>().unwrap_or(0.0);
                let binding = unit_combo.active_text().unwrap();
                let unit = binding.as_str();

                let (px, mm, inch) = match unit {
                    "Pixel (px)" => (input, dpi::px_to_mm(input), dpi::px_to_inch(input)),
                    "Millimeter (mm)" => (dpi::mm_to_px(input), input, dpi::mm_to_inch(input)),
                    "Inch (in)" => (dpi::inch_to_px(input), dpi::inch_to_mm(input), input),
                    _ => (0.0, 0.0, 0.0),
                };

                result_label.set_text(&format!("{}px = {:.2}mm = {:.2}in", px, mm, inch));

                // Zeichne das Pixel-Quadrat
                drawing_area.queue_draw();
                let square_color_clone = square_color.clone(); // Klone für den Draw-Handler
                drawing_area.connect_draw(move |_, cr| {
                    let square_rgba = RGBA::parse(&square_color_clone).unwrap();
                    cr.set_source_rgba(
                        square_rgba.red() as f64,
                        square_rgba.green() as f64,
                        square_rgba.blue() as f64,
                        1.0,
                    );
                    cr.rectangle(10.0, 10.0, px, px);
                    cr.fill().unwrap();
                    Inhibit(false)
                });
            }
        });

        // Logik für Update-Knopf
        update_button.connect_clicked({
            let update_label = update_label.clone();

            move |_| {
                update_label.set_text("Update läuft...");

                let output = Command::new("sh")
                    .arg("-c")
                    .arg("/usr/local/share/duckpx/update.sh")
                    .output()
                    .expect("Update fehlgeschlagen!");

                if output.status.success() {
                    update_label.set_text("✅ Update erfolgreich!");
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    update_label.set_text(&format!("❌ Fehler: {}", error_msg));
                }
            }
        });

        // Logik für GitHub-Knopf
        github_button.connect_clicked(move |_| {
            webbrowser::open("https://github.com/Change-Goose-Open-Surce-Software/duckpx")
                .unwrap();
        });

        // Logik für Einstellungen-Knopf
        settings_button.connect_clicked({
            let window = window.clone();
            let config_toolbar_pos = config.ui.toolbar_position.clone(); // Klone für späteren Gebrauch

            move |_| {
                let current_config = config::Config::load(); // Lade Config neu im Closure
                
                let settings_window = ApplicationWindow::builder()
                    .title("DuckPx Einstellungen")
                    .transient_for(&window)
                    .build();

                let vbox = GtkBox::new(Orientation::Vertical, 5);

                // Dropdown für Toolbar-Position
                let toolbar_pos_combo = ComboBoxText::new();
                toolbar_pos_combo.append_text("Oben");
                toolbar_pos_combo.append_text("Unten");
                toolbar_pos_combo.set_active(match current_config.ui.toolbar_position.as_str() {
                    "top" => Some(0),
                    "bottom" => Some(1),
                    _ => Some(0),
                });

                // Farbauswahl (Hintergrund)
                let bg_color_label = Label::new(Some("Hintergrundfarbe (Hex, z. B. #FFFFFF):"));
                let bg_color_entry = Entry::builder()
                    .text(&current_config.colors.background)
                    .build();

                // Farbauswahl (Quadrat)
                let square_color_label = Label::new(Some("Quadratfarbe (Hex, z. B. #FFA500):"));
                let square_color_entry = Entry::builder()
                    .text(&current_config.colors.square)
                    .build();

                // Speichern-Knopf
                let save_button = Button::with_label("Speichern");
                save_button.connect_clicked({
                    let settings_window = settings_window.clone();
                    let toolbar_pos_combo = toolbar_pos_combo.clone();
                    let bg_color_entry = bg_color_entry.clone();
                    let square_color_entry = square_color_entry.clone();

                    move |_| {
                        let mut new_config = config::Config::load();
                        new_config.ui.toolbar_position = match toolbar_pos_combo.active() {
                            Some(0) => "top".to_string(),
                            Some(1) => "bottom".to_string(),
                            _ => "top".to_string(),
                        };
                        new_config.colors.background = bg_color_entry.text().to_string();
                        new_config.colors.square = square_color_entry.text().to_string();

                        // Speichere in Datei
                        let config_dir = dirs::config_dir().unwrap().join("duckpx");
                        let config_path = config_dir.join("config.toml");
                        let toml_string = toml::to_string(&new_config).unwrap();
                        std::fs::write(config_path, toml_string).unwrap();

                        settings_window.close();
                    }
                });

                vbox.pack_start(&toolbar_pos_combo, false, false, 0);
                vbox.pack_start(&bg_color_label, false, false, 0);
                vbox.pack_start(&bg_color_entry, false, false, 0);
                vbox.pack_start(&square_color_label, false, false, 0);
                vbox.pack_start(&square_color_entry, false, false, 0);
                vbox.pack_start(&save_button, false, false, 0);

                settings_window.add(&vbox);
                settings_window.show_all();
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
                vbox.pack_start(&drawing_area, false, false, 0);
                vbox.pack_start(&update_label, false, false, 0);
            },
            "bottom" => {
                vbox.pack_start(&input_entry, false, false, 0);
                vbox.pack_start(&unit_combo, false, false, 0);
                vbox.pack_start(&calculate_button, false, false, 0);
                vbox.pack_start(&result_label, false, false, 0);
                vbox.pack_start(&drawing_area, false, false, 0);
                vbox.pack_start(&update_label, false, false, 0);
                vbox.pack_end(&toolbar, false, false, 0);
            },
            _ => (),
        }

        window.add(&vbox);
        window.show_all();
    });

    app.run();
}
