use mini_monitor::laporan::kumpulkan_laporan;

#[test]
fn kumpulkan_laporan_tidak_panic() {
    let l = kumpulkan_laporan();
    assert!(!l.waktu.is_empty());
    assert!(!l.status.is_empty());
}
