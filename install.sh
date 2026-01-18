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
sudo apt install wget -y  || { echo "❌ Installation von wget fehlgeschlagen!"; exit 1; }
sudo apt install git -y  || { echo "❌ Installation von git fehlgeschlagen!"; exit 1; }
sudo apt install build-essential -y  || { echo "❌ Installation von build-essential fehlgeschlagen!"; exit 1; }
sudo apt install libgtk-3-dev -y  || { echo "❌ Installation von libgtk-3-dev fehlgeschlagen!"; exit 1; }
sudo apt install cargo -y  || { echo "❌ Installation von cargo fehlgeschlagen!"; exit 1; }
sudo apt install rustc -y  || { echo "❌ Installation von rustc fehlgeschlagen!"; exit 1; }

#Erstellen der Notwendigen Ordner
mkdir -p ~/.local/bin/
mkdir -p ~/.local/share/duckpx
mkdir -p ~/.local/share/duckpx/src
mkdir -p ~/.config/duckpx

#holen der Datein von Github
wget -O ~/.local/share/duckpx/src/main.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/main.rs
wget -O ~/.local/share/duckpx/Cargo.toml https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/Cargo.toml
wget -O ~/.local/share/duckpx/src/config.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/config.rs
wget -O ~/.local/share/duckpx/src/dpi.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/dpi.rs
wget -O ~/.config/duckpx/config.toml https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/config.toml
wget -O ~/.local/share/applications/duckpx.desktop https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/duckpx.desktop
wget -O ~/.local/share/duckpx/update.sh https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/install.sh
wget -O ~/.local/share/duckpx/icon.png https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/icon.png
wget -O ~/.local/share/duckpx/src/i18n.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/i18n.rs

# ~/.local/bin/ überprüfen
export PATH=$PATH:~/.local/bin/
source ~/.bashrc
source ~/.zshrc

#Funktionalitätder .desktop datei sicherstellen
sudo update-desktop-database
update-desktop-database

#Datein Komprimieren
# Rust-Projekt bauen
echo "🔧 Baue DuckPx mit Cargo..."
cd ~/.local/share/duckpx || { echo "❌ Verzeichnis nicht gefunden!"; exit 1; }
cargo clean  # Sauberer Build (löscht alte Artefakte)
cargo build --release || { echo "❌ Build fehlgeschlagen! Prüfe Cargo.toml."; exit 1; }
cp target/release/duckpx ~/.local/bin/ || { echo "❌ Kopieren fehlgeschlagen!"; exit 1; }

#Scripte Ausfühbar machen
sudo chmod +x ~/.local/share/duckpx/update.sh
sudo chmod +x ~/.local/bin/duckpx

#Aufräumen
rm -f ./install.sh*

echo DuckPx Start now!
duckpx
