mod angka;
use crate::angka::tambah;

fn main() {
    let mut x = 10;
    x = 20;
    println!("cetak ini {}", x + 10);
    let hasil = tambah(1, 2);
    println!("nilai tambah {}", hasil)
}
