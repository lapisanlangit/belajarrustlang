fn main() {
    let umur = 17;

    if umur >= 18 {
        println!("Dewasa");
    } else if umur >= 13 {
        println!("Remaja");
    } else {
        println!("anak-anak");
    }

    let status = if umur >= 18 {
        " Dewasa"
    } else {
        " Belum Dewasa"
    };
    println!("Status {}", status);

    let mut angka = 5;

    while angka > 0 {
        println!("Countdown {}", angka);
        angka -= 1;
    }
    println!("Launch !");

    for i in 1..=10 {
        println!("Angka {}", i);
    }

    let nama = ["satu", "dua", "tiga"];
    println!("{}", nama[0]);
    for orang in nama {
        println!("Halo {}", orang);
    }
    //cara lain

    let nama_vec = vec!["satu", "dua", "tiga"]; // Infer Vec<&str>

    for data in &nama_vec {
        println!("tes vec {}", data);
    }

    let angka = 7;

    match angka {
        1 => println!("Satu"),
        2 | 3 => println!("Dua atau tiga"),
        4..=10 => println!("Antara 4 s.d. 10"),
        _ => println!("Lainnya"),
    }

    #[derive(Debug)]
    struct Pegawai {
        nip: String,
        nama: String,
    }

    let pegawai = vec![
        Pegawai {
            nip: "01".to_string(),
            nama: "lala".to_string(),
        },
        Pegawai {
            nip: "02".to_string(),
            nama: "lili".to_string(),
        },
    ];

    println!("{:#?}", pegawai);
    let hasilnya = pegawai.iter().find(|p| p.nip == "01");
    println!("hasilnya {:?}", hasilnya);
}
