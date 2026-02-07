use std::io::{self, Write, Read};
use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;



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
            let _ = cleaning_mode();
        }else{
            println!("unknown mode");
        
        }
    }

}

fn workspace_mode() -> io::Result<()>{
    println!("Entering Workspace (Noodle) Mode | type (exit) to leave");
    //io::stdout().flush().unwrap();

    let home = env::var("HOME").expect("cannot read file home dir");
    let mut path = PathBuf::from(home);
        path.push(".zsh_history");
    
    let mut file = fs::OpenOptions::new().append(true).open(&path)?;

    writeln!(file, "{}", "start section")?;
    
    // will work on this later
    loop {
        let mut content = String::new();
        io::stdin().read_line(&mut content).expect("fail to read line (content)");
     
        file.write_all(content.as_bytes())?;

        //exit
        if content.trim() == "exit noodle mode"{
            writeln!(&mut file, "{}", "end Section")?;
            println!("complete work");
            break;
        }


    }
    Ok(())
    
}

fn cleaning_mode() -> io::Result<()>{
    println!("Entering Cleaning Mode");
    

    let home = env::var("HOME").expect("cannot read file home dir");
    let mut p = PathBuf::from(home);
        p.push(".zsh_history");
    

    //let mut file = fs::File::open(&p)?;
    //let mut buff = Vec::new();

       //file.read_to_end(&mut buff)?;
    let mut file = fs::OpenOptions::new().append(true).open(&p)?;

    //let c = String::from_utf8_lossy(&buff);
    writeln!(file, "{}", "start section")?;

    Ok(())

}

fn main() {
   //run_prog();

    // test func
    let a = Command::new("echo").arg("test").output().expect("dail");
    if a.status.success(){
        let out = str::from_utf8(&a.stdout).expect("fail");
        println!("output: {}", out);
    }
}


