use serde::{Deserialize, Serialize};

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
