enum StatusPesanan {
    Dipesan,
    Diproses,
    Dikirim,
    Diterima,
    Dibatalkan,
}

struct Pesanan {
    id: u32,
    nama_produk: String,
    harga: f64,
    status: StatusPesanan,
}

impl Pesanan {
    fn update_status(&mut self, status_baru: StatusPesanan) {
        self.status = status_baru;
    }

    fn info(&self) {
        let status_text = match self.status {
            StatusPesanan::Dipesan => "Dipesan",
            StatusPesanan::Diproses => "Diproses",
            StatusPesanan::Dikirim => "Dikirim",
            StatusPesanan::Diterima => "Diterima",
            StatusPesanan::Dibatalkan => "Dibatalkan",
        };

        println!(
            "Pesanan #{}: {} - Rp{} - Status: {}",
            self.id, self.nama_produk, self.harga, status_text
        );
    }
}

fn main() {
    let mut pesanan1 = Pesanan {
        id: 1,
        nama_produk: String::from("Laptop"),
        harga: 10_000_000.0,
        status: StatusPesanan::Dipesan,
    };

    pesanan1.info();
    pesanan1.update_status(StatusPesanan::Diproses);
    pesanan1.info();
}
