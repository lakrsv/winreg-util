const HKEY_LOCAL_MACHINE: &str = "HKEY_LOCAL_MACHINE";
const HKEY_CURRENT_CONFIG: &str = "HKEY_CURRENT_CONFIG";
const HKEY_CLASSES_ROOT: &str = "HKEY_CLASSES_ROOT";
const HKEY_CURRENT_USER: &str = "HKEY_CURRENT_USER";
const HKEY_USERS: &str = "HKEY_USERS";
const HKEY_PERFORMANCE_DATA: &str = "HKEY_PERFORMANCE_DATA";
const HKEY_DYN_DATA: &str = "HKEY_DYN_DATA";

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
                HKEY_LOCAL_MACHINE => Ok(Key::HkeyLocalMachine),
                HKEY_CURRENT_CONFIG => Ok(Key::HkeyCurrentConfig),
                HKEY_CLASSES_ROOT => Ok(Key::HkeyClassesRoot),
                HKEY_CURRENT_USER => Ok(Key::HkeyCurrentUser),
                HKEY_USERS => Ok(Key::HkeyUsers),
                HKEY_PERFORMANCE_DATA => Ok(Key::HkeyPerformanceData),
                HKEY_DYN_DATA => Ok(Key::HkeyDynData),
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
