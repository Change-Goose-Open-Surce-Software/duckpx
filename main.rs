use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, ComboBoxText, Label, DrawingArea, Box as GtkBox, Orientation};
use std::rc::Rc;
use std::cell::RefCell;

mod config;
mod dpi;
mod translations;

fn main() {
    let app = Application::builder()
        .application_id("org.changegoose.duckpx")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let config = Rc::new(RefCell::new(config::Config::load()));
    let translations = Rc::new(translations::Translations::new());
    
    let current_lang = config.borrow().language.current.clone();
    let toolbar_position = config.borrow().ui.toolbar_position.clone();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&translations.get(&current_lang, "app_title"))
        .default_width(500)
        .default_height(600)
        .build();

    // Layout basierend auf Toolbar-Position
    let (main_box, toolbar_box, content_box) = match toolbar_position.as_str() {
        "left" | "right" => {
            let main = GtkBox::new(Orientation::Horizontal, 5);
            let toolbar = GtkBox::new(Orientation::Vertical, 5);
            let content = GtkBox::new(Orientation::Vertical, 5);
            (main, toolbar, content)
        },
        _ => {
            let main = GtkBox::new(Orientation::Vertical, 5);
            let toolbar = GtkBox::new(Orientation::Horizontal, 5);
            let content = GtkBox::new(Orientation::Vertical, 5);
            (main, toolbar, content)
        }
    };

    // Buttons erstellen
    let update_button = Button::with_label(&translations.get(&current_lang, "update"));
    let github_button = Button::with_label(&translations.get(&current_lang, "github"));
    let settings_button = Button::with_label(&translations.get(&current_lang, "settings"));
    let version_button = Button::with_label(&translations.get(&current_lang, "version"));
    let manual_button = Button::with_label(&translations.get(&current_lang, "manual"));

    // Unit-Dropdown
    let unit_combo = ComboBoxText::new();
    unit_combo.append_text(&translations.get(&current_lang, "pixel"));
    unit_combo.append_text(&translations.get(&current_lang, "millimeter"));
    unit_combo.append_text(&translations.get(&current_lang, "inch"));
    unit_combo.set_active(Some(0));

    // Input
    let input_entry = Entry::builder()
        .placeholder_text(&translations.get(&current_lang, "input_placeholder"))
        .build();

    // Calculate Button
    let calculate_button = Button::with_label(&translations.get(&current_lang, "calculate"));

    // Result Label - GRÖSSER
    let result_label = Label::new(Some(&translations.get(&current_lang, "result_default")));
    result_label.set_markup(&format!("<span size='x-large'>{}</span>", 
        translations.get(&current_lang, "result_default")));

    // Drawing Area
    let _drawing_area = DrawingArea::new();
    _drawing_area.set_size_request(300, 300);

    // Calculate Logic
    let calculate_fn = {
        let input_entry = input_entry.clone();
        let unit_combo = unit_combo.clone();
        let result_label = result_label.clone();
        let current_lang = current_lang.clone();
        let translations = translations.clone();
        move |_: &Button| {
            let input = input_entry.text().parse::<f64>();
            if let Ok(value) = input {
                let active_text = unit_combo.active_text();
                let pixel_text = translations.get(&current_lang, "pixel");
                let mm_text = translations.get(&current_lang, "millimeter");
                let inch_text = translations.get(&current_lang, "inch");
                
                let result = if let Some(text) = active_text {
                    let text_str = text.as_str();
                    if text_str == pixel_text {
                        dpi::px_to_mm(value)
                    } else if text_str == mm_text {
                        dpi::mm_to_px(value)
                    } else if text_str == inch_text {
                        dpi::px_to_inch(value)
                    } else {
                        value
                    }
                } else {
                    value
                };
                
                result_label.set_markup(&format!("<span size='x-large'>Ergebnis: {:.2}</span>", result));
            }
        }
    };

    calculate_button.connect_clicked(calculate_fn);

    // Layout zusammenfügen
    toolbar_box.add(&update_button);
    toolbar_box.add(&github_button);
    toolbar_box.add(&settings_button);
    toolbar_box.add(&version_button);
    toolbar_box.add(&manual_button);
    content_box.add(&input_entry);
    content_box.add(&unit_combo);
    content_box.add(&calculate_button);
    content_box.add(&result_label);

    main_box.add(&toolbar_box);
    main_box.add(&content_box);
    window.set_child(Some(&main_box));

    window.show();
}
