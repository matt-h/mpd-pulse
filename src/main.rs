use config::{Config, File, FileFormat};
use dirs;
use std::path::Path;
use std::process::Command;

use pulsectl::controllers::AppControl;
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

fn mpd_wait() {
    Command::new("mpc")
        .arg("idle")
        .status()
        .expect("failed to execute process");
}

fn main() {
    let mut config = Config::new();

    let config_path = Path::new("/etc/mpd-pulse.conf");

    // Start off by merging in the "default" configuration file
    config
        .merge(
            File::from(config_path)
                .format(FileFormat::Ini)
                .required(false),
        )
        .unwrap();

    let mut user_config_path = dirs::config_dir().unwrap();
    user_config_path.push("mpd-pulse.conf");

    // Load in user config
    config
        .merge(
            File::from(user_config_path)
                .format(FileFormat::Ini)
                .required(false),
        )
        .unwrap();

    let device_name = match config.get::<String>("device_name") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("error: {:?}", err);
            // TODO: Not sure how to use this from above.
            let mut user_config_path = dirs::config_dir().unwrap();
            user_config_path.push("mpd-pulse.conf");
            eprintln!(
                "Make sure you have device_name defined in a config file at {:?} or {:?}",
                user_config_path, config_path
            );
            std::process::exit(1);
        }
    };
    let mpd_name = match config.get::<String>("mpd_name") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("error: {:?}", err);
            // TODO: Not sure how to use this from above.
            let mut user_config_path = dirs::config_dir().unwrap();
            user_config_path.push("mpd-pulse.conf");
            eprintln!(
                "Make sure you have mpd_name defined in a config file at {:?} or {:?}",
                user_config_path, config_path
            );
            std::process::exit(1);
        }
    };

    let mut handler = SinkController::create();

    loop {
        let device = handler.get_device_by_name(&device_name).ok();
        if device.is_none() {
            println!("Playback Device {} not found", device_name);
            continue;
        }
        let applications = handler
            .list_applications()
            .expect("Error getting application list");

        let mpd = applications
            .iter()
            .find(|&app| app.proplist.get_str("application.name").unwrap() == mpd_name);
        if mpd.is_none() {
            continue;
        }
        match handler.move_app_by_index(mpd.unwrap().index, device.unwrap().index) {
            Err(_e) => continue,
            _ => (),
        }

        mpd_wait();
    }
}
