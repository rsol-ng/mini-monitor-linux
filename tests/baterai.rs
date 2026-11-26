mod common;

use std::fs;

use common::{tmp, tulis};
use mini_monitor::baterai::{persen_baterai, status_baterai, status_charger};
use mini_monitor::model::Charger;

#[test]
fn persen_baterai_hitung_dan_guard() {
    let d = tmp("persen");
    tulis(&d.join("charge_now"), "50\n");
    tulis(&d.join("charge_full"), "100\n");
    assert_eq!(persen_baterai(&d), Some(50));

    tulis(&d.join("charge_full"), "200\n");
    assert_eq!(persen_baterai(&d), Some(25)); // floor division: 50*100/200

    tulis(&d.join("charge_now"), "1\n");
    tulis(&d.join("charge_full"), "3\n");
    assert_eq!(persen_baterai(&d), Some(33));

    tulis(&d.join("charge_full"), "0\n");
    assert_eq!(persen_baterai(&d), None);

    let _ = fs::remove_file(d.join("charge_now"));
    assert_eq!(persen_baterai(&d), None);
}

#[test]
fn status_charger_tiga_kasus() {
    let d = tmp("charger");
    tulis(&d.join("online"), "1\n");
    assert!(matches!(status_charger(&d), Charger::Terpasang));
    tulis(&d.join("online"), "0\n");
    assert!(matches!(status_charger(&d), Charger::TidakTerpasang));
    tulis(&d.join("online"), "2\n");
    assert!(matches!(status_charger(&d), Charger::TidakDiketahui));
    let _ = fs::remove_file(d.join("online"));
    assert!(matches!(status_charger(&d), Charger::TidakDiketahui));
}

#[test]
fn status_baterai_terjemahan() {
    let d = tmp("status");
    for (mentah, harapan) in [
        ("Charging", "mengisi"),
        ("Discharging", "mengosongkan"),
        ("Full", "penuh"),
        ("Not charging", "tidak mengisi"),
        ("Custom", "custom"),
    ] {
        tulis(&d.join("status"), mentah);
        assert_eq!(status_baterai(&d), harapan);
    }
    let _ = fs::remove_file(d.join("status"));
    assert_eq!(status_baterai(&d), "tidak diketahui");
}
