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
        picture = QPATHTOF(pictures\liveireland.paa);
        url = "http://lynx.prostreaming.net:8058/stream?type=http&nocache=325927";
        condition = "missionNamespace getVariable ['LiveIrelandAvailable', false]";
    };
};
