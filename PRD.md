# PRD — mini-monitor

## 1. Gambaran

CLI sederhana untuk laptop Linux: nampilin **suhu**, **kondisi baterai**, dan **tanggal/jam saat ini** dalam satu layar printable.
Disengaja gampang — ini proyek belajar Rust pertama (phase 2 setelah hello world), bukan produk.

- **Nama binary:** `mmon`
- **Bahasa:** Rust (edition 2021+)
- **Target OS:** Linux saja (baca `/sys`)
- **Dep eksternal: 1 buah saja** → `chrono` (jam lokal)

## 2. Masalah

Mau tahu kondisi laptop (panas atau nggak, baterai tersisa berapa, charger nyambung nggak) tanpa buka `sensors` + `upower` satu-satu.

## 3. Fitur (MVP)

| # | Fitur | Sumber data | Prioritas |
|---|-------|-------------|-----------|
| F1 | Tampilkan tanggal & jam lokal | `chrono` | Wajib |
| F2 | Baterai: %, status (Charging/Discharging/Full), AC terpasang atau tidak | `/sys/class/power_supply/BAT0/…`, `ADP1` | Wajib |
| F3 | Suhu: semua thermal zone dengan namanya | `/sys/class/thermal/thermal_zone*/{type,temp}` | Wajib |
| F4 | Refresh otomatis tiap 2 detik (sampai Ctrl+C) | loop + `std::thread::sleep` | Opsional |

Tidak ada flag CLI khusus. Tanpa argumen = tampil sekali. Argumen `watch` = loop F4. Selesai.

## 4. Contoh output

```
mini-monitor — Sab 30 Okt 2026 14:32:07 WIB

Baterai : 62% (Discharging) | Charger: tidak terpasang
Suhu    : acpitz 46.0°C
```

## 5. Spesifikasi teknis (minimum)

- Baca file pakai `std::fs::read_to_string`, trim trailing newline, parse pakai `str::parse::<i64>()`.
  - Baterai: `percent = charge_now / charge_full * 100` (nilai dalam µAh, integer division cukup).
  - Suhu: nilai `/sys` dalam milli-Celsius → bagi 1000, tampil 1 desimal.
- Barcode / warna / ANSI: **tidak**. Plain text.
- Error: kalau file `/sys` tidak ada (misal laptop nggak punya baterai), tampil `?` atau skip — jangan panic, jangan bikin sistem error handling yang kompleks. Cukup fungsi balik `Option<i64>`.

## 6. Non-goals (dilarang keras, biar nggak over-engineering)

- TUI / GUI / chart
- Warna, emoji, atau fancy output
- Config file
- Cross-platform (Windows/macOS)
- Deteksi disk usage, CPU load, memory — bisa jadi fase 4 belakangan kalau terjamah belajar

## 7. Rencana latihan (urut, jangan lompat)

1. `cargo new mini-monitor` → tampilkan jam lokal pakai `chrono`. ✅ milestone: compile & print waktu
2. Fungsi `baca_baterai()`: baca baterai dari `/sys`, print persen. Belajar: `Option`, `?`, integer division
3. Fungsi `baca_suhu()`: baca semua thermal zone. Belajar: loop directory (`std::fs::read_dir`), `String` vs `&str`
4. Rangkai di `main()`, output sesuai contoh §4. Belajar: struct kecil (`Baterai`, `Suhu`) + `impl Display` (kalau ribet, langsung pakai `{:?}` juga boleh)
5. (Opsional) mode `watch`: loop 2 detik + `print!("\x1b[2J\x1b[H")` untuk clear screen. Belajar: `std::thread::sleep`, `std::env::args`

## 8. Definisi selesai

- `cargo run` → output sesuai §4
- Angka baterai/suhu cocok dengan `upower -i /org/freedesktop/UPower/devices/battery_BAT0` dan `sensors`
- Dicabut charger / psensor nggak ada → program tidak panic
- `cargo clippy` bebas warning mayor

## 9. Addendum — Modul UI (di luar scope, atas permintaan)

Ditambahkan setelah MVP jadi: tampilan dibikin lebih menarik (kotak, warna, bar baterai).

- Nama tampilan di title bar: **r3zA0x** (nama package tetap `mini-monitor`).
- Kotak diposisikan di tengah terminal (horizontal & vertikal), dihitung dari ukuran terminal lewat crate `terminal_size`; kalau output dipipe, kotak tetap di pojok kiri atas.
- Semua kode hiasan ada di **`src/ui.rs`**, ditandai jelas di header file dan saat `mod ui;` di `main.rs`.
- Ini sengaja melanggar non-goals §6 (warna/fancy output), tapi **terisolasi**: logic baca sensor & waktu tetap di `main.rs`, UI cuma menerima struct `Laporan` lalu merender.
- Warna otomatis nonaktif kalau output dipipe atau `NO_COLOR=1`; saat itu kotak tetap tampil polos.
- Menghapus `src/ui.rs` + `mod ui;` + panggilan `ui::` di `main.rs` = kembali persis ke output polos §4.
- UI ini bukan bagian penilaian "selesai" §8 — kalau ada bug tampilan, data tetap benar.

