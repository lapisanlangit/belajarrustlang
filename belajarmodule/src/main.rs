mod matematika;
use matematika::tambah;
mod kurang;
fn main() {
    let hasil = tambah::fntambah(1, 2);
    println!("hasilnya {}", hasil);
    let hasil2 = kurang::kurang(1, 2);
    println!("hasilkurang {}", hasil2);
    tambah::tampilkan_hasil();
}
