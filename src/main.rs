mod cheker;
use std::io;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;

//use forward_goto::rewrite_forward_goto;

//#[rewrite_forward_goto]
fn main() {
    File::create("C:\\AMLauncher\\filse\\text.txt");
    let path = "C:\\AMLauncher\\filse\\text.txt";
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Error creating file");
    
    let mut version:String = String::from("");


    println!("[1].v1.0.5.0");
    println!("[2].v1.0.4.0");
    println!("[3].v1.0.3.0");
    println!("[4].v1.0.1.0");
    println!("[5].v1.0.0.0");
    println!("Чтобы запустить версия вам нужно ввести [число] версии");

    match io::stdin().read_line(&mut version) {
        Ok(_) => {},
        Err(e) => println!("Ошыбка ввода - {}", e)
    }
    let v: i64 = version.trim().parse().expect("error 1111");
    if v == 1 {
        
        f.write_all("10500".as_bytes()).expect("Error writing file");
        
        cheker::function();
        
    }

    if v == 2 {
        
        f.write_all("10400".as_bytes()).expect("Error writing file");
        
        cheker::function();
        
    }

    if v == 3 {
        
        f.write_all("10300".as_bytes()).expect("Error writing file");
        
        cheker::function();
        
    }

    if v == 4 {
        
        f.write_all("10100".as_bytes()).expect("Error writing file");
        
        cheker::function();
        
    }

    if v == 5 {
        
        f.write_all("10000".as_bytes()).expect("Error writing file");
        
        cheker::function();
        
    }



}
