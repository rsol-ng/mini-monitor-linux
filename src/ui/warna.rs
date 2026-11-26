//! Palet warna ANSI dan keputusan apakah warna dipakai.

use std::io::IsTerminal;

// ===== Palet warna ANSI (hanya hiasan) =======================
pub const RESET: &str = "\x1b[0m";
pub const TEBAL_CYAN: &str = "\x1b[1;36m";
pub const REDUP: &str = "\x1b[2m";
pub const MERAH: &str = "\x1b[31m";
pub const HIJAU: &str = "\x1b[32m";
pub const KUNING: &str = "\x1b[33m";

/// Warna hanya dipakai kalau output ke terminal dan NO_COLOR tidak di-set.
pub fn pakai_warna() -> bool {
    std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

/// Bungkus teks dengan kode warna; balik apa adanya kalau warna dimatikan.
pub fn warna(kode: &str, teks: &str) -> String {
    if pakai_warna() {
        format!("{kode}{teks}{RESET}")
    } else {
        teks.to_string()
    }
}
