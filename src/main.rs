use std::io::{self, Write};
use std::fs;



pub fn welcome(){

    println!("
        ================================== \n
        Noodle, just lke your workspace!    \n
        ==================================
            ");
    println!("Enter Mode: (noodle | clean)");
    let mut mode = String::new();
    
    // read input from user
    io::stdin().read_line(&mut mode).expect("fail to readline (mode)");
    check_mode(mode);
    
    // check input mode
    fn check_mode(s:String){
    
        let check = s.trim();// remove whitespace (\n, \r) from input str
        if check == "noodle"{
            workspace_mode();
        }else if check == "clean"{
            cleaning_mode();
        }else{
            println!("unknown mode");
        
        }
    }

}

fn workspace_mode() -> io::Result<()>{
    println!("Entering Workspace (Noodle) Mode");
    

    let file_path = ".noodle_history";

    let mut content = String::new();
    io::stdin().read_line(&mut content).expect("fail to read line (content)");
    
    let mut file = fs::OpenOptions::new()
        .create(true).append(true).open(file_path)?;
    
    file.write_all(content.as_bytes())?;
    println!("file written");
    Ok(())
}

fn cleaning_mode(){
    println!("Entering Cleaning Mode");

}

fn main() {
   welcome();
}


