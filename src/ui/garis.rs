//! Border kotak (atas, tengah, bawah).

use crate::ui::layout::LEBAR;
use crate::ui::warna::{TEBAL_CYAN, warna};

pub fn garis_atas() -> String {
    let judul = " r3zA0x ";
    let sisa = LEBAR - 3 - judul.chars().count();
    format!("╭─{}{}╮", warna(TEBAL_CYAN, judul), "─".repeat(sisa))
}

pub fn garis_tengah() -> String {
    format!("├{}┤", "─".repeat(LEBAR - 2))
}

pub fn garis_bawah() -> String {
    format!("╰{}╯", "─".repeat(LEBAR - 2))
}
