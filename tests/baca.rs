mod common;

use common::{tmp, tulis};
use mini_monitor::baca::{baca_angka, baca_file};

#[test]
fn baca_file_trim_dan_missing() {
    let d = tmp("baca_file");
    let f = d.join("data");
    tulis(&f, "  halo\n");
    assert_eq!(baca_file(&f).as_deref(), Some("halo"));
    assert_eq!(baca_file(d.join("tidak_ada")).as_deref(), None);
}

#[test]
fn baca_angka_parse() {
    let d = tmp("angka");
    let f = d.join("n");
    tulis(&f, "42\n");
    assert_eq!(baca_angka(&f), Some(42));
    tulis(&f, "bukan angka");
    assert_eq!(baca_angka(&f), None);
    assert_eq!(baca_angka(d.join("tidak_ada")), None);
}
