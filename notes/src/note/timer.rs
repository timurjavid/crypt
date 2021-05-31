use crate::file_io;
use std::io::Error;
use serde_json::json;
use chrono::{Utc, Duration};

/// Generates the json required for the timer and writes it to the filename provided.
pub fn add_timer(time: i64, password: &String, filename: &String) -> Result<String, Error>{

    let time_of_expiry = Utc::now() + Duration::seconds(time);
    
    let data = json!({
        "expiry": time_of_expiry.to_rfc2822()
    });

    return file_io::write_json(password, filename, &data);
}

#[cfg(test)]
mod tests{
    use super::add_timer;
    use crate::file_io;
    use std::fs;
    use serde_json::from_str;
    use chrono::{DateTime, Duration};
    use std::{thread, time};


    // Create a random filename, with a preset password and timer and return it
    fn setup() -> (String, String, i64) {
        let password = "abcd".to_string();
        let x = rand::random::<u64>();
        let filename = "temp".to_string();
        let filename = [filename, x.to_string()].concat();
        let time_seconds = 5;

        return (password, filename, time_seconds);
    }

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_add_timer_output(){
        let setup_res = setup();

        add_timer(setup_res.2, &setup_res.0, &setup_res.1).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        let obj: serde_json::Value = from_str(&plaintext).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        let expiry = DateTime::parse_from_rfc2822(obj["expiry"].as_str().unwrap()).unwrap();
        let created = DateTime::parse_from_rfc2822(obj["time-edited"].as_str().unwrap()).unwrap();

        assert!(expiry == created + Duration::seconds(5));
    }

    #[test]
    fn test_add_timer_expired(){
        let setup_res = setup();

        add_timer(1, &setup_res.0, &setup_res.1).unwrap();

        // Wait for 2 seconds for timer to expire
        thread::sleep(time::Duration::from_secs(2));

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        assert_eq!(plaintext, "File has expired!");
    }


}