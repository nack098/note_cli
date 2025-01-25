#[derive(Clone, Debug, Copy)]
pub enum SingleArg {
    List,
    Create,
    Help,
}

fn help() {
    println!(
        "
Note CLI - Take a short note in command line.
Usage:
    list        List of notes
    create      Create new note
    read        Read note
    delete      Delete note
        "
    );
}

fn list(path: std::path::PathBuf) {
    let mut notes: Vec<_> = match std::fs::read_dir(path) {
        Ok(val) => val.map(|r| r.unwrap()).collect(),
        Err(_) => panic!(
            "
System Error
Unable to read folder
            "
        ),
    };
    println!("List of notes:");
    if notes.is_empty() {
        println!("There is no note yet. Try to add some!");
    }

    notes.sort_by_key(|dir| dir.path());
    for (i, path) in notes.into_iter().enumerate() {
        println!("{}: {}", i + 1, path.file_name().into_string().unwrap());
    }
}

fn create(path: std::path::PathBuf) {
    let mut input = String::new();
    print!("Title: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    input = input.trim().to_string();
    let mut path = path.clone();
    path.push(&input);
    let mut file = match std::fs::File::create_new(path) {
        Ok(val) => val,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::AlreadyExists {
                panic!(
                    "
Value Error
This title already exist
                    "
                )
            } else {
                panic!(
                    "
System Error
Cannot create new file
            "
                )
            }
        }
    };
    input.clear();
    println!("Content:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    input = input.trim().to_string();
    use std::io::Write;
    match file.write_all(input.as_bytes()) {
        Ok(_) => (),
        Err(_) => panic!(
            "
System Error
Cannot write file
            "
        ),
    };
}

pub fn single_arg_cmd(op: SingleArg, path: std::path::PathBuf) {
    match op {
        SingleArg::Help => help(),
        SingleArg::List => list(path),
        SingleArg::Create => create(path),
    }
}
