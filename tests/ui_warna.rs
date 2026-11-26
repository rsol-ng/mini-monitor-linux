use std::io::IsTerminal;

use mini_monitor::ui::warna::{MERAH, RESET, pakai_warna, warna};

#[test]
fn pakai_warna_mengikuti_terminal_dan_no_color() {
    let di_terminal = std::io::stdout().is_terminal();
    let no_color = std::env::var_os("NO_COLOR").is_some();
    assert_eq!(pakai_warna(), di_terminal && !no_color);
}

#[test]
fn warna_mengikuti_pakai_warna() {
    let hasil = warna(MERAH, "halo");
    if pakai_warna() {
        assert_eq!(hasil, format!("{MERAH}halo{RESET}"));
    } else {
        assert_eq!(hasil, "halo");
    }
}
