pub mod parse_action{
    use os_info;
    use serde_json::{Value, json};
    use serde::Deserialize;
    use std::{env, fmt::format, io::Write, path::Path, process::{Command, CommandArgs, Stdio}};
    use zeroize::{Zeroize,ZeroizeOnDrop};
     
    #[derive(Debug,Deserialize, Zeroize, ZeroizeOnDrop)]
    #[allow(non_snake_case)]
    struct DataInfo<'a>{
        passUser: String,
        //USANDO SKIP LE INDICO A ZEROIZE QUE NO ES IMPORTANTE QUE BORRE ESTOS DATOS, SOLO SE TIENE QUE ENFOCAR EN passUser
        #[zeroize(skip)]
        pub contenedorName: &'a str,
        #[zeroize(skip)]
        pub aliasCif:&'a str,
        #[zeroize(skip)]
        pub rutesys: &'a str,
        #[zeroize(skip)]
        pub typeD: &'a str,
        #[zeroize(skip)]
        pub space: &'a str,
        #[zeroize(skip)]
        pub fileSystem: &'a str,
        #[zeroize(skip)]
        pub montado: &'a str
    }

    fn json_action_linuxtype() -> Value{
       let data:Value = json!({

        "ubuntu": "apt-get",
        "Debian": "apt-get",
        "CentOS/RHEL": "yum install",
        "Fedora": "dnf install",
        "Arch Linux": "pacman -S",
        "Alpine Linux": "apk install"

       }); 
       return data;
    }
    
    #[allow(non_snake_case)]
    pub fn action_parse_user<'a>(dataSerde: &'a str) -> Result<String, Box<dyn std::error::Error>>{
        //let data_info: DataInfo<'a> = serde_json::from_str(&dataSerde)?;
        let info_response:&String = match serde_json::from_str::<DataInfo>(dataSerde) {
            Ok(data_info) => &command_data_info_execute(data_info).to_string(),
            Err(e) => &e.to_string(),
        };
        Ok(info_response.to_string())
    }
    
    //Verifica si existe fallocate o dd
    fn commerror_verify(cmnd_install:&str) -> i8{
        let info_machine = os_info::get().os_type();
        let num = match json_action_linuxtype().get(info_machine.to_string()) {
            Some(clave_data) => {
                let utile = clave_data.to_string();
                let datos_split: Vec<&str> = utile.split(" ").collect();
                
                let execute_install = Command::new(datos_split[0].replace("\"", " ").trim())
                    .arg(datos_split[1].replace("\"", " ").trim())
                    .arg(cmnd_install.trim())
                    .output()
                    .expect("[-] ERROR CON PERMISOS DE USUARIO");
                if execute_install.status.success() {
                    1
                } else {
                    0
                }
            }
            None => {
                2
            }
        };

        return num;
    }
    
    //verifica el contenedor, si existe devuelvo un codigo 2, si falla lo crea
    fn verificate_extension_contenedor(rute:String) -> i8{
        let phat_archive = Path::new(&rute);
        if (!phat_archive.exists()){
            return 2
        }else{
            //OK EXISTE, PERO TIENE  VERIFICAR SI ESTA EN USO
            //SI DICHO CONTENEDOR ESTA EN USO QUE SUCEDE????
            let comm_file = Command::new("file").arg(&rute).output().expect("[ERROR] -> ");
            let info = String::from_utf8_lossy(&comm_file.stdout);
            match info.contains("LUKS"){
                true => return 1,
                false => return 2,
                 
            }
        }
    }

    fn runcommand_data(program:&&str,argsd:&&[&str], pass:&mut String)-> Result<(),String>{
        let exec = if program.contains("cryptsetup"){
            //EJECUTA EL COMANDO CREANDO UNA CONTRASEÑA DE CIFRADO
            let mut crypto = Command::new(program).args(*argsd)
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            //EL COMANDO PIDE UNA CONTRASEÑA ENTONCES
            crypto.stdin.as_mut()//si dicho comando no se pasa de forma correcta la contraseña va fallar
                .unwrap()// POR ESO SE ESCRIBA CON WRITE_ALL Y LA CONTRASEÑA SE COMVIERTE A BYTES, para que le STDIN PUEDA LEERLO
                .write_all(pass.as_bytes())
                .unwrap();
            pass.zeroize();
            let data_crypto = crypto.wait().unwrap();
            if !data_crypto.success(){
                return Err(format!("ERROR EN LA CONTRASEÑA -> {}",data_crypto));
            }
        }else{
            let execute = Command::new(program)
                .args(*argsd)
                .output()
                .map_err(|e| format!("Error en {} producido por: {}",program,e))?;
            if !execute.status.success(){
                return Err(format!("Execute -> {} Error -> {}\n",program,String::from_utf8_lossy(&execute.stderr)));
            }
            
        };
        
        Ok(exec)
    }
    
    fn command_data_info_execute(data_env:DataInfo) -> &str{
        let mut message = "";
        let disk_space:Vec<&str> = data_env.space.split("GB").collect();
        let create_content:String = data_env.rutesys.to_string() + "/" + data_env.contenedorName+".img";
        let envspace = disk_space[0].trim().to_owned() + "G";
        let name_montado = data_env.rutesys.to_string() + "/" + data_env.montado;
    
        match data_env.typeD {
            "fallocate" => {
                
                match verificate_extension_contenedor(create_content.clone()){
                    1 => {
                        message = "[USO] -> CONTENEDOR EN USO,335\n";
                    },
                    2 => {
                        let command_fallocate = Command::new("fallocate")
                           .args(["-l",envspace.as_str().trim(),&create_content]) //CREA UN CONTENEDOR OK -> PERO Y SI ESE CONTENEDOR CON ESE NOMBRE YA EXISTE?
                           .output()//TENDRIA QUE VERIFICAR, OK -> EXISTE CONTENEDOR CON ESE NOMBRE (NO) ->  CREALO -> (SI) -> ESTA EN USO? Y SI NO ESTA EN USO,USALO
                           .expect("[-] OCURRIO UN PROBLEMA");
                        if command_fallocate.status.success(){
                            let usingcif = format!("/dev/mapper/{}",data_env.aliasCif);
                            let components = format!("mkfs.{}",data_env.fileSystem);
                            let data: &[(&str, &[&str])] = &[
                                ("chattr", &["+C", &create_content]),
                                ("cryptsetup", &["luksFormat","--batch-mode",&create_content,"-"]),
                                ("cryptsetup", &["open","--batch-mode",&create_content,data_env.aliasCif,"-"]),
                                (components.as_str(), &[usingcif.as_str()]),
                                ("mkdir",&[name_montado.as_str()]),
                                ("mount",&["-o","compress=zstd",usingcif.as_str(),name_montado.as_str()]),
                                
                            ];
                            
                            for (cmdprogram,argsf) in data{
                                let info = match runcommand_data(cmdprogram, argsf,&mut data_env.passUser.to_string()){
                                    Ok(()) => "OK",
                                    Err(e) => "FATAL ERROR"
                                };
                                message = info;    
                            }
                             
                            
                        }
                        message = "USO DE FALLOC,200\n";
                        
                       
                    },
                    3 => message = "LA RUTA NO EXISTE,332\n",
                    _ => message = "[-] OCURRIO UN ERROR FATAL USANDO fallocate,766\n" 
                }
               
           
            
            },
            "dd" => {
               
                match verificate_extension_contenedor(create_content.clone()) {
                    1 => message = "[USO] -> CONTENEDOR EN USO,335",
                    2 => {
                        let convert:i32 = (disk_space[0].trim().parse::<i64>().unwrap() * 1024) as i32; //aplico lo siguente ejemplo: 5 x 1024 
                        let countx = format!("count={}",convert.to_string());
                        let of_content = format!("of={}",&create_content);
                        let usingcif = format!("/dev/mapper/{}",data_env.aliasCif);
                        let components = format!("mkfs.{}",data_env.fileSystem);//la extensión signicia el filesystem a usar
                        let command_dd = Command::new("dd")
                            .args(["if=/dev/zero",of_content.as_str(),"bs=1M",countx.as_str()]).output().expect("");
                        if command_dd.status.success(){
                            let datacomm: &[(&str,&[&str])] = &[
                               ("cryptsetup", &["luksFormat","--batch-mode",&create_content,"-"]),
                               ("cryptsetup", &["open","--batch-mode",&create_content,data_env.aliasCif,"-"]),
                               (components.as_str(), &[usingcif.as_str()]),
                               ("mkdir",&[name_montado.as_str()]),
                               ("mount",&["-o","compress=zstd",usingcif.as_str(),name_montado.as_str()]),
                            ];
                            for (cmdprogram,argsf) in datacomm{
                                let info = match runcommand_data(cmdprogram, argsf,&mut data_env.passUser.to_string()){
                                    Ok(()) => "OK",
                                    Err(e) => "FATAL ERROR"
                                };
                                message = info;    
                            };
                        }
                        
                    },
                    3 =>message = "OCURRIO UN ERROR EN LA RUTA,332\n",
                    _ => message = "[-] OCURRIO UN ERROR FATAL USANDO dd,766\n"
                }
            },
            _ => message = "ERROR FATAL"
        }
        return message;
    }
    
} 
