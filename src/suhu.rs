//! Data suhu dari thermal zone.

use std::fs;
use std::path::Path;

use crate::baca::{baca_angka, baca_file};

/// Baca semua thermal zone, hasilnya (nama, suhu dalam °C).
pub fn daftar_suhu(base: &Path) -> Vec<(String, f64)> {
    let mut hasil = Vec::new();

    let Ok(entries) = fs::read_dir(base) else {
        return hasil;
    };

    for entry in entries.flatten() {
        let nama_file = entry.file_name().to_string_lossy().to_string();
        if !nama_file.starts_with("thermal_zone") {
            continue;
        }

        let path = entry.path();
        let nama = baca_file(path.join("type")).unwrap_or(nama_file);

        // Nilai di /sys satuannya milli-Celsius, jadi 46000 = 46.0 °C.
        let Some(mili) = baca_angka(path.join("temp")) else {
            continue;
        };
        hasil.push((nama, mili as f64 / 1000.0));
    }

    hasil.sort_by(|a, b| a.0.cmp(&b.0));
    hasil
}
