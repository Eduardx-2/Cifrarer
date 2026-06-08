/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/javafx/FXML2.java to edit this template
 */
package cifrarergui;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.PrintWriter;
import java.net.Socket;
import java.net.URL;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.ResourceBundle;
import java.util.logging.Level;
import java.util.logging.Logger;
import javafx.collections.FXCollections;
import javafx.collections.ObservableList;
import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.fxml.Initializable;
import javafx.scene.Node;
import javafx.scene.control.Alert;
import javafx.scene.control.Alert.AlertType;
import javafx.scene.control.ButtonType;
import javafx.scene.control.ChoiceBox;
import javafx.scene.control.DialogPane;
import javafx.scene.control.PasswordField;
import javafx.scene.control.Spinner;
import javafx.scene.control.SpinnerValueFactory;
import javafx.scene.control.TextField;
import javafx.scene.control.TextInputControl;
import javafx.scene.layout.Region;
import javafx.stage.DirectoryChooser;
import javafx.stage.Stage;
import org.json.JSONObject;
import sessionUs.DiskcReq;


/**
 *
 * @author eduardx_2
 */
public class FXMLDocumentController implements Initializable {
    
    @FXML
    private ChoiceBox<String> checkDesing;
    @FXML
    private TextField montadoText;
    @FXML
    private ChoiceBox<String> checkFile;
    @FXML
    private TextField textCifrado;
    @FXML
    private TextField textConte;
    @FXML
    private Spinner<String> memdisps;
    @FXML
    private PasswordField passTextInf;
    
    
    private String rutedata = "";
    
    private final String datainf[] = {"fallocate", "dd"};
    private final String dataFile[] = {"ext4","btrfs"};
    
    @FXML
    private void handleButtonActionDir(ActionEvent event) {
        DirectoryChooser dir = new DirectoryChooser();
        dir.setTitle("Seleccione una carpeta");
        dir.setInitialDirectory(new File(System.getProperty("user.home")));
        Node nod = (Node) event.getSource();
        Stage stage = (Stage) nod.getScene().getWindow();
        File selectFile = dir.showDialog(stage);
        if (selectFile != null){
            rutedata = selectFile.toString();
            
        }else{
            errorData(3,"USTED NO SELECCIONO UNA RUTA");
        }
        
        
    }
    
    @Override
    public void initialize(URL url, ResourceBundle rb) {
        checkDesing.getItems().addAll(datainf);
        checkFile.getItems().addAll(dataFile);
      //  checkDesing.setOnAction(this::attbutton);
        try {
            jsonSession();
        } catch (IOException ex) {
            
            errorData(2,ex.toString());
           
        }
       
    }    
    
    
    @FXML
    private void attbutton(ActionEvent event) throws InterruptedException{
        
        boolean datosInput = camposData(
                textCifrado, textConte, passTextInf,montadoText
        );
        
        String confPass = null;
        if (passTextInf.getText().length() <= 10){
            Alert alertPass = new Alert(AlertType.CONFIRMATION);
            alertPass.setResizable(false);
            alertPass.setWidth(650);
           
            DialogPane panec = alertPass.getDialogPane();
            panec.setMinHeight(Region.USE_PREF_SIZE);
            panec.getStylesheets().add(
                getClass().getResource("/sessionUs/panelcss.css").toExternalForm()
            );
            alertPass.setTitle(rutedata);
            alertPass.setHeaderText("Contraseña debíl");
            alertPass.setContentText("Usted ingreso una contraseña debíl, está seguro de continuar?");
            ButtonType btnAccept = new ButtonType("OK");
            ButtonType btnNegad = new ButtonType("NO");
                
            alertPass.getButtonTypes().setAll(btnAccept,btnNegad);
            Optional<ButtonType> buttn = alertPass.showAndWait();
            if (buttn.isPresent()){
                if (buttn.get() == btnAccept){
                    confPass = passTextInf.getText(); 
               }else if (buttn.get() == btnNegad){
                    return;
                    
                }
            }
               // confPass = passTextInf.getText();
              
        }
        
        if(datosInput && !rutedata.trim().isEmpty()){
            confPass = passTextInf.getText(); 
            JSONObject jsons = new JSONObject();
            jsons.put("passUser", confPass);
            jsons.put("contenedorName", textConte.getText().toLowerCase());
            jsons.put("aliasCif", textCifrado.getText().toLowerCase());
            jsons.put("rutesys", rutedata);
            jsons.put("typeD", checkDesing.getValue());
            jsons.put("space", memdisps.getValue());
            jsons.put("fileSystem",checkFile.getValue());
            jsons.put("montado",montadoText.getText());
            try{
                Socket sock = new Socket("127.0.0.1",8080);
                PrintWriter wirter = new PrintWriter(sock.getOutputStream(),true);
                BufferedReader buff = new BufferedReader(new InputStreamReader(sock.getInputStream()));
                wirter.println(jsons.toString());
                
                
                show_alert_session(buff.readLine(),checkDesing.getValue());
               
            }catch (IOException e){
                errorData(2,e.toString());
            }
        }else {
            errorData(2,"USTED NO INGRESO UN CAMPO REQUERIDO O "
                    + "NO SELECCIONO LA RUTA");
            
        }
        
        
        
    }
    
    private void show_alert_session(String data_response,String command){
        Alert alert = null;
        String[] infod = data_response.split(",");
       
        switch (Integer.parseInt(infod[1])){
            case 200:
               
                alert = new Alert(AlertType.INFORMATION);
                alert.setTitle(String.format("Se creo el contenedor usando: %s", command));
            case 332:
                alert = new Alert(AlertType.WARNING);
                alert.setTitle(String.format(infod[0]));
            case 335:
                alert = new Alert(AlertType.WARNING);
                alert.setTitle(String.format(infod[0]));
            case 766:
                alert = new Alert(AlertType.ERROR);
                alert.setTitle(String.format(infod[0]));
        }
        alert.setResizable(false);
        alert.setContentText(infod[0]);
        DialogPane pane = alert.getDialogPane();
        pane.setMaxHeight(Region.USE_PREF_SIZE);
        pane.getStylesheets().add(
                getClass().getResource("/cifrarergui/style.css").toExternalForm()
        );
        alert.show();
    }
    
    private void jsonSession() throws IOException{
        JSONObject jsob = new JSONObject(DiskcReq.diskSessidAction());
        int disponible = (int) Float.parseFloat(jsob.get("dispo").toString());
       
        ObservableList<String> valor = FXCollections.observableArrayList();

        for (int i = 5; i <= disponible; i += 5){
            valor.add(i + " GB");
        }
        
        SpinnerValueFactory.ListSpinnerValueFactory<String> factory = new SpinnerValueFactory.ListSpinnerValueFactory<>(valor);

        factory.setValue(valor.get(0));

        memdisps.setValueFactory(factory);
        memdisps.setEditable(true);
        
    }
    
    private void errorData(int dataError, String mensaje){
        Alert alert = null;
        switch (dataError){
            case 1:
                alert = new Alert(AlertType.WARNING);
                alert.setTitle("WARNING!!!");
            case 2:
                alert = new Alert(AlertType.ERROR);
                alert.setTitle("ERROR!!");
            case 3:
                alert = new Alert(AlertType.INFORMATION);
                alert.setTitle("INFORMACIÓN!!");
        }
        alert.setResizable(false);
        alert.setContentText(mensaje);
        DialogPane pane = alert.getDialogPane();
        pane.setMaxHeight(Region.USE_PREF_SIZE);
        pane.getStylesheets().add(
                getClass().getResource("/cifrarergui/style.css").toExternalForm()
        );
        alert.show();
        
    }
    
    private boolean camposData(TextInputControl... input){
        boolean valid = true;
        for(TextInputControl datos : input){
            if (datos.getText().trim().isEmpty()){
                datos.setStyle("-fx-border-color: red;");
                valid = false;
            }else{
                datos.setStyle("");
            }
        }
        return valid;
    }
 
}
