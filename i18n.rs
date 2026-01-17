use std::collections::HashMap;

pub struct I18n {
    translations: HashMap<String, HashMap<String, String>>,
}

impl I18n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // Deutsch
        let mut de = HashMap::new();
        de.insert("app_title".to_string(), "DuckPx".to_string());
        de.insert("update".to_string(), "Update".to_string());
        de.insert("github".to_string(), "GitHub".to_string());
        de.insert("settings".to_string(), "Einstellungen".to_string());
        de.insert("version".to_string(), "Version".to_string());
        de.insert("manual".to_string(), "Anleitung".to_string());
        de.insert("restart".to_string(), "Neustart".to_string());
        de.insert("input_placeholder".to_string(), "Größe eingeben (z. B. 50)".to_string());
        de.insert("calculate".to_string(), "Berechnen".to_string());
        de.insert("result_placeholder".to_string(), "Ergebnis erscheint hier".to_string());
        de.insert("pixel".to_string(), "Pixel (px)".to_string());
        de.insert("millimeter".to_string(), "Millimeter (mm)".to_string());
        de.insert("inch".to_string(), "Inch (in)".to_string());
        de.insert("settings_title".to_string(), "DuckPx Einstellungen".to_string());
        de.insert("toolbar_position".to_string(), "Toolbar-Position:".to_string());
        de.insert("manual_sidebar_position".to_string(), "Anleitungs-Gliederungs-Position:".to_string());
        de.insert("square_color".to_string(), "Quadratfarbe:".to_string());
        de.insert("language".to_string(), "Sprache:".to_string());
        de.insert("save".to_string(), "Speichern".to_string());
        de.insert("top".to_string(), "Oben".to_string());
        de.insert("bottom".to_string(), "Unten".to_string());
        de.insert("left".to_string(), "Links".to_string());
        de.insert("right".to_string(), "Rechts".to_string());
        de.insert("manual_title".to_string(), "DuckPx Anleitung".to_string());
        de.insert("manual_intro".to_string(), "Einführung".to_string());
        de.insert("manual_basic".to_string(), "Grundlagen".to_string());
        de.insert("manual_conversion".to_string(), "Umrechnung".to_string());
        de.insert("manual_settings".to_string(), "Einstellungen".to_string());
        de.insert("manual_examples".to_string(), "Beispiele".to_string());
        
        // Englisch
        let mut en = HashMap::new();
        en.insert("app_title".to_string(), "DuckPx".to_string());
        en.insert("update".to_string(), "Update".to_string());
        en.insert("github".to_string(), "GitHub".to_string());
        en.insert("settings".to_string(), "Settings".to_string());
        en.insert("version".to_string(), "Version".to_string());
        en.insert("manual".to_string(), "Manual".to_string());
        en.insert("restart".to_string(), "Restart".to_string());
        en.insert("input_placeholder".to_string(), "Enter size (e.g. 50)".to_string());
        en.insert("calculate".to_string(), "Calculate".to_string());
        en.insert("result_placeholder".to_string(), "Result appears here".to_string());
        en.insert("pixel".to_string(), "Pixel (px)".to_string());
        en.insert("millimeter".to_string(), "Millimeter (mm)".to_string());
        en.insert("inch".to_string(), "Inch (in)".to_string());
        en.insert("settings_title".to_string(), "DuckPx Settings".to_string());
        en.insert("toolbar_position".to_string(), "Toolbar Position:".to_string());
        en.insert("manual_sidebar_position".to_string(), "Manual Sidebar Position:".to_string());
        en.insert("square_color".to_string(), "Square Color:".to_string());
        en.insert("language".to_string(), "Language:".to_string());
        en.insert("save".to_string(), "Save".to_string());
        en.insert("top".to_string(), "Top".to_string());
        en.insert("bottom".to_string(), "Bottom".to_string());
        en.insert("left".to_string(), "Left".to_string());
        en.insert("right".to_string(), "Right".to_string());
        en.insert("manual_title".to_string(), "DuckPx Manual".to_string());
        en.insert("manual_intro".to_string(), "Introduction".to_string());
        en.insert("manual_basic".to_string(), "Basics".to_string());
        en.insert("manual_conversion".to_string(), "Conversion".to_string());
        en.insert("manual_settings".to_string(), "Settings".to_string());
        en.insert("manual_examples".to_string(), "Examples".to_string());
        
        // Französisch
        let mut fr = HashMap::new();
        fr.insert("app_title".to_string(), "DuckPx".to_string());
        fr.insert("update".to_string(), "Mise à jour".to_string());
        fr.insert("github".to_string(), "GitHub".to_string());
        fr.insert("settings".to_string(), "Paramètres".to_string());
        fr.insert("version".to_string(), "Version".to_string());
        fr.insert("manual".to_string(), "Manuel".to_string());
        fr.insert("restart".to_string(), "Redémarrer".to_string());
        fr.insert("input_placeholder".to_string(), "Entrer taille (p. ex. 50)".to_string());
        fr.insert("calculate".to_string(), "Calculer".to_string());
        fr.insert("result_placeholder".to_string(), "Le résultat apparaît ici".to_string());
        fr.insert("pixel".to_string(), "Pixel (px)".to_string());
        fr.insert("millimeter".to_string(), "Millimètre (mm)".to_string());
        fr.insert("inch".to_string(), "Pouce (in)".to_string());
        fr.insert("settings_title".to_string(), "Paramètres DuckPx".to_string());
        fr.insert("toolbar_position".to_string(), "Position de la barre:".to_string());
        fr.insert("manual_sidebar_position".to_string(), "Position de la barre du manuel:".to_string());
        fr.insert("square_color".to_string(), "Couleur du carré:".to_string());
        fr.insert("language".to_string(), "Langue:".to_string());
        fr.insert("save".to_string(), "Enregistrer".to_string());
        fr.insert("top".to_string(), "Haut".to_string());
        fr.insert("bottom".to_string(), "Bas".to_string());
        fr.insert("left".to_string(), "Gauche".to_string());
        fr.insert("right".to_string(), "Droite".to_string());
        fr.insert("manual_title".to_string(), "Manuel DuckPx".to_string());
        fr.insert("manual_intro".to_string(), "Introduction".to_string());
        fr.insert("manual_basic".to_string(), "Notions de base".to_string());
        fr.insert("manual_conversion".to_string(), "Conversion".to_string());
        fr.insert("manual_settings".to_string(), "Paramètres".to_string());
        fr.insert("manual_examples".to_string(), "Exemples".to_string());
        
        // Russisch
        let mut ru = HashMap::new();
        ru.insert("app_title".to_string(), "DuckPx".to_string());
        ru.insert("update".to_string(), "Обновить".to_string());
        ru.insert("github".to_string(), "GitHub".to_string());
        ru.insert("settings".to_string(), "Настройки".to_string());
        ru.insert("version".to_string(), "Версия".to_string());
        ru.insert("manual".to_string(), "Руководство".to_string());
        ru.insert("restart".to_string(), "Перезапуск".to_string());
        ru.insert("input_placeholder".to_string(), "Введите размер (напр. 50)".to_string());
        ru.insert("calculate".to_string(), "Вычислить".to_string());
        ru.insert("result_placeholder".to_string(), "Результат появится здесь".to_string());
        ru.insert("pixel".to_string(), "Пиксель (px)".to_string());
        ru.insert("millimeter".to_string(), "Миллиметр (mm)".to_string());
        ru.insert("inch".to_string(), "Дюйм (in)".to_string());
        ru.insert("settings_title".to_string(), "Настройки DuckPx".to_string());
        ru.insert("toolbar_position".to_string(), "Положение панели:".to_string());
        ru.insert("manual_sidebar_position".to_string(), "Положение боковой панели:".to_string());
        ru.insert("square_color".to_string(), "Цвет квадрата:".to_string());
        ru.insert("language".to_string(), "Язык:".to_string());
        ru.insert("save".to_string(), "Сохранить".to_string());
        ru.insert("top".to_string(), "Сверху".to_string());
        ru.insert("bottom".to_string(), "Снизу".to_string());
        ru.insert("left".to_string(), "Слева".to_string());
        ru.insert("right".to_string(), "Справа".to_string());
        ru.insert("manual_title".to_string(), "Руководство DuckPx".to_string());
        ru.insert("manual_intro".to_string(), "Введение".to_string());
        ru.insert("manual_basic".to_string(), "Основы".to_string());
        ru.insert("manual_conversion".to_string(), "Преобразование".to_string());
        ru.insert("manual_settings".to_string(), "Настройки".to_string());
        ru.insert("manual_examples".to_string(), "Примеры".to_string());
        
        // Chinesisch
        let mut zh = HashMap::new();
        zh.insert("app_title".to_string(), "DuckPx".to_string());
        zh.insert("update".to_string(), "更新".to_string());
        zh.insert("github".to_string(), "GitHub".to_string());
        zh.insert("settings".to_string(), "设置".to_string());
        zh.insert("version".to_string(), "版本".to_string());
        zh.insert("manual".to_string(), "手册".to_string());
        zh.insert("restart".to_string(), "重启".to_string());
        zh.insert("input_placeholder".to_string(), "输入大小 (例如 50)".to_string());
        zh.insert("calculate".to_string(), "计算".to_string());
        zh.insert("result_placeholder".to_string(), "结果显示在这里".to_string());
        zh.insert("pixel".to_string(), "像素 (px)".to_string());
        zh.insert("millimeter".to_string(), "毫米 (mm)".to_string());
        zh.insert("inch".to_string(), "英寸 (in)".to_string());
        zh.insert("settings_title".to_string(), "DuckPx 设置".to_string());
        zh.insert("toolbar_position".to_string(), "工具栏位置:".to_string());
        zh.insert("manual_sidebar_position".to_string(), "手册侧边栏位置:".to_string());
        zh.insert("square_color".to_string(), "方块颜色:".to_string());
        zh.insert("language".to_string(), "语言:".to_string());
        zh.insert("save".to_string(), "保存".to_string());
        zh.insert("top".to_string(), "顶部".to_string());
        zh.insert("bottom".to_string(), "底部".to_string());
        zh.insert("left".to_string(), "左侧".to_string());
        zh.insert("right".to_string(), "右侧".to_string());
        zh.insert("manual_title".to_string(), "DuckPx 手册".to_string());
        zh.insert("manual_intro".to_string(), "介绍".to_string());
        zh.insert("manual_basic".to_string(), "基础".to_string());
        zh.insert("manual_conversion".to_string(), "转换".to_string());
        zh.insert("manual_settings".to_string(), "设置".to_string());
        zh.insert("manual_examples".to_string(), "示例".to_string());
        
        translations.insert("de".to_string(), de);
        translations.insert("en".to_string(), en);
        translations.insert("fr".to_string(), fr);
        translations.insert("ru".to_string(), ru);
        translations.insert("zh".to_string(), zh);
        
        I18n { translations }
    }
    
    pub fn t(&self, lang: &str, key: &str) -> String {
        self.translations
            .get(lang)
            .and_then(|lang_map| lang_map.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}
