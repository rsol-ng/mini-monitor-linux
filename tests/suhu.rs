mod common;

use std::fs;

use common::{tmp, tulis};
use mini_monitor::suhu::daftar_suhu;

#[test]
fn daftar_suhu_baca_sortir_dan_skip() {
    let d = tmp("suhu");
    let zona0 = d.join("thermal_zone0");
    let zona1 = d.join("thermal_zone1");
    fs::create_dir_all(&zona0).unwrap();
    fs::create_dir_all(&zona1).unwrap();
    tulis(&zona0.join("type"), "cpu_thermal\n");
    tulis(&zona0.join("temp"), "46000\n");
    tulis(&zona1.join("type"), "gpu\n");
    tulis(&zona1.join("temp"), "55000\n");

    // Direktori bukan thermal_zone harus diabaikan.
    let lain = d.join("misc");
    fs::create_dir_all(&lain).unwrap();
    tulis(&lain.join("type"), "x\n");
    tulis(&lain.join("temp"), "99999\n");

    // Zone tanpa file temp harus dilewati.
    let rusak = d.join("thermal_zone2");
    fs::create_dir_all(&rusak).unwrap();
    tulis(&rusak.join("type"), "rusak\n");

    assert_eq!(
        daftar_suhu(&d),
        vec![
            ("cpu_thermal".to_string(), 46.0),
            ("gpu".to_string(), 55.0),
        ]
    );
    assert!(daftar_suhu(&d.join("tidak_ada")).is_empty());
}
