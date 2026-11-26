use mini_monitor::ui::layout::LEBAR;
use mini_monitor::ui::teks::{baris, label, lebar_terlihat};

#[test]
fn lebar_terlihat_abaikan_ansi() {
    assert_eq!(lebar_terlihat(""), 0);
    assert_eq!(lebar_terlihat("abc"), 3);
    assert_eq!(lebar_terlihat("\x1b[31mabc\x1b[0m"), 3);
    assert_eq!(lebar_terlihat("\x1b[1;36mr3zA0x\x1b[0m"), 6);
}

#[test]
fn baris_padding_sampai_lebar_kotak() {
    let b = baris("abc");
    assert!(b.starts_with("│ abc"));
    assert_eq!(lebar_terlihat(&b), LEBAR);

    // Teks lebih panjang dari isi → saturating, tidak panic.
    let panjang = baris(&"x".repeat(80));
    assert!(panjang.contains("xxxxxxxx"));
    assert!(lebar_terlihat(&panjang) > LEBAR);
}

#[test]
fn label_rata_kiri_8_karakter() {
    assert_eq!(lebar_terlihat(&label("Baterai")), 8);
    assert_eq!(lebar_terlihat(&label("Suhu")), 8);
    assert!(label("Suhu").contains("Suhu"));
}
