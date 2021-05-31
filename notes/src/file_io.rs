extern crate serde_json;
extern crate chrono;
use std::path::Path;
use std::io::{Write, Read, Error, ErrorKind};
use serde_json::json;
use super::crypt;
use chrono::{Utc, DateTime};
use std::fs::{File, remove_file, OpenOptions, create_dir};

/// Writes a json object to an encrypted file, handling each data case.
pub fn write_json(password: &String, filename: &String, json_object: &serde_json::Value) -> Result<String, Error>{

    let mut file_path: String = "./files/".to_owned();

    let dir_exists = Path::new(&file_path).exists();

    // Currently, there is an error with testing when this directory does not exist.
    // If the directory does not exist, each test will try to create the directory.
    // Since creating the directory is asynchronous, most tests will try to recreate it.
    // By the time they reach the creation code, the directory will already exist.
    // Thus, a lot of tests will fail on the first run.
    match dir_exists {
        true => (),
        false => {
            let created_dir = create_dir(&file_path);
            match created_dir {
                Ok(_) => (),
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    file_path.push_str(filename);

    // Copy the json object passed in
    let mut new_object = json_object.to_owned();

    let path = Path::new(&file_path);

    let mut file_exists = std::path::Path::new(&file_path).exists();

    // If the file exists, check to see if it is expired.
    // If the file is expired, it will no longer exist.
    if file_exists {
        file_exists = !is_expired(password, filename);
    }

    // If file exists, read old object
    if file_exists {
        let old_data = read_json(password, &filename).unwrap();
        let old_object_val: serde_json::Value = serde_json::from_str(&old_data).unwrap();
        let old_object = old_object_val.as_object().unwrap();

        for (k,v) in old_object {
            // If entry does not exist in the new object, write entry from file to new object
            if new_object[k].is_null() {
                new_object[k] = (*v).to_owned();
            } else {
                if k == "tasks" { // If adding task information
                    new_object[k] = json!(handle_write_task(&k, &v, json_object));
                } else if k == "trackers" { // If adding tracker information
                    new_object[k] = json!(handle_write_tracker(&k, &v, json_object));
                } else if k == "tags" { // If adding tag
                    new_object[k] = json!(handle_write_tag(&k, &v, json_object));
                }
            }
        }
    }

    let file = OpenOptions::new().write(true).open(&path);

    // Create file if it doesn't exist
    let mut file = match file {
        Ok(file) => file,
        Err(_) => {
            File::create(&path)?
        },
    };

    // Add the time edited to the file
    new_object["time-edited"] = json!(Utc::now().to_rfc2822());

    // Encrypt the JSON
    let mut enc_result = crypt::encrypt(password, &new_object.to_string());

    // Append IV to the beginning of the written message.
    // IV will always be 16 bytes long, and doesn't need to be a secret.
    let mut data_to_write = vec![];
    data_to_write.append(&mut enc_result.1);
    data_to_write.append(&mut enc_result.0);


    // Write to file
    file.write_all(&data_to_write[..])?;

    return Ok(format!("Wrote to file {}", file_path));
}

/// Handles writing tasks to file.
fn handle_write_task(k: &String, v: &serde_json::Value, json_object: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut v_clone = v.to_owned();
    let mut new_object = json_object.to_owned();
    let old_arr = new_object[k].as_array_mut().unwrap();
    let file_arr = v_clone.as_array_mut().unwrap();
    let mut new_arr = vec!();
    // Append new task to the list of tasks
    let old_task = &old_arr[0];
    let mut new_task;
    for file_task in file_arr { // For each task in file
        if file_task["task_name"] == old_task["task_name"]{ // If the task being added already exists
            let completed = old_task["completed"].as_null();

            let time_completed;
            let time_added;
            let completed = match completed {
                None => {
                    // If completed is in json, then update time completed
                    time_completed = old_task["time_completed"].to_owned();
                    // If completed is in json, then we are not adding this task for the first time
                    time_added = file_task["time_added"].to_owned();
                    old_task["completed"].to_owned()
                }, // Old task is not null, use new value
                Some(()) => {
                    // If completed is not in json, then we are not completing this task
                    // Keep old time completed value
                    time_completed = file_task["time_completed"].to_owned();
                    // If completed is not in json, then we are adding this task for the first time
                    // Add time_added to new json
                    time_added = old_task["time_added"].to_owned();
                    file_task["completed"].to_owned()
                }, // Old task is null, use file value
            };

            let new_subtasks: serde_json::Value;
            if old_task["subtasks"].is_null() {
                new_subtasks = file_task["subtasks"].to_owned();
            }
            else{
                let mut new_subtasks_vec = vec!();
                if file_task["subtasks"].is_null() {// If file doesn't have subtasks, then use new subtasks
                    new_subtasks = old_task["subtasks"].to_owned(); 
                } else { // Otherwise, update subtasks
                    for file_subtask in file_task["subtasks"].as_array_mut().unwrap() { 
                        if file_subtask["task_name"] == old_task["subtasks"][0]["task_name"] { // If subtask name matches, use new value
                            new_subtasks_vec.push(
                                json!(
                                    {
                                        "task_name": file_subtask["task_name"],
                                        "completed": old_task["subtasks"][0]["completed"],
                                        "time_completed": old_task["subtasks"][0]["time_completed"],
                                    }
                                )
                            )
                        }
                        else{ // Otherwise use old subtask
                            new_subtasks_vec.push(file_subtask.to_owned());
                        }
                    }
                    new_subtasks = json!(new_subtasks_vec);
                }
            }

            // Updated completed status of task and subtasks
            new_task = json!(
                {
                    "task_name": file_task["task_name"],
                    "completed": completed,
                    "subtasks": new_subtasks,
                    "time_completed": time_completed,
                    "time_added": time_added,

                });
            new_arr.append(&mut vec!(new_task));
        } else{ // Otherwise add the file version to write
            new_arr.append(&mut vec!(file_task.to_owned()));
        }
    }
    return new_arr;
}

/// Handles writing tracker to file.
fn handle_write_tracker(k: &String, v: &serde_json::Value, json_object: &serde_json::Value) -> Vec<serde_json::Value> {
    let new_object = json_object.to_owned();
    let mut tracker_object = new_object[k][0].to_owned();
    let mut v_clone = v.to_owned();
    let file_arr = v_clone.as_array_mut().unwrap();
    let mut new_arr = vec!();
    let mut new_data = vec!();
    let mut new_data_time = vec!();
    // Check the data being added
    for value in file_arr { // For each tracker in the file
        if value["tracker_name"] == tracker_object["tracker_name"] { // If the tracker being added already exists.
            new_data.append(value["data"].as_array_mut().unwrap());
            // Add the new data point to the existing tracker.
            new_data.append(tracker_object["data"].as_array_mut().unwrap());
            // Add the new data point time to the existing tracker.
            new_data_time.append(value["data_time"].as_array_mut().unwrap());
            new_data_time.append(tracker_object["data_time"].as_array_mut().unwrap());

            new_arr.append(&mut vec!(
                json!({
                    "tracker_name": value["tracker_name"],
                    "data": new_data,
                    "data_time": new_data_time,
                })
            ))
        } else {
            // Otherwise create a new tracker
            new_arr.append(&mut vec!(value.to_owned()));
        }
    }
    return new_arr;
}

/// Handles writing tags to file.
fn handle_write_tag(k: &String, v: &serde_json::Value, json_object: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut new_object = json_object.to_owned();
    let mut v_clone = v.to_owned();
    let old_arr = new_object[k].as_array_mut().unwrap();
    let file_arr = v_clone.as_array_mut().unwrap();
    let mut new_arr = vec!();
    if !file_arr.contains(&old_arr[0]) { // If the tag doesn't exist
        // Add the tag
        new_arr.append(old_arr);
        new_arr.append(file_arr);
    } else { // Otherwise, keep the tags the same, but update time updated since file was read.
        new_arr.append(file_arr);
    }
    return new_arr;
}

/// Loads a file, used for CLI.
pub fn load_file(password: &String, filename: &String) -> Result<String, Error>{
    if is_expired(password, filename){
        return Ok("File has expired!".to_string());
    } else{
        return read_json(password, filename);
    }
}

/// Returns whether the file has expired.
fn is_expired(password: &String, filename: &String)-> bool{
    let plaintext = read_json(password, filename);
    let plaintext = match plaintext {
        Ok(p) => p,
        Err(_) => return false,
    };

    let json: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
    
    if json["expiry"].is_null() {
        return false;
    } else{
        let parsed_time = DateTime::parse_from_rfc2822(json["expiry"].as_str().unwrap()).unwrap();

        if parsed_time <= Utc::now() {
            let file_path = ["./files/", filename].concat();
            remove_file(file_path).unwrap();
            return true;
        }
        else{
            return false;
        }
    }
}
/// Reads an encrypted string file, and returns the contents.
fn read_json(password: &String, filename: &String) -> Result<String, Error>{
    let mut file_path: String = "./files/".to_owned();

    file_path.push_str(filename);

    let path = Path::new(&file_path);

    let mut file = File::open(&path)?;

    let mut read_bytes = Vec::new();
    file.read_to_end(&mut read_bytes)?;


    // The first 16 bytes will be the IV
    // Split off the ciphertext, which occurs 16 bytes after the start of the file.
    let ciphertext = read_bytes.split_off(16);
    read_bytes.resize(16, 0);

    // Decrypt the file
    let plaintext = crypt::decrypt(password, &ciphertext, &read_bytes);

    
    match plaintext {
        Ok(t) => {
            return Ok(t)},
        Err(error) => return Err(Error::new(ErrorKind::Other, error.to_string())),
    };
}

#[cfg(test)]
mod tests{
    use super::{write_json, read_json};
    use std::fs;
    use serde_json::json;
    use serde_json::from_str;
    use rand;

    fn setup() -> (String, String, serde_json::Value) {
        let password = "abcd".to_string();
        let x = rand::random::<u64>();
        let filename = "temp".to_string();
        let filename = [filename, x.to_string()].concat();
        let plaintext = "plaintext".to_string();

        let data = json!({
            "string": plaintext
        });

        return (password, filename, data);
    }

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    /// Checks if writing the file creates the file on disk.
    #[test]
    fn test_write_file_exists() {
        let setup_res = setup();

        let write_out = write_json(&setup_res.0, &setup_res.1, &setup_res.2);

        let file_path = ["./files/", &setup_res.1].concat();

        // Fail test if write fails. Otherwise, check filename exists.
        match write_out {
            Ok(_) => {
                let file_exists = std::path::Path::new(&file_path).exists();
                teardown(&file_path);
                assert!(file_exists);
            },
            Err(_) => {
                teardown(&file_path);
                assert!(false)
            },
        }
    }

    /// Checks if the output of read file is expected as JSON.
    #[test]
    fn test_read_string_file_output() {
        let setup_res = setup();

        let write_out = write_json(&setup_res.0, &setup_res.1, &setup_res.2);

        // Fail test if write fails.
        match write_out {
            Ok(_) => (),
            Err(_) => assert!(false),
        }

        let file_path = ["./files/", &setup_res.1].concat();

        let output = read_json(&setup_res.0, &setup_res.1).unwrap();
        let obj: serde_json::Value = from_str(&output).unwrap();

        teardown(&file_path);

        // Check output
        assert!(obj["string"] == setup_res.2["string"]);


    }

    /// Checks if the file cannot be read without the correct password.
    #[test]
    fn test_read_string_incorrect_pass() {
        let setup_res = setup();

        let bad_password = "abcf".to_string();

        let write_out = write_json(&setup_res.0, &setup_res.1, &setup_res.2);

        // Fail test if write fails.
        match write_out {
            Ok(_) => (),
            Err(_) => assert!(false),
        }

        let output = read_json(&bad_password, &setup_res.1);
        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        // Check if reading the file fails.
        match output {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
    /// Checks if the file cannot be read without the correct password.
    #[test]
    fn test_is_expired() {
        let setup_res = setup();

        let bad_password = "abcf".to_string();

        let write_out = write_json(&setup_res.0, &setup_res.1, &setup_res.2);

        // Fail test if write fails.
        match write_out {
            Ok(_) => (),
            Err(_) => assert!(false),
        }

        let output = read_json(&bad_password, &setup_res.1);
        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        // Check if reading the file fails.
        match output {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}