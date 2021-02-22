use std::time::Duration;

#[test]
fn radio() {
    live_radio::init();
    live_radio::create(
        "http://pulseedm.cdnstream1.com:8124/1373_128",
        "test",
        1.0f32,
    );
    live_radio::pos("test", 0f32, 0f32, 0f32);
    std::thread::sleep(Duration::from_secs(5));
    live_radio::destroy("test");
    std::thread::sleep(Duration::from_secs(2));
}
