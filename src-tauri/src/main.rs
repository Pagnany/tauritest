// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, Result};

mod auftrag;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    if name.is_empty() {
        return "Hello, World!".to_string();
    }
    format!("Hello, {}! and maiu", name)
}

#[tauri::command]
fn get_position() -> String {
    let mut auftrags_positionen: Vec<auftrag::AuftragsPosition> = Vec::new();
    for i in 1..=10 {
        let auftrags_position = auftrag::AuftragsPosition::new(
            i,
            "Artikel".to_string(),
            i,
            auftrag::Preis::new(1.0, 1.0, 1.0),
        );
        auftrags_positionen.push(auftrags_position);
    }
    let json = serde_json::to_string(&auftrags_positionen).unwrap();
    json
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_position])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_tables() {
    let conn = Connection::open("./auftrag.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS auftrag (
            id INTEGER PRIMARY KEY,
            auftragsnummer INTEGER NOT NULL,
            kunde TEXT NOT NULL,
            auftragspositionen TEXT NOT NULL,
            kopftext TEXT NOT NULL,
            besteller TEXT NOT NULL
        )",
        params![],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS auftragsposition (
            id INTEGER PRIMARY KEY,
            auftragid INTEGER NOT NULL,
            position INTEGER NOT NULL,
            artikel TEXT NOT NULL,
            menge INTEGER NOT NULL,
            preisid INTEGER NOT NULL
        )",
        params![],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS preis (
            id INTEGER PRIMARY KEY,
            position INTEGER NOT NULL,
            netto REAL NOT NULL,
            brutto REAL NOT NULL,
            mwst REAL NOT NULL
        )",
        params![],
    )
    .unwrap();
}
