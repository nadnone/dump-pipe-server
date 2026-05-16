use std::{io::{BufRead, BufReader, Write}, net::TcpStream, sync::{Arc, Mutex}};
use crate::{constants::Errors, logs_manager::LogsManager};

#[derive(Clone)]
struct Nickname {
    socket_addr: String,
    name: String
}

impl Nickname {
    pub fn new(name: String, addr: String) -> Nickname {

        return Nickname {
            socket_addr: addr,
            name: name
        }
    }
}

#[derive(Clone)]
pub struct SocketsConnector {
    socket_list: Vec<Arc<Mutex<TcpStream>>>,
    nicknames: Vec<Nickname>
}

impl SocketsConnector {

    pub fn create() -> SocketsConnector {
        return SocketsConnector {
            socket_list: Vec::new(),
            nicknames: Vec::new()
        }
    }

    pub fn get_sock_addr_by_nickname(&self, name: String) -> String {

        for i in 0..self.nicknames.len() {
            
            if name == self.nicknames[i].name {

                return self.nicknames[i].socket_addr.clone();
            }
        }

        return String::new(); // introuvable

    }

    pub fn set_sock_with_nickname(&mut self, sock: &TcpStream, nickname: String) {


        let sock_addr = sock.peer_addr().unwrap().ip().to_string();

        let nickname = Nickname::new(nickname, sock_addr);
        self.nicknames.push(nickname);
            
        let _ = sock.try_clone().unwrap().write_all("Registered.\n".as_bytes());
    }

    pub fn remove_sock_by_addr(&mut self, addr: String, port: u16) {

        println!("removing old socket {}", addr);

        for i in 0..self.socket_list.len() {
            
            if self.socket_list[i].lock().unwrap().peer_addr().unwrap().ip().to_string() == addr {

                return self.socket_list.retain( |sock| sock.lock().unwrap().peer_addr().unwrap().ip().to_string() != addr && sock.lock().unwrap().peer_addr().unwrap().port() != port);
            }
        }
    }

    pub fn add_to_socketlist(&mut self, socket: TcpStream) {

        self.socket_list.push(Arc::new(Mutex::new(socket)));
    }

    pub fn get_sock_list(&self) -> Vec<Arc<Mutex<TcpStream>>> {
        return self.socket_list.clone();
    }

    pub fn connect_to(&mut self, destination: String, current_socket: &TcpStream) {


        for i in 0..self.socket_list.len() {

            let sock = self.socket_list[i].clone();
            let sock_rslt = sock.lock().unwrap().peer_addr();
            if sock_rslt.is_err()
            {
                LogsManager::appends_log(format!("{} : Erreur inconnue avec la socket", Errors::FATAL.to_str()));
                return;
            }
            let sock_addr = sock_rslt.unwrap().ip().to_string();

            /*let curr_sock: String = current_socket.peer_addr().unwrap().ip().to_string();

            // si la destination est al même que la source
            if destination == curr_sock
            {
                // pas besoin de se connecter à soi-même
                let _ = current_socket.write_all("Same address, abort.\n".as_bytes());
                LogsManager::appends_log("[!] Same address, abort.".to_string());
                continue;
            }
            // si la socket de destination a été trouvée
            else */if sock_addr == destination 
            {

                let curr_sock = current_socket.try_clone();
                if curr_sock.is_err() {
                    return;
                }

                // spawn
                Self::spawn_new_thread(curr_sock.unwrap().try_clone().unwrap(), sock.lock().unwrap().try_clone().unwrap());
            }
            else {
                println!("Exception non gerée");
            }


            LogsManager::appends_log(format!("source: {}, destination: {}", current_socket.peer_addr().unwrap(), destination));

        }

    }

    fn spawn_new_thread(mut curr_sock: TcpStream, mut dest_sock: TcpStream) {

        let sock = curr_sock.try_clone().expect(Errors::FATAL.to_str());

        let destination = dest_sock.peer_addr().unwrap().ip().to_string();
        let source = curr_sock.peer_addr().unwrap().ip().to_string();
        let _ = curr_sock.write_all(format!("Mise en relation avec {}\n", destination).as_bytes());
        LogsManager::appends_log(format!("Mise en relation de {} avec {}", source, destination));

            loop {
                
                let mut msg: String = String::new();
                let mut reader = BufReader::new(sock.try_clone().expect(Errors::FATAL.to_str()));

                match reader.read_line( &mut msg) {
                    Ok(_) => {

                        if dest_sock.write_all(msg.as_bytes()).is_err() {
                            LogsManager::appends_log("[!] Erreur d'envoie du message".to_string());
                            break;
                        }

                    }
                    Err(e) => {
                        LogsManager::appends_log(format!("[!] Erreur interne au thread: {}", e));
                        return;
                    } 
                }
      
            }
            
          

    }
}
