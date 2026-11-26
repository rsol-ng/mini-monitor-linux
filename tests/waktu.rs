use mini_monitor::waktu::waktu_sekarang;

#[test]
fn waktu_sekarang_format() {
    let w = waktu_sekarang();
    let (tanggal, jam) = w.split_once(" · ").expect("pemisah tengah ada");
    assert_eq!(jam.len(), 8); // HH:MM:SS
    let mut bagian_jam = jam.split(':');
    assert_eq!(bagian_jam.next().unwrap().len(), 2);
    assert_eq!(bagian_jam.next().unwrap().len(), 2);
    assert_eq!(bagian_jam.next().unwrap().len(), 2);
    assert!(bagian_jam.next().is_none());

    let bagian: Vec<&str> = tanggal.split(", ").collect();
    assert_eq!(bagian.len(), 2);
    assert!(["Min", "Sen", "Sel", "Rab", "Kam", "Jum", "Sab"].contains(&bagian[0]));
    let detail: Vec<&str> = bagian[1].split_whitespace().collect();
    assert_eq!(detail.len(), 3); // dd Mmm yyyy
    assert_eq!(detail[1].len(), 3); // Jan..Des
}
