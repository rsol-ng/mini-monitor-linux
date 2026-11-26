//! Tanggal dan jam lokal.

use chrono::{Datelike, Local, Timelike};

/// Tanggal dan jam lokal, contoh: Jum, 30 Okt 2026 · 14:32:07
pub fn waktu_sekarang() -> String {
    let now = Local::now();
    let hari = ["Min", "Sen", "Sel", "Rab", "Kam", "Jum", "Sab"];
    let bulan = [
        "Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des",
    ];

    format!(
        "{}, {:02} {} {} · {:02}:{:02}:{:02}",
        hari[now.weekday().num_days_from_sunday() as usize],
        now.day(),
        bulan[now.month0() as usize],
        now.year(),
        now.hour(),
        now.minute(),
        now.second()
    )
}
