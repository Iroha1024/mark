use std::{env, error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};

pub fn create_config() -> Result<PathBuf, Box<dyn Error>> {
    let mut cwd = env::current_exe().unwrap();
    cwd.pop();
    let json_path = cwd.join("config.json");
    if !json_path.exists() {
        let value = json!({
            "storage": {
                "public": [
                    {
                        "name": "winrar",
                        "url": "https://www.win-rar.com/fileadmin/winrar-versions/winrar/winrar-x64-602.exe"
                    },
                    {
                        "name": "node",
                        "url": "https://nodejs.org/dist/v14.18.0/node-v14.18.0-x64.msi"
                    },
                    {
                        "name": "网易云音乐",
                        "url": "https://d1.music.126.net/dmusic/cloudmusicsetup2.9.4.199325.exe"
                    },
                    {
                        "name": "AutoHideDesktopIcons",
                        "url": "http://www.softwareok.com/Download/AutoHideDesktopIcons.zip"
                    },
                    {
                        "name": "StartIsBackPlusPlus",
                        "url": "https://s3.amazonaws.com/startisback/StartIsBackPlusPlus_setup.exe"
                    },
                    {
                        "name": "vscode",
                        "url": "https://az764295.vo.msecnd.net/stable/7f6ab5485bbc008386c4386d08766667e155244e/VSCodeUserSetup-x64-1.60.2.exe"
                    },
                    {
                        "name": "GitHubDesktop",
                        "url": "https://desktop.githubusercontent.com/github-desktop/releases/2.9.4-24101633/GitHubDesktopSetup-x64.exe"
                    },
                    {
                        "name": "typora",
                        "url": "https://www.typora.io/windows/typora-setup-x64.exe?"
                    },
                ],
                "private": []
            }
        });
        println!("{}", to_string_pretty(&value).unwrap());
        fs::write(&json_path, to_string_pretty(&value).unwrap())?;
    }
    Ok(json_path)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub storage: Storage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Storage {
    pub public: Vec<StorageItem>,
    pub private: Vec<StorageItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageItem {
    pub name: String,
    pub url: String,
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let config = serde_json::from_str(data.as_str())?;
    Ok(config)
}

pub fn write_config(path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    fs::write(path, to_string_pretty(&config).unwrap())?;
    Ok(())
}
