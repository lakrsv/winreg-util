const HKEY_LOCAL_MACHINE: &str = "HKEY_LOCAL_MACHINE";
const HKEY_LOCAL_MACHINE_SHORT: &str = "HKLM";
const HKEY_CURRENT_CONFIG: &str = "HKEY_CURRENT_CONFIG";
const HKEY_CURRENT_CONFIG_SHORT: &str = "HKCC";
const HKEY_CLASSES_ROOT: &str = "HKEY_CLASSES_ROOT";
const HKEY_CLASSES_ROOT_SHORT: &str = "HKCR";
const HKEY_CURRENT_USER: &str = "HKEY_CURRENT_USER";
const HKEY_CURRENT_USER_SHORT: &str = "HKCU";
const HKEY_USERS: &str = "HKEY_USERS";
const HKEY_USERS_SHORT: &str = "HKU";
const HKEY_PERFORMANCE_DATA: &str = "HKEY_PERFORMANCE_DATA";
const HKEY_PERFORMANCE_DATA_SHORT: &str = "HKPD";
const HKEY_DYN_DATA: &str = "HKEY_DYN_DATA";
const HKEY_DYN_DATA_SHORT: &str = "HKDD";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Key {
    HkeyLocalMachine,
    HkeyCurrentConfig,
    HkeyClassesRoot,
    HkeyCurrentUser,
    HkeyUsers,
    HkeyPerformanceData,
    HkeyDynData,
}

impl Key {
    pub fn get_name(&self) -> &str {
        match self {
            Key::HkeyLocalMachine => HKEY_LOCAL_MACHINE,
            Key::HkeyCurrentConfig => HKEY_CURRENT_CONFIG,
            Key::HkeyClassesRoot => HKEY_CLASSES_ROOT,
            Key::HkeyCurrentUser => HKEY_CURRENT_USER,
            Key::HkeyUsers => HKEY_USERS,
            Key::HkeyPerformanceData => HKEY_PERFORMANCE_DATA,
            Key::HkeyDynData => HKEY_DYN_DATA,
        }
    }
    pub fn get_name_short(&self) -> &str {
        match self {
            Key::HkeyLocalMachine => HKEY_LOCAL_MACHINE_SHORT,
            Key::HkeyCurrentConfig => HKEY_CURRENT_CONFIG_SHORT,
            Key::HkeyClassesRoot => HKEY_CLASSES_ROOT_SHORT,
            Key::HkeyCurrentUser => HKEY_CURRENT_USER_SHORT,
            Key::HkeyUsers => HKEY_USERS_SHORT,
            Key::HkeyPerformanceData => HKEY_PERFORMANCE_DATA_SHORT,
            Key::HkeyDynData => HKEY_DYN_DATA_SHORT,
        }
    }
}

impl TryFrom<&str> for Key {
    type Error = KeyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let root = value.split('\\').map(|s| s.to_uppercase()).next();
        match root {
            None => Err(KeyParseError {
                msg: format!("No registry key could be parsed from {}", value),
            }),
            Some(key) => match key.as_str() {
                HKEY_LOCAL_MACHINE | HKEY_LOCAL_MACHINE_SHORT => Ok(Key::HkeyLocalMachine),
                HKEY_CURRENT_CONFIG | HKEY_CURRENT_CONFIG_SHORT => Ok(Key::HkeyCurrentConfig),
                HKEY_CLASSES_ROOT | HKEY_CLASSES_ROOT_SHORT => Ok(Key::HkeyClassesRoot),
                HKEY_CURRENT_USER | HKEY_CURRENT_USER_SHORT => Ok(Key::HkeyCurrentUser),
                HKEY_USERS | HKEY_USERS_SHORT => Ok(Key::HkeyUsers),
                HKEY_PERFORMANCE_DATA | HKEY_PERFORMANCE_DATA_SHORT => Ok(Key::HkeyPerformanceData),
                HKEY_DYN_DATA | HKEY_DYN_DATA_SHORT => Ok(Key::HkeyDynData),
                invalid_key => Err(KeyParseError {
                    msg: format!("No root key with name '{}' exists", invalid_key),
                }),
            },
        }
    }
}

#[derive(Debug)]
pub struct KeyParseError {
    msg: String,
}

impl KeyParseError {
    pub fn msg(&self) -> &str {
        &self.msg
    }
}
