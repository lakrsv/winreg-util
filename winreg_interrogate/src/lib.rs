use std::path::PathBuf;
use winreg::RegKey;

pub fn test(){
    dbg!(RegKey::load_app_key(PathBuf::from("C://Users/lakrs/Documents/CreativeWork/RustProjects/winreg_util/export-test/HKEY_LOCAL_MACHINE-SOFTWARE.reg"), false));
}
