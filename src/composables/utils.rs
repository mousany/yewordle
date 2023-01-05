use gloo::timers::future::TimeoutFuture;
use yew::platform::spawn_local;

pub fn set_timeout<F: FnOnce() + 'static>(handler: F, timeout: u64) {
    spawn_local(async move {
        TimeoutFuture::new(timeout as u32).await;
        handler();
    });
}
