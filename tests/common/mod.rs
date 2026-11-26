//! Helper bersama untuk seluruh integration test.
//! Tiap test crate pakai subset berbeda, jadi helper yang tak dipakai diizinkan.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// Direktori sementara unik per test (pakai PID biar nggak tabrakan).
pub fn tmp(nama: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("mmon-test-{nama}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

pub fn tulis(path: &Path, isi: &str) {
    fs::write(path, isi).unwrap();
}

/// Hitung kemunculan karakter, kebal terhadap kode warna ANSI.
pub fn blok(teks: &str, c: char) -> usize {
    teks.chars().filter(|x| *x == c).count()
}
