# DuckPx 1.2.0 - Änderungen

## Übersicht der bearbeiteten Dateien

### 1. **config.rs** (Vollständig überarbeitet)
- ✅ Hintergrundfarben-Konfiguration entfernt
- ✅ Neue Einstellung `language` hinzugefügt
- ✅ Neue Einstellung `manual_sidebar_position` hinzugefügt
- ✅ Toolbar-Position erweitert auf 4 Optionen (top, bottom, left, right)
- ✅ Automatische Spracherkennung basierend auf `$LANG` Umgebungsvariable
- ✅ Fallback auf Englisch wenn Sprache nicht erkannt wird

### 2. **i18n.rs** (Neu erstellt)
- ✅ Komplettes Internationalisierungssystem
- ✅ 5 Sprachen unterstützt:
  - Deutsch (de)
  - Englisch (en)
  - Französisch (fr)
  - Russisch (ru)
  - Chinesisch (zh)
- ✅ Alle UI-Strings übersetzt
- ✅ Einfache `t()` Funktion für Übersetzungen

### 3. **main.rs** (Vollständig neu geschrieben)
- ✅ Mehrsprachigkeit komplett integriert
- ✅ Enter-Taste funktioniert jetzt im Eingabefeld
- ✅ Ergebnisanzeige vergrößert (`xx-large` statt `x-large`)
- ✅ Update-Button öffnet Terminal-Emulator
- ✅ Neuer "Version"-Button
- ✅ Neuer "Manual"-Button mit ausführlicher Anleitung
- ✅ Neuer "Neustart"-Button
- ✅ Toolbar-Position auf 4 Seiten erweiterbar
- ✅ Manual-Fenster mit Kategorien und Sidebar
- ✅ Sidebar-Position im Manual-Fenster konfigurierbar
- ✅ Absturzproblem behoben durch bessere Speicherverwaltung (Rc/RefCell)
- ✅ Hintergrundfarben-Einstellung aus Settings entfernt

### 4. **dpi.rs** (Unverändert)
- Keine Änderungen notwendig

### 5. **Cargo.toml** (Aktualisiert)
- ✅ Version auf 1.2.0 erhöht

### 6. **update.sh** (Neu erstellt)
- ✅ Professionelles Update-Skript
- ✅ Lädt neueste Version von GitHub
- ✅ Führt Installation automatisch aus
- ✅ Fehlerbehandlung und Status-Ausgaben

### 7. **version.html** (Neu erstellt)
- ✅ Schöne HTML-Seite mit Versionshistorie
- ✅ Übersicht aller Features und Bugfixes
- ✅ Moderne, responsive Gestaltung
- ✅ Vollständige Changelog-Information

## Implementierte Features

### Bugfixes (aus Konzept)
1. ✅ **Update-Button**: Öffnet Terminal-Emulator und führt `/usr/local/share/duckpx/update.sh` aus
2. ✅ **Hintergrundfarbe**: Einstellung entfernt, nutzt jetzt Systemstandard
3. ✅ **Absturz-Problem**: Durch bessere Speicherverwaltung behoben (keine Drawing-Area Reconnects mehr)

### Neue Funktionen (aus Konzept)
1. ✅ **Mehrsprachigkeit**: 
   - 5 Sprachen (DE, EN, FR, RU, ZH)
   - Automatische Erkennung der Systemsprache
   - Manuell wählbar in Einstellungen

2. ✅ **Version-Button**: Öffnet `/usr/local/share/duckpx/version.html`

3. ✅ **Man-Button**: 
   - Ausführliche Anleitung
   - Kategorien: Einführung, Grundlagen, Umrechnung, Einstellungen, Beispiele
   - Kategorien anklickbar für Navigation
   - Mehrsprachig

4. ✅ **Anleitungs-Gliederungs-Position**: Einstellbar auf Oben, Unten, Links, Rechts

5. ✅ **Toolbar-Position**: Erweitert auf Oben, Unten, Links, Rechts

6. ✅ **Enter-Taste**: Funktioniert jetzt im Eingabefeld

7. ✅ **Größere Umrechnung**: Ergebnis jetzt in `xx-large` und `bold`

8. ✅ **Neustart-Button**: In Toolbar integriert

## Technische Verbesserungen

- **Bessere Code-Struktur**: Module für i18n, config, dpi
- **Saubere Speicherverwaltung**: Verwendung von `Rc<RefCell<>>` für Config
- **Keine Memory Leaks**: Drawing-Area wird nur einmal connected
- **Flexibles Layout**: Dynamische Anordnung je nach Konfiguration
- **Robuste Fehlerbehandlung**: Mehrere Terminal-Emulatoren werden versucht

## Installation & Verwendung

Die Dateien sollten wie folgt strukturiert sein:

```
duckpx/
├── src/
│   ├── main.rs          (NEU)
│   ├── config.rs        (ÜBERARBEITET)
│   ├── i18n.rs          (NEU)
│   └── dpi.rs           (UNVERÄNDERT)
├── Cargo.toml           (AKTUALISIERT)
└── /usr/local/share/duckpx/
    ├── update.sh        (NEU)
    └── version.html     (NEU)
```

## Konfigurationsdatei

Die Konfiguration wird gespeichert unter:
`~/.config/duckpx/config.toml`

Beispiel:
```toml
[colors]
square = "#FFA500"

[ui]
toolbar_position = "top"
manual_sidebar_position = "left"

language = "de"
```

## Kompilierung

```bash
cargo build --release
```

## Hinweise

- Alle Texte sind in den 5 Sprachen verfügbar
- Das Programm erkennt automatisch die Systemsprache
- Alle Einstellungen werden persistent gespeichert
- Der Neustart-Button startet die Anwendung automatisch neu
- Terminal-Update unterstützt: gnome-terminal, konsole, xfce4-terminal, xterm, terminator
