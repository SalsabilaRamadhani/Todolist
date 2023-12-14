use std::io;
use std::collections::HashMap;

fn main() {
    // Menampilkan menu
    println!("Pilih menu:");
    println!("1. Tambah tugas");
    println!("2. Lihat tugas");
    println!("3. Edit tugas");
    println!("4. Hapus tugas");

    // Meminta input pengguna
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut todo_list = HashMap::new();

    // Memproses input pengguna
    match input {
        "1" => {
            // Menambahkan tugas baru
            println!("Tambahkan tugas baru:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            todo_list.insert(task.trim(), description.trim());
        }
        "2" => {
            // Menampilkan daftar tugas
            println!("Daftar tugas:");
            for (task, description) in todo_list.iter() {
                println!("* {}: {}", task, description);
            }
        }
        "3" => {
            // Mengedit tugas
            println!("Edit tugas:");
            println!("Masukkan nama tugas yang ingin diedit:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            match todo_list.get_mut(task.trim()) {
                Some(description) => {
                    // Edit deskripsi tugas
                    println!("Masukkan deskripsi tugas yang baru:");
                    let mut new_description = String::new();
                    io::stdin().read_line(&mut new_description).unwrap();
                    *description = new_description.trim();
                }
                None => {
                    // Tugas tidak ditemukan
                    println!("Tugas tidak ditemukan");
                }
            }
        }
        "4" => {
            // Menghapus tugas
            println!("Hapus tugas:");
            println!("Masukkan nama tugas yang ingin dihapus:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            todo_list.remove(task.trim());
        }
        _ => {
            // Input tidak valid
            println!("Input tidak valid");
        }
    }
}
