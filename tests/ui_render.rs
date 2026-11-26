use mini_monitor::model::{Charger, Laporan};
use mini_monitor::ui::{bangun_kotak, tampilkan};

fn laporan_contoh() -> Laporan {
    Laporan {
        waktu: "Sab, 30 Okt 2026 · 14:32:07".to_string(),
        persen: Some(62),
        status: "mengosongkan".to_string(),
        charger: Charger::TidakTerpasang,
        suhu: vec![("acpitz".to_string(), 46.0)],
    }
}

#[test]
fn bangun_kotak_isinya_lengkap() {
    let k = bangun_kotak(&laporan_contoh(), false);
    assert_eq!(k.len(), 9);
    for potongan in [
        "r3zA0x",
        "14:32:07",
        "62%",
        "mengosongkan",
        "tidak terpasang",
        "acpitz 46.0°C",
    ] {
        assert!(k.iter().any(|b| b.contains(potongan)), "hilang: {potongan}");
    }
    assert!(k.iter().any(|b| b.contains('╭')));
    assert!(k.iter().any(|b| b.contains('╯')));
}

#[test]
fn bangun_kotak_mode_watch_dan_kasus_kosong() {
    let kw = bangun_kotak(&laporan_contoh(), true);
    assert_eq!(kw.len(), 10);
    assert!(kw.last().unwrap().contains("refresh tiap 2 detik"));

    let mut l = laporan_contoh();
    l.suhu.clear();
    let k = bangun_kotak(&l, false);
    assert!(k.iter().any(|b| b.contains("tidak terbaca")));

    l.persen = None;
    let k = bangun_kotak(&l, false);
    assert!(k.iter().any(|b| b.contains("?")));
}

#[test]
fn bangun_kotak_semua_status_charger() {
    for (charger, teks) in [
        (Charger::Terpasang, "terpasang"),
        (Charger::TidakTerpasang, "tidak terpasang"),
        (Charger::TidakDiketahui, "tidak diketahui"),
    ] {
        let mut l = laporan_contoh();
        l.charger = charger;
        let k = bangun_kotak(&l, false);
        assert!(k.iter().any(|b| b.contains(teks)), "charger {teks}");
    }
}

#[test]
fn tampilkan_tidak_panic() {
    tampilkan(&laporan_contoh(), false);
    tampilkan(&laporan_contoh(), true);
}
