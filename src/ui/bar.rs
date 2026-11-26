//! Bar baterai dan pewarnaan suhu (hiasan, bukan pengganti angka).

use crate::ui::warna::{HIJAU, KUNING, MERAH, REDUP, warna};

pub fn warna_baterai(persen: i64) -> &'static str {
    // Range pattern: ambang batas jadi satu tempat, urut, dan eksklusif.
    match persen {
        ..=19 => MERAH,
        20..=59 => KUNING,
        _ => HIJAU,
    }
}

pub fn bar_baterai(persen: Option<i64>) -> String {
    let Some(persen) = persen else {
        return warna(REDUP, "░░░░░░░░░░");
    };

    // 1 blok mewakili 10%, dibulatkan ke blok terdekat.
    let penuh = ((persen.clamp(0, 100) + 5) / 10) as usize;
    let terisi = "█".repeat(penuh);
    let kosong = "░".repeat(10 - penuh);

    format!(
        "{}{}",
        warna(warna_baterai(persen), &terisi),
        warna(REDUP, &kosong)
    )
}

pub fn warna_suhu(celsius: f64) -> &'static str {
    // f64 nggak bisa jadi pattern, jadi pakai guard. NaN otomatis jatuh ke MERAH.
    match celsius {
        suhu if suhu < 60.0 => HIJAU,
        suhu if suhu < 75.0 => KUNING,
        _ => MERAH,
    }
}
