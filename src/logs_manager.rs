use std::{fs::{File, exists}, io::Write};

use chrono::Local;

pub struct LogsManager {

}

impl LogsManager {

    pub fn appends_log(message: String) {


        let date_now = Local::now().format("%Y-%m-%d").to_string();
        let time_now = Local::now().format("%H:%M:%S").to_string();
        let filename = format!("logs_{}.txt", date_now);


        let mut _log_file;

        if !exists(&filename).is_ok()
        {
            _log_file = File::create(filename.as_str()).expect("[!] Impossible de créer le fichier de logs pour aujourd'hui, CRASH !");
            _log_file = File::open(filename).expect("Impossible d'ouvrir le fichier créer, CRASH !");
        }
        else {

            _log_file = File::options()
                .append(true)
                .open(filename)
                .expect(format!("impossible d'ouvrir dans le fichier log_{}.txt", date_now.as_str()).as_str());
        
        }

        let log_to_append = format!("[{}][{}] : {}\n", time_now, date_now, message);
        _log_file.write_all(log_to_append.as_bytes()).expect("Erreur d'ecriture des logs");


    }
}