use std::io::{BufRead, BufReader, Write, stdout};
use std::{clone, thread};
use std::net::TcpListener;

use serde_json::Value;
use zeroize::Zeroize;


mod data_comp;

fn main() {
    server_socket_data();
    thread::park();
   // let using = r#"{"passUser":"datauser", "contenedorName":"user22","aliasCif":"data1111","rutesys":"/r/t/t/gfdd","typeD":"dd", "space":"10gb"}"#;
 //   let _ = data_comp::parse_action::action_parse_user(using);
}

fn server_socket_data(){
    thread::spawn(|| {
        let escuchads = TcpListener::bind("127.0.0.1:8080").expect("[-] OCURRIO UN ERROR INICIANDO LA ESCUCHA");
        for live in escuchads.incoming(){
            match live{
                Ok(mut tcp_live ) => {
                    let mut radare = BufReader::new(&mut tcp_live);
                    let mut line = String::new();
                    
                    while let Ok(bytesrev) = radare.read_line(&mut line){
                        if bytesrev == 0 { break;}
                        
                        match serde_json::from_str::<Value>(&mut line){
                            Ok(jsoninfo) => {
                                
                                let info= data_comp::parse_action::action_parse_user(&jsoninfo.to_string());
                                line.zeroize();
                                jsoninfo.to_string().zeroize();       
                                match info{
                                    //mut es la referencia del objeto
                                    //con flush le dice que envie los datos que estan en el buffer -> siempre y cuando lleguen nuevos datos
                                    Ok(information) => {radare.get_mut().write_all(information.as_bytes()).unwrap();radare.get_mut().flush().unwrap();},
                                    Err(e)=> {radare.get_mut().write_all(e.to_string().as_bytes()).unwrap();radare.get_mut().flush().unwrap();}
                                }
                            }
                            Err(e) => eprintln!("[-] ERROR JSON -> [{}]",e)
                        }
                    }
                }
                Err(e) => eprint!("[ERROR EN IPC] -> {}",e) 
            }
        }
    });
}
