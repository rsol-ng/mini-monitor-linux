/// Satu laporan lengkap hasil pengumpulan data.
pub struct Laporan {
    pub waktu: String,
    pub persen: Option<i64>,
    pub status: String,
    pub charger: Charger,
    pub suhu: Vec<(String, f64)>,
}

/// Status charger: tiga kemungkinan, bukan `Option<bool>` yang ambigu.
#[derive(Clone, Copy)]
pub enum Charger {
    Terpasang,
    TidakTerpasang,
    TidakDiketahui,
}
