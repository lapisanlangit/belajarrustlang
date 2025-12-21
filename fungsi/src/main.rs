fn main() {
    salam();
    let nama="Umar";
    salam_ke(nama);
    let hasil=tambah(3,4);
    println!("Hasil {}",hasil);
}


fn salam() {
    println!("Halo dari Rust");
}


fn salam_ke(nama:&str){
    println!("Halo, {}",nama);

}


fn tambah(a:i32,b:i32)-> i32{
    return a+b;
}
