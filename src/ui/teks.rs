//! Baris teks di dalam kotak.

use crate::ui::layout::ISI;
use crate::ui::warna::{REDUP, warna};

/// Hitung lebar teks yang benar-benar terlihat (abaikan kode ANSI).
pub fn lebar_terlihat(teks: &str) -> usize {
    let mut lebar = 0;
    let mut dalam_escape = false;

    for c in teks.chars() {
        match c {
            '\x1b' => dalam_escape = true,
            'm' if dalam_escape => dalam_escape = false,
            _ if !dalam_escape => lebar += 1,
            _ => {}
        }
    }

    lebar
}

/// Satu baris isi di dalam kotak, padding kanan biar border rata.
pub fn baris(isi: &str) -> String {
    let sisa = ISI.saturating_sub(lebar_terlihat(isi));
    format!("│ {isi}{} │", " ".repeat(sisa))
}

/// Label kolom kiri (Baterai/Charger/Suhu) dengan warna redup.
pub fn label(teks: &str) -> String {
    warna(REDUP, &format!("{teks:<8}"))
}
