use mini_monitor::ui::garis::{garis_atas, garis_bawah, garis_tengah};
use mini_monitor::ui::layout::LEBAR;
use mini_monitor::ui::teks::lebar_terlihat;

#[test]
fn garis_lebar_kotak() {
    for g in [garis_atas(), garis_tengah(), garis_bawah()] {
        assert_eq!(lebar_terlihat(&g), LEBAR);
    }
    assert!(garis_atas().contains("r3zA0x"));
    assert!(garis_atas().starts_with("╭─"));
    assert!(garis_tengah().starts_with("├"));
    assert!(garis_bawah().starts_with("╰"));
}
