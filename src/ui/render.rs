//! Merakit kotak dari laporan dan mencetaknya ke terminal.

use crate::model::{Charger, Laporan};
use crate::ui::bar::{bar_baterai, warna_baterai, warna_suhu};
use crate::ui::garis::{garis_atas, garis_bawah, garis_tengah};
use crate::ui::layout::{LEBAR, ukuran_terminal};
use crate::ui::teks::{baris, label};
use crate::ui::warna::{HIJAU, REDUP, TEBAL_CYAN, warna};

/// Susun baris-baris kotak dari laporan (dipisahkan supaya bisa diuji tanpa stdout).
pub fn bangun_kotak(laporan: &Laporan, mode_watch: bool) -> Vec<String> {
    let mut kotak: Vec<String> = Vec::new();

    // -- Header: tanggal & jam ---------------------------------
    kotak.push(garis_atas());
    kotak.push(baris(&format!("  {}", warna(TEBAL_CYAN, &laporan.waktu))));
    kotak.push(baris(""));
    kotak.push(garis_tengah());

    // -- Baterai ------------------------------------------------
    let persen_teks = match laporan.persen {
        Some(p) => warna(warna_baterai(p), &format!("{p:>3}%")),
        None => warna(REDUP, "  ?%"),
    };

    kotak.push(baris(&format!(
        "  {}{} {}  {}",
        label("Baterai"),
        bar_baterai(laporan.persen),
        persen_teks,
        warna(REDUP, &format!("({})", laporan.status)),
    )));

    let charger_teks = match laporan.charger {
        Charger::Terpasang => warna(HIJAU, "terpasang"),
        Charger::TidakTerpasang => warna(REDUP, "tidak terpasang"),
        Charger::TidakDiketahui => warna(REDUP, "tidak diketahui"),
    };

    kotak.push(baris(&format!("  {}{}", label("Charger"), charger_teks)));

    // -- Suhu ---------------------------------------------------
    kotak.push(garis_tengah());

    match laporan.suhu.as_slice() {
        [] => kotak.push(baris(&format!(
            "  {}{}",
            label("Suhu"),
            warna(REDUP, "tidak terbaca")
        ))),
        suhu => {
            for (i, (nama, celsius)) in suhu.iter().enumerate() {
                let teks_label = match i {
                    0 => "Suhu",
                    _ => "",
                };
                let nilai = warna(warna_suhu(*celsius), &format!("{nama} {celsius:.1}°C"));
                kotak.push(baris(&format!("  {}{}", label(teks_label), nilai)));
            }
        }
    }

    kotak.push(garis_bawah());

    // -- Hint di luar kotak, khusus mode watch ------------------
    if mode_watch {
        kotak.push(warna(REDUP, "  refresh tiap 2 detik · Ctrl+C untuk keluar"));
    }

    kotak
}

/// Render satu laporan lengkap, di tengah layar.
pub fn tampilkan(laporan: &Laporan, mode_watch: bool) {
    let kotak = bangun_kotak(laporan, mode_watch);

    // -- Tempel di tengah terminal ------------------------------
    // Kalau output dipipe (bukan terminal), ukuran tak diketahui → tetap
    // di pojok kiri atas supaya tidak ada spasi/baris kosong aneh.
    let (kolom, tinggi) = ukuran_terminal().unwrap_or((0, 0));
    let kiri = kolom.saturating_sub(LEBAR) / 2;
    let atas = tinggi.saturating_sub(kotak.len()) / 2;

    for _ in 0..atas {
        println!();
    }

    let spasi = " ".repeat(kiri);
    for g in &kotak {
        println!("{spasi}{g}");
    }
}
