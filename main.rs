use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Label, DrawingArea};
use std::fs;
use std::path::Path;
use std::process;
mod config;
mod dpi;

fn main() {
    // GTK initialisieren
    let app = Application::builder()
        .application_id("org.changegoose.duckpx")
        .build();
    app.connect_activate(|app| {
        // Fenster erstellen
        let window = ApplicationWindow::builder()
            .application(app)
            .title("DuckPx – Pixel & Millimeter")
            .default_width(400)
            .default_height(300)
            .build();

        // UI-Elemente
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        window.add(&vbox);

        // Eingabefeld
        let entry = Entry::builder()
            .placeholder_text("Größe (z. B. 5px oder 5mm)")
            .build();
        vbox.pack_start(&entry, false, false, 5);

        // Button
        let button = Button::with_label("Zeige Größe");
        vbox.pack_start(&button, false, false, 5);

        // Zeichnungsbereich
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(300, 150);
        vbox.pack_start(&drawing_area, true, true, 5);

        // Label für Ergebnis
        let result_label = Label::new(None);
        vbox.pack_start(&result_label, false, false, 5);

        // Konfiguration laden
        let config = config::load_config();
        let bg_color = gdk::Color::parse(&config.background_color).unwrap();
        let fg_color = gdk::Color::parse(&config.foreground_color).unwrap();

        // Button-Logik
        button.connect_clicked({
            let entry = entry.clone();
            let drawing_area = drawing_area.clone();
            let result_label = result_label.clone();
            move |_| {
                let input = entry.text().to_string();
                if let Some((value, is_mm)) = parse_input(&input) {
                    let pixels = if is_mm {
                        dpi::mm_to_px(value)
                    } else {
                        value
                    };
                    let mm = dpi::px_to_mm(pixels);
                    drawing_area.queue_draw();
                    drawing_area.connect_draw({
                        let fg_color = fg_color.clone();
                        move |_, cr| {
                            cr.set_source_rgb(fg_color.red() as f64, fg_color.green() as f64, fg_color.blue() as f64);
                            cr.rectangle(10.0, 10.0, pixels as f64, pixels as f64);
                            cr.fill().unwrap();
                            Inhibit(false)
                        }
                    });
                    result_label.set_text(&format!("{}px = {:.2}mm", pixels, mm));
                } else {
                    result_label.set_text("Ungültige Eingabe! Beispiel: 5px oder 5mm");
                }
            }
        });

        window.show_all();
    });
    app.run();
}

// Eingabe parsen (z. B. "5px" oder "5mm")
fn parse_input(input: &str) -> Option<(f64, bool)> {
    if input.ends_with("px") {
        input.trim_end_matches("px").parse::<f64>().ok().map(|v| (v, false))
    } else if input.ends_with("mm") {
        input.trim_end_matches("mm").parse::<f64>().ok().map(|v| (v, true))
    } else {
        None
    }
}
