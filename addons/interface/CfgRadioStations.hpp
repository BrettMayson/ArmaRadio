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
    class Hirschmilch {
        name = "Hirschmilch";
        url = "http://hirschmilch.de:7000/stream/5/";
    };
    class BBC {
        name = "BBC Radio";
        class BBCRadioOne {
            name = "BBC - Radio One";
            url = "http://as-hls-ww-live.akamaized.net/pool_01505109/live/ww/bbc_radio_one/bbc_radio_one.isml/bbc_radio_one-audio%3d96000.norewind.m3u8";
        };
        class BBCRadio1Xtra {
            name = "BBC - Radio 1Xtra";
            url = "http://as-hls-ww-live.akamaized.net/pool_92079267/live/ww/bbc_1xtra/bbc_1xtra.isml/bbc_1xtra-audio%3d96000.norewind.m3u8";
        };
        class BBCRadio1Dance {
            name = "BBC - Radio 1Dance";
            url = "http://as-hls-ww-live.akamaized.net/pool_62063831/live/ww/bbc_radio_one_dance/bbc_radio_one_dance.isml/bbc_radio_one_dance-audio%3d96000.norewind.m3u8";
        };
        class BBCRadio2 {
            name = "BBC - Radio 2";
            url = "http://as-hls-ww-live.akamaized.net/pool_74208725/live/ww/bbc_radio_two/bbc_radio_two.isml/bbc_radio_two-audio%3d96000.norewind.m3u8";
        };
        class BBCRadio6Music {
            name = "BBC - Radio 6 Music";
            url = "http://as-hls-ww-live.akamaized.net/pool_81827798/live/ww/bbc_6music/bbc_6music.isml/bbc_6music-audio%3d96000.norewind.m3u8";
        };
    };
    class BBCWorldService {
        name = "BBC - World Service";
        url = "http://stream.live.vc.bbcmedia.co.uk/bbc_world_service";
    };
};
