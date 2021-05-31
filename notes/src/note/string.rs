use crate::file_io;
use serde_json::json;
use std::io::Error;

/// Generates the json required for a string note and writes it to the filename provided.
pub fn add_string(password: &String, filename: &String, string: &String) -> Result<String, Error>{

    let data = json!({
        "string": string,
    });

    return file_io::write_json(password, filename, &data);
}

#[cfg(test)]
mod tests{
    use super::add_string;
    use crate::file_io;
    use std::fs;
    use serde_json::from_str;

    // Create a random filename, with a preset password and string and return it
    fn setup() -> (String, String, String) {
        let password = "abcd".to_string();
        let x = rand::random::<u64>();
        let filename = "temp".to_string();
        let filename = [filename, x.to_string()].concat();
        let plaintext = "plaintext".to_string();

        return (password, filename, plaintext);
    }

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_add_string_output(){
        let setup_res = setup();

        add_string(&setup_res.0, &setup_res.1, &setup_res.2).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        let obj: serde_json::Value = from_str(&plaintext).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(obj["string"] == setup_res.2);
    }
}