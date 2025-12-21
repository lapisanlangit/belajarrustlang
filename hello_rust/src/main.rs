fn main() {
    let mobilku = String::from("🚗 Mobil Merah Keren");  // Kamu punya mobil (owner)

    let mobil_adik = mobilku;  // Kamu kasih SELAMANYA ke adik → ownership pindah!

    // println!("{}", mobilku);  // ERROR! Kamu sudah ga punya lagi!
    println!("Adik sekarang punya: {}", mobil_adik);  // OK
}  // mobil_adik keluar scope → mobil otomatis "dibersihkan" (drop)

fn main() {
    let mobilku = String::from("🚙 Mobil Biru");

    lihat_mobil(&mobilku);     // Pinjam buat dilihat
    lihat_mobil_lagi(&mobilku); // Boleh pinjam lagi, bahkan bareng-bareng!

    println!("Mobilku masih utuh: {}", mobilku);  // Masih punya kamu!
                                                  //
    //kalau ini pindah ownership
    lihat_mobil(mobilku);
}

fn lihat_mobil(mobil: &String) {
    println!("Wah, mobilnya bagus: {}", mobil);
    // mobil.push_str(" rusak");  // ERROR! Ga boleh diubah
}

fn lihat_mobil_lagi(mobil: &String) {
    println!("Lihat lagi: {}", mobil);
}


fn main() {
    let mut mobilku = String::from("🚕 Taksi Kuning");

    mainin_mobil(&mut mobilku);  // Pinjam buat dimainin (hanya 1 orang!)

    // lihat_mobil(&mobilku);    // ERROR kalau ditulis di sini!
    // Karena lagi dipinjam mutable, ga boleh dipinjam immutable sekaligus

    println!("Setelah dimainin: {}", mobilku);
}

fn mainin_mobil(mobil: &mut String) {
    mobil.push_str(" dengan stiker balap 🏁");  // Boleh diubah!
    println!("Sedang dimainin: {}", mobil);
}
