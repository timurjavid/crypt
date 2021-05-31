use crate::file_io;
use std::io::Error;
use serde_json::json;
use chrono::{Utc};

/// Generates the json required for a tracker, adding data and writes it to the filename provided.
pub fn add_tracker(password: &String, filename: &String, tracker_name: &String, data: &f64) -> Result<String, Error>{
    
    let data = json!({
        "trackers": [
            json!({
                "tracker_name": tracker_name,
                "data": [data],
                "data_time": [Utc::now().to_rfc2822()]}),
        ]
    });

    return file_io::write_json(password, filename, &data);
}

#[cfg(test)]
mod tests{
    use super::add_tracker;
    use crate::file_io;
    use std::fs;
    use serde_json::from_str;

    // Create a random filename, with a preset password and tracker and return it
    fn setup() -> (String, String, String) {
        let password = "abcd".to_string();
        // Create random filename
        let x = rand::random::<u64>();
        let filename = "temp".to_string();
        let filename = [filename, x.to_string()].concat();

        let tracker = "tracker".to_string();

        return (password, filename, tracker);
    }

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }


    #[test]
    fn test_add_tracker_output(){
        let setup_res = setup();
        let data = 5;

        add_tracker(&setup_res.0, &setup_res.1, &setup_res.2, &(data as f64)).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        let obj: serde_json::Value = from_str(&plaintext).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(obj["trackers"][0]["tracker_name"] == setup_res.2 && obj["trackers"][0]["data"][0] == (data as f64));
    }

    #[test]
    fn test_add_tracker_append(){
        let setup_res = setup();
        let data = 5;
        let data2 = 7;

        add_tracker(&setup_res.0, &setup_res.1, &setup_res.2, &(data as f64)).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();
        let obj_before: serde_json::Value = from_str(&plaintext).unwrap();

        add_tracker(&setup_res.0, &setup_res.1, &setup_res.2, &(data2 as f64)).unwrap();

        let plaintext2 = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();
        let obj_after: serde_json::Value = from_str(&plaintext2).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(obj_before["trackers"][0]["tracker_name"] == setup_res.2);
        assert!(obj_before["trackers"][0]["data"][0] == (data as f64));

        assert!(obj_after["trackers"][0]["tracker_name"] == setup_res.2);
        assert!(obj_after["trackers"][0]["data"][0] == (data as f64));
        assert!(obj_after["trackers"][0]["data"][1] == (data2 as f64));

    }


}