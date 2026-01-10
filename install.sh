#!/bin/bash

#Intro des Scriptes
sudo apt update
echo Hi!
echo Nice that you use a programm from Change Goose

#Liste der Abhänigkeiten
echo We need the following programs
echo wget
echo git
echo build-essential
echo libgtk-3-dev
echo cargo
echo rustc

#instalation der benötigten Programme
sudo apt install wget -y
sudo apt install git -y
sudo apt install build-essential -y
sudo apt install libgtk-3-dev -y
sudo apt install cargo -y
sudo apt install rustc -y

#Erstellen der Notwendigen Ordner
sudo mkdir -p /usr/local/bin/
sudo mkdir -p /usr/local/share/duckpx
sudo mkdir -p /usr/local/share/duckpx/src
mkdir -p ~/.config/duckpx

#holen der Datein von Github
wget -O /usr/local/share/duckpx/src/main.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/main.rs
wget -O /usr/local/share/duckpx/Cargo.toml https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/Cargo.toml
wget -O /usr/local/share/duckpx/src/config.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/config.rs
wget -O /usr/local/share/duckpx/src/dpi.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/dpi.rs
wget -O ~/.config/duckpx/config.toml https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/config.toml
wget -O /usr/share/applications/duckpx.desktop https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/duckpx.desktop
wget -O /usr/local/share/duckpx/update.sh https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/install.sh
wget -O /usr/local/share/duckpx/icon.png https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/icon.png
wget -O /usr/local/share/duckpx/src/translations.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/translations.rs

#Funktionalitätder .desktop datei sicherstellen
sudo update-desktop-database

#Datein Komprimieren
# Rust-Projekt bauen
echo "🔧 Baue DuckPx mit Cargo..."
cd /usr/local/share/duckpx || { echo "❌ Verzeichnis nicht gefunden!"; exit 1; }
cargo clean  # Sauberer Build (löscht alte Artefakte)
cargo build --release || { echo "❌ Build fehlgeschlagen! Prüfe Cargo.toml."; exit 1; }
sudo cp target/release/duckpx /usr/local/bin/ || { echo "❌ Kopieren fehlgeschlagen!"; exit 1; }

#Scripte Ausfühbar machen
sudo chmod +x /usr/local/share/duckpx/update.sh
sudo chmod +x /usr/local/bin/duckpx

echo DuckPx Start now!
duckpx
