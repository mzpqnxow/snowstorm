#[cfg(target_arch = "mips")]
use portable_atomic::AtomicU64;


#[cfg(not(target_arch = "mips"))]
use core::sync::atomic::AtomicU64;

use std::{
    sync::{
        atomic::{Ordering},
        Once,
    },
    thread,
    time::{Duration, SystemTime},
};

static ONCE: Once = Once::new();
static TS: AtomicU64 = AtomicU64::new(0);

#[inline]
fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[inline]
pub fn timestamp() -> u64 {
    TS.load(Ordering::Relaxed)
}

pub fn init() {
    ONCE.call_once(|| {
        let interval = Duration::from_millis(200);
        TS.store(now(), Ordering::Relaxed);
        thread::spawn(move || loop {
            TS.store(now(), Ordering::Relaxed);
            thread::sleep(interval);
        });
    });
}
