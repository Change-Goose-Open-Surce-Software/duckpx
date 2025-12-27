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
        
        // Klone die Werte, die wir später brauchen
        let toolbar_position = config.ui.toolbar_position.clone();
        let square_color = config.colors.square.clone();

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

        // Update-Knopf
        let update_button = Button::with_label("Update");

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
            let square_color = square_color.clone();

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
                let square_color_for_draw = square_color.clone();
                drawing_area.connect_draw(move |_, cr| {
                    let square_rgba = RGBA::parse(&square_color_for_draw).unwrap();
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
                drawing_area.queue_draw();
            }
        });

        // Logik für Update-Knopf
        update_button.connect_clicked(move |button| {
            button.set_label("Update läuft...");

            let output = Command::new("sh")
                .arg("-c")
                .arg("/usr/local/share/duckpx/update.sh")
                .output()
                .expect("Update fehlgeschlagen!");

            if output.status.success() {
                button.set_label("✅ Erfolgreich!");
            } else {
                button.set_label("❌ Fehler!");
            }

            // Nach 3 Sekunden zurücksetzen
            let button_clone = button.clone();
            glib::timeout_add_seconds_local(3, move || {
                button_clone.set_label("Update");
                glib::Continue(false)
            });
        });

        // Logik für GitHub-Knopf
        github_button.connect_clicked(move |_| {
            webbrowser::open("https://github.com/Change-Goose-Open-Surce-Software/duckpx")
                .unwrap();
        });

        // Logik für Einstellungen-Knopf
        settings_button.connect_clicked({
            let window = window.clone();

            move |_| {
                let current_config = config::Config::load();
                
                let settings_window = ApplicationWindow::builder()
                    .title("DuckPx Einstellungen")
                    .transient_for(&window)
                    .default_width(350)
                    .default_height(300)
                    .build();

                let vbox = GtkBox::new(Orientation::Vertical, 10);
                vbox.set_margin_start(10);
                vbox.set_margin_end(10);
                vbox.set_margin_top(10);
                vbox.set_margin_bottom(10);

                // Dropdown für Toolbar-Position
                let toolbar_label = Label::new(Some("Toolbar-Position:"));
                toolbar_label.set_halign(gtk::Align::Start);
                let toolbar_pos_combo = ComboBoxText::new();
                toolbar_pos_combo.append_text("Oben");
                toolbar_pos_combo.append_text("Unten");
                toolbar_pos_combo.set_active(match current_config.ui.toolbar_position.as_str() {
                    "top" => Some(0),
                    "bottom" => Some(1),
                    _ => Some(0),
                });

                // Farbauswahl für Hintergrund
                let bg_color_label = Label::new(Some("Hintergrundfarbe:"));
                bg_color_label.set_halign(gtk::Align::Start);
                let bg_color_combo = ComboBoxText::new();
                bg_color_combo.append(Some("#FFFFFF"), "Weiß");
                bg_color_combo.append(Some("#000000"), "Schwarz");
                bg_color_combo.append(Some("#F0F0F0"), "Hellgrau");
                bg_color_combo.append(Some("#333333"), "Dunkelgrau");
                bg_color_combo.append(Some("#E8F4F8"), "Hellblau");
                bg_color_combo.set_active_id(Some(&current_config.colors.background));

                // Farbauswahl für Quadrat
                let square_color_label = Label::new(Some("Quadratfarbe:"));
                square_color_label.set_halign(gtk::Align::Start);
                let square_color_combo = ComboBoxText::new();
                square_color_combo.append(Some("#FFA500"), "Orange");
                square_color_combo.append(Some("#FF0000"), "Rot");
                square_color_combo.append(Some("#00FF00"), "Grün");
                square_color_combo.append(Some("#0000FF"), "Blau");
                square_color_combo.append(Some("#FFFF00"), "Gelb");
                square_color_combo.append(Some("#FF00FF"), "Magenta");
                square_color_combo.append(Some("#00FFFF"), "Cyan");
                square_color_combo.append(Some("#800080"), "Lila");
                square_color_combo.append(Some("#FFC0CB"), "Rosa");
                square_color_combo.append(Some("#A52A2A"), "Braun");
                square_color_combo.append(Some("#000000"), "Schwarz");
                square_color_combo.append(Some("#808080"), "Grau");
                square_color_combo.set_active_id(Some(&current_config.colors.square));

                // Speichern-Knopf
                let save_button = Button::with_label("Speichern");
                save_button.connect_clicked({
                    let settings_window = settings_window.clone();
                    let toolbar_pos_combo = toolbar_pos_combo.clone();
                    let bg_color_combo = bg_color_combo.clone();
                    let square_color_combo = square_color_combo.clone();

                    move |_| {
                        let mut new_config = config::Config::load();
                        new_config.ui.toolbar_position = match toolbar_pos_combo.active() {
                            Some(0) => "top".to_string(),
                            Some(1) => "bottom".to_string(),
                            _ => "top".to_string(),
                        };
                        
                        // Hole die ausgewählten Hex-Werte
                        if let Some(bg_hex) = bg_color_combo.active_id() {
                            new_config.colors.background = bg_hex.to_string();
                        }
                        if let Some(square_hex) = square_color_combo.active_id() {
                            new_config.colors.square = square_hex.to_string();
                        }

                        // Speichere in Datei
                        let config_dir = dirs::config_dir().unwrap().join("duckpx");
                        let config_path = config_dir.join("config.toml");
                        let toml_string = toml::to_string(&new_config).unwrap();
                        std::fs::write(config_path, toml_string).unwrap();

                        settings_window.close();
                    }
                });

                vbox.pack_start(&toolbar_label, false, false, 0);
                vbox.pack_start(&toolbar_pos_combo, false, false, 0);
                vbox.pack_start(&bg_color_label, false, false, 0);
                vbox.pack_start(&bg_color_combo, false, false, 0);
                vbox.pack_start(&square_color_label, false, false, 0);
                vbox.pack_start(&square_color_combo, false, false, 0);
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
        match toolbar_position.as_str() {
            "top" => {
                vbox.pack_start(&toolbar, false, false, 0);
                vbox.pack_start(&input_entry, false, false, 0);
                vbox.pack_start(&unit_combo, false, false, 0);
                vbox.pack_start(&calculate_button, false, false, 0);
                vbox.pack_start(&result_label, false, false, 0);
                vbox.pack_start(&drawing_area, true, true, 0);
            },
            "bottom" => {
                vbox.pack_start(&input_entry, false, false, 0);
                vbox.pack_start(&unit_combo, false, false, 0);
                vbox.pack_start(&calculate_button, false, false, 0);
                vbox.pack_start(&result_label, false, false, 0);
                vbox.pack_start(&drawing_area, true, true, 0);
                vbox.pack_end(&toolbar, false, false, 0);
            },
            _ => (),
        }

        window.add(&vbox);
        window.show_all();
    });

    app.run();
}
