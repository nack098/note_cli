#[derive(Clone, Debug, Copy)]
pub enum MultipleArg {
    Read,
    Delete,
}

fn read(path: std::path::PathBuf, index: usize) {
    let mut notes: Vec<_> = match std::fs::read_dir(path) {
        Ok(val) => val.map(|r| r.unwrap()).collect(),
        Err(_) => panic!(
            "
System Error
Unable to read folder
            "
        ),
    };

    if index > notes.len() || index == 0 {
        panic!(
            "
System Error
Note does not exist. Please check again
use:
    note_cli list
            "
        );
    }

    notes.sort_by_key(|dir| dir.path());

    let mut file = match std::fs::File::open(notes[index - 1].path()) {
        Ok(val) => val,
        Err(_) => panic!(
            "
System Error
Unable to open file
            "
        ),
    };
    println!(
        "{}'s contents:",
        notes[index - 1].file_name().into_string().unwrap()
    );
    let mut content: String = String::new();
    use std::io::Read;

    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => panic!(
            "
System Error
Unable to read file
            "
        ),
    }
    println!("{}", content);
}

fn delete(path: std::path::PathBuf, index: usize) {
    let mut notes: Vec<_> = match std::fs::read_dir(path) {
        Ok(val) => val.map(|r| r.unwrap()).collect(),
        Err(_) => panic!(
            "
System Error
Unable to read folder
            "
        ),
    };

    if index > notes.len() || index == 0 {
        panic!(
            "
System Error
Note does not exist. Please check again
use:
note_cli list
            "
        );
    }

    notes.sort_by_key(|dir| dir.path());

    match std::fs::remove_file(notes[index - 1].path()) {
        Ok(_) => (),
        Err(_) => println!(
            "
System Error
Unable to remove file
            "
        ),
    }
}

pub fn multiple_arg_cmd(op: MultipleArg, path: std::path::PathBuf, index: usize) {
    match op {
        MultipleArg::Read => read(path, index),
        MultipleArg::Delete => delete(path, index),
    }
}
