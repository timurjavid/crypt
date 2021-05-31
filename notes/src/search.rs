
use std::fs::read_dir;
use super::file_io::load_file;
use chrono::{DateTime, Local};

/// Gets all files in the files folder, if password works.
/// Returns vector of tuples containing (filename, plaintext)
pub fn get_all_files(password: &String) -> Vec<(String, String)>{
    let paths = read_dir("./files").unwrap();
    let mut plaintexts = vec!();
    for path in paths {
        // path needs to be unwrapped, turned into a filename which is a DirEntry
        // then needs to be converted to str, unwrapped, then converted to String
        let filename = path.unwrap().file_name().to_str().unwrap().to_string();
        let plaintext = load_file(password, &filename);
        let plaintext = match plaintext {
            Ok(p) => p,
            Err(_) => continue,
        };
        plaintexts.append(&mut vec!((filename, plaintext)));
    }
    return plaintexts;
}

/// Search files based on keyword
pub fn search(password: &String, search_term: &String) -> Vec<String> {
    let files = get_all_files(password);
    let mut return_vec = vec!();
    for (filename, plaintext) in files {
        if plaintext.contains(search_term){
            return_vec.push(plaintext.clone());
            println!("File with name {} contains search term. \n {}", filename, plaintext);
        }
    }
    return return_vec;
}

/// Filter files based on tag
pub fn filter_tag(password: &String, tag_search: &String) -> Vec<String> {
    let files = get_all_files(password);
    let mut return_vec = vec!();
    for (filename, plaintext) in files {
        let mut obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let tags = obj["tags"].as_array_mut();

        let tags = match tags {
            None => continue,
            Some(arr) => arr,
        };
        
        for tag_obj in tags {
            let tag = tag_obj.as_str().unwrap();
            if tag_search == tag {
                return_vec.push(plaintext.clone());
                println!("File with name {} contains tag. \n {}", filename, plaintext);
            }
        }
    }
    return return_vec;
}

/// Filter all files based on task (or subtask)
pub fn filter_task(password: &String, task_search: &String) -> Vec<String> {
    let files = get_all_files(password);
    let mut return_vec = vec!();
    for (filename, plaintext) in files {
        let mut obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let tasks = obj["tasks"].as_array_mut();

        let tasks = match tasks {
            None => continue,
            Some(arr) => arr,
        };
        
        for task_obj in tasks {
            let task_name = task_obj["task_name"].as_str().unwrap();
            // Check task name first
            if task_search == task_name {
                return_vec.push(plaintext.clone());
                println!("File with name {} contains (sub)task. \n {}", filename, plaintext);
                continue;
            }
            let subtasks = task_obj["subtasks"].to_owned();
            // Check subtask names
            if !subtasks.is_null() {
                let subtasks = task_obj["subtasks"].as_array_mut().unwrap();
                for subtask_obj in subtasks {
                    let subtask_name = subtask_obj["task_name"].as_str().unwrap();
                    if task_search == subtask_name {
                        return_vec.push(plaintext.clone());
                        println!("File with name {} contains (sub)task. \n {}", filename, plaintext);
                        continue;
                    }
                }
            }
        }
    }
    return return_vec;
}

/// Filter based on time range
pub fn filter_time(password: &String, time_start: &DateTime<Local>, time_end: &DateTime<Local>){
    let files = get_all_files(password);
    for (filename, plaintext) in files {
        let obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let time = obj["time-edited"].as_str();

        let time = match time {
            None => continue,
            Some(t) => t,
        };
        
        let time = DateTime::parse_from_rfc2822(time).unwrap();
        if time_start < &time && &time < time_end {
            println!("File with name {} is within time range. \n {}", filename, plaintext);
        }
    }

}



#[cfg(test)]
mod tests{
    use super::{search, get_all_files};
    use crate::note::string::add_string;
    use std::fs;

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    /// Checks if the file cannot be read without the correct password.
    #[test]
    fn test_get_all_files_num() {
        let password = "all_files".to_string();
        let other_password = "all_files_other".to_string();
        let mut filenames = vec!();
        // Create 4 files with the same password
        // Store filenames in a vector for deletion later
        for _i in 0..4 {
            let x = rand::random::<u64>();
            let filename = ["temp", &x.to_string()].concat();
            filenames.append(&mut vec!(["temp", &x.to_string()].concat()));
            add_string(&password, &filename, &password).unwrap();
        }

        // Create one file with a different password
        let x = rand::random::<u64>();
        let filename = ["temp2", &x.to_string()].concat();
        add_string(&other_password, &filename, &password).unwrap();

        let all_files = get_all_files(&password);

        // Teardown special file with other password
        let file_path = ["./files/", &filename].concat();
        teardown(&file_path);

        // Teardown first four files
        for filename in filenames {
            let file_path = ["./files/", &filename].concat();
            teardown(&file_path);
        }

        assert_eq!(all_files.len(), 4);
    }
    #[test]
    fn test_search_one() {
        let password = "test_search_one".to_string();
        let string = "hello!".to_string();

        let x = rand::random::<u64>();
        let filename = ["temp", &x.to_string()].concat();

        add_string(&password, &filename, &string).unwrap();

        let vector = search(&password, &"hello".to_string());


        // Teardown
        let file_path = ["./files/", &filename].concat();
        teardown(&file_path);

        assert_eq!(vector.len(), 1);
    }
    #[test]
    fn test_search_with_other() {
        let password = "test_search_with_other".to_string();
        let other_password = "test_search_with_other2".to_string();
        let string = "hello!".to_string();

        let x = rand::random::<u64>();
        let filename1 = ["temp", &x.to_string()].concat();
        let x = rand::random::<u64>();
        let filename2 = ["temp", &x.to_string()].concat();

        add_string(&password, &filename1, &string).unwrap();

        add_string(&other_password, &filename2, &string).unwrap();

        let vector = search(&password, &"hello".to_string());

        // Teardown
        let file_path = ["./files/", &filename1].concat();
        teardown(&file_path);
        let file_path = ["./files/", &filename2].concat();
        teardown(&file_path);

        assert_eq!(vector.len(), 1);
    }

}