use serde::{Deserialize, Serialize};
use crate::preis::Preis;

#[derive(Serialize, Deserialize)]
pub struct Auftrag {
    auftragsnummer: i32,
    kunde: String,
    auftragspositionen: Vec<AuftragsPosition>,
    kopftext: String,
    besteller: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuftragsPosition {
    position: i32,
    artikel: String,
    menge: i32,
    preis: Preis,
}

impl AuftragsPosition {
    pub fn new(position: i32, artikel: String, menge: i32, preis: Preis) -> AuftragsPosition {
        AuftragsPosition {
            position,
            artikel,
            menge,
            preis,
        }
    }
}