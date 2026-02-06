use std::io::{self, Write};
use std::fs;



pub fn run_prog(){

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
            let _ = workspace_mode();
        }else if check == "clean"{
            cleaning_mode();
        }else{
            println!("unknown mode");
        
        }
    }

}

fn workspace_mode() -> io::Result<()>{
    println!("Entering Workspace (Noodle) Mode | type (exit) to leave");
    //io::stdout().flush().unwrap();

    let file_path = ".noodle_history";

    loop {
        let mut content = String::new();
        io::stdin().read_line(&mut content).expect("fail to read line (content)");
     
        //if file !exist = create else open the file
        let mut file = fs::OpenOptions::new()
            .create(true).append(true).open(file_path)?;

        writeln!(&mut file, "{}", "start section")?;
        file.write_all(content.as_bytes())?;
        
        //exit
        if content.trim() == "exit"{
            writeln!(&mut file, "{}", "end Section")?;
            println!("complete work");
            break;
        } 


    }
    
    println!("file written");
    Ok(())
    
}

fn cleaning_mode(){
    println!("Entering Cleaning Mode");

}

fn main() {
   run_prog();
}


