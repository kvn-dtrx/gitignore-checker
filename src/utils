// ---
// description: mod.rs for Utils
// ---

// ---

use log::LevelFilter;
use once_cell::sync::OnceCell;
use std::io::Write;

static LOGGER_INIT: OnceCell<()> = OnceCell::new();

pub fn init_logger() {
    LOGGER_INIT.get_or_init(|| {
        env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();
    });
}
