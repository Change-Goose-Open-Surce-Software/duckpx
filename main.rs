use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, ComboBoxText, Label, DrawingArea, Box as GtkBox, Orientation, ScrolledWindow};
use webbrowser;
use std::process::Command;
use gtk::gdk::RGBA;
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
    let square_color = config.borrow().colors.square.clone();

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
    let restart_button = Button::with_label(&translations.get(&current_lang, "restart"));

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
    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(300, 300);

    // Calculate Logic
    let calculate_fn = {
        let input_entry = input_entry.clone();
        let unit_combo = unit_combo.clone();
        let result_label = result_label.clone();
        let drawing_area = drawing_area.clone();
        let square_color = square_color.clone();

        Rc::new(move || {
            let input = input_entry.text().parse::<f64>().unwrap_or(0.0);
            let binding = unit_combo.active_text().unwrap_or_else(|| "Pixel (px)".into());
            let unit = binding.as_str();

            let (px, mm, inch) = if unit.contains("px") || unit.contains("像素") || unit.contains("Пиксель") {
                (input, dpi::px_to_mm(input), dpi::px_to_inch(input))
            } else if unit.contains("mm") || unit.contains("毫米") || unit.contains("Миллиметр") {
                (dpi::mm_to_px(input), input, dpi::mm_to_inch(input))
            } else {
                (dpi::inch_to_px(input), dpi::inch_to_mm(input), input)
            };

            // GRÖSSERE ANZEIGE
            result_label.set_markup(&format!(
                "<span size='xx-large' weight='bold'>{:.0}px = {:.2}mm = {:.2}in</span>", 
                px, mm, inch
            ));

            // Drawing
            let square_color_for_draw = square_color.clone();
            drawing_area.connect_draw(move |_, cr| {
                if let Ok(square_rgba) = RGBA::parse(&square_color_for_draw) {
                    cr.set_source_rgba(
                        square_rgba.red() as f64,
                        square_rgba.green() as f64,
                        square_rgba.blue() as f64,
                        1.0,
                    );
                    let size = px.min(280.0);
                    cr.rectangle(10.0, 10.0, size, size);
                    let _ = cr.fill();
                }
                Inhibit(false)
            });
            drawing_area.queue_draw();
        })
    };

    // Calculate Button Click
    calculate_button.connect_clicked({
        let calculate_fn = calculate_fn.clone();
        move |_| {
            calculate_fn();
        }
    });

    // Enter-Taste für Berechnung
    input_entry.connect_activate({
        let calculate_fn = calculate_fn.clone();
        move |_| {
            calculate_fn();
        }
    });

    // Update Button - öffnet Terminal
    update_button.connect_clicked(move |_| {
        // Versuche verschiedene Terminal-Emulatoren
        let terminals = vec!["gnome-terminal", "konsole", "xfce4-terminal", "xterm", "terminator"];
        
        for terminal in terminals {
            let result = Command::new(terminal)
                .arg("--")
                .arg("bash")
                .arg("-c")
                .arg("sudo /usr/local/share/duckpx/update.sh; read -p 'Drücke Enter zum Beenden...'")
                .spawn();
            
            if result.is_ok() {
                break;
            }
        }
    });

    // GitHub Button
    github_button.connect_clicked(move |_| {
        let _ = webbrowser::open("https://github.com/Change-Goose-Open-Surce-Software/duckpx");
    });

    // Version Button
    version_button.connect_clicked(move |_| {
        let _ = webbrowser::open("file:///usr/local/share/duckpx/version.html");
    });

    // Manual Button
    manual_button.connect_clicked({
        let window = window.clone();
        let config = config.clone();
        let translations = translations.clone();
        
        move |_| {
            show_manual_window(&window, &config, &translations);
        }
    });

    // Restart Button
    restart_button.connect_clicked({
        let window = window.clone();
        move |_| {
            window.close();
            let app = Application::builder()
                .application_id("org.changegoose.duckpx")
                .build();
            app.connect_activate(|app| {
                build_ui(app);
            });
            app.run();
        }
    });

    // Settings Button
    settings_button.connect_clicked({
        let window = window.clone();
        let config = config.clone();
        let translations = translations.clone();
        
        move |_| {
            show_settings_window(&window, &config, &translations);
        }
    });

    // Toolbar zusammenbauen
    toolbar_box.pack_start(&update_button, false, false, 0);
    toolbar_box.pack_start(&github_button, false, false, 0);
    toolbar_box.pack_start(&version_button, false, false, 0);
    toolbar_box.pack_start(&manual_button, false, false, 0);
    toolbar_box.pack_start(&settings_button, false, false, 0);
    toolbar_box.pack_start(&restart_button, false, false, 0);

    // Content zusammenbauen
    content_box.pack_start(&input_entry, false, false, 0);
    content_box.pack_start(&unit_combo, false, false, 0);
    content_box.pack_start(&calculate_button, false, false, 0);
    content_box.pack_start(&result_label, false, false, 10);
    content_box.pack_start(&drawing_area, true, true, 0);

    // Layout nach Position
    match toolbar_position.as_str() {
        "top" => {
            main_box.pack_start(&toolbar_box, false, false, 0);
            main_box.pack_start(&content_box, true, true, 0);
        },
        "bottom" => {
            main_box.pack_start(&content_box, true, true, 0);
            main_box.pack_end(&toolbar_box, false, false, 0);
        },
        "left" => {
            main_box.pack_start(&toolbar_box, false, false, 0);
            main_box.pack_start(&content_box, true, true, 0);
        },
        "right" => {
            main_box.pack_start(&content_box, true, true, 0);
            main_box.pack_end(&toolbar_box, false, false, 0);
        },
        _ => (),
    }

    window.add(&main_box);
    window.show_all();
}

fn show_settings_window(
    parent: &ApplicationWindow,
    config: &Rc<RefCell<config::Config>>,
    translations: &Rc<translations::Translations>
) {
    let current_config = config.borrow().clone();
    let current_lang = current_config.language.current.clone();
    
    let settings_window = ApplicationWindow::builder()
        .title(&translations.get(&current_lang, "settings_title"))
        .transient_for(parent)
        .default_width(400)
        .default_height(450)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);

    // Toolbar Position
    let toolbar_label = Label::new(Some(&translations.get(&current_lang, "toolbar_position")));
    toolbar_label.set_halign(gtk::Align::Start);
    let toolbar_pos_combo = ComboBoxText::new();
    toolbar_pos_combo.append_text(&translations.get(&current_lang, "top"));
    toolbar_pos_combo.append_text(&translations.get(&current_lang, "bottom"));
    toolbar_pos_combo.append_text(&translations.get(&current_lang, "left"));
    toolbar_pos_combo.append_text(&translations.get(&current_lang, "right"));
    toolbar_pos_combo.set_active(match current_config.ui.toolbar_position.as_str() {
        "top" => Some(0),
        "bottom" => Some(1),
        "left" => Some(2),
        "right" => Some(3),
        _ => Some(0),
    });

    // Manual Position
    let manual_label = Label::new(Some(&translations.get(&current_lang, "manual_position")));
    manual_label.set_halign(gtk::Align::Start);
    let manual_pos_combo = ComboBoxText::new();
    manual_pos_combo.append_text(&translations.get(&current_lang, "top"));
    manual_pos_combo.append_text(&translations.get(&current_lang, "bottom"));
    manual_pos_combo.append_text(&translations.get(&current_lang, "left"));
    manual_pos_combo.append_text(&translations.get(&current_lang, "right"));
    manual_pos_combo.set_active(match current_config.ui.manual_sidebar_position.as_str() {
        "top" => Some(0),
        "bottom" => Some(1),
        "left" => Some(2),
        "right" => Some(3),
        _ => Some(2),
    });

    // Square Color
    let square_color_label = Label::new(Some(&translations.get(&current_lang, "square_color")));
    square_color_label.set_halign(gtk::Align::Start);
    let square_color_combo = ComboBoxText::new();
    square_color_combo.append(Some("#FFA500"), "Orange");
    square_color_combo.append(Some("#FF0000"), "Rot / Red / Rouge / Красный / 红色");
    square_color_combo.append(Some("#00FF00"), "Grün / Green / Vert / Зеленый / 绿色");
    square_color_combo.append(Some("#0000FF"), "Blau / Blue / Bleu / Синий / 蓝色");
    square_color_combo.append(Some("#FFFF00"), "Gelb / Yellow / Jaune / Желтый / 黄色");
    square_color_combo.append(Some("#FF00FF"), "Magenta");
    square_color_combo.append(Some("#00FFFF"), "Cyan");
    square_color_combo.append(Some("#800080"), "Lila / Purple / Violet / Фиолетовый / 紫色");
    square_color_combo.append(Some("#FFC0CB"), "Rosa / Pink / Rose / Розовый / 粉色");
    square_color_combo.append(Some("#A52A2A"), "Braun / Brown / Brun / Коричневый / 棕色");
    square_color_combo.append(Some("#000000"), "Schwarz / Black / Noir / Черный / 黑色");
    square_color_combo.append(Some("#808080"), "Grau / Gray / Gris / Серый / 灰色");
    square_color_combo.set_active_id(Some(&current_config.colors.square));

    // Language
    let lang_label = Label::new(Some(&translations.get(&current_lang, "language")));
    lang_label.set_halign(gtk::Align::Start);
    let lang_combo = ComboBoxText::new();
    lang_combo.append(Some("de"), "Deutsch");
    lang_combo.append(Some("en"), "English");
    lang_combo.append(Some("fr"), "Français");
    lang_combo.append(Some("ru"), "Русский");
    lang_combo.append(Some("zh"), "中文");
    lang_combo.set_active_id(Some(&current_config.language.current));

    // Save Button
    let save_button = Button::with_label(&translations.get(&current_lang, "save"));
    save_button.connect_clicked({
        let settings_window = settings_window.clone();
        let config = config.clone();
        let toolbar_pos_combo = toolbar_pos_combo.clone();
        let manual_pos_combo = manual_pos_combo.clone();
        let square_color_combo = square_color_combo.clone();
        let lang_combo = lang_combo.clone();
        let parent = parent.clone();

        move |_| {
            let mut new_config = config.borrow().clone();
            
            new_config.ui.toolbar_position = match toolbar_pos_combo.active() {
                Some(0) => "top".to_string(),
                Some(1) => "bottom".to_string(),
                Some(2) => "left".to_string(),
                Some(3) => "right".to_string(),
                _ => "top".to_string(),
            };

            new_config.ui.manual_sidebar_position = match manual_pos_combo.active() {
                Some(0) => "top".to_string(),
                Some(1) => "bottom".to_string(),
                Some(2) => "left".to_string(),
                Some(3) => "right".to_string(),
                _ => "left".to_string(),
            };

            if let Some(square_hex) = square_color_combo.active_id() {
                new_config.colors.square = square_hex.to_string();
            }

            if let Some(lang_code) = lang_combo.active_id() {
                new_config.language.current = lang_code.to_string();
            }

            new_config.save();
            *config.borrow_mut() = new_config;

            settings_window.close();
            parent.close();
            
            // Neustart
            let app = Application::builder()
                .application_id("org.changegoose.duckpx")
                .build();
            app.connect_activate(|app| {
                build_ui(app);
            });
            app.run();
        }
    });

    vbox.pack_start(&toolbar_label, false, false, 0);
    vbox.pack_start(&toolbar_pos_combo, false, false, 0);
    vbox.pack_start(&manual_label, false, false, 0);
    vbox.pack_start(&manual_pos_combo, false, false, 0);
    vbox.pack_start(&square_color_label, false, false, 0);
    vbox.pack_start(&square_color_combo, false, false, 0);
    vbox.pack_start(&lang_label, false, false, 0);
    vbox.pack_start(&lang_combo, false, false, 0);
    vbox.pack_start(&save_button, false, false, 0);

    settings_window.add(&vbox);
    settings_window.show_all();
}

fn show_manual_window(
    parent: &ApplicationWindow,
    config: &Rc<RefCell<config::Config>>,
    translations: &Rc<translations::Translations>
) {
    let current_config = config.borrow().clone();
    let current_lang = current_config.language.current.clone();
    let sidebar_pos = current_config.ui.manual_sidebar_position.clone();
    
    let manual_window = ApplicationWindow::builder()
        .title(&translations.get(&current_lang, "manual_title"))
        .transient_for(parent)
        .default_width(800)
        .default_height(600)
        .build();

    let main_box = match sidebar_pos.as_str() {
        "left" | "right" => GtkBox::new(Orientation::Horizontal, 0),
        _ => GtkBox::new(Orientation::Vertical, 0),
    };

    // Sidebar mit Kategorien
    let sidebar = GtkBox::new(Orientation::Vertical, 5);
    sidebar.set_margin_start(5);
    sidebar.set_margin_end(5);
    sidebar.set_margin_top(5);
    sidebar.set_margin_bottom(5);

    let overview_btn = Button::with_label(&translations.get(&current_lang, "manual_overview"));
    let usage_btn = Button::with_label(&translations.get(&current_lang, "manual_usage"));
    let settings_btn = Button::with_label(&translations.get(&current_lang, "manual_settings"));
    let examples_btn = Button::with_label(&translations.get(&current_lang, "manual_examples"));
    let troubleshooting_btn = Button::with_label(&translations.get(&current_lang, "manual_troubleshooting"));

    sidebar.pack_start(&overview_btn, false, false, 0);
    sidebar.pack_start(&usage_btn, false, false, 0);
    sidebar.pack_start(&settings_btn, false, false, 0);
    sidebar.pack_start(&examples_btn, false, false, 0);
    sidebar.pack_start(&troubleshooting_btn, false, false, 0);

    // Content Area - KORRIGIERTE ZEILE
    let scroll = ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
    let content_label = Label::new(None);
    content_label.set_line_wrap(true);
    content_label.set_margin_start(20);
    content_label.set_margin_end(20);
    content_label.set_margin_top(20);
    content_label.set_margin_bottom(20);
    content_label.set_halign(gtk::Align::Start);
    content_label.set_valign(gtk::Align::Start);

    scroll.add(&content_label);

    // Manual Content Generator
    let get_manual_content = move |section: &str| -> String {
        match current_lang.as_str() {
            "de" => match section {
                "overview" => "=== UEBERSICHT ===\n\nDuckPx ist ein Tool zur Umrechnung zwischen Pixeln, Millimetern und Inches.\n\nEs bietet eine visuelle Darstellung der eingegebenen Groesse und unterstuetzt mehrere Sprachen.".to_string(),
                "usage" => "=== VERWENDUNG ===\n\n1. Geben Sie eine Zahl in das Eingabefeld ein\n2. Waehlen Sie die Einheit (Pixel, Millimeter oder Inch)\n3. Druecken Sie 'Berechnen' oder Enter\n4. Das Ergebnis wird angezeigt und ein Quadrat in der entsprechenden Groesse gezeichnet".to_string(),
                "settings" => "=== EINSTELLUNGEN ===\n\n- Toolbar-Position: Oben, Unten, Links oder Rechts\n- Anleitungs-Position: Position dieser Anleitung\n- Quadratfarbe: Farbe des angezeigten Quadrats\n- Sprache: Deutsch, English, Francais, Russki, Zhongwen".to_string(),
                "examples" => "=== BEISPIELE ===\n\n50 Pixel = 13.23 mm = 0.52 inch\n100 mm = 377.95 px = 3.94 inch\n2 inch = 190.08 px = 50.80 mm".to_string(),
                "troubleshooting" => "=== FEHLERBEHEBUNG ===\n\nProblem: Das Programm startet nicht\nLoesung: Fuehren Sie 'duckpx' im Terminal aus\n\nProblem: Update funktioniert nicht\nLoesung: Fuehren Sie manuell aus:\nsudo /usr/local/share/duckpx/update.sh".to_string(),
                _ => "".to_string(),
            },
            "fr" => match section {
                "overview" => "=== APERCU ===\n\nDuckPx est un outil de conversion entre pixels, millimetres et pouces.\n\nIl offre une representation visuelle de la taille saisie et prend en charge plusieurs langues.".to_string(),
                "usage" => "=== UTILISATION ===\n\n1. Entrez un nombre dans le champ de saisie\n2. Selectionnez l'unite (Pixel, Millimetre ou Pouce)\n3. Appuyez sur 'Calculer' ou Entree\n4. Le resultat s'affiche et un carre de la taille correspondante est dessine".to_string(),
                "settings" => "=== PARAMETRES ===\n\n- Position de la barre: Haut, Bas, Gauche ou Droite\n- Position du manuel: Position de ce manuel\n- Couleur du carre: Couleur du carre affiche\n- Langue: Deutsch, English, Francais, Russki, Zhongwen".to_string(),
                "examples" => "=== EXEMPLES ===\n\n50 Pixel = 13.23 mm = 0.52 pouce\n100 mm = 377.95 px = 3.94 pouces\n2 pouces = 190.08 px = 50.80 mm".to_string(),
                "troubleshooting" => "=== DEPANNAGE ===\n\nProbleme: Le programme ne demarre pas\nSolution: Executez 'duckpx' dans le terminal\n\nProbleme: La mise a jour ne fonctionne pas\nSolution: Executez manuellement:\nsudo /usr/local/share/duckpx/update.sh".to_string(),
                _ => "".to_string(),
            },
            "ru" => match section {
                "overview" => "=== OBZOR ===\n\nDuckPx - eto instrument dlya preobrazovaniya mezhdu pikselyami, millimetrami i dyuymami.\n\nOn predostavlyaet vizualnoe predstavlenie vvedennogo razmera i podderzhivaet neskolko yazykov.".to_string(),
                "usage" => "=== ISPOLZOVANIE ===\n\n1. Vvedite chislo v pole vvoda\n2. Vyberte edinitsu izmereniya (Piksel, Millimetr ili Dyuym)\n3. Nazhmite 'Vychislit' ili Enter\n4. Rezultat otobrazhaetsya, i risuetsya kvadrat sootvetstvuyushchego razmera".to_string(),
                "settings" => "=== NASTROYKI ===\n\n- Pozitsiya paneli: Sverkhu, Snizu, Sleva ili Sprava\n- Pozitsiya rukovodstva: Pozitsiya etogo rukovodstva\n- Tsvet kvadrata: Tsvet otobrazhaemogo kvadrata\n- Yazyk: Deutsch, English, Francais, Russki, Zhongwen".to_string(),
                "examples" => "=== PRIMERY ===\n\n50 Piksel = 13.23 mm = 0.52 dyuyma\n100 mm = 377.95 px = 3.94 dyuyma\n2 dyuyma = 190.08 px = 50.80 mm".to_string(),
                "troubleshooting" => "=== USTRANENIE NEPOLADOK ===\n\nProblema: Programma ne zapuskaetsya\nReshenie: Vypolnite 'duckpx' v terminale\n\nProblema: Obnovlenie ne rabotaet\nReshenie: Vypolnite vruchnuyu:\nsudo /usr/local/share/duckpx/update.sh".to_string(),
                _ => "".to_string(),
            },
            "zh" => match section {
                "overview" => "=== GAISHU ===\n\nDuckPx shi yige zai xiangsu, haomi he yingcun zhijian zhuanhuan de gongju.\n\nTa tigong shuru daxiao de keshihua biaoshi, bing zhichi duo zhong yuyan.".to_string(),
                "usage" => "=== SHIYONG FANGFA ===\n\n1. Zai shurukuang zhong shuru shuzi\n2. Xuanze danwei (xiangsu, haomi huo yingcun)\n3. An 'Jisuan' huo huiche\n4. Xianshi jieguo bing huizhi xiangying daxiao de zhengfangxing".to_string(),
                "settings" => "=== SHEZHI ===\n\n- Gongjulan weizhi: Dingbu, Dibu, Zuoce huo Youce\n- Shouce weizhi: Ci shouce de weizhi\n- Fangxing yanse: Xianshi fangxing de yanse\n- Yuyan: Deutsch, English, Francais, Russki, Zhongwen".to_string(),
                "examples" => "=== SHILI ===\n\n50 xiangsu = 13.23 haomi = 0.52 yingcun\n100 haomi = 377.95 xiangsu = 3.94 yingcun\n2 yingcun = 190.08 xiangsu = 50.80 haomi".to_string(),
                "troubleshooting" => "=== GUZHANG PAICHU ===\n\nWenti: Chengxu wufa qidong\nJiejue fangan: Zai zhongduan zhong yunxing 'duckpx'\n\nWenti: Gengxin bu qizuoyong\nJiejue fangan: Shoudong zhixing:\nsudo /usr/local/share/duckpx/update.sh".to_string(),
                _ => "".to_string(),
            },
            _ => match section {
                "overview" => "=== OVERVIEW ===\n\nDuckPx is a tool for converting between pixels, millimeters and inches.\n\nIt provides a visual representation of the entered size and supports multiple languages.".to_string(),
                "usage" => "=== USAGE ===\n\n1. Enter a number in the input field\n2. Select the unit (Pixel, Millimeter or Inch)\n3. Press 'Calculate' or Enter\n4. The result is displayed and a square of the corresponding size is drawn".to_string(),
                "settings" => "=== SETTINGS ===\n\n- Toolbar Position: Top, Bottom, Left or Right\n- Manual Position: Position of this manual\n- Square Color: Color of the displayed square\n- Language: Deutsch, English, Francais, Russki, Zhongwen".to_string(),
                "examples" => "=== EXAMPLES ===\n\n50 Pixel = 13.23 mm = 0.52 inch\n100 mm = 377.95 px = 3.94 inch\n2 inch = 190.08 px = 50.80 mm".to_string(),
                "troubleshooting" => "=== TROUBLESHOOTING ===\n\nProblem: The program does not start\nSolution: Run 'duckpx' in the terminal\n\nProblem: Update does not work\nSolution: Run manually:\nsudo /usr/local/share/duckpx/update.sh".to_string(),
