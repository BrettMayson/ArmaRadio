use std::time::Duration;

#[test]
fn radio() {
    dynulo_radio::init();
    dynulo_radio::create(
        "http://pulseedm.cdnstream1.com:8124/1373_128",
        "test",
        1.0f32,
    );
    dynulo_radio::pos("test", 0f32, 0f32, 0f32);
    std::thread::sleep(Duration::from_secs(5));
    dynulo_radio::destroy("test");
    std::thread::sleep(Duration::from_secs(2));
}
