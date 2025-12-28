#[derive(Debug)]
enum Arah {
    Utara,
    Timur,
    Selatan,
    Barat,
}
// enum Pesan {
//     Berhenti,
//     maju(u32),
//     Belok(i8, i8),
//     Tulis(String),
// }
enum Pesan {
    Quit,
    Pindah { x: i32, y: i32 },
    Tulis(String),
}

fn proses_pesan(pesan: Pesan) {
    match pesan {
        Pesan::Quit => println!("Program berhenti"),
        Pesan::Pindah { x, y } => println!("Pindah ke ({}, {})", x, y),
        Pesan::Tulis(text) => println!("Pesan: {}", text),
    }
}

// fn main() {
//     let msg1 = Pesan::Tulis(String::from("Hello Rust!"));
//     let msg2 = Pesan::Pindah { x: 10, y: 20 };
//
//     proses_pesan(msg1);
//     proses_pesan(msg2);
// }
fn main() {
    let kemana = Arah::Timur;
    match kemana {
        Arah::Utara => println!("ke utara"),
        Arah::Timur => println!("ke timur"),
        Arah::Selatan => println!("ke selatan"),
        Arah::Barat => println!("ke barat"),
    }

    // let daftar_pesan = vec![
    //     Pesan::maju(10),
    //     Pesan::Belok(-5, 3),
    //     Pesan::Tulis(String::from("Halo Robot")),
    //     Pesan::Berhenti,
    // ];
    //
    // for p in daftar_pesan {
    //     match p {
    //         Pesan::Berhenti => println!("Robot berhenti"),
    //         Pesan::maju(kecepatan) => println!("Maju dengan kecepatan {}", kecepatan),
    //         Pesan::Belok(x, y) => println!("Belok ke ({}, {})", x, y),
    //         Pesan::Tulis(teks) => println!("Pesan teks: {}", teks),
    //     }
    // }
}
