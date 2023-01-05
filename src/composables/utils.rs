use std::time::Duration;

use async_std::task::sleep;
use yew::platform::spawn_local;

pub fn set_timeout<F: FnOnce() + 'static>(handler: F, timeout: u64) {
    spawn_local(async move {
        sleep(Duration::from_millis(timeout)).await;
        handler();
    });
}
