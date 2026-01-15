// src/matematika/bagi.rs
pub fn bagi(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
pub fn tampilkan_pesan(msg: &str) -> &str {
    msg
}
