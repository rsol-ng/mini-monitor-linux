//! Mengumpulkan semua data jadi satu laporan.

use std::path::Path;

use crate::baterai::{persen_baterai, status_baterai, status_charger};
use crate::konstanta::{AC, BAT0, THERMAL};
use crate::model::Laporan;
use crate::suhu::daftar_suhu;
use crate::waktu::waktu_sekarang;

pub fn kumpulkan_laporan() -> Laporan {
    Laporan {
        waktu: waktu_sekarang(),
        persen: persen_baterai(Path::new(BAT0)),
        status: status_baterai(Path::new(BAT0)),
        charger: status_charger(Path::new(AC)),
        suhu: daftar_suhu(Path::new(THERMAL)),
    }
}
