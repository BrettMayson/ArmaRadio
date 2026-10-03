// Original layout: BrettMayson and mharis001. Added controls: Joncantplay.
class GVAR(displayClassic): GVAR(display) {
    class controls: controls {
        class Background: Background {
            x = QUOTE(CENTER_X - GRID_W(70));
            y = QUOTE(CENTER_Y - GRID_H(55));
            w = QUOTE(GRID_W(140));
            h = QUOTE(GRID_H(124));
            colorBackground[] = {0.16, 0.16, 0.16, 0.96};
        };
        class Footer: Footer {
            x = QUOTE(CENTER_X - GRID_W(70));
            y = QUOTE(CENTER_Y + GRID_H(58));
            w = QUOTE(GRID_W(140));
            h = QUOTE(GRID_H(11));
            colorBackground[] = {0.1, 0.1, 0.1, 1};
        };
        class Title: Title {
            x = QUOTE(CENTER_X - GRID_W(70));
            y = QUOTE(CENTER_Y - GRID_H(55));
            w = QUOTE(GRID_W(140));
            h = QUOTE(GRID_H(5));
        };
        class List: List {
            x = QUOTE(CENTER_X - GRID_W(69));
            y = QUOTE(CENTER_Y - GRID_H(49));
            w = QUOTE(GRID_W(79));
            h = QUOTE(GRID_H(90));
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class SearchButton: SearchButton {
            x = QUOTE(CENTER_X - GRID_W(69));
            y = QUOTE(CENTER_Y + GRID_H(42));
            w = QUOTE(GRID_W(5));
            h = QUOTE(GRID_H(5));
        };
        class SearchBar: SearchBar {
            x = QUOTE(CENTER_X - GRID_W(63));
            y = QUOTE(CENTER_Y + GRID_H(42));
            w = QUOTE(GRID_W(51));
            h = QUOTE(GRID_H(5));
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class Presets: Presets {
            x = QUOTE(CENTER_X - GRID_W(10));
            y = QUOTE(CENTER_Y + GRID_H(42));
            w = QUOTE(GRID_W(20));
            h = QUOTE(GRID_H(5));
        };
        class DetailsBackground: DetailsBackground {
            x = QUOTE(CENTER_X + GRID_W(11));
            y = QUOTE(CENTER_Y - GRID_H(49));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(106));
            colorBackground[] = {0, 0, 0, 0};
        };
        class PictureBackground: PictureBackground {
            x = QUOTE(CENTER_X + GRID_W(15));
            y = QUOTE(CENTER_Y - GRID_H(45));
            w = QUOTE(GRID_W(50));
            h = QUOTE(GRID_H(50));
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class Picture: Picture {
            x = QUOTE(CENTER_X + GRID_W(15));
            y = QUOTE(CENTER_Y - GRID_H(45));
            w = QUOTE(GRID_W(50));
            h = QUOTE(GRID_H(50));
        };
        class PictureDefault: PictureDefault {
            x = QUOTE(CENTER_X + GRID_W(15));
            y = QUOTE(CENTER_Y - GRID_H(45));
            w = QUOTE(GRID_W(50));
            h = QUOTE(GRID_H(50));
            colorText[] = {0.2, 0.2, 0.2, 0.5};
        };
        class Name: Name {
            x = QUOTE(CENTER_X + GRID_W(11));
            y = QUOTE(CENTER_Y + GRID_H(7));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(7));
        };
        class Description: Description {
            x = QUOTE(CENTER_X + GRID_W(11));
            y = QUOTE(CENTER_Y + GRID_H(15));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(7));
        };
        class CopyTitle: CopyTitle {
            x = QUOTE(CENTER_X + GRID_W(11));
            y = QUOTE(CENTER_Y + GRID_H(15));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(7));
        };
        class Status: Status {
            x = QUOTE(CENTER_X + GRID_W(11));
            y = QUOTE(CENTER_Y + GRID_H(22));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(12));
            colorBackground[] = {0.04, 0.055, 0.07, 1};
        };
        class Retry: Retry {
            x = QUOTE(CENTER_X + GRID_W(11));
            y = QUOTE(CENTER_Y + GRID_H(35));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(5));
        };
        class Empty: Empty {
            x = QUOTE(CENTER_X - GRID_W(68));
            y = QUOTE(CENTER_Y - GRID_H(16));
            w = QUOTE(GRID_W(77));
            h = QUOTE(GRID_H(8));
        };
        class Power: Power {
            x = QUOTE(CENTER_X + GRID_W(17));
            y = QUOTE(CENTER_Y + GRID_H(41));
            w = QUOTE(GRID_W(10));
            h = QUOTE(GRID_H(10));
        };
        class Mute: Mute {
            x = QUOTE(CENTER_X + GRID_W(30));
            y = QUOTE(CENTER_Y + GRID_H(42));
            w = QUOTE(GRID_W(37));
            h = QUOTE(GRID_H(7));
        };
        class VolumeIcon: VolumeIcon {
            x = QUOTE(CENTER_X + GRID_W(13));
            y = QUOTE(CENTER_Y + GRID_H(51));
            w = QUOTE(GRID_W(5));
            h = QUOTE(GRID_H(5));
        };
        class VolumeBarBackground: VolumeBarBackground {
            x = QUOTE(CENTER_X + GRID_W(19));
            y = QUOTE(CENTER_Y + GRID_H(52));
            w = QUOTE(GRID_W(36));
            h = QUOTE(GRID_H(4));
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class VolumeBarFill: VolumeBarFill {
            x = QUOTE(CENTER_X + GRID_W(19));
            y = QUOTE(CENTER_Y + GRID_H(52));
            w = QUOTE(GRID_W(36));
            h = QUOTE(GRID_H(4));
            colorBackground[] = {1, 1, 1, 1};
        };
        class VolumeBarMouse: VolumeBarMouse {
            x = QUOTE(CENTER_X + GRID_W(19));
            y = QUOTE(CENTER_Y + GRID_H(52));
            w = QUOTE(GRID_W(36));
            h = QUOTE(GRID_H(4));
        };
        class VolumeText: VolumeText {
            x = QUOTE(CENTER_X + GRID_W(57));
            y = QUOTE(CENTER_Y + GRID_H(51));
            w = QUOTE(GRID_W(12));
            h = QUOTE(GRID_H(6));
        };
        class StreamerStatus: StreamerStatus {
            x = QUOTE(CENTER_X - GRID_W(67));
            y = QUOTE(CENTER_Y + GRID_H(60));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(6));
        };
        class StreamerToggle: StreamerToggle {
            x = QUOTE(CENTER_X - GRID_W(7));
            y = QUOTE(CENTER_Y + GRID_H(60));
            w = QUOTE(GRID_W(48));
            h = QUOTE(GRID_H(7));
        };
        class ButtonOK: ButtonOK {
            x = QUOTE(CENTER_X + GRID_W(44));
            y = QUOTE(CENTER_Y + GRID_H(60));
            w = QUOTE(GRID_W(25));
            h = QUOTE(GRID_H(7));
        };
    };
};
