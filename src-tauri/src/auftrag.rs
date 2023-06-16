use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct Preis {
    netto: f32,
    brutto: f32,
    mwst: f32,
}

impl Preis {
    pub fn new(netto: f32, brutto: f32, mwst: f32) -> Preis {
        Preis {
            netto,
            brutto,
            mwst,
        }
    }
}
