use std::io;

fn main() {
    println!("==========================");
    println!("===== CONVERTER SUHU =====");
    println!("==========================");
    println!("");
    println!("Pilihan Menu:");
    println!("1. Fahrenheit ke Celcius");
    println!("2. Celsius ke Fahrenheit");
    println!("3. Celcius ke Kelvin");
    println!("4. Kelvin ke Celcius");
    println!("5. Kelvin ke Fahrenheit");
    println!("6. Fahrenheit ke Kelvin");

    println!("Masukan Pilihan Kamu: ");

    let mut pilihan = String::new();

    io::stdin()
        .read_line(&mut pilihan)
        .expect("Gagal Mendapatakan Pilihan Menu!");

    let pilihan: usize = pilihan.trim().parse().expect("Pilihan Bukan Angka!");

    if pilihan == 1 {
        println!("=================================");
        println!("===== Fahrenheit ke Celcius =====");
        println!("=================================");
        println!("");

        println!("Masukan Nilai Fahrenheit: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Inputan Berupa Angka!");
        let input: f32 = input.trim().parse().expect("Harus Berupa Angka!");

        // rumus
        let hasil: f32 = (input - 32.0) / 1.8;
        println!("Hasil Dari {input} Fahrenheit Ke Celcius Adalah {hasil} Celcius.");
    } else if pilihan == 2 {
        println!("=================================");
        println!("===== Celsius ke Fahrenheit =====");
        println!("=================================");
        println!("");

        println!("Masukan Nilai Celcius: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Inputan Berupa Angka!");
        let input: f32 = input.trim().parse().expect("Harus Berupa Angka!");

        // rumus
        let hasil: f32 = (input * 1.8) + 32.0;
        println!("Hasil Dari {input} Celcius Ke Fahrenheit Adalah {hasil} Fahrenheit.");
    } else if pilihan == 3 {
        println!("=================================");
        println!("======= Celsius ke Kelvin =======");
        println!("=================================");
        println!("");

        println!("Masukan Nilai Celcius: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Inputan Berupa Angka!");
        let input: f32 = input.trim().parse().expect("Harus Berupa Angka!");

        // rumus
        let hasil: f32 = input + 273.15;
        println!("Hasil Dari {input} Celcius Ke Kelvin Adalah {hasil} Kelvin.");
    } else if pilihan == 4 {
        println!("=================================");
        println!("======= Kelvin ke Celsius =======");
        println!("=================================");
        println!("");

        println!("Masukan Nilai Kelvin: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Inputan Berupa Angka!");
        let input: f32 = input.trim().parse().expect("Harus Berupa Angka!");

        // rumus
        let hasil: f32 = input - 273.15;
        println!("Hasil Dari {input} Kelvin ke Celcius Adalah {hasil} Celcius.");
    } else if pilihan == 5 {
        println!("=================================");
        println!("===== Kelvin ke Fahrenheit ======");
        println!("=================================");
        println!("");

        println!("Masukan Nilai Kelvin: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Inputan Berupa Angka!");
        let input: f32 = input.trim().parse().expect("Harus Berupa Angka!");

        // rumus
        let hasil: f32 = input * 1.8;
        println!("Hasil Dari {input} Kelvin ke Fahrenheit Adalah {hasil} Fahrenheit.");
    } else if pilihan == 6 {
        println!("=================================");
        println!("===== Fahrenheit ke Kelvin ======");
        println!("=================================");
        println!("");

        println!("Masukan Nilai Fahrenheit: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Inputan Berupa Angka!");
        let input: f32 = input.trim().parse().expect("Harus Berupa Angka!");

        // rumus
        let hasil: f32 = (((input - 32.0) * 5.0) / 9.0) + 273.15;
        println!("Hasil Dari {input} Kelvin ke Fahrenheit Adalah {hasil} Fahrenheit.");
    } else {
        println!("Menu Tidak Tersedia!");
    }
}
