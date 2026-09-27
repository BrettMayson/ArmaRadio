class CfgRadioStations {
    class ClassicRock109 {
        name = "Classic Rock 109";
        url = "http://listen.classicrock109.com:10042";
    };
    class PulseEDM {
        name = "PulseEDM Dance Music";
        url = "http://pulseedm.cdnstream1.com:8124/1373_128";
    };
    class LifeIreland {
        name = "Live Ireland";
        picture = "https://www.liveradio.ie/files/images/184739/resized/180x172c/liveireland_radio.jpg";
        url = "http://192.111.140.11:8058/stream?type=http&nocache=325927";
        condition = "missionNamespace getVariable ['LiveIrelandAvailable', false]";
    };
};
