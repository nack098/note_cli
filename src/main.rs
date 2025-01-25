use app_dirs2::*;
use note_cli::run;

const APP_INFO: AppInfo = AppInfo {
    name: "note_cli",
    author: "Nakuya",
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path: std::path::PathBuf = get_app_dir(AppDataType::UserData, &APP_INFO, "note").unwrap();
    run(args, path)
}
