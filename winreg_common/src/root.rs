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
pub enum RootKey {
    HkeyLocalMachine,
    HkeyCurrentConfig,
    HkeyClassesRoot,
    HkeyCurrentUser,
    HkeyUsers,
    HkeyPerformanceData,
    HkeyDynData,
}

impl RootKey {
    pub fn get_name(&self) -> &str {
        match self {
            RootKey::HkeyLocalMachine => HKEY_LOCAL_MACHINE,
            RootKey::HkeyCurrentConfig => HKEY_CURRENT_CONFIG,
            RootKey::HkeyClassesRoot => HKEY_CLASSES_ROOT,
            RootKey::HkeyCurrentUser => HKEY_CURRENT_USER,
            RootKey::HkeyUsers => HKEY_USERS,
            RootKey::HkeyPerformanceData => HKEY_PERFORMANCE_DATA,
            RootKey::HkeyDynData => HKEY_DYN_DATA,
        }
    }
    pub fn get_name_short(&self) -> &str {
        match self {
            RootKey::HkeyLocalMachine => HKEY_LOCAL_MACHINE_SHORT,
            RootKey::HkeyCurrentConfig => HKEY_CURRENT_CONFIG_SHORT,
            RootKey::HkeyClassesRoot => HKEY_CLASSES_ROOT_SHORT,
            RootKey::HkeyCurrentUser => HKEY_CURRENT_USER_SHORT,
            RootKey::HkeyUsers => HKEY_USERS_SHORT,
            RootKey::HkeyPerformanceData => HKEY_PERFORMANCE_DATA_SHORT,
            RootKey::HkeyDynData => HKEY_DYN_DATA_SHORT,
        }
    }
}

impl TryFrom<&str> for RootKey {
    type Error = RootKeyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let root = value.split('\\').map(|s| s.to_uppercase()).next();
        match root {
            None => Err(RootKeyParseError {
                msg: format!("No registry key could be parsed from {}", value),
            }),
            Some(key) => match key.as_str() {
                HKEY_LOCAL_MACHINE | HKEY_LOCAL_MACHINE_SHORT => Ok(RootKey::HkeyLocalMachine),
                HKEY_CURRENT_CONFIG | HKEY_CURRENT_CONFIG_SHORT => Ok(RootKey::HkeyCurrentConfig),
                HKEY_CLASSES_ROOT | HKEY_CLASSES_ROOT_SHORT => Ok(RootKey::HkeyClassesRoot),
                HKEY_CURRENT_USER | HKEY_CURRENT_USER_SHORT => Ok(RootKey::HkeyCurrentUser),
                HKEY_USERS | HKEY_USERS_SHORT => Ok(RootKey::HkeyUsers),
                HKEY_PERFORMANCE_DATA | HKEY_PERFORMANCE_DATA_SHORT => {
                    Ok(RootKey::HkeyPerformanceData)
                }
                HKEY_DYN_DATA | HKEY_DYN_DATA_SHORT => Ok(RootKey::HkeyDynData),
                invalid_key => Err(RootKeyParseError {
                    msg: format!("No root key with name '{}' exists", invalid_key),
                }),
            },
        }
    }
}

#[derive(Debug)]
pub struct RootKeyParseError {
    msg: String,
}

impl RootKeyParseError {
    pub fn msg(&self) -> &str {
        &self.msg
    }
}
