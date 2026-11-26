//! Pembaca file `/sys` yang gagal dengan aman (balik `None`).

use std::fs;
use std::path::Path;

pub fn baca_file(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|isi| isi.trim().to_string())
}

pub fn baca_angka(path: impl AsRef<Path>) -> Option<i64> {
    baca_file(path)?.parse().ok()
}
