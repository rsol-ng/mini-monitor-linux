//! Data baterai dan charger.

use std::path::Path;

use crate::baca::{baca_angka, baca_file};
use crate::model::Charger;

pub fn persen_baterai(base: &Path) -> Option<i64> {
    let sekarang = baca_angka(base.join("charge_now"))?;
    let penuh = baca_angka(base.join("charge_full"))?;

    // Range pattern bikin kasus "charge_full 0 / negatif" eksplisit.
    match penuh {
        1.. => Some(sekarang * 100 / penuh),
        _ => None,
    }
}

/// Status charger dari file "online" (0/1). Nilai lain = tidak diketahui.
pub fn status_charger(base: &Path) -> Charger {
    match baca_angka(base.join("online")) {
        Some(1) => Charger::Terpasang,
        Some(0) => Charger::TidakTerpasang,
        _ => Charger::TidakDiketahui,
    }
}

/// Status dari kernel diterjemahkan ke bahasa Indonesia.
pub fn status_baterai(base: &Path) -> String {
    match baca_file(base.join("status")).as_deref() {
        Some("Charging") => "mengisi".to_string(),
        Some("Discharging") => "mengosongkan".to_string(),
        Some("Full") => "penuh".to_string(),
        Some("Not charging") => "tidak mengisi".to_string(),
        Some(lain) => lain.to_lowercase(),
        None => "tidak diketahui".to_string(),
    }
}
