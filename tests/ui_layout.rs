use mini_monitor::ui::{bersihkan_layar, layout::ukuran_terminal};

#[test]
fn ukuran_terminal_wajar() {
    if let Some((kolom, baris)) = ukuran_terminal() {
        assert!(kolom > 0 && baris > 0);
    }
}

#[test]
fn bersihkan_layar_tidak_panic() {
    bersihkan_layar();
}
