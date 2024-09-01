use std::io;
use std::process::Command;
use std::fs;
use std::io::ErrorKind;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;


fn main() {
    let path = "text.txt";
    
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Error creating file");

    
    let mut file_data:String = String::from("");
    
    f.read_to_string(&mut file_data).expect("Error reading file!");

    

    let data:String = String::from(file_data);
    
    let a: i64 = data.trim().parse().expect("error 1111");
    
    if a == 012 {
        println!("Запуск Малювекі");
        
        let exe = "C:\\Mvk\\Mvk\\Mvk\\MvkLauncher\\bin\\Release\\MvkLauncher.exe";
        let output = Command::new(exe)
        .output()
        .expect("Failed to execute process");
    }
    

    
    else if a == 0 {
        open::that("https://github.com/Marcusi5482/AMLauncher/releases/download/Mvk/Mvk.zip",);

        
        f.write_all("1".as_bytes()).expect("Error writing file");

        println!("Перезапустите лаунчер");
        println!("Чтобы выйти вам нужно зажать Ctrl + c");

        let mut a_str:String = String::new();
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {},
            Err(e) => println!("Ошыбка ввода - {}", e)
        }

    }
    
    
    else if a == 01 {
        println!("Распаковка архива");
    
        let exe = "filse\\unzip.bat";
        
        Command::new(exe);
        let output = Command::new(exe)
            .output()
            .expect("Failed to execute process");
        println!("Архив бил распакован");

        f.write_all("2".as_bytes()).expect("Error writing file");

        println!("Перезапустите лаунчер");
        println!("Чтобы выйти вам нужно зажать Ctrl + c");
        let mut a_str:String = String::new();
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {},
            Err(e) => println!("Ошыбка ввода - {}", e)
        }

        
    }
}
