use std::collections::HashMap;

pub struct Translations {
    texts: HashMap<String, HashMap<String, String>>,
}

impl Translations {
    pub fn new() -> Self {
        let mut translations = Translations {
            texts: HashMap::new(),
        };
        translations.init();
        translations
    }

    fn init(&mut self) {
        // Deutsche Übersetzungen
        let mut de = HashMap::new();
        de.insert("app_title".to_string(), "DuckPx".to_string());
        de.insert("update".to_string(), "Update".to_string());
        de.insert("github".to_string(), "GitHub".to_string());
        de.insert("settings".to_string(), "Einstellungen".to_string());
        de.insert("version".to_string(), "Version".to_string());
        de.insert("manual".to_string(), "Anleitung".to_string());
        de.insert("restart".to_string(), "Neustart".to_string());
        de.insert("pixel".to_string(), "Pixel (px)".to_string());
        de.insert("millimeter".to_string(), "Millimeter (mm)".to_string());
        de.insert("inch".to_string(), "Inch (in)".to_string());
        de.insert("input_placeholder".to_string(), "Größe eingeben (z. B. 50)".to_string());
        de.insert("calculate".to_string(), "Berechnen".to_string());
        de.insert("result_default".to_string(), "Ergebnis erscheint hier".to_string());
        de.insert("settings_title".to_string(), "DuckPx Einstellungen".to_string());
        de.insert("toolbar_position".to_string(), "Toolbar-Position:".to_string());
        de.insert("manual_position".to_string(), "Anleitungs-Position:".to_string());
        de.insert("square_color".to_string(), "Quadratfarbe:".to_string());
        de.insert("language".to_string(), "Sprache:".to_string());
        de.insert("save".to_string(), "Speichern".to_string());
        de.insert("top".to_string(), "Oben".to_string());
        de.insert("bottom".to_string(), "Unten".to_string());
        de.insert("left".to_string(), "Links".to_string());
        de.insert("right".to_string(), "Rechts".to_string());
        de.insert("manual_title".to_string(), "DuckPx Anleitung".to_string());
        de.insert("manual_intro".to_string(), "Willkommen zu DuckPx!".to_string());
        de.insert("manual_overview".to_string(), "Übersicht".to_string());
        de.insert("manual_usage".to_string(), "Verwendung".to_string());
        de.insert("manual_settings".to_string(), "Einstellungen".to_string());
        de.insert("manual_examples".to_string(), "Beispiele".to_string());
        de.insert("manual_troubleshooting".to_string(), "Fehlerbehebung".to_string());

        // Englische Übersetzungen
        let mut en = HashMap::new();
        en.insert("app_title".to_string(), "DuckPx".to_string());
        en.insert("update".to_string(), "Update".to_string());
        en.insert("github".to_string(), "GitHub".to_string());
        en.insert("settings".to_string(), "Settings".to_string());
        en.insert("version".to_string(), "Version".to_string());
        en.insert("manual".to_string(), "Manual".to_string());
        en.insert("restart".to_string(), "Restart".to_string());
        en.insert("pixel".to_string(), "Pixel (px)".to_string());
        en.insert("millimeter".to_string(), "Millimeter (mm)".to_string());
        en.insert("inch".to_string(), "Inch (in)".to_string());
        en.insert("input_placeholder".to_string(), "Enter size (e.g. 50)".to_string());
        en.insert("calculate".to_string(), "Calculate".to_string());
        en.insert("result_default".to_string(), "Result will appear here".to_string());
        en.insert("settings_title".to_string(), "DuckPx Settings".to_string());
        en.insert("toolbar_position".to_string(), "Toolbar Position:".to_string());
        en.insert("manual_position".to_string(), "Manual Position:".to_string());
        en.insert("square_color".to_string(), "Square Color:".to_string());
        en.insert("language".to_string(), "Language:".to_string());
        en.insert("save".to_string(), "Save".to_string());
        en.insert("top".to_string(), "Top".to_string());
        en.insert("bottom".to_string(), "Bottom".to_string());
        en.insert("left".to_string(), "Left".to_string());
        en.insert("right".to_string(), "Right".to_string());
        en.insert("manual_title".to_string(), "DuckPx Manual".to_string());
        en.insert("manual_intro".to_string(), "Welcome to DuckPx!".to_string());
        en.insert("manual_overview".to_string(), "Overview".to_string());
        en.insert("manual_usage".to_string(), "Usage".to_string());
        en.insert("manual_settings".to_string(), "Settings".to_string());
        en.insert("manual_examples".to_string(), "Examples".to_string());
        en.insert("manual_troubleshooting".to_string(), "Troubleshooting".to_string());

        // Französische Übersetzungen
        let mut fr = HashMap::new();
        fr.insert("app_title".to_string(), "DuckPx".to_string());
        fr.insert("update".to_string(), "Mise a jour".to_string());
        fr.insert("github".to_string(), "GitHub".to_string());
        fr.insert("settings".to_string(), "Parametres".to_string());
        fr.insert("version".to_string(), "Version".to_string());
        fr.insert("manual".to_string(), "Manuel".to_string());
        fr.insert("restart".to_string(), "Redemarrer".to_string());
        fr.insert("pixel".to_string(), "Pixel (px)".to_string());
        fr.insert("millimeter".to_string(), "Millimetre (mm)".to_string());
        fr.insert("inch".to_string(), "Pouce (in)".to_string());
        fr.insert("input_placeholder".to_string(), "Entrer la taille (p. ex. 50)".to_string());
        fr.insert("calculate".to_string(), "Calculer".to_string());
        fr.insert("result_default".to_string(), "Le resultat apparaitra ici".to_string());
        fr.insert("settings_title".to_string(), "Parametres DuckPx".to_string());
        fr.insert("toolbar_position".to_string(), "Position de la barre:".to_string());
        fr.insert("manual_position".to_string(), "Position du manuel:".to_string());
        fr.insert("square_color".to_string(), "Couleur du carre:".to_string());
        fr.insert("language".to_string(), "Langue:".to_string());
        fr.insert("save".to_string(), "Enregistrer".to_string());
        fr.insert("top".to_string(), "Haut".to_string());
        fr.insert("bottom".to_string(), "Bas".to_string());
        fr.insert("left".to_string(), "Gauche".to_string());
        fr.insert("right".to_string(), "Droite".to_string());
        fr.insert("manual_title".to_string(), "Manuel DuckPx".to_string());
        fr.insert("manual_intro".to_string(), "Bienvenue dans DuckPx!".to_string());
        fr.insert("manual_overview".to_string(), "Apercu".to_string());
        fr.insert("manual_usage".to_string(), "Utilisation".to_string());
        fr.insert("manual_settings".to_string(), "Parametres".to_string());
        fr.insert("manual_examples".to_string(), "Exemples".to_string());
        fr.insert("manual_troubleshooting".to_string(), "Depannage".to_string());

        // Russische Übersetzungen
        let mut ru = HashMap::new();
        ru.insert("app_title".to_string(), "DuckPx".to_string());
        ru.insert("update".to_string(), "Obnovit".to_string());
        ru.insert("github".to_string(), "GitHub".to_string());
        ru.insert("settings".to_string(), "Nastroyki".to_string());
        ru.insert("version".to_string(), "Versiya".to_string());
        ru.insert("manual".to_string(), "Rukovodstvo".to_string());
        ru.insert("restart".to_string(), "Perezapusk".to_string());
        ru.insert("pixel".to_string(), "Piksel (px)".to_string());
        ru.insert("millimeter".to_string(), "Millimetr (mm)".to_string());
        ru.insert("inch".to_string(), "Dyuym (in)".to_string());
        ru.insert("input_placeholder".to_string(), "Vvedite razmer (napr. 50)".to_string());
        ru.insert("calculate".to_string(), "Vychislit".to_string());
        ru.insert("result_default".to_string(), "Rezultat poyavitsya zdes".to_string());
        ru.insert("settings_title".to_string(), "Nastroyki DuckPx".to_string());
        ru.insert("toolbar_position".to_string(), "Pozitsiya paneli:".to_string());
        ru.insert("manual_position".to_string(), "Pozitsiya rukovodstva:".to_string());
        ru.insert("square_color".to_string(), "Tsvet kvadrata:".to_string());
        ru.insert("language".to_string(), "Yazyk:".to_string());
        ru.insert("save".to_string(), "Sokhranit".to_string());
        ru.insert("top".to_string(), "Sverkhu".to_string());
        ru.insert("bottom".to_string(), "Snizu".to_string());
        ru.insert("left".to_string(), "Sleva".to_string());
        ru.insert("right".to_string(), "Sprava".to_string());
        ru.insert("manual_title".to_string(), "Rukovodstvo DuckPx".to_string());
        ru.insert("manual_intro".to_string(), "Dobro pozhalovat v DuckPx!".to_string());
        ru.insert("manual_overview".to_string(), "Obzor".to_string());
        ru.insert("manual_usage".to_string(), "Ispolzovanie".to_string());
        ru.insert("manual_settings".to_string(), "Nastroyki".to_string());
        ru.insert("manual_examples".to_string(), "Primery".to_string());
        ru.insert("manual_troubleshooting".to_string(), "Ustranenie nepoladok".to_string());

        // Chinesische Übersetzungen (Pinyin zur Vermeidung von Encoding-Problemen)
        let mut zh = HashMap::new();
        zh.insert("app_title".to_string(), "DuckPx".to_string());
        zh.insert("update".to_string(), "Gengxin".to_string());
        zh.insert("github".to_string(), "GitHub".to_string());
        zh.insert("settings".to_string(), "Shezhi".to_string());
        zh.insert("version".to_string(), "Banben".to_string());
        zh.insert("manual".to_string(), "Shouce".to_string());
        zh.insert("restart".to_string(), "Chongqi".to_string());
        zh.insert("pixel".to_string(), "Xiangsu (px)".to_string());
        zh.insert("millimeter".to_string(), "Haomi (mm)".to_string());
        zh.insert("inch".to_string(), "Yingcun (in)".to_string());
        zh.insert("input_placeholder".to_string(), "Shuru daxiao (liru 50)".to_string());
        zh.insert("calculate".to_string(), "Jisuan".to_string());
        zh.insert("result_default".to_string(), "Jieguo jiang xianshi zai zheli".to_string());
        zh.insert("settings_title".to_string(), "DuckPx Shezhi".to_string());
        zh.insert("toolbar_position".to_string(), "Gongjulan weizhi:".to_string());
        zh.insert("manual_position".to_string(), "Shouce weizhi:".to_string());
        zh.insert("square_color".to_string(), "Fangxing yanse:".to_string());
        zh.insert("language".to_string(), "Yuyan:".to_string());
        zh.insert("save".to_string(), "Baocun".to_string());
        zh.insert("top".to_string(), "Dingbu".to_string());
        zh.insert("bottom".to_string(), "Dibu".to_string());
        zh.insert("left".to_string(), "Zuoce".to_string());
        zh.insert("right".to_string(), "Youce".to_string());
        zh.insert("manual_title".to_string(), "DuckPx Shouce".to_string());
        zh.insert("manual_intro".to_string(), "Huanying shiyong DuckPx!".to_string());
        zh.insert("manual_overview".to_string(), "Gaishu".to_string());
        zh.insert("manual_usage".to_string(), "Shiyong fangfa".to_string());
        zh.insert("manual_settings".to_string(), "Shezhi".to_string());
        zh.insert("manual_examples".to_string(), "Shili".to_string());
        zh.insert("manual_troubleshooting".to_string(), "Guzhang paichu".to_string());

        self.texts.insert("de".to_string(), de);
        self.texts.insert("en".to_string(), en);
        self.texts.insert("fr".to_string(), fr);
        self.texts.insert("ru".to_string(), ru);
        self.texts.insert("zh".to_string(), zh);
    }

    pub fn get(&self, lang: &str, key: &str) -> String {
        self.texts
            .get(lang)
            .and_then(|lang_map| lang_map.get(key))
            .cloned()
            .unwrap_or_else(|| {
                self.texts
                    .get("en")
                    .and_then(|lang_map| lang_map.get(key))
                    .cloned()
                    .unwrap_or_else(|| key.to_string())
            })
    }
}
