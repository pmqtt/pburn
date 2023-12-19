use std::fmt::format;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn get_container_ip(container_id: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("inspect")
        .arg(container_id)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            match std::str::from_utf8(&output.stdout) {
                Ok(s) => {
                    // Parsen der JSON-Ausgabe, um die IP-Adresse zu finden
                    let json: serde_json::Value = serde_json::from_str(s)
                        .map_err(|e| format!("Fehler beim Parsen der JSON-Ausgabe: {}", e))?;

                    // Zugriff auf die IP-Adresse
                    if let Some(networks) = json[0].get("NetworkSettings").and_then(|ns| ns.get("Networks")) {
                        networks.as_object()
                            .and_then(|nws| nws.values().next())
                            .and_then(|nw| nw.get("IPAddress"))
                            .and_then(|ip| ip.as_str())
                            .map(|ip| ip.to_string())
                            .ok_or("IP-Adresse nicht gefunden".to_string())
                    } else {
                        Err("Netzwerkeinstellungen nicht gefunden".to_string())
                    }
                }
                Err(e) => Err(format!("Fehler beim Konvertieren des Outputs: {}", e)),
            }
        }
        Ok(output) => {
            // Fehlerbehandlung für den Fall, dass der Befehl nicht erfolgreich war
            match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(s.to_string()),
                Err(e) => Err(format!("Fehler beim Lesen des Fehleroutputs: {}", e)),
            }
        }
        Err(e) => Err(format!("Fehler beim Ausführen des Befehls: {}", e)),
    }
}

pub fn create_mongo_db_container(name: &str,image: &str, user: &str, pwd: &str, port: &str) -> Result<String, String> {
    let mongo_user: String = format!("MONGO_INITDB_ROOT_USERNAME={}",user);
    let mongo_pwd: String = format!("MONGO_INITDB_ROOT_PASSWORD={}",pwd);
    let docker_port: String = format!("{}:27017",port);
    let output = Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg(name)
        .arg("-e")
        .arg(mongo_user.as_str())
        .arg("-e")
        .arg(mongo_pwd.as_str())
        .arg("-p")
        .arg(docker_port.as_str())
        .arg(image)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            match std::str::from_utf8(&output.stdout) {
                Ok(s) => {
                    thread::sleep(Duration::from_secs(5));
                    Ok(s.trim().to_string())
                }, // Die Container-ID wird zurückgegeben
                Err(e) => Err(format!("Fehler beim Konvertieren des Outputs: {}", e)),
            }
        }
        Ok(output) => {
            match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(s.to_string()),
                Err(e) => Err(format!("Fehler beim Lesen des Fehleroutputs: {}", e)),
            }
        }
        Err(e) => Err(format!("Fehler beim Ausführen des Befehls: {}", e)),
    }
}
