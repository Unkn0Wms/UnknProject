use std::{fs, path::PathBuf};

use vdf_reader::{entry::Table, Reader};
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
    RegKey,
};

#[derive(Debug, Clone)]
pub struct SteamAccount {
    pub username: String,
    pub name: String,
}

impl SteamAccount {
    fn locate_steam() -> Result<PathBuf, String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let installation_regkey = hklm
            .open_subkey_with_flags("SOFTWARE\\Wow6432Node\\Valve\\Steam", KEY_READ)
            .or_else(|_| hklm.open_subkey_with_flags("SOFTWARE\\Valve\\Steam", KEY_READ))
            .map_err(|e| format!("Failed to open Steam registry key: {e}"))?;

        installation_regkey
            .get_value::<String, _>("InstallPath")
            .map(PathBuf::from)
            .map_err(|e| format!("Failed to get InstallPath: {e}"))
    }

    fn parse_user() -> Result<Self, String> {
        let path = Self::locate_steam()?.join("config/loginusers.vdf");
        let raw =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read loginusers.vdf: {e}"))?;

        let file = Table::load(&mut Reader::from(raw.as_str()))
            .map_err(|e| format!("Failed to parse VDF: {e}"))?;
        let users = file
            .get("users")
            .and_then(|u| u.as_table())
            .ok_or("Missing or invalid users table")?;

        users
            .iter()
            .find_map(|(_, user_data)| {
                let user_info = user_data.as_table()?;
                let username = user_info.get("AccountName")?.as_str()?;
                let name = user_info.get("PersonaName")?.as_str()?;

                if user_info.get("MostRecent")?.as_str() == Some("1") {
                    log::info!("Parsed steam user: {}", name);

                    Some(Self {
                        username: username.to_owned(),
                        name: name.to_owned(),
                    })
                } else {
                    None
                }
            })
            .ok_or_else(|| "No recent user found".to_string())
    }

    pub fn new() -> Result<Self, String> {
        Self::parse_user()
    }

    pub fn default() -> Self {
        Self {
            username: "unknown".to_string(),
            name: "unknown".to_string(),
        }
    }
}
