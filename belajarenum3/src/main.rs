// Tambahkan derive Debug di enum juga!
#[derive(Debug)]
enum JenisGaji {
    Bulanan { gaji_pokok: f64, tunjangan: f64 },
    Lembur { jam_lembur: u32, tarif_per_jam: f64 },
    Harian { hari_kerja: u32, upah_harian: f64 },
}

#[derive(Debug)]
struct Pegawai {
    nama: String,
    jenis_gaji: JenisGaji,
}

impl Pegawai {
    fn hitung_gaji(&self) -> f64 {
        match &self.jenis_gaji {
            JenisGaji::Bulanan {
                gaji_pokok,
                tunjangan,
            } => gaji_pokok + tunjangan,
            JenisGaji::Lembur {
                jam_lembur,
                tarif_per_jam,
            } => {
                // Dereference dulu (*jam_lembur), lalu cast ke f64
                (*jam_lembur as f64) * tarif_per_jam
                // atau lebih idiomatic:
                // f64::from(*jam_lembur) * tarif_per_jam
            }
            JenisGaji::Harian {
                hari_kerja,
                upah_harian,
            } => {
                (*hari_kerja as f64) * upah_harian
                // atau: f64::from(*hari_kerja) * upah_harian
            }
        }
    }

    fn deskripsi(&self) -> String {
        match &self.jenis_gaji {
            JenisGaji::Bulanan { .. } => "Pegawai tetap (bulanan)".to_string(),
            JenisGaji::Lembur { .. } => "Pegawai lembur".to_string(),
            JenisGaji::Harian { .. } => "Pegawai harian/lepas".to_string(),
        }
    }
}

fn main() {
    let pegawai = vec![
        Pegawai {
            nama: "Andi".to_string(),
            jenis_gaji: JenisGaji::Bulanan {
                gaji_pokok: 5_000_000.0,
                tunjangan: 1_000_000.0,
            },
        },
        Pegawai {
            nama: "Budi".to_string(),
            jenis_gaji: JenisGaji::Lembur {
                jam_lembur: 50,
                tarif_per_jam: 75_000.0,
            },
        },
        Pegawai {
            nama: "Cici".to_string(),
            jenis_gaji: JenisGaji::Harian {
                hari_kerja: 22,
                upah_harian: 150_000.0,
            },
        },
    ];

    println!("=== Slip Gaji Bulan Ini ===\n");
    for p in &pegawai {
        println!("Nama      : {}", p.nama);
        println!("Status    : {}", p.deskripsi());
        println!("Total Gaji: Rp {:.0}\n", p.hitung_gaji());
    }

    // Bonus: bisa cetak seluruh struct dengan {:?} karena sudah Debug
    println!("Debug seluruh data: {:?}", pegawai);
}
