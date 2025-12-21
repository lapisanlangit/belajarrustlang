struct Person {
    nama:String,
    umur : u8,
}
struct Koordinat(i32,i32,i32);


//struct dengan methode

impl Person{
    fn salam(&self) {
        println!("Halo saya {}",self.nama);
    }


    fn tambah_umur(&mut self){
        self.umur +=1;
    }

    fn baru(nama:String, umur:u8)-> Person {
        Person {nama, umur}
    }
}

fn main() {
    let mut orang=Person{
        nama:String::from("Umar Jati"),
        umur:25,
   };

    let titik=Koordinat(1,2,3);

    orang.umur=10;
    orang.nama=String::from("Andi Saputra");
    println!("nama {}",orang.nama);
    println!("umur {}",orang.umur);
    println!("Titik {}",titik.1);


    let mut p=Person::baru(String::from("Budi"),20);

    p.salam();
    p.tambah_umur();

    println!("Nilai umur {}",orang.umur);
}


