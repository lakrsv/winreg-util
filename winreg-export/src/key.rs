#[derive(Debug, PartialEq, Eq, Hash)]
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
    pub fn get_name(&self) -> String {
        match self {
            Key::HkeyLocalMachine => String::from("HKEY_LOCAL_MACHINE"),
            Key::HkeyCurrentConfig => String::from("HKEY_CURRENT_CONFIG"),
            Key::HkeyClassesRoot => String::from("HKEY_CLASSES_ROOT"),
            Key::HkeyCurrentUser => String::from("HKEY_CURRENT_USER"),
            Key::HkeyUsers => String::from("HKEY_USERS"),
            Key::HkeyPerformanceData => String::from("HKEY_PERFORMANCE_DATA"),
            Key::HkeyDynData => String::from("HKEY_DYN_DATA"),
        }
    }
}
