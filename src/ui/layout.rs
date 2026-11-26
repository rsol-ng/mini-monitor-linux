//! Ukuran kotak dan penempatan di terminal.

use std::io::IsTerminal;

use terminal_size::{Height, Width, terminal_size};

pub const LEBAR: usize = 50; // lebar total kotak
pub const ISI: usize = LEBAR - 4; // ruang teks di dalam border

/// Ukuran terminal (kolom, baris); None kalau output bukan terminal.
pub fn ukuran_terminal() -> Option<(usize, usize)> {
    let (Width(kolom), Height(baris)) = terminal_size()?;
    Some((kolom as usize, baris as usize))
}

/// Bersihkan layar untuk mode watch. Hanya kalau output ke terminal.
pub fn bersihkan_layar() {
    if std::io::stdout().is_terminal() {
        print!("\x1b[2J\x1b[H");
    }
}
