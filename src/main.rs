use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

mod logs_manager;
mod sockets;
use crate::logs_manager::LogsManager;
use crate::sockets::SocketsConnector;


fn handle_client(stream: TcpStream, socket_connector: &mut SocketsConnector) {


    let source = stream.peer_addr().unwrap();

    LogsManager::appends_log(format!("Connection de -> ip: {} port: {}", source.ip(), source.port()));





    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("erreur de clonage, erreur interne."));

    if reader.read_line(&mut buffer).is_err() 
    {
        println!("[!] erreur de lecture de destination pour {}", source);
        return;
    }

    if buffer.is_empty() {
        println!("[!] Error empty order");
        return;
    }


    let command = buffer.split_whitespace().nth(0);
    if command.is_none() {
        println!("[!] Aucun opcode !");
        return;
    }

    let destination = buffer.split_whitespace().nth(1);
    if destination.is_none()
    {
        println!("[!] Aucune adresse");
        return;
    }

    let opcode = command.unwrap();

    match opcode.to_uppercase().as_str() {
        
        "CTO" => {
            socket_connector.add_to_socketlist( stream.try_clone().expect("erreur interne de clonage.") );
            socket_connector.connect_to(destination.unwrap().to_string(), &stream );
        }

        _ => {

        }
    }

}



fn main() -> std::io::Result<()> {


    println!("Serveur en fonctionnement.");

    let mut socket_connector = SocketsConnector::create();


    let listener = TcpListener::bind("127.0.0.1:44444").unwrap();

    loop {


        for stream in listener.incoming() {

            if stream.is_err() {
                LogsManager::appends_log("Coonnexion morte".to_string());
            }

            handle_client(stream?, &mut socket_connector);
            
        }
        
    }
    
   
}