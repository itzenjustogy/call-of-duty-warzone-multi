use rand::Rng;

pub struct Features {
    aimbot_enabled: bool,
    esp_enabled: bool,
    no_recoil_enabled: bool,
}

impl Features {
    pub fn new() -> Self {
        Features {
            aimbot_enabled: false,
            esp_enabled: false,
            no_recoil_enabled: false,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    pub fn toggle_esp(&mut self) {
        self.esp_enabled = !self.esp_enabled;
    }

    pub fn toggle_no_recoil(&mut self) {
        self.no_recoil_enabled = !self.no_recoil_enabled;
    }

    pub fn randomize_settings(&mut self) {
        let mut rng = rand::thread_rng();
        self.aimbot_enabled = rng.gen_bool(0.5);
        self.esp_enabled = rng.gen_bool(0.5);
        self.no_recoil_enabled = rng.gen_bool(0.5);
    }
}