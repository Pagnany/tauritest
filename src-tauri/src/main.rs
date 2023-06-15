// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

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
    let mut auftrags_positionen: Vec<AuftragsPosition> = Vec::new();
    for i in 1..=10 {
        auftrags_positionen.push(AuftragsPosition {
            position: i,
            artikel: format!("Artikel {}", i),
            menge: i,
            preis: Preis::new(i as f32 * 10.0, i as f32 * 11.0, i as f32 * 1.0),
        });
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

struct Auftrag {
    auftragsnummer: i32,
    kunde: String,
    auftragspositionen: Vec<AuftragsPosition>,
    kopftext: String,
    besteller: String,
}

#[derive(Serialize, Deserialize)]
struct AuftragsPosition {
    position: i32,
    artikel: String,
    menge: i32,
    preis: Preis,
}

#[derive(Serialize, Deserialize)]
struct Preis {
    netto: f32,
    brutto: f32,
    mwst: f32,
}

impl Preis {
    fn new(netto: f32, brutto: f32, mwst: f32) -> Preis {
        Preis {
            netto,
            brutto,
            mwst,
        }
    }
}
