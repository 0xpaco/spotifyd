use librespot::playback::mixer::{AudioFilter, Mixer};
use alsa;

pub struct AlsaMixer {
    pub device: String,
    pub mixer: String,
}

impl Mixer for AlsaMixer {
    fn open() -> AlsaMixer {
        AlsaMixer {
            device: "default".to_string(),
            mixer: "Master".to_string(),
        }
    }
    fn start(&self) {}
    fn stop(&self) {}

    fn volume(&self) -> u16 {
        let selem_id = alsa::mixer::SelemId::new(&*self.mixer, 0);
        match alsa::mixer::Mixer::new(&self.device, false)
            .ok()
            .as_ref()
            .and_then(|ref mixer| mixer.find_selem(&selem_id))
            .and_then(|elem| {
                let (min, max) = elem.get_playback_volume_range();
                elem.get_playback_volume(alsa::mixer::SelemChannelId::mono())
                    .ok()
                    .map(|volume| {
                        let volume_steps = max - min + 1;
                        ((volume - min) * (0xFFFF / volume_steps)) as u16
                    })
            }) {
            Some(vol) => vol,
            _ => {
                error!(
                    "Couldn't read volume from alsa device with name \"{}\".",
                    self.device
                );
                0
            }
        }
    }

    fn set_volume(&self, volume: u16) {
        match alsa::mixer::Mixer::new(&self.device, false)
            .ok()
            .and_then(|mixer| {
                let selem_id = alsa::mixer::SelemId::new(&*self.mixer, 0);
                mixer.find_selem(&selem_id).and_then(|elem| {
                    let (min, max) = elem.get_playback_volume_range();

                    let volume_steps = max - min + 1;
                    let normalised_volume: i64 =
                        min + ((volume as i64) / (0xFFFF / volume_steps)) as i64;

                    elem.set_playback_volume_all(normalised_volume).ok()
                })
            }) {
            Some(_) => (),
            _ => error!("Couldn't set volume of alsa device."),
        };
    }

    fn get_audio_filter(&self) -> Option<Box<AudioFilter + Send>> {
        None
    }
}