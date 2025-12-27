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
sudo mkdir -p /usr/local/share/duckpx
sudo mkdir -p /usr/local/share/duckpx/src
mkdir -p ~/.config/duckpx

#holen der Datein von Github
wget -O /usr/local/share/duckpx/start.sh https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/start.sh
wget -O /usr/local/share/duckpx/src/main.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/main.rs
wget -O /usr/local/share/duckpx/Cargo.toml https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/Cargo.toml
wget -O /usr/local/share/duckpx/src/config.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/config.rs
wget -O /usr/local/share/duckpx/src/dpi.rs https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/dpi.rs
wget -O ~/.config/duckpx/config.toml https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/config.toml
wget -O /usr/share/applications/duckpx.desktop https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/main/duckpx.desktop
wget -O /usr/local/share/duckpx/update.sh https://raw.githubusercontent.com/Change-Goose-Open-Surce-Software/duckpx/install.sh

#Funktionalitätder .desktop datei sicherstellen
sudo update-desktop-database

#Datein Komprimieren
# Rust-Projekt bauen
echo "🔧 Baue DuckPx mit Cargo..."
cd /usr/local/share/duckpx  # Wechsle in das Projektverzeichnis
cargo build --release       # Baue das Projekt im Release-Modus (optimiert)
sudo cp target/release/duckpx /usr/local/bin/  # Kopiere die fertige Binärdatei nach /usr/local/bin/

#Scripte Ausfühbar machen
sudo chmod +x /usr/local/share/duckpx/start.sh
sudo chmod +x /usr/local/share/duckpx/update.sh
sudo chmod +x /usr/local/bin/duckpx

echo Duckpx are installed now! Start it with the Command duckpx
