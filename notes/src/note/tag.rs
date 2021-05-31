use crate::file_io;
use std::io::Error;
use serde_json::json;

/// Generates the json required for a tag and writes it to the filename provided.
pub fn add_tag(password: &String, filename: &String, tag: &String) -> Result<String, Error>{
    
    let data = json!({
        "tags": [
            tag,
        ]
    });

    return file_io::write_json(password, filename, &data);
}

#[cfg(test)]
mod tests{
    use super::add_tag;
    use crate::file_io;
    use std::fs;
    use serde_json::from_str;

    // Create a random filename, with a preset password and tag and return it
    fn setup() -> (String, String, String) {
        let password = "abcd".to_string();
        let x = rand::random::<u64>();
        let filename = "temp".to_string();
        let filename = [filename, x.to_string()].concat();
        let tag = "tag".to_string();

        return (password, filename, tag);
    }

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_add_tag_output(){
        let setup_res = setup();

        add_tag(&setup_res.0, &setup_res.1, &setup_res.2).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        let obj: serde_json::Value = from_str(&plaintext).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(obj["tags"][0] == setup_res.2);
    }
}