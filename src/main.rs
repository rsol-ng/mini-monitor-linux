//! Entry point binary `mmon`. Semua logic ada di pustaka `mini_monitor`.

use std::thread::sleep;
use std::time::Duration;

use mini_monitor::laporan::kumpulkan_laporan;
use mini_monitor::ui::{bersihkan_layar, tampilkan};

fn main() {
    // Tanpa argumen: tampil sekali. `watch`: refresh tiap 2 detik.
    match std::env::args().nth(1).as_deref() {
        None => tampilkan(&kumpulkan_laporan(), false),
        Some("watch") => loop {
            bersihkan_layar();
            tampilkan(&kumpulkan_laporan(), true);
            sleep(Duration::from_secs(2));
        },
        Some(lain) => {
            eprintln!("argumen tidak dikenal: {lain}");
            eprintln!("pakai: mmon [jomok]");
            std::process::exit(2);
        }
    }
}
