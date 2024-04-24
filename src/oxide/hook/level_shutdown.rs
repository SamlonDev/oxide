use crate::{define_hook, sdk::client_mode::ClientMode};

fn subhooks(hook: &mut LevelShutdownHook) {
    hook.before = Some(|_| {
        o!().last_entity_cache = None;
        None
    });
}

define_hook!(
    LevelShutdownHook,
    "LevelShutdown",
    (),
    (),
    subhooks,
    client_mode,
    &ClientMode
);
