use forward_goto::rewrite_forward_goto;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
#[rewrite_forward_goto]
pub fn function() {
    
    println!("Проверка версии");

    let path = "C:\\AMLauncher\\filse\\text.txt";
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Error creating file");

    let mut file: String = String::from("");
    f.read_to_string(&mut file).expect("Error reading file!");

    let file: i64 = file.trim().parse().expect("error cheker 19");

    if file == 10500 {
        let path = "C:\\AMLauncher\\database\\v1.0.5.0.txt";
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Error creating file");

        let mut filev: String = String::from("");

        f.read_to_string(&mut filev).expect("Error reading file!");
        let filev: i64 = filev.trim().parse().expect("error cheker 36");

        if filev == 10500 {
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\versions\\v1.0.5.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Установка версии");
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\downloadsfilse\\betav1.0.5.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Распаковка архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\unzipfilse\\unzipv1.0.5.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Удаление архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\delfilse\\delzipv1.0.5.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Aрхив бил удален");

            f.write_all("1".as_bytes()).expect("Error writing file");
            forward_goto!('v10500);
            

        }
        else if filev == 105001 {
            forward_label!('v10500);
            println!("Запуск Малювекі");
        
            let exe = "C:\\AMLauncher\\versions\\v1.0.5.0\\MvkLauncher.exe";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");
        }
        

    }

    else if file == 10400 {
        let path = "C:\\AMLauncher\\database\\v1.0.4.0.txt";
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Error creating file");

        let mut filev: String = String::from("");

        f.read_to_string(&mut filev).expect("Error reading file!");
        let filev: i64 = filev.trim().parse().expect("error cheker 36");

        if filev == 10400 {
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\versions\\v1.0.4.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Установка версии");
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\downloadsfilse\\betav1.0.4.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Распаковка архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\unzipfilse\\unzipv1.0.4.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Удаление архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\delfilse\\delzipv1.0.4.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Aрхив бил удален");

            f.write_all("1".as_bytes()).expect("Error writing file");
            forward_goto!('v10400);
            

        }
        else if filev == 104001 {
            forward_label!('v10400);
            println!("Запуск Малювекі");
        
            let exe = "C:\\AMLauncher\\versions\\v1.0.4.0\\MvkLauncher.exe";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");
        }
        

    }

    else if file == 10300 {
        let path = "C:\\AMLauncher\\database\\v1.0.3.0.txt";
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Error creating file");

        let mut filev: String = String::from("");

        f.read_to_string(&mut filev).expect("Error reading file!");
        let filev: i64 = filev.trim().parse().expect("error cheker 36");

        if filev == 10300 {
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\versions\\v1.0.3.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Установка версии");
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\downloadsfilse\\betav1.0.3.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Распаковка архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\unzipfilse\\unzipv1.0.3.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Удаление архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\delfilse\\delzipv1.0.3.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Aрхив бил удален");

            f.write_all("1".as_bytes()).expect("Error writing file");
            forward_goto!('v10300);
            

        }
        else if filev == 103001 {
            forward_label!('v10300);
            println!("Запуск Малювекі");
        
            let exe = "C:\\AMLauncher\\versions\\v1.0.3.0\\MvkLauncher.exe";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");
        }
        

    }

    else if file == 10100 {
        let path = "C:\\AMLauncher\\database\\v1.0.1.0.txt";
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Error creating file");

        let mut filev: String = String::from("");

        f.read_to_string(&mut filev).expect("Error reading file!");
        let filev: i64 = filev.trim().parse().expect("error cheker 36");

        if filev == 10100 {
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\versions\\v1.0.1.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Установка версии");
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\downloadsfilse\\betav1.0.1.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Распаковка архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\unzipfilse\\unzipv1.0.1.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Удаление архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\delfilse\\delzipv1.0.1.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Aрхив бил удален");

            f.write_all("1".as_bytes()).expect("Error writing file");
            forward_goto!('v10300);
            

        }
        else if filev == 101001 {
            forward_label!('v10300);
            println!("Запуск Малювекі");
        
            let exe = "C:\\AMLauncher\\versions\\v1.0.1.0\\MvkLauncher.exe";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");
        }
        

    }

    else if file == 10000 {
        let path = "C:\\AMLauncher\\database\\v1.0.0.0.txt";
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Error creating file");

        let mut filev: String = String::from("");

        f.read_to_string(&mut filev).expect("Error reading file!");
        let filev: i64 = filev.trim().parse().expect("error cheker 36");

        if filev == 10000 {
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\versions\\v1.0.0.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Установка версии");
            
            let exe = "C:\\AMLauncher\\filse\\databasefilse\\downloadsfilse\\betav1.0.0.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Распаковка архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\unzipfilse\\unzipv1.0.0.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Удаление архива");

            let exe = "C:\\AMLauncher\\filse\\databasefilse\\delfilse\\delzipv1.0.0.0.bat";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");

            println!("Aрхив бил удален");

            f.write_all("1".as_bytes()).expect("Error writing file");
            forward_goto!('v10300);
            

        }
        else if filev == 100001 {
            forward_label!('v10300);
            println!("Запуск Малювекі");
        
            let exe = "C:\\AMLauncher\\versions\\v1.0.0.0\\MvkLauncher.exe";
            let output = Command::new(exe)
                .output()
                .expect("Failed to execute process");
        }
        

    }
}