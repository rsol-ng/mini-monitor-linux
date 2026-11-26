mod common;

use common::blok;
use mini_monitor::ui::bar::{bar_baterai, warna_baterai, warna_suhu};
use mini_monitor::ui::warna::{HIJAU, KUNING, MERAH};

#[test]
fn warna_baterai_ambang() {
    assert_eq!(warna_baterai(0), MERAH);
    assert_eq!(warna_baterai(19), MERAH);
    assert_eq!(warna_baterai(20), KUNING);
    assert_eq!(warna_baterai(59), KUNING);
    assert_eq!(warna_baterai(60), HIJAU);
    assert_eq!(warna_baterai(100), HIJAU);
    assert_eq!(warna_baterai(-1), MERAH);
}

#[test]
fn bar_baterai_blok() {
    assert_eq!(blok(&bar_baterai(None), '░'), 10);
    assert_eq!(blok(&bar_baterai(Some(0)), '░'), 10);

    let b50 = bar_baterai(Some(50));
    assert_eq!(blok(&b50, '█'), 5);
    assert_eq!(blok(&b50, '░'), 5);

    assert_eq!(blok(&bar_baterai(Some(100)), '█'), 10);
    assert_eq!(blok(&bar_baterai(Some(150)), '█'), 10); // clamp 100
    assert_eq!(blok(&bar_baterai(Some(-10)), '░'), 10); // clamp 0
}

#[test]
fn warna_suhu_ambang_dan_nan() {
    assert_eq!(warna_suhu(20.0), HIJAU);
    assert_eq!(warna_suhu(59.9), HIJAU);
    assert_eq!(warna_suhu(60.0), KUNING);
    assert_eq!(warna_suhu(74.9), KUNING);
    assert_eq!(warna_suhu(75.0), MERAH);
    assert_eq!(warna_suhu(f64::NAN), MERAH);
}
