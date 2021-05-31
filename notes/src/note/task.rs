use crate::file_io;
use std::io::Error;
use serde_json::json;
use chrono::{Utc};

/// Generates the json required for a task and writes it to the filename provided.
pub fn add_task(password: &String, filename: &String, task_name: &String, completed: &bool) -> Result<String, Error>{

    let data;
    if *completed == true {
        data = json!(
            {
                "tasks": [
                    json!(
                        {
                        "task_name":task_name,
                        "completed": completed,
                        "subtasks": null,
                        "time_added": null,
                        "time_completed": Utc::now().to_rfc2822(),
                        }
                    ),
                ]
            }
        );
    } else {
        data = json!(
            {
                "tasks": [
                    json!(
                        {
                        "task_name":task_name,
                        "completed": completed,
                        "subtasks": null,
                        "time_added": Utc::now().to_rfc2822(),
                        "time_completed": null,
                        }
                    ),
                ]
            }
        );
    }


    return file_io::write_json(password, filename, &data);
}

pub fn add_subtask(password: &String, filename: &String, task_name: &String, subtask_name: &String, completed: &bool) -> Result<String, Error>{
    let data;

    if *completed == true {
        data = json!(
            {
                "tasks": [
                    json!(
                        {
                        "task_name":task_name,
                        "completed": null,
                        "subtasks": [json!(
                                        {
                                            "task_name": subtask_name,
                                            "completed": completed,
                                            "time_added": null,
                                            "time_completed": Utc::now().to_rfc2822(),
                                        }),]
                        }
                    ),
                ]
            }
        );
    } else {
        data = json!(
            {
                "tasks": [
                    json!(
                        {
                        "task_name":task_name,
                        "completed": null,
                        "subtasks": [json!(
                                        {
                                            "task_name": subtask_name,
                                            "completed": completed,
                                            "time_added": Utc::now().to_rfc2822(),
                                            "time_completed": null,
                                        }),]
                        }
                    ),
                ]
            }
        );

    }

    return file_io::write_json(password, filename, &data);

}

#[cfg(test)]
mod tests{
    use super::{add_task, add_subtask};
    use crate::file_io;
    use std::fs;
    use serde_json::from_str;

    // Create a random filename, with a preset password and task and return it
    fn setup() -> (String, String, String) {
        let password = "abcd".to_string();
        // Create random filename
        let x = rand::random::<u64>();
        let filename = "temp".to_string();
        let filename = [filename, x.to_string()].concat();

        let task = "task".to_string();

        return (password, filename, task);
    }

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_add_task_output(){
        let setup_res = setup();

        add_task(&setup_res.0, &setup_res.1, &setup_res.2, &false).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        let obj: serde_json::Value = from_str(&plaintext).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(obj["tasks"][0]["task_name"] == setup_res.2 && obj["tasks"][0]["completed"] == false);
    }

    #[test]
    fn test_add_subtask_output(){
        let setup_res = setup();
        let subtask_name = "temp_subtask".to_string();

        add_subtask(&setup_res.0, &setup_res.1, &setup_res.2, &subtask_name, &false).unwrap();

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1).unwrap();

        let obj: serde_json::Value = from_str(&plaintext).unwrap();

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(obj["tasks"][0]["subtasks"][0]["task_name"] == subtask_name && obj["tasks"][0]["subtasks"][0]["completed"] == false);
    }

    #[test]
    fn test_add_subtask_update(){
        let setup_res = setup();
        let subtask_name = "temp_subtask".to_string();

        add_subtask(&setup_res.0, &setup_res.1, &setup_res.2, &subtask_name, &false).unwrap(); // Add subtask as false

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1,).unwrap();

        let obj_before: serde_json::Value = from_str(&plaintext).unwrap();

        add_subtask(&setup_res.0, &setup_res.1, &setup_res.2, &subtask_name, &true).unwrap(); // Update subtask to true

        let plaintext = file_io::load_file(&setup_res.0, &setup_res.1,).unwrap();

        let obj_after: serde_json::Value = from_str(&plaintext).unwrap(); 

        let file_path = ["./files/", &setup_res.1].concat();
        teardown(&file_path);

        assert!(!obj_before["tasks"][0]["subtasks"].is_null());
        assert!(obj_before["tasks"][0]["subtasks"][0]["task_name"] == subtask_name);
        assert!(obj_before["tasks"][0]["subtasks"][0]["completed"] == false);

        assert!(obj_after["tasks"][0]["subtasks"][0]["task_name"] == subtask_name);
        assert!(obj_after["tasks"][0]["subtasks"][0]["completed"] == true);
    }
}