use std::sync::atomic::{AtomicBool, Ordering};

use mod_api::{
    ClientHost, ClientTickPostEvent, Hooks, KeyConflictContext, KeyRegistration, Registrar,
    RegistrationContext, SettingChangedEvent, SettingSpec,
};

const ID: &str = "fullbright";
const KEY: &str = "toggle";
const MASTER: &str = "active";
static ACTIVE: AtomicBool = AtomicBool::new(true);

pub fn active_for_render() -> bool {
    ACTIVE.load(Ordering::Relaxed)
}

fn sync_active(host: &dyn ClientHost) -> bool {
    let active = host.setting_bool(MASTER).unwrap_or(true);
    ACTIVE.store(active, Ordering::Relaxed);
    active
}

#[derive(Default)]
struct Fullbright {
    held: AtomicBool,
}

#[mod_api::horizon_mod]
impl Hooks for Fullbright {
    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        registrar.register_key_mapping(
            KeyRegistration::new(KEY, "key.keyboard.g")
                .label("key.fullbright.toggle")
                .context(KeyConflictContext::InGame),
        );
        let _ = registrar.register_setting(SettingSpec::toggle(MASTER, "fullbright.setting.active", true));
    }

    fn client_tick_post(&self, host: &mut dyn ClientHost, _: &mut ClientTickPostEvent) {
        let active = sync_active(host);
        let down = host.is_down(ID, KEY);
        let was_down = self.held.swap(down, Ordering::Relaxed);
        let mut odd = false;
        while host.consume_click(ID, KEY) {
            odd = !odd;
        }
        if !was_down && (down || odd) && host.playing() && host.on(ID) {
            host.set_setting(ID, MASTER, if active { "false" } else { "true" });
            sync_active(host);
        }
    }

    fn setting_changed(&self, host: &mut dyn ClientHost, _: &mut SettingChangedEvent<'_>) {
        sync_active(host);
    }
}
