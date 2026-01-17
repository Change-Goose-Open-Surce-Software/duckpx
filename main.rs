use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, ComboBoxText, Label, DrawingArea, Box as GtkBox, Orientation, ScrolledWindow, TextView};
use webbrowser;
use std::process::Command;
use gtk::gdk::RGBA;
use std::rc::Rc;
use std::cell::RefCell;

mod config;
mod dpi;
mod i18n;

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
    let i18n = Rc::new(i18n::I18n::new());
    
    let lang = config.borrow().language.clone();
    let toolbar_position = config.borrow().ui.toolbar_position.clone();
    let square_color = config.borrow().colors.square.clone();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&i18n.t(&lang, "app_title"))
        .default_width(600)
        .default_height(600)
        .build();

    let main_box = match toolbar_position.as_str() {
        "left" | "right" => GtkBox::new(Orientation::Horizontal, 5),
        _ => GtkBox::new(Orientation::Vertical, 5),
    };
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);
    main_box.set_margin_top(10);
    main_box.set_margin_bottom(10);

    let toolbar = match toolbar_position.as_str() {
        "left" | "right" => GtkBox::new(Orientation::Vertical, 5),
        _ => GtkBox::new(Orientation::Horizontal, 5),
    };

    let update_button = Button::with_label(&i18n.t(&lang, "update"));
    let github_button = Button::with_label(&i18n.t(&lang, "github"));
    let settings_button = Button::with_label(&i18n.t(&lang, "settings"));
    let version_button = Button::with_label(&i18n.t(&lang, "version"));
    let manual_button = Button::with_label(&i18n.t(&lang, "manual"));
    let restart_button = Button::with_label(&i18n.t(&lang, "restart"));

    let content_box = GtkBox::new(Orientation::Vertical, 10);

    let input_entry = Entry::builder()
        .placeholder_text(&i18n.t(&lang, "input_placeholder"))
        .build();

    let unit_combo = ComboBoxText::new();
    unit_combo.append_text(&i18n.t(&lang, "pixel"));
    unit_combo.append_text(&i18n.t(&lang, "millimeter"));
    unit_combo.append_text(&i18n.t(&lang, "inch"));
    unit_combo.set_active(Some(0));

    let calculate_button = Button::with_label(&i18n.t(&lang, "calculate"));

    let result_label = Label::new(Some(&i18n.t(&lang, "result_placeholder")));
    result_label.set_markup(&format!("<span size='x-large'>{}</span>", &i18n.t(&lang, "result_placeholder")));

    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(300, 300);

    let perform_calculation = {
        let input_entry = input_entry.clone();
        let unit_combo = unit_combo.clone();
        let result_label = result_label.clone();
        let drawing_area = drawing_area.clone();
        let square_color = square_color.clone();
        let i18n = i18n.clone();
        let lang = lang.clone();

        Rc::new(move || {
            let input = input_entry.text().parse::<f64>().unwrap_or(0.0);
            if input <= 0.0 {
                return;
            }

            let binding = unit_combo.active_text().unwrap();
            let unit = binding.as_str();

            let (px, mm, inch) = match unit {
                s if s == i18n.t(&lang, "pixel") => (input, dpi::px_to_mm(input), dpi::px_to_inch(input)),
                s if s == i18n.t(&lang, "millimeter") => (dpi::mm_to_px(input), input, dpi::mm_to_inch(input)),
                s if s == i18n.t(&lang, "inch") => (dpi::inch_to_px(input), dpi::inch_to_mm(input), input),
                _ => (0.0, 0.0, 0.0),
            };

            result_label.set_markup(&format!(
                "<span size='xx-large' weight='bold'>{:.0}px = {:.2}mm = {:.2}in</span>",
                px, mm, inch
            ));

            let square_color_for_draw = square_color.clone();
            drawing_area.connect_draw(move |_, cr| {
                let square_rgba = RGBA::parse(&square_color_for_draw).unwrap();
                cr.set_source_rgba(
                    square_rgba.red() as f64,
                    square_rgba.green() as f64,
                    square_rgba.blue() as f64,
                    1.0,
                );
                cr.rectangle(10.0, 10.0, px.min(280.0), px.min(280.0));
                cr.fill().unwrap();
                Inhibit(false)
            });
            drawing_area.queue_draw();
        })
    };

    calculate_button.connect_clicked({
        let perform_calculation = perform_calculation.clone();
        move |_| {
            perform_calculation();
        }
    });

    input_entry.connect_activate({
        let perform_calculation = perform_calculation.clone();
        move |_| {
            perform_calculation();
        }
    });

    update_button.connect_clicked(move |button| {
        button.set_label("Update...");
        
        let home = dirs::home_dir().unwrap();
        let update_script = home.join(".local/share/duckpx/update.sh");
        let update_cmd = format!("bash -c 'bash {}; read -p \"Press Enter...\"'", update_script.display());
        
        let terminals = vec!["gnome-terminal", "konsole", "xfce4-terminal", "xterm", "terminator"];
        let mut success = false;
        
        for terminal in terminals {
            let result = Command::new(terminal)
                .arg("-e")
                .arg(&update_cmd)
                .spawn();
            
            if result.is_ok() {
                success = true;
                break;
            }
        }
        
        if success {
            button.set_label("Update OK");
        } else {
            button.set_label("Update Error");
        }
    });

    github_button.connect_clicked(move |_| {
        webbrowser::open("https://github.com/Change-Goose-Open-Surce-Software/duckpx").unwrap();
    });

    version_button.connect_clicked(move |_| {
        let home = dirs::home_dir().unwrap();
        let version_file = home.join(".local/share/duckpx/version.html");
        webbrowser::open(&format!("file://{}", version_file.display())).unwrap();
    });

    manual_button.connect_clicked({
        let window = window.clone();
        let config = config.clone();
        let i18n = i18n.clone();

        move |_| {
            show_manual_window(&window, &config, &i18n);
        }
    });

    restart_button.connect_clicked({
        let window = window.clone();
        
        move |_| {
            window.close();
            std::process::Command::new(std::env::current_exe().unwrap())
                .spawn()
                .unwrap();
            std::process::exit(0);
        }
    });

    settings_button.connect_clicked({
        let window = window.clone();
        let config = config.clone();
        let i18n = i18n.clone();

        move |_| {
            show_settings_window(&window, &config, &i18n);
        }
    });

    toolbar.pack_start(&update_button, false, false, 0);
    toolbar.pack_start(&github_button, false, false, 0);
    toolbar.pack_start(&version_button, false, false, 0);
    toolbar.pack_start(&manual_button, false, false, 0);
    toolbar.pack_start(&settings_button, false, false, 0);
    toolbar.pack_start(&restart_button, false, false, 0);

    content_box.pack_start(&input_entry, false, false, 0);
    content_box.pack_start(&unit_combo, false, false, 0);
    content_box.pack_start(&calculate_button, false, false, 0);
    content_box.pack_start(&result_label, false, false, 10);
    content_box.pack_start(&drawing_area, true, true, 0);

    match toolbar_position.as_str() {
        "top" => {
            main_box.pack_start(&toolbar, false, false, 0);
            main_box.pack_start(&content_box, true, true, 0);
        },
        "bottom" => {
            main_box.pack_start(&content_box, true, true, 0);
            main_box.pack_end(&toolbar, false, false, 0);
        },
        "left" => {
            main_box.pack_start(&toolbar, false, false, 0);
            main_box.pack_start(&content_box, true, true, 0);
        },
        "right" => {
            main_box.pack_start(&content_box, true, true, 0);
            main_box.pack_end(&toolbar, false, false, 0);
        },
        _ => {
            main_box.pack_start(&toolbar, false, false, 0);
            main_box.pack_start(&content_box, true, true, 0);
        }
    }

    window.add(&main_box);
    window.show_all();
}

fn show_settings_window(parent: &ApplicationWindow, config: &Rc<RefCell<config::Config>>, i18n: &Rc<i18n::I18n>) {
    let current_config = config.borrow().clone();
    let lang = current_config.language.clone();
    
    let settings_window = ApplicationWindow::builder()
        .title(&i18n.t(&lang, "settings_title"))
        .transient_for(parent)
        .default_width(400)
        .default_height(450)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);

    let lang_label = Label::new(Some(&i18n.t(&lang, "language")));
    lang_label.set_halign(gtk::Align::Start);
    let lang_combo = ComboBoxText::new();
    lang_combo.append(Some("de"), "Deutsch");
    lang_combo.append(Some("en"), "English");
    lang_combo.append(Some("fr"), "Francais");
    lang_combo.append(Some("ru"), "Russian");
    lang_combo.append(Some("zh"), "Chinese");
    lang_combo.set_active_id(Some(&current_config.language));

    let toolbar_label = Label::new(Some(&i18n.t(&lang, "toolbar_position")));
    toolbar_label.set_halign(gtk::Align::Start);
    let toolbar_pos_combo = ComboBoxText::new();
    toolbar_pos_combo.append(Some("top"), &i18n.t(&lang, "top"));
    toolbar_pos_combo.append(Some("bottom"), &i18n.t(&lang, "bottom"));
    toolbar_pos_combo.append(Some("left"), &i18n.t(&lang, "left"));
    toolbar_pos_combo.append(Some("right"), &i18n.t(&lang, "right"));
    toolbar_pos_combo.set_active_id(Some(&current_config.ui.toolbar_position));

    let manual_sidebar_label = Label::new(Some(&i18n.t(&lang, "manual_sidebar_position")));
    manual_sidebar_label.set_halign(gtk::Align::Start);
    let manual_sidebar_combo = ComboBoxText::new();
    manual_sidebar_combo.append(Some("top"), &i18n.t(&lang, "top"));
    manual_sidebar_combo.append(Some("bottom"), &i18n.t(&lang, "bottom"));
    manual_sidebar_combo.append(Some("left"), &i18n.t(&lang, "left"));
    manual_sidebar_combo.append(Some("right"), &i18n.t(&lang, "right"));
    manual_sidebar_combo.set_active_id(Some(&current_config.ui.manual_sidebar_position));

    let square_color_label = Label::new(Some(&i18n.t(&lang, "square_color")));
    square_color_label.set_halign(gtk::Align::Start);
    let square_color_combo = ComboBoxText::new();
    square_color_combo.append(Some("#FFA500"), "Orange");
    square_color_combo.append(Some("#FF0000"), "Red");
    square_color_combo.append(Some("#00FF00"), "Green");
    square_color_combo.append(Some("#0000FF"), "Blue");
    square_color_combo.append(Some("#FFFF00"), "Yellow");
    square_color_combo.append(Some("#FF00FF"), "Magenta");
    square_color_combo.append(Some("#00FFFF"), "Cyan");
    square_color_combo.append(Some("#800080"), "Purple");
    square_color_combo.append(Some("#FFC0CB"), "Pink");
    square_color_combo.append(Some("#A52A2A"), "Brown");
    square_color_combo.append(Some("#000000"), "Black");
    square_color_combo.append(Some("#808080"), "Gray");
    square_color_combo.set_active_id(Some(&current_config.colors.square));

    let save_button = Button::with_label(&i18n.t(&lang, "save"));
    save_button.connect_clicked({
        let settings_window = settings_window.clone();
        let config = config.clone();
        let lang_combo = lang_combo.clone();
        let toolbar_pos_combo = toolbar_pos_combo.clone();
        let manual_sidebar_combo = manual_sidebar_combo.clone();
        let square_color_combo = square_color_combo.clone();

        move |_| {
            let mut new_config = config.borrow().clone();
            
            if let Some(new_lang) = lang_combo.active_id() {
                new_config.language = new_lang.to_string();
            }
            
            if let Some(toolbar_pos) = toolbar_pos_combo.active_id() {
                new_config.ui.toolbar_position = toolbar_pos.to_string();
            }
            
            if let Some(manual_pos) = manual_sidebar_combo.active_id() {
                new_config.ui.manual_sidebar_position = manual_pos.to_string();
            }
            
            if let Some(square_hex) = square_color_combo.active_id() {
                new_config.colors.square = square_hex.to_string();
            }

            let config_dir = dirs::home_dir().unwrap().join(".local/share/duckpx");
            let config_path = config_dir.join("config.toml");
            let toml_string = toml::to_string(&new_config).unwrap();
            std::fs::write(config_path, toml_string).unwrap();
            
            *config.borrow_mut() = new_config;

            settings_window.close();
        }
    });

    vbox.pack_start(&lang_label, false, false, 0);
    vbox.pack_start(&lang_combo, false, false, 0);
    vbox.pack_start(&toolbar_label, false, false, 0);
    vbox.pack_start(&toolbar_pos_combo, false, false, 0);
    vbox.pack_start(&manual_sidebar_label, false, false, 0);
    vbox.pack_start(&manual_sidebar_combo, false, false, 0);
    vbox.pack_start(&square_color_label, false, false, 0);
    vbox.pack_start(&square_color_combo, false, false, 0);
    vbox.pack_start(&save_button, false, false, 0);

    settings_window.add(&vbox);
    settings_window.show_all();
}

fn show_manual_window(parent: &ApplicationWindow, config: &Rc<RefCell<config::Config>>, i18n: &Rc<i18n::I18n>) {
    let current_config = config.borrow().clone();
    let lang = current_config.language.clone();
    let sidebar_pos = current_config.ui.manual_sidebar_position.clone();
    
    let manual_window = ApplicationWindow::builder()
        .title(&i18n.t(&lang, "manual_title"))
        .transient_for(parent)
        .default_width(800)
        .default_height(600)
        .build();

    let main_box = match sidebar_pos.as_str() {
        "left" | "right" => GtkBox::new(Orientation::Horizontal, 0),
        _ => GtkBox::new(Orientation::Vertical, 0),
    };

    let sidebar = match sidebar_pos.as_str() {
        "left" | "right" => GtkBox::new(Orientation::Vertical, 5),
        _ => GtkBox::new(Orientation::Horizontal, 5),
    };
    sidebar.set_margin_start(5);
    sidebar.set_margin_end(5);
    sidebar.set_margin_top(5);
    sidebar.set_margin_bottom(5);

    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_margin_start(10);
    text_view.set_margin_end(10);
    text_view.set_margin_top(10);
    text_view.set_margin_bottom(10);
    
    let buffer = text_view.buffer().unwrap();
    
    let intro_button = Button::with_label(&i18n.t(&lang, "manual_intro"));
    let basic_button = Button::with_label(&i18n.t(&lang, "manual_basic"));
    let conversion_button = Button::with_label(&i18n.t(&lang, "manual_conversion"));
    let settings_button = Button::with_label(&i18n.t(&lang, "manual_settings"));
    let examples_button = Button::with_label(&i18n.t(&lang, "manual_examples"));

    intro_button.connect_clicked({
        let buffer = buffer.clone();
        let lang = lang.clone();
        move |_| {
            buffer.set_text(&get_manual_intro(&lang));
        }
    });

    basic_button.connect_clicked({
        let buffer = buffer.clone();
        let lang = lang.clone();
        move |_| {
            buffer.set_text(&get_manual_basic(&lang));
        }
    });

    conversion_button.connect_clicked({
        let buffer = buffer.clone();
        let lang = lang.clone();
        move |_| {
            buffer.set_text(&get_manual_conversion(&lang));
        }
    });

    settings_button.connect_clicked({
        let buffer = buffer.clone();
        let lang = lang.clone();
        move |_| {
            buffer.set_text(&get_manual_settings(&lang));
        }
    });

    examples_button.connect_clicked({
        let buffer = buffer.clone();
        let lang = lang.clone();
        move |_| {
            buffer.set_text(&get_manual_examples(&lang));
        }
    });

    buffer.set_text(&get_manual_intro(&lang));

    sidebar.pack_start(&intro_button, false, false, 0);
    sidebar.pack_start(&basic_button, false, false, 0);
    sidebar.pack_start(&conversion_button, false, false, 0);
    sidebar.pack_start(&settings_button, false, false, 0);
    sidebar.pack_start(&examples_button, false, false, 0);

    scrolled_window.add(&text_view);

    match sidebar_pos.as_str() {
        "top" => {
            main_box.pack_start(&sidebar, false, false, 0);
            main_box.pack_start(&scrolled_window, true, true, 0);
        },
        "bottom" => {
            main_box.pack_start(&scrolled_window, true, true, 0);
            main_box.pack_end(&sidebar, false, false, 0);
        },
        "left" => {
            main_box.pack_start(&sidebar, false, false, 0);
            main_box.pack_start(&scrolled_window, true, true, 0);
        },
        "right" => {
            main_box.pack_start(&scrolled_window, true, true, 0);
            main_box.pack_end(&sidebar, false, false, 0);
        },
        _ => {
            main_box.pack_start(&sidebar, false, false, 0);
            main_box.pack_start(&scrolled_window, true, true, 0);
        }
    }

    manual_window.add(&main_box);
    manual_window.show_all();
}

fn get_manual_intro(lang: &str) -> String {
    match lang {
        "de" => "EINFUEHRUNG\n\nDuckPx ist ein Tool zur Umrechnung von Pixeln, Millimetern und Inches.\n\nEs zeigt die umgerechneten Werte visuell an und hilft bei der Arbeit mit verschiedenen Masseinheiten.\n\nDas Programm basiert auf 96 DPI Standard.".to_string(),
        "en" => "INTRODUCTION\n\nDuckPx is a tool for converting pixels, millimeters, and inches.\n\nIt displays converted values visually and helps working with different units of measurement.\n\nThe program is based on the 96 DPI standard.".to_string(),
        "fr" => "INTRODUCTION\n\nDuckPx est un outil de conversion de pixels, millimetres et pouces.\n\nIl affiche les valeurs converties visuellement et aide a travailler avec differentes unites de mesure.\n\nLe programme est base sur la norme 96 DPI.".to_string(),
        "ru" => "VVEDENIE\n\nDuckPx - eto instrument dlya preobrazovaniya pikseley, millimetrov i dyuymov.\n\nOn vizualno otobrazhaet preobrazovannye znacheniya i pomogaet rabotat s razlichnymi edinitsami izmereniya.\n\nProgramma osnovana na standarte 96 DPI.".to_string(),
        "zh" => "JIESHAO\n\nDuckPx shi yige yongyu zhuanhuan xiangshu, haomi he yingcun de gongju.\n\nTa yikeshi xianshi zhuanhuan hou de zhi, bing bangzhu shiyong butong de celiang danwei.\n\nChengxu jiyu 96 DPI biaozhun.".to_string(),
        _ => "INTRODUCTION".to_string(),
    }
}

fn get_manual_basic(lang: &str) -> String {
    match lang {
        "de" => "GRUNDLAGEN\n\n1. Geben Sie einen Wert in das Eingabefeld ein\n2. Waehlen Sie die Einheit (Pixel, Millimeter oder Inch)\n3. Druecken Sie 'Berechnen' oder Enter\n4. Das Ergebnis wird angezeigt und visualisiert\n\nDas Programm zeigt ein farbiges Quadrat in der berechneten Groesse an.".to_string(),
        "en" => "BASICS\n\n1. Enter a value in the input field\n2. Select the unit (Pixel, Millimeter, or Inch)\n3. Press 'Calculate' or Enter\n4. The result will be displayed and visualized\n\nThe program shows a colored square in the calculated size.".to_string(),
        "fr" => "NOTIONS DE BASE\n\n1. Entrez une valeur dans le champ de saisie\n2. Selectionnez l'unite (Pixel, Millimetre ou Pouce)\n3. Appuyez sur 'Calculer' ou Entree\n4. Le resultat sera affiche et visualise\n\nLe programme affiche un carre colore dans la taille calculee.".to_string(),
        "ru" => "OSNOVY\n\n1. Vvedite znachenie v pole vvoda\n2. Vyberte edinitsu izmereniya (Piksel, Millimetr ili Dyuym)\n3. Nazhmite 'Vychislit' ili Enter\n4. Rezultat budet otobrazhen i vizualizirovan\n\nProgramma pokazyvaet tsvetnoj kvadrat v vychislennom razmere.".to_string(),
        "zh" => "JICHU\n\n1. Zai shurudazhong shuru zhi\n2. Xuanze danwei (Xiangshu, Haomi huo Yingcun)\n3. An 'Jisuan' huo Enter\n4. Jieguo jiang xianshi bing keshihua\n\nChengxu xianshi jisuan daxiao de caise fangkuai.".to_string(),
        _ => "BASICS".to_string(),
    }
}

fn get_manual_conversion(lang: &str) -> String {
    match lang {
        "de" => "UMRECHNUNG\n\nUmrechnungsformeln (96 DPI):\n\n1 Pixel = 0.264583333 mm\n1 Millimeter = 3.7795275591 px\n1 Inch = 96 px\n1 Inch = 25.4 mm\n\nDas Programm rechnet automatisch zwischen allen Einheiten um.\n\nDie Berechnung basiert auf dem 96 DPI Standard, der auf den meisten Bildschirmen verwendet wird.".to_string(),
        "en" => "CONVERSION\n\nConversion formulas (96 DPI):\n\n1 Pixel = 0.264583333 mm\n1 Millimeter = 3.7795275591 px\n1 Inch = 96 px\n1 Inch = 25.4 mm\n\nThe program automatically converts between all units.\n\nThe calculation is based on the 96 DPI standard used on most screens.".to_string(),
        "fr" => "CONVERSION\n\nFormules de conversion (96 DPI):\n\n1 Pixel = 0.264583333 mm\n1 Millimetre = 3.7795275591 px\n1 Pouce = 96 px\n1 Pouce = 25.4 mm\n\nLe programme convertit automatiquement entre toutes les unites.\n\nLe calcul est base sur la norme 96 DPI utilisee sur la plupart des ecrans.".to_string(),
        "ru" => "PREOBRAZOVANIE\n\nFormuly preobrazovaniya (96 DPI):\n\n1 Piksel = 0.264583333 mm\n1 Millimetr = 3.7795275591 px\n1 Dyuym = 96 px\n1 Dyuym = 25.4 mm\n\nProgramma avtomaticheski preobrazuet mezhdu vsemi edinitsami.\n\nVychislenie osnovano na standarte 96 DPI, ispolzuemom na bolshinstve ekranov.".to_string(),
        "zh" => "ZHUANHUAN\n\nZhuanhuan gongshi (96 DPI):\n\n1 Xiangshu = 0.264583333 haomi\n1 Haomi = 3.7795275591 xiangshu\n1 Yingcun = 96 xiangshu\n1 Yingcun = 25.4 haomi\n\nChengxu zidong zai suoyou danwei zhijian zhuanhuan.\n\nJisuan jiyu daduoshu pingmu shang shiyong de 96 DPI biaozhun.".to_string(),
        _ => "CONVERSION".to_string(),
    }
}

fn get_manual_settings(lang: &str) -> String {
    match lang {
        "de" => "EINSTELLUNGEN\n\nAnpassbare Optionen:\n\n- Sprache (Deutsch, Englisch, Franzoesisch, Russisch, Chinesisch)\n- Toolbar-Position (Oben, Unten, Links, Rechts)\n- Anleitungs-Sidebar-Position (Oben, Unten, Links, Rechts)\n- Quadratfarbe (verschiedene Farben verfuegbar)\n\nAenderungen werden sofort gespeichert und beim naechsten Start geladen.".to_string(),
        "en" => "SETTINGS\n\nCustomizable options:\n\n- Language (German, English, French, Russian, Chinese)\n- Toolbar Position (Top, Bottom, Left, Right)\n- Manual Sidebar Position (Top, Bottom, Left, Right)\n- Square Color (various colors available)\n\nChanges are saved immediately and loaded on next start.".to_string(),
        "fr" => "PARAMETRES\n\nOptions personnalisables:\n\n- Langue (Allemand, Anglais, Francais, Russe, Chinois)\n- Position de la barre d'outils (Haut, Bas, Gauche, Droite)\n- Position de la barre laterale du manuel (Haut, Bas, Gauche, Droite)\n- Couleur du carre (plusieurs couleurs disponibles)\n\nLes modifications sont enregistrees immediatement et chargees au prochain demarrage.".to_string(),
        "ru" => "NASTROYKI\n\nNastraivaemye parametry:\n\n- Yazyk (Nemetskiy, Angliyskiy, Frantsuzskiy, Russkiy, Kitayskiy)\n- Polozhenie paneli instrumentov (Sverkhu, Snizu, Sleva, Sprava)\n- Polozhenie bokovoy paneli rukovodstva (Sverkhu, Snizu, Sleva, Sprava)\n- Tsvet kvadrata (dostupny razlichnye tsveta)\n\nIzmenenia sohranyayutsya nemedlenno i zagruzhayutsya pri sleduyushchem zapuske.".to_string(),
        "zh" => "SHEZHI\n\nKezidingyi xuanxiang:\n\n- Yuyan (Deyu, Yingyu, Fayu, Eyu, Zhongwen)\n- Gongjulan weizhi (Dingbu, Dibu, Zuoce, Youce)\n- Shouce cebianguang weizhi (Dingbu, Dibu, Zuoce, Youce)\n- Fangkuai yanse (tigong duozhong yanse)\n\nGengga hui lijiqubaocun bing zai xia ci qidong shi jiazai.".to_string(),
        _ => "SETTINGS".to_string(),
    }
}

fn get_manual_examples(lang: &str) -> String {
    match lang {
        "de" => "BEISPIELE\n\nBeispiel 1: Pixel zu Millimeter\n- Eingabe: 100 px\n- Ergebnis: 100px = 26.46mm = 1.04in\n\nBeispiel 2: Millimeter zu Pixel\n- Eingabe: 50 mm\n- Ergebnis: 189px = 50.00mm = 1.97in\n\nBeispiel 3: Inch zu allen Einheiten\n- Eingabe: 2 in\n- Ergebnis: 192px = 50.80mm = 2.00in\n\nTipp: Sie koennen Enter druecken statt auf Berechnen zu klicken!".to_string(),
        "en" => "EXAMPLES\n\nExample 1: Pixels to Millimeters\n- Input: 100 px\n- Result: 100px = 26.46mm = 1.04in\n\nExample 2: Millimeters to Pixels\n- Input: 50 mm\n- Result: 189px = 50.00mm = 1.97in\n\nExample 3: Inches to all units\n- Input: 2 in\n- Result: 192px = 50.80mm = 2.00in\n\nTip: You can press Enter instead of clicking Calculate!".to_string(),
        "fr" => "EXEMPLES\n\nExemple 1: Pixels vers Millimetres\n- Entree: 100 px\n- Resultat: 100px = 26.46mm = 1.04in\n\nExemple 2: Millimetres vers Pixels\n- Entree: 50 mm\n- Resultat: 189px = 50.00mm = 1.97in\n\nExemple 3: Pouces vers toutes les unites\n- Entree: 2 in\n- Resultat: 192px = 50.80mm = 2.00in\n\nAstuce: Vous pouvez appuyer sur Entree au lieu de cliquer sur Calculer!".to_string(),
        "ru" => "PRIMERY\n\nPrimer 1: Pikseli v Millimetry\n- Vvod: 100 px\n- Rezultat: 100px = 26.46mm = 1.04in\n\nPrimer 2: Millimetry v Pikseli\n- Vvod: 50 mm\n- Rezultat: 189px = 50.00mm = 1.97in\n\nPrimer 3: Dyuymy vo vse edinitsy\n- Vvod: 2 in\n- Rezultat: 192px = 50.80mm = 2.00in\n\nSovet: Vy mozhete nazhat Enter vmesto togo chtoby nazhimat Vychislit!".to_string(),
        "zh" => "SHILI\n\nShili 1: Xiangshu dao Haomi\n- Shuru: 100 px\n- Jieguo: 100px = 26.46mm = 1.04in\n\nShili 2: Haomi dao Xiangshu\n- Shuru: 50 mm\n- Jieguo: 189px = 50.00mm = 1.97in\n\nShili 3: Yingcun dao suoyou danwei\n- Shuru: 2 in\n- Jieguo: 192px = 50.80mm = 2.00in\n\nTishi: Nin keyi an Enter erbuhi dianji Jisuan!".to_string(),
        _ => "EXAMPLES".to_string(),
    }
}
