use std::io::stdin;
use std::process::Command;
use std::process::Stdio;

fn execute_complex_command(command: String, args: std::str::SplitWhitespace) -> Result<String, String> {
    if Command::new(command).args(args).spawn().is_ok(){
        Ok(String::from("Comando ejecutado exitosamente."))
    }else{
        Err(String::from("No es posible completar la operación."))
    }
}

fn execute_simple_command(command: String) -> Result<String, String> {
    if Command::new(command).spawn().is_ok(){
        Ok(String::from("Comando ejecutado exitosamente."))
    }else{
        Err(String::from("No es posible completar la operación."))
    } 
}

#[test]
fn success_command() {
    let result = execute_simple_command("ls".to_string());
    assert_eq!(">>> Comando ejecutado exitosamente.", result.unwrap());
}

#[test]
fn fail_command() {
    let result = execute_simple_command("lssss".to_string());
    assert!(result.is_err());
}

fn main() {
    println!(">>> Bienvenido al nuevo shell de Rust");
    
    let mut quit = false;

    while !quit {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut previous_command = Vec::new();

        if input != "\n" {
            
            let mut commands = input.trim().split(" | ");

            while let Some(current_command) = commands.next()  {

                let mut elements = current_command.split_whitespace();
                
                let command_to_execute = elements.next().unwrap();
                let mut args = Vec::new();

                while let Some(arg) = elements.next()  { args.push(arg); }

                if command_to_execute == "exit" || command_to_execute == "exit()" {
                    quit = true;
                    println!("Bye! 👻");
                }else{

                    // let s = match str::from_utf8(previous_command) {
                    //     Ok(v) => v,
                    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    // };
                    
                    let output = Command::new(command_to_execute)
                        .args(args)
                        .output()
                        .expect("No es posible completar la operación.");
                        //.spawn();
                    
                    previous_command = output.stdout;

                    //println!("{:?}", previous_command)
                }
            }
        }
    }
    
}
