use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};

const SAVED_REQUEST_PATH: &str = "./saved_requests/";

pub fn get_saved_requests() -> Vec<String> {
    let mut entries = fs::read_dir(SAVED_REQUEST_PATH)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    let mut result: Vec<String> = vec![];

    // The entries have now been sorted by their path.
    for entries_as_strings in entries {
        result.push(
            entries_as_strings
                .to_str()
                .unwrap()
                .to_string()
                .replace(SAVED_REQUEST_PATH, ""),
        );
    }

    // entries
    result
}

pub fn read_saved_request(name: String) -> String {
    let path = SAVED_REQUEST_PATH.to_string() + &name;
    if let Err(_e) = File::open(&path) {
        return "Nothing to show.".to_string();
    }
    read_and_return_string(path)
}

fn read_and_return_string(path: String) -> String {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();

    // read the whole file
    Some(f.read_to_string(&mut buffer));
    buffer
}

pub fn update_saved_request(name: &String, content: Vec<String>) {
    let path = SAVED_REQUEST_PATH.to_string() + name;
    let mut buffer = BufWriter::new(File::create(path).unwrap());

    for line in content {
        Some(buffer.write_all(line.as_bytes()));
        Some(buffer.write(b"\n"));
    }
    Some(buffer.flush());
}
