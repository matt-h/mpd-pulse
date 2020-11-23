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
    let mut handler = SinkController::create();

    loop {
        let device_name = "alsa_output.pci-0000_00_14.2.analog-stereo";
        let mpd_name = "ALSA plug-in [mpd]";

        let device = handler.get_device_by_name(device_name).ok();
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
