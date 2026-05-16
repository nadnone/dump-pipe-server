use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread};

mod logs_manager;
mod sockets;
mod constants;
use crate::logs_manager::LogsManager;
use crate::sockets::SocketsConnector;
use crate::constants::Errors;

fn handle_client(stream: TcpStream, socket_connector: &mut SocketsConnector) {


    let source = stream.peer_addr().unwrap();

    LogsManager::appends_log(format!("Connection de -> ip: {} port: {}", source.ip(), source.port()));

    let mut sock = stream.try_clone().expect(Errors::FATAL.to_str());

    let err = sock.write_all(format!("You address is: {}\n", source).as_bytes()).is_err();
    if err {
        LogsManager::appends_log(format!("Impossible d'écrire dans la socket : {}", source));
    }



    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect(Errors::FATAL.to_str()));


    for essais in 0..5 {

        let _ = stream.try_clone().unwrap().write_all(format!("Vous avez {} essais.\n", 5 - essais ).as_bytes());     
        
        if reader.read_line(&mut buffer).is_err() 
        {
            println!("[!] erreur de lecture de destination pour {}", source);
            return;
        }

        if buffer.is_empty() {
            let _ = stream.try_clone().unwrap().write_all("Ordre inconnu".as_bytes());
            println!("[!] Error empty order");
            continue;
        }


        let command = buffer.split_whitespace().nth(0);
        if command.is_none() {
            let _ = stream.try_clone().unwrap().write_all("[!] Aucun opcode !\n".as_bytes());
            continue;
        }

        let destination = buffer.split_whitespace().nth(1);
        if destination.is_none()
        {
            let _ = stream.try_clone().unwrap().write_all("[!] Aucune adresse !\n".as_bytes());
            continue;
        }

        let opcode = command.unwrap();

        match opcode.to_uppercase().as_str() {
            
            "CTO" => {
                socket_connector.add_to_socketlist( sock.try_clone().expect(Errors::FATAL.to_str()) );
                socket_connector.connect_to(destination.unwrap().to_string(), &stream );
            }

            _ => {

            }
        }

    }

}



fn main() -> std::io::Result<()> {


    println!("Serveur en fonctionnement.");

    let mut socket_connector_main = SocketsConnector::create();


    let listener = TcpListener::bind("0.0.0.0:44444").unwrap();

    loop {


        for stream in listener.incoming() {


            if stream.is_err() {
                LogsManager::appends_log(format!("{} : Connexion morte", Errors::WARN.to_str()).to_string());
            }

            // on clone les élements à mettre dans le thread
            let sock = stream?.try_clone().expect(Errors::FATAL.to_str());
            
            // on push la socket à la liste
            socket_connector_main.add_to_socketlist(sock.try_clone().expect(Errors::FATAL.to_str()));

            let mut socket_connector = socket_connector_main.copy();
            
            // le thread
            thread::spawn(move || {
            
                handle_client(sock, &mut socket_connector);

            });

            
        }
        
    }
    
   
}