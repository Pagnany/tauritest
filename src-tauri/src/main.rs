// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    if name.is_empty() {
        return "Hello, World!".to_string();
    }
    format!("Hello, {}! and maiu", name)
}

#[tauri::command]
fn get_position () -> String {
    let mut auftrags_positionen: Vec<AuftragsPosition> = Vec::new();
    auftrags_positionen.push(AuftragsPosition {
        artikel: "Artikel 1".to_string(),
        menge: 1,
        preis: 1.0,
    });
    auftrags_positionen.push(AuftragsPosition {
        artikel: "Artikel 2".to_string(),
        menge: 2,
        preis: 2.0,
    });
    auftrags_positionen.push(AuftragsPosition {
        artikel: "Artikel 3".to_string(),
        menge: 3,
        preis: 3.0,
    });
    let json = serde_json::to_string(&auftrags_positionen).unwrap();
    format!("{}", json)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_position])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize)]
struct AuftragsPosition {
    artikel: String,
    menge: i32,
    preis: f32,
}