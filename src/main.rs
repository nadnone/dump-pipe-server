use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

mod logs_manager;
mod sockets;
mod constants;
use crate::logs_manager::LogsManager;
use crate::sockets::SocketsConnector;
use crate::constants::Errors;

fn handle_client(stream: TcpStream, socket_connector: Arc<std::sync::Mutex<SocketsConnector>>) {


    let source = stream.peer_addr().unwrap();

    LogsManager::appends_log(format!("Connection de -> ip: {} port: {}", source.ip(), source.port()));

    let mut sock = stream.try_clone().expect(Errors::FATAL.to_str());

    let err = sock.write_all(format!("You address is: {}\n", source).as_bytes()).is_err();
    if err {
        LogsManager::appends_log(format!("Impossible d'écrire dans la socket : {}", source));
        socket_connector.lock().unwrap().remove_sock_by_addr(sock.peer_addr().unwrap().ip().to_string(), sock.peer_addr().unwrap().port());
    }



    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect(Errors::FATAL.to_str()));


    for essais in 0..5 {

        let _ = stream.try_clone().unwrap().write_all(format!("Vous avez {} essais.\n", 5 - essais ).as_bytes());     
        
        if reader.read_line(&mut buffer).is_err() 
        {
            println!("[!] erreur de lecture du buffer pour {}", source);
            continue;
        }

        if buffer.is_empty() {
            let _ = stream.try_clone().unwrap().write_all("Ordre inconnu".as_bytes());
            println!("[!] Déconnection");
            socket_connector.lock().unwrap().remove_sock_by_addr(sock.peer_addr().unwrap().ip().to_string(), sock.peer_addr().unwrap().port());
            return;
        }


        let command = buffer.split_whitespace().nth(0);
        if command.is_none() {
            let _ = stream.try_clone().unwrap().write_all("[!] Aucun opcode !\n".as_bytes());
            continue;
        }


        let opcode: &str = command.unwrap();

        // on ajoute la socket à la liste pour savoir qu'elle existe
        socket_connector.lock().unwrap().add_to_socketlist( sock.try_clone().expect(Errors::FATAL.to_str()) );

        match opcode.to_uppercase().as_str() {
            
            // connect to ip address
            "CTO" => {
                
                let destination = buffer.split_whitespace().nth(1);
                if destination.is_none()
                {
                    let _ = stream.try_clone().unwrap().write_all("[!] Aucune adresse !\n".as_bytes());
                    continue;
                }
                
                socket_connector.lock().unwrap().connect_to(destination.unwrap().to_string(), &stream );
            }
            // enregister son pseudonymme à la socket
            "REGISTER" => {

                let nickname = buffer.split_whitespace().nth(1);
                if nickname.is_none() {
                    let _ = stream.try_clone().unwrap().write_all("[!] Aucun pseudonymme !\n".as_bytes());
                    continue;
                }
                socket_connector.lock().unwrap().set_sock_with_nickname(&sock, nickname.unwrap().to_string());
            }
            // connect to nickname 
            "CTO_NICKNAME" => {

                println!("tentative de connexion");

                let name = buffer.split_whitespace().nth(1);

                let addr = socket_connector.lock().unwrap().get_sock_addr_by_nickname(name.clone().unwrap().to_string());

                if addr.is_empty() {
                    let _ = sock.write_all( format!("Pseudonyme introuvable\n").as_bytes());
                    continue;
                }

                println!("connection à {}", addr);

                socket_connector.lock().unwrap().connect_to(addr, &stream);
    
            }

            _ => {

                println!("Exception non gerée");
            }

        }

        // on nettoye le buffer netre chaque commandes
        buffer.clear();

    }

    socket_connector.lock().unwrap().remove_sock_by_addr(sock.peer_addr().unwrap().ip().to_string(), sock.peer_addr().unwrap().port());


}



fn main() -> std::io::Result<()> {


    println!("Serveur en fonctionnement.");

    let socket_connector_main = Arc::new(Mutex::new(SocketsConnector::create()));

    let listener = TcpListener::bind("0.0.0.0:44444").expect(Errors::FATAL.to_str());

    loop {


        for stream in listener.incoming() {


            if stream.is_err() {
                LogsManager::appends_log(format!("{} : Connexion morte", Errors::WARN.to_str()).to_string());
            }

            // on clone les élements à mettre dans le thread
            let sock = stream?.try_clone().expect(Errors::FATAL.to_str());
            
            // on push la socket à la liste
            socket_connector_main
                .lock()
                .unwrap()
                .add_to_socketlist(
                    sock.try_clone().expect(
                        Errors::FATAL.to_str()
                    )
                );



            // le thread
            let sock_manager = socket_connector_main.clone();
            thread::spawn(move || {
            
                println!("Nouvelle connexion en cours.");

                println!("sockets: {:?}", sock_manager.lock().unwrap().get_sock_list());

                handle_client(sock, sock_manager);

                println!("déconnection");
            });

            
        }
        
    }
    
   
}