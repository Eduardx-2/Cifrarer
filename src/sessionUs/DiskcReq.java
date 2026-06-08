/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package sessionUs;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

import java.net.HttpURLConnection;
import java.net.URL;

/**
 *
 * @author eduardx_2
 */
public class DiskcReq {
    private static final String URILOCAL = "http://127.0.0.1:9000/api_disk";
    
    public static String diskSessidAction() throws IOException{
        String jsond = null;
        URL urls = new URL(URILOCAL);
        HttpURLConnection connect = (HttpURLConnection) urls.openConnection();
        connect.setRequestMethod("GET");
        connect.connect();
        int respoCode = connect.getResponseCode();
        if (respoCode == 200){
            String line;
            StringBuilder build = new StringBuilder();
            try (BufferedReader buff = new BufferedReader(new InputStreamReader(urls.openStream()))) {
                while((line = buff.readLine()) != null){
                    build.append(line);
                }
                jsond = build.toString();
            }
            
        }
        return jsond;
    }
    
}
