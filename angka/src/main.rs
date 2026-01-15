use crate::angka::tambah;
mod angka;
mod matematika;
use matematika::hitung::kurang; // Import fungsi
fn main() {
    let mut x = 10;
    x = 20;
    println!("cetak ini {}", x + 10);
    let hasil = tambah(1, 2);
    println!("nilai tambah {}", hasil);

    let a = 30;
    let b = 10;

    let hasilhitung = kurang(a, b);
    println!("hasilnya {}", hasilhitung)

    // let hasilhi
}
