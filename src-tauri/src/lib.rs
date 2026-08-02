// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use bcrypt::{hash, verify, DEFAULT_COST};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct PasswordManager {
    #[serde(skip)]
    state: State,
    master_password: String,
    #[serde(skip)]
    encryption_key: Option<String>,
    password: HashMap<String, Password>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    username: String,
    password: String,
}

#[derive(Debug, PartialEq, Default)]
enum State {
    #[default]
    Locked,
    Unlocked,
}

#[tauri::command]
fn launch_program(state: tauri::State<'_, Mutex<Option<PasswordManager>>>) -> Result<(), String> {
    let file_path = get_full_file_path("/.config/password-manager")
        .expect("Couldn't find the HOME env variable.");

    let _ = create_folder(&file_path);

    if let Ok(existing_manager) = PasswordManager::load(file_path) {
        let mut manager = state.lock().unwrap();
        *manager = Some(existing_manager)
    }
    Ok(())
}
#[tauri::command]
fn is_first_launch(state: tauri::State<'_, Mutex<Option<PasswordManager>>>) -> bool {
    let manager = state.lock().unwrap();

    manager.is_none()
}

#[tauri::command]
fn create_first_password(
    master_pass: String,
    state: tauri::State<'_, Mutex<Option<PasswordManager>>>,
) -> Result<(), String> {
    let mut manager = state.lock().unwrap();

    let mut new_manager = PasswordManager::new(master_pass.clone());

    let file_path = get_full_file_path("/.config/password-manager")
        .expect("Couldn't find the HOME env variable.");
    new_manager
        .save_config_file(file_path)
        .map_err(|err| format!("Couldn't save the config file : {err}"))?;
    new_manager.open_manager(master_pass).unwrap();

    *manager = Some(new_manager);

    Ok(())
}

#[tauri::command]
fn check_password(
    master_pass: String,
    state: tauri::State<'_, Mutex<Option<PasswordManager>>>,
) -> Result<bool, String> {
    let mut manager = state.lock().unwrap();

    if let Some(manager) = manager.as_mut() {
        match manager.open_manager(master_pass) {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Manager not initialized yet.".to_string())
    }
}

#[tauri::command]
fn password_to_vec(
    state: tauri::State<'_, Mutex<Option<PasswordManager>>>,
) -> Result<Vec<(String, String, String)>, String> {
    let mut password_vec: Vec<(String, String, String)> = Vec::new();
    let manager_guard = state.lock().unwrap();

    if let Some(manager) = manager_guard.as_ref() {
        let key = manager
            .encryption_key
            .as_ref()
            .ok_or("Erreur : Cannot find the encryption key")?;

        let mc = new_magic_crypt!(key, 256);

        for (url, entry) in manager.password.iter() {
            let decrypted_password = decrypt_password(entry.password.clone(), mc.clone());

            password_vec.push((url.clone(), entry.username.clone(), decrypted_password));
        }
    } else {
        return Err("Error : Vault is not open yet.".to_string());
    }

    Ok(password_vec)
}

#[tauri::command]
fn add_password(
    state: tauri::State<'_, Mutex<Option<PasswordManager>>>,
    url: String,
    username: String,
    password: String,
) -> Result<(), String> {
    let mut manager_guard = state.lock().unwrap();

    if let Some(manager) = manager_guard.as_mut() {
        let key = manager.encryption_key.as_ref().unwrap();
        let mc = new_magic_crypt!(key, 256);

        let new_password = Password {
            username,
            password: encrypt_password(password, mc),
        };

        manager.password.insert(url, new_password);
        Ok(())
    } else {
        Err("error if there is no manager open!".to_string())
    }
}

#[tauri::command]
fn delete_entry(
    state: tauri::State<'_, Mutex<Option<PasswordManager>>>,
    url: String,
) -> Result<(), String> {
    let mut manager_guard = state.lock().unwrap();

    if let Some(manager) = manager_guard.as_mut() {
        if manager.password.remove(&url).is_some() {
            Ok(())
        } else {
            return Err("Url not found.".to_string());
        }
    } else {
        Err("Could not acces the manager".to_string())
    }
}

#[tauri::command]
fn modify_entry(
    state: tauri::State<'_, Mutex<Option<PasswordManager>>>,
    url: String,
    username: String,
    new_password: String,
) -> Result<(), String> {
    let mut manager_guard = state.lock().unwrap();

    if let Some(manager) = manager_guard.as_mut() {
        if let Some(entry) = manager.password.get_mut(&url) {
            let key = manager.encryption_key.as_ref().unwrap();
            let mc = new_magic_crypt!(key, 256);

            entry.username = username;
            entry.password = encrypt_password(new_password, mc);

            Ok(())
        } else {
            Err("Url not found.".to_string())
        }
    } else {
        Err("Couldn't acces manager".to_string())
    }
}

impl PasswordManager {
    pub fn new(master_password: String) -> Self {
        PasswordManager {
            state: State::Locked,
            master_password: hash(master_password, DEFAULT_COST)
                .expect("Error hashing the master password."),
            password: HashMap::new(),
            encryption_key: None,
        }
    }

    pub fn load(path: String) -> Result<Self, &'static str> {
        let new_path = format!("{}/passwords.json", path);
        if let Ok(json_data) = fs::read_to_string(new_path) {
            // File exists
            if let Ok(mut manager) = serde_json::from_str::<PasswordManager>(&json_data) {
                // We managed to read it properly
                manager.state = State::Locked;
                manager.encryption_key = None;
                // no need for the master pass and the passwords because serde did it for us by serialize it from the json file.
                return Ok(manager);
            } else {
                return Err("Error reading from json file, serde?");
            }
        }
        Err("Error loading the password.json file.")
    }

    pub fn open_manager(&mut self, master_pass: String) -> Result<(), &'static str> {
        if verify(&master_pass, &self.master_password)
            .expect("Error hashing password to verify it.")
        {
            self.unlock_manager(master_pass);
            Ok(())
        } else {
            Err("Wrong password.")
        }
    }

    fn unlock_manager(&mut self, master_pass: String) {
        self.state = State::Unlocked;
        self.encryption_key = Some(master_pass);
    }

    pub fn close_manager(&mut self, path: String) {
        self.state = State::Locked;
        self.encryption_key = None;
        self.save_config_file(path).expect("Error saving the file.")
    }

    pub fn add_password(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), String> {
        if self.state == State::Locked {
            Err("The manager is locked.".to_owned())
        } else {
            let key = self.encryption_key.as_ref().unwrap();
            let mc = new_magic_crypt!(key, 256);

            let new_password = Password {
                username,
                password: encrypt_password(password, mc),
            };

            self.password.insert(url, new_password);
            Ok(())
        }
    }

    pub fn modify_password(
        &mut self,
        url: String,
        username: String,
        new_password: String,
    ) -> Result<(), &'static str> {
        if self.state == State::Locked {
            return Err("The manager is locked.");
        }
        if let Some(entry) = self.password.get_mut(&url) {
            let key = self.encryption_key.as_ref().unwrap();
            let mc = new_magic_crypt!(key, 256);

            entry.username = username;
            entry.password = encrypt_password(new_password, mc);

            Ok(())
        } else {
            Err("Url not found.")
        }
    }

    pub fn delete_password(&mut self, url: String) -> Result<(), &'static str> {
        if self.state == State::Locked {
            return Err("The manager is locked.");
        }
        if self.password.remove(&url).is_some() {
            Ok(())
        } else {
            Err("Url not found.")
        }
    }

    pub fn list_passwords(&self) -> Result<(), &'static str> {
        if self.state == State::Locked {
            return Err("The manager is locked.");
        }
        let key = self.encryption_key.as_ref().unwrap();
        let mc = new_magic_crypt!(key, 256);

        for (url, entry) in self.password.iter() {
            let decrypted_password = decrypt_password(entry.password.clone(), mc.clone());

            println!(
                "URL : {} | username : {} | password : {}",
                url, entry.username, decrypted_password
            );
        }

        Ok(())
    }

    fn save_config_file(&self, path: String) -> Result<(), &'static str> {
        if self.state == State::Unlocked {
            Err("The manager is unlocked, you must lock it before saving the file.")
        } else {
            let new_config_path = format!("{}/passwords.json", path);
            let json_data =
                serde_json::to_string_pretty(self).expect("Error serializing to the json format.");

            fs::write(new_config_path, json_data)
                .expect("Error trying to save the file on the disk.");
            Ok(())
        }
    }
}

pub fn get_full_file_path(relative_path: &str) -> Result<String, &'static str> {
    if let Ok(home) = env::var("HOME") {
        Ok(format!("{}{}", home, relative_path))
    } else {
        Err("Couldn't find the HOME env variable.")
    }
}

pub fn create_folder(path: &str) -> Result<(), String> {
    if fs::create_dir_all(path).is_err() {
        return Err(format!("Error creating the {} folder", path));
    }
    Ok(())
}

fn encrypt_password(password: String, key: MagicCrypt256) -> String {
    key.encrypt_str_to_base64(password)
}

fn decrypt_password(password: String, key: MagicCrypt256) -> String {
    key.decrypt_base64_to_string(password)
        .expect("Error decrypting the password.")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_state: Mutex<Option<PasswordManager>> = Mutex::new(None);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(initial_state)
        .invoke_handler(tauri::generate_handler![
            launch_program,
            check_password,
            is_first_launch,
            create_first_password,
            password_to_vec,
            add_password,
            delete_entry,
            modify_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
