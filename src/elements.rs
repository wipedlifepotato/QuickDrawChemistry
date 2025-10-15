#[derive(Debug, Clone)]
pub struct Element {
    pub x: f32,
    pub y: f32,
    pub rotations: f32,
    pub val: String,
}

impl Element {
    pub fn new() -> Self {
        Element {
            x: 0.0,
            y: 0.0,
            rotations: 0.0,
            val: String::new(),
        }
    }

    pub fn with_value(val: &str) -> Self {
        Element {
            x: 0.0,
            y: 0.0,
            rotations: 0.0,
            val: val.to_string(),
        }
    }
}

pub fn get_elements() -> Vec<String> {
    vec![
        // Full periodic table elements
        "H".to_string(), "He".to_string(), "Li".to_string(), "Be".to_string(), "B".to_string(), "C".to_string(), "N".to_string(), "O".to_string(), "F".to_string(), "Ne".to_string(),
        "Na".to_string(), "Mg".to_string(), "Al".to_string(), "Si".to_string(), "P".to_string(), "S".to_string(), "Cl".to_string(), "Ar".to_string(), "K".to_string(), "Ca".to_string(),
        "Sc".to_string(), "Ti".to_string(), "V".to_string(), "Cr".to_string(), "Mn".to_string(), "Fe".to_string(), "Co".to_string(), "Ni".to_string(), "Cu".to_string(), "Zn".to_string(),
        "Ga".to_string(), "Ge".to_string(), "As".to_string(), "Se".to_string(), "Br".to_string(), "Kr".to_string(), "Rb".to_string(), "Sr".to_string(), "Y".to_string(), "Zr".to_string(),
        "Nb".to_string(), "Mo".to_string(), "Tc".to_string(), "Ru".to_string(), "Rh".to_string(), "Pd".to_string(), "Ag".to_string(), "Cd".to_string(), "In".to_string(), "Sn".to_string(),
        "Sb".to_string(), "Te".to_string(), "I".to_string(), "Xe".to_string(), "Cs".to_string(), "Ba".to_string(), "La".to_string(), "Ce".to_string(), "Pr".to_string(), "Nd".to_string(),
        "Pm".to_string(), "Sm".to_string(), "Eu".to_string(), "Gd".to_string(), "Tb".to_string(), "Dy".to_string(), "Ho".to_string(), "Er".to_string(), "Tm".to_string(), "Yb".to_string(),
        "Lu".to_string(), "Hf".to_string(), "Ta".to_string(), "W".to_string(), "Re".to_string(), "Os".to_string(), "Ir".to_string(), "Pt".to_string(), "Au".to_string(), "Hg".to_string(),
        "Tl".to_string(), "Pb".to_string(), "Bi".to_string(), "Po".to_string(), "At".to_string(), "Rn".to_string(), "Fr".to_string(), "Ra".to_string(), "Ac".to_string(), "Th".to_string(),
        "Pa".to_string(), "U".to_string(), "Np".to_string(), "Pu".to_string(), "Am".to_string(), "Cm".to_string(), "Bk".to_string(), "Cf".to_string(), "Es".to_string(), "Fm".to_string(),
        "Md".to_string(), "No".to_string(), "Lr".to_string(),
        // Hydrocarbons and organic
        "CH4".to_string(), "CH3".to_string(), "CH2".to_string(), "CH".to_string(), "C2H6".to_string(), "C3H8".to_string(), "C4H10".to_string(), "C5H12".to_string(), "C6H14".to_string(),
        // Alkenes
        "C2H4".to_string(), "C3H6".to_string(),
        // Alkynes
        "C2H2".to_string(),
        // Aromatics
        "C6H6".to_string(),
        // Alcohols
        "CH3OH".to_string(), "C2H5OH".to_string(),
        // Aldehydes
        "CH2O".to_string(), "CH3CHO".to_string(),
        // Ketones
        "CH3COCH3".to_string(),
        // Acids
        "HCOOH".to_string(), "CH3COOH".to_string(),
        // Inorganic acids
        "HCl".to_string(), "HBr".to_string(), "HI".to_string(), "H2SO4".to_string(), "HNO3".to_string(), "H3PO4".to_string(),
        // Other inorganic
        "H2O".to_string(), "CO2".to_string(), "NH3".to_string(), "NaOH".to_string(), "NaCl".to_string(), "KCl".to_string(), "CaO".to_string(), "MgO".to_string(),
        // Bonds and symbols
        "-".to_string(), "=".to_string(), "≡".to_string(), "+".to_string(), "*".to_string(), "→".to_string(), "⇌".to_string(), "⌬".to_string(),
    ]
}