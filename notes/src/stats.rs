use super::search::get_all_files;
use chrono::{Duration, DateTime, Local};

pub fn get_tracker_stats_all(password: &String) -> Vec<(String, Vec<(String, Vec<f64>, Vec<String>)>)> {
    let files = get_all_files(password);
    let mut return_vec = vec!();
    for (filename, plaintext) in files {
        let mut obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let trackers = obj["trackers"].as_array_mut();

        let trackers = match trackers {
            None => continue,
            Some(arr) => arr,
        };

        let mut tracker_vec = vec!();

        for tracker_obj in trackers {
            // Make an owned copy of the tracker so we don't borrow twice
            let mut owned_tracker_1 = tracker_obj.clone();
            let owned_tracker_2 = tracker_obj.clone();
            // Get json arrays
            let tracker_data_json = tracker_obj["data"].as_array_mut().unwrap();
            let tracker_data_times_json = owned_tracker_1["data_time"].as_array_mut().unwrap();
            let tracker_name = owned_tracker_2["tracker_name"].as_str().unwrap().to_string();

            // Create normal arrays
            let mut tracker_data = vec!();
            let mut tracker_data_times = vec!();
            // Get values out of the json arrays
            for datum in tracker_data_json {
                let owned_datum = datum.to_owned(); // Take ownership of value
                let datum_f64: f64 = serde_json::from_value(owned_datum).unwrap(); // Convert value to f64
                tracker_data.push(datum_f64);
            }

            for datum_date in tracker_data_times_json {
                tracker_data_times.push(datum_date.as_str().unwrap().to_string()); // Convert json value to string
            }


            println!("File with name {} has tracker with name {}.\n Data \n {:?} \n Times \n {:?}. \n", filename, tracker_name, tracker_data, tracker_data_times);
            tracker_vec.push((tracker_name, tracker_data, tracker_data_times));
        }
        return_vec.push((filename.clone(), tracker_vec))
    }
    return return_vec;
}

pub fn get_tracker_stats_time(password: &String, start: &DateTime<Local>, end: &DateTime<Local>) -> Vec<(Vec<f64>,Vec<String>)> {
    let files = get_all_files(password);
    let mut return_vec = vec!();
    for (filename, plaintext) in files {
        let mut obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let trackers = obj["trackers"].as_array_mut();

        let trackers = match trackers {
            None => continue,
            Some(arr) => arr,
        };

        for tracker_obj in trackers {
            // Make an owned copy of the tracker so we don't borrow twice
            let mut owned_tracker = tracker_obj.to_owned();
            // Get json arrays
            let tracker_data_json = tracker_obj["data"].as_array_mut().unwrap();
            let tracker_data_times_json = owned_tracker["data_time"].as_array_mut().unwrap();
        
            // Create normal arrays
            let mut tracker_data = vec!();
            let mut tracker_data_times = vec!();

            for i in 0..tracker_data_json.len() {
                let datum_date = tracker_data_times_json[i].to_owned();
                // Take time str and convert to datetime
                let time = DateTime::parse_from_rfc2822(datum_date.as_str().unwrap()).unwrap();

                // If time of data is within range then add it to the output
                if time >= *start && time <= *end {
                    let datum_f64: f64 = serde_json::from_value(tracker_data_json[i].to_owned()).unwrap(); // Convert value to f64
                    tracker_data.push(datum_f64);
                    tracker_data_times.push(time.to_rfc2822()); // Convert json value to string
                }
            }

            println!("File with name {} has tracker with data \n {:?} \n and times \n {:?}. \n", filename, tracker_data, tracker_data_times);
            return_vec.push((tracker_data, tracker_data_times));
        }
    }
    return return_vec;
}

fn get_task_duration(task_obj: &serde_json::Value) -> Duration {
    // Get task start and end times
    let task_start = task_obj["time_added"].to_owned();
    let task_end = task_obj["time_completed"].to_owned();
    let mut task_duration: Duration = Duration::seconds(0);

    // If both start and end times are not null, then calculate duration taken
    if !task_start.is_null() && !task_end.is_null() {
        let datetime_start = DateTime::parse_from_rfc2822(task_start.as_str().unwrap()).unwrap();
        let datetime_end = DateTime::parse_from_rfc2822(task_start.as_str().unwrap()).unwrap();
        task_duration = datetime_end - datetime_start;
    }

    return task_duration;
}

pub fn get_task_stats(password: &String){
    let files = get_all_files(password);

    for (filename, plaintext) in files {
        let mut obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let tasks = obj["tasks"].as_array_mut();

        let tasks = match tasks {
            None => continue,
            Some(arr) => arr,
        };
        
        for task_obj in tasks {
            let task_duration = get_task_duration(&task_obj);
            
            println!("File with name {} has task with name {}. This task took {} minutes to complete", filename, task_obj["task_name"], task_duration.num_minutes());

            let subtasks = task_obj["subtasks"].to_owned();
            // Check subtask names
            if !subtasks.is_null() {
                let subtasks = task_obj["subtasks"].as_array_mut().unwrap();
                for subtask_obj in subtasks {
                    let subtask_duration = get_task_duration(&subtask_obj);

                    println!("File with name {} has subtask with name {}. This subtask took {} minutes to complete", filename, subtask_obj["task_name"], subtask_duration.num_minutes());
                }
            }
        }
    }
}

pub fn get_tag_stats(password: &String, tags_input: Vec<String>) -> Vec<String>{

    let files = get_all_files(password);
    let mut return_vec = vec!();

    for (filename, plaintext) in files {
        let mut obj: serde_json::Value = serde_json::from_str(&plaintext).unwrap();
        let tags = obj["tags"].as_array_mut();

        let tags = match tags {
            None => continue,
            Some(arr) => arr,
        };
        
        let mut tag_remaining = tags_input.len();

        for tag in tags {
            // For each tag in the tag list, if it is a tag that we search for,
            // then reduce the tag remaining by one
            if tags_input.contains(&tag.to_owned().as_str().unwrap().to_string()) {
                tag_remaining -= 1;
            }
        }

        // If all tags were found, remaining should be zero
        if tag_remaining <= 0 {
            return_vec.push(filename.clone());
            println!("File with name {} contains tags {:?}", filename, tags_input);
        }
    }
    return return_vec;
}

fn mean(data: &Vec<f64>) -> f64{
    let sum: f64 = Iterator::sum(data.iter());
    return sum / (data.len() as f64);
}

fn net_change(data: &Vec<f64>) -> f64{
    return data[data.len() - 1] - data[0];
}

fn median(data: &Vec<f64>) -> f64 {
    if data.len() % 2 != 0 {
        return data[(data.len()/2 as usize)]
    } else {
        let mut two_middle_elements = vec!();
        two_middle_elements.push(data[(data.len()/2-1 as usize)]);
        two_middle_elements.push(data[(data.len()/2 as usize)]);
        return mean(&two_middle_elements);
    }
}

pub fn get_advanced_stats(password: &String) -> Vec<(String, Vec<(String, f64, f64, f64)>)>{
    let tracker_files = get_tracker_stats_all(password);
    let mut return_vec = vec!();

    for (filename, trackers) in tracker_files {

        println!("File with name {}", filename);
        let mut tracker_vec = vec!();

        for (tracker_name, tracker_data, _) in trackers {
            let mean = mean(&tracker_data);
            let net_change = net_change(&tracker_data);
            let median = median(&tracker_data);
            println!("Tracker with name {}", tracker_name.clone());
            println!("Average: {}", mean);
            println!("Net gain/loss: {}", net_change);
            println!("Median: {}", median);
            tracker_vec.push((tracker_name.clone(), mean, net_change, median));
        }
        return_vec.push((filename.clone(), tracker_vec));
    }
    return return_vec;
}

#[cfg(test)]
mod tests{
    use super::{mean, net_change, median, get_tag_stats};
    use std::fs;
    use crate::note::tag::add_tag;

    fn teardown(file_path: &str) {
        fs::remove_file(file_path).unwrap();
    }

    fn setup_vec() -> Vec<f64>{
        let mut test_vec: Vec<f64> = vec!();
        test_vec.push(1.0);
        test_vec.push(2.0);
        test_vec.push(3.0);
        test_vec.push(4.0);
        test_vec.push(5.0);
        return test_vec;
    }

    fn setup_tag() -> (String, String, String, String, String) { 
        let password = "abcd".to_string();
        // Create random filename
        let x = rand::random::<u64>();


        let filename1 = "temp".to_string();
        let filename1 = [filename1, x.to_string()].concat();
        let tag1 = "tag1".to_string();
        let tag2 = "tag2".to_string();

        let filename2 = "temp".to_string();
        let filename2 = [filename2, x.to_string(), "2".to_string()].concat();


        // Add both tags to first file
        add_tag(&password, &filename1, &tag1).unwrap();
        add_tag(&password, &filename1, &tag2).unwrap();


        // Add only one tag to second file
        add_tag(&password, &filename2, &tag1).unwrap();


        return (password, filename1, filename2, tag1, tag2);
    }


    /// Test if the mean is calculated correctly
    #[test]
    fn test_mean() {
        let test_vec = setup_vec();
        let actual_mean = 3.0;

        assert_eq!(actual_mean, mean(&test_vec))
    }

    /// Test if the net change is calculated correctly
    #[test]
    fn test_net_change() {
        let test_vec = setup_vec();
        let actual_net_change = 4.0;

        assert_eq!(actual_net_change, net_change(&test_vec))
    }

    /// Test if the median is the correct data point, if the data list has an odd number of elements
    #[test]
    fn test_median_odd() {
        let test_vec = setup_vec();
        let actual_median = 3.0;

        assert_eq!(actual_median, median(&test_vec))
    }

    /// Test if the median is the correct data point, if the data list has an even number of elements
    #[test]
    fn test_median_even() {
        let mut test_vec = setup_vec();
        test_vec.push(6.0);
        let actual_median = 3.5;

        assert_eq!(actual_median, median(&test_vec))
    }

    /// Test tag stats returns both files with shared tag.
    #[test]
    fn test_tag_stats_two_files() {
        // Filename1 has both tags, filename1 only has tag1
        let (password, filename1, filename2, tag1, _) = setup_tag();

        let mut tag_input = vec!();
        tag_input.push(tag1);

        let filenames = get_tag_stats(&password, tag_input);

        let file_path_1 = ["./files/", &filename1].concat();
        let file_path_2 = ["./files/", &filename2].concat();
        teardown(&file_path_1);
        teardown(&file_path_2);

        assert!(filenames.contains(&filename1));
        assert!(filenames.contains(&filename2));
    }

    /// Test tag stats returns one file with unshared tag.
    #[test]
    fn test_tag_stats_one_file() {
        // Filename1 has both tags, filename1 only has tag1
        let (password, filename1, filename2, _, tag2) = setup_tag();

        let mut tag_input = vec!();
        tag_input.push(tag2);

        let filenames = get_tag_stats(&password, tag_input);

        let file_path_1 = ["./files/", &filename1].concat();
        let file_path_2 = ["./files/", &filename2].concat();
        teardown(&file_path_1);
        teardown(&file_path_2);

        assert!(filenames.contains(&filename1));
        assert!(!filenames.contains(&filename2));
    }




}