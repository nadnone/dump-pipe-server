use std::{fs::{File}, io::Write, thread::sleep, time::Duration};

use chrono::Local;

pub struct LogsManager {

}

impl LogsManager {

    pub fn appends_log(message: String) {


        let date_now = Local::now().format("%Y-%m-%d").to_string();
        let time_now = Local::now().format("%H:%M:%S").to_string();
        let filename = format!("./logs_{}.txt", date_now);


        let mut _log_file = File::options()
            .create(true)
            .append(true)
            .open(filename)
            .expect(format!("impossible d'ouvrir le fichier log_{}.txt", date_now.as_str()).as_str());
        

        sleep(Duration::from_millis(500));

        let log_to_append = format!("[{}][{}] : {}\n", time_now, date_now, message);
        _log_file.write_all(log_to_append.as_bytes()).expect("Erreur d'ecriture des logs");


    }
}