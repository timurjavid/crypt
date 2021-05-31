extern crate clap;
use clap::App;
use clap::load_yaml;
mod crypt;
mod file_io;
mod note;
mod search;
mod stats;
use chrono::{offset::TimeZone, Local, DateTime, NaiveDateTime};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let password = matches.value_of("PASSWORD").unwrap().to_string();

    // Handle saving
    if matches.is_present("save") {

        let filename = matches.value_of("FILENAME").unwrap().to_string();

        // Needs to have INPUT or TASK present if type is save
        if matches.value_of("INPUT") == None 
        && matches.value_of("TASK") == None 
        && matches.value_of("TRACK") == None
        && matches.value_of("TAG") == None
        && matches.value_of("TIMER") == None{
            println!("Save functionality requires INPUT, TASK, TRACK, or TAG argument! Use flag -h for help.");
            return;
        }

        match matches.value_of("INPUT") {
            None => (),
            Some(input) => {
                handle_input(input, &password, &filename);           
            }
        }

        match matches.value_of("TAG") {
            None =>(),
            Some(tag) => {
                handle_tag(tag, &password, &filename);
            }
        }

        match matches.value_of("TASK") {
            None => (),
            Some(task) => {
                handle_task(&matches, task, &password, &filename);
            }
        }

        match matches.value_of("TRACK") {
            None => (),
            Some(track) => {
                handle_track(&matches, track, &password, &filename);
            }
        }

        match matches.value_of("TIMER"){
            None => (),
            Some(time) => {     
                handle_time(time, &password, &filename);
            }
        }
    }


    // Handle basic search
    // Currently, search only works if all the files that you want to search have the same password.
    match matches.value_of("SEARCH") {
        None => (),
        Some(search_term) => {
            search::search(&password, &search_term.to_string());
        }
    }

    // Handle tag filter
    // Currently, search only works if all the files that you want to search have the same password.
    match matches.value_of("FILTER_TAG") {
        None => (),
        Some(filter_tag) => {
            search::filter_tag(&password, &filter_tag.to_string());
        }
    }

    // Handle task search
    // Currently, search only works if all the files that you want to search have the same password.
    match matches.value_of("FILTER_TASK") {
        None => (),
        Some(filter_task) => {
            search::filter_task(&password, &filter_task.to_string());
        }
    }

    if matches.is_present("FILTER_TIME_START") || matches.is_present("FILTER_TIME_END") {
        // Needs both to be present
        if !(matches.is_present("FILTER_TIME_START") && matches.is_present("FILTER_TIME_END")){
            println!("Filter time requires both FILTER_TIME_START and FILTER_TIME_END! Use -h for help.");
            return;
        }
        let time_start = matches.value_of("FILTER_TIME_START").unwrap();
        let time_end = matches.value_of("FILTER_TIME_END").unwrap();
        handle_time_filter(&password, &time_start.to_string(), &time_end.to_string());
    }

    if matches.is_present("TRACKER_STAT") {
        stats::get_tracker_stats_all(&password);
    }

    if matches.is_present("TASK_STAT") {
        stats::get_task_stats(&password);
    }

    if matches.is_present("ADVANCED_STAT") {
        stats::get_advanced_stats(&password);
    }

    if matches.is_present("TRACKER_STAT_START") || matches.is_present("TRACKER_STAT_END") {
        // Needs both to be present
        if !(matches.is_present("TRACKER_STAT_START") && matches.is_present("TRACKER_STAT_END")){
            println!("Tracker stat requires both TRACKER_STAT_START and TRACKER_STAT_END! Use -h for help.");
            return;
        }
        let time_start = matches.value_of("TRACKER_STAT_START").unwrap();
        let time_end = matches.value_of("TRACKER_STAT_END").unwrap();
        handle_tracker_stat_time(&password, &time_start.to_string(), &time_end.to_string());
    }

    // Handle loading
    if matches.is_present("load"){
        let filename = matches.value_of("FILENAME").unwrap().to_string();
        handle_load(&password, &filename);
    }
}

fn handle_input(input: &str, password: &String, filename: &String) {
    // Try writing to file and print error if one is returned. Otherwise, print the filename of the file made.
    let success = note::string::add_string(password, filename, &input.to_string());
    match success {
        Ok(s) => println!("{}",s),
        Err(e) => println!("{}",e),
    }
}

fn handle_tag(tag: &str, password: &String, filename: &String) {
    let success = note::tag::add_tag(password, filename, &tag.to_string());
        match success {
            Ok(s) => println!("{}",s),
            Err(e) => println!("{}",e),
    }
}

fn handle_task(matches: &clap::ArgMatches, task: &str, password: &String, filename: &String) {
    match matches.value_of("SUBTASK") {
        None => { // Handle normal task
            match matches.value_of("TASK_COMPLETE"){
                None => {
                    println!("Task functionality requires TASK_COMPLETE to be y or n! Use flag -h for help.");
                    return;
                }
                Some(task_complete) => {
                    let complete = get_complete(&task_complete);
                    let success = note::task::add_task(&password, &filename, &task.to_string(), &complete);
                    match success {
                        Ok(s) => println!("{}",s),
                        Err(e) => println!("{}",e)
                    }
                }
            }
        }
        Some(subtask) => { // Handle subtask
            match matches.value_of("TASK_COMPLETE"){
                None => {
                    println!("Subtask functionality requires TASK_COMPLETE to be y or n! Use flag -h for help.");
                    return;
                }
                Some(task_complete) => {
                    let complete = get_complete(&task_complete);
                    let success = note::task::add_subtask(&password, &filename, &task.to_string(), &subtask.to_string(), &complete);
                    match success {
                        Ok(s) => println!("{}",s),
                        Err(e) => println!("{}",e)
                    }
                }
            }
        }
    }
}

fn handle_track(matches: &clap::ArgMatches, track: &str, password: &String, filename: &String) {
    match matches.value_of("TRACK_DATA"){ // Make sure there is data to be tracked
        None => {
            // Needs data
            println!("Track functionality requires data in TRACK_DATA field! Use flag -h for help.");
            return;
        }
        Some(track_data) => {
            // Parse the data into a float
            let data = track_data.parse();

            // If data didn't parse, then return
            let data = match data {
                Err(e) => {
                    println!("TRACK_DATA needs to be a number! Use flag -h for help. Error: {}", e);
                    return
                },
                Ok(d) => d,
            };

            let success = note::track::add_tracker(&password, &filename, &track.to_string(), &data);
            match success {
                Ok(s) => println!("{}",s),
                Err(e) => println!("{}",e)
            }
        }
    }
}

fn handle_time(time: &str, password: &String, filename: &String) {
    let time = time.parse::<u64>();
    let time = match time {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to parse time! {}", e);
            return;
        },
    };
    let success = note::timer::add_timer(time as i64, password, filename);
    match success {
        Ok(s) => {println!("{:#}",s)},
        Err(e) => println!("{}",e),
    }
}

fn handle_load(password: &String, filename: &String) {
    // Try loading and print error if one is returned. Otherwise, print the file contents.
    let success = file_io::load_file(&password.to_string(), &filename.to_string());
    match success {
        Ok(s) => {println!("{:#}",s)},
        Err(e) => println!("{}",e),
    }
}

fn handle_time_filter(password: &String, time_start: &String, time_end: &String) {

    // Parse time start
    let time_start_parsed = NaiveDateTime::parse_from_str(time_start, "%Y-%m-%d %H:%M:%S");
    let time_start_parsed = match time_start_parsed {
        Ok(t) => t,
        Err(e) => {
            println!("Time start parse error! {}", e);
            return;
        }
    };

    // Parse time end
    let time_end_parsed = NaiveDateTime::parse_from_str(time_end, "%Y-%m-%d %H:%M:%S");
    let time_end_parsed = match time_end_parsed {
        Ok(t) => t,
        Err(e) => {
            println!("Time end parse error! {}", e);
            return;
        }
    };

    // Add time zone information to datetime to be able to compare to file.
    // This assumes the time zone is the local one.
    let date_time_start: DateTime<Local> = Local.from_local_datetime(&time_start_parsed).unwrap();
    let date_time_end: DateTime<Local> = Local.from_local_datetime(&time_end_parsed).unwrap();

    search::filter_time(password, &date_time_start, &date_time_end);

}

fn handle_tracker_stat_time(password: &String, time_start: &String, time_end: &String) {

    // Parse time start
    let time_start_parsed = NaiveDateTime::parse_from_str(time_start, "%Y-%m-%d %H:%M:%S");
    let time_start_parsed = match time_start_parsed {
        Ok(t) => t,
        Err(e) => {
            println!("Time start parse error! {}", e);
            return;
        }
    };

    // Parse time end
    let time_end_parsed = NaiveDateTime::parse_from_str(time_end, "%Y-%m-%d %H:%M:%S");
    let time_end_parsed = match time_end_parsed {
        Ok(t) => t,
        Err(e) => {
            println!("Time end parse error! {}", e);
            return;
        }
    };

    // Add time zone information to datetime to be able to compare to file.
    // This assumes the time zone is the local one.
    let date_time_start: DateTime<Local> = Local.from_local_datetime(&time_start_parsed).unwrap();
    let date_time_end: DateTime<Local> = Local.from_local_datetime(&time_end_parsed).unwrap();

    stats::get_tracker_stats_time(password, &date_time_start, &date_time_end);
}

fn get_complete(task_complete: &str) -> bool {
    if task_complete == "y" {
        return true;
    } else if task_complete == "n" {
        return false;
    } else {
        println!("TASK_COMPLETE needs to be y or n! Use flag -h for help.");
        panic!();
    } 
}