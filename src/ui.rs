//! ┌──────────────────────────────────────────────────────────┐
//! │  MODUL UI / HIASAN — DI LUAR SCOPE PRD                   │
//! │                                                          │
//! │  Semua kode tampilan dipecah per sub-modul di `src/ui/`. │
//! │  Logic baca sensor & waktu tetap di pustaka luar `ui`.   │
//! │  File/folder ini boleh dihapus tanpa mengubah data.      │
//! │  Warna otomatis mati kalau output dipipe / NO_COLOR=1.   │
//! └──────────────────────────────────────────────────────────┘

pub mod bar;
pub mod garis;
pub mod layout;
pub mod render;
pub mod teks;
pub mod warna;

pub use layout::bersihkan_layar;
pub use render::{bangun_kotak, tampilkan};
