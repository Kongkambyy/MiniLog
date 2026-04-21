use dialoguer::Input;
use std::fs;

fn main() {
    let path: String = Input::new()
        .with_prompt("Indtast sti til logfil")
        .default("log.txt".into())
        .interact_text()
        .unwrap();

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            println!("fejl: Kunne ikke læse filen '{}': {}", path, e);
            return;
        }
    };

    let mut total = 0;
    let mut info = 0;
    let mut warning = 0;
    let mut error = 0;
    let mut invalid = 0;

    for line in content.lines() {
        total += 1;

        if line.contains(" info ") {
            info += 1;
        } else if line.contains(" warning ") {
            warning += 1;
        } else if line.contains(" error ") {
            error += 1;
        } else if !line.trim().is_empty() {
            invalid += 1;
        }
    }

    println!("Opsummering af {}", path);
    println!("antal linjer i alt: {}", total);
    println!("info:    {}", info);
    println!("warning: {}", warning);
    println!("error:   {}", error);
    if invalid > 0 {
        println!("ugyldige linjer: {}", invalid);
    }
}