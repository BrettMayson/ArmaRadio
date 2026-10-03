class ctrlStatic;
class ctrlStaticTitle;
class ctrlStaticBackground;
class ctrlStaticFooter;
class ctrlStaticPictureKeepAspect;
class ctrlEdit;
class ctrlListbox;
class ctrlButton;
class ctrlButtonOK;
class ctrlButtonSearch;
class ctrlButtonPictureKeepAspect;

class GVAR(display) {
    idd = -1;
    movingEnable = 1;
    enableSimulation = 1;
    onLoad = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(display),_this select 0)]);
    class controls {
        class Background: ctrlStaticBackground {
            x = QUOTE(CENTER_X - GRID_W(76));
            y = QUOTE(CENTER_Y - GRID_H(58));
            w = QUOTE(GRID_W(152));
            h = QUOTE(GRID_H(116));
            colorBackground[] = {0.055, 0.065, 0.08, 0.72};
        };
        class Footer: ctrlStaticFooter {
            x = QUOTE(CENTER_X - GRID_W(76));
            y = QUOTE(CENTER_Y + GRID_H(43));
            w = QUOTE(GRID_W(152));
            h = QUOTE(GRID_H(15));
            colorBackground[] = {0.08, 0.095, 0.115, 0.35};
        };
        class Title: ctrlStaticTitle {
            text = CSTRING(DisplayName);
            x = QUOTE(CENTER_X - GRID_W(76));
            y = QUOTE(CENTER_Y - GRID_H(58));
            w = QUOTE(GRID_W(152));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(4.2));
            colorBackground[] = {0.1, 0.43, 0.44, 1};
        };
        class List: ctrlListbox {
            idc = IDC_LIST;
            x = QUOTE(CENTER_X - GRID_W(73));
            y = QUOTE(CENTER_Y - GRID_H(48));
            w = QUOTE(GRID_W(87));
            h = QUOTE(GRID_H(79));
            sizeEx = QUOTE(GRID_H(4.2));
            rowHeight = QUOTE(GRID_H(5.3));
            colorBackground[] = {0.025, 0.035, 0.045, 0.25};
            colorText[] = {0.92, 0.94, 0.97, 1};
            colorSelect[] = {1, 1, 1, 1};
            colorSelect2[] = {1, 1, 1, 1};
            colorSelectBackground[] = {0.12, 0.43, 0.44, 1};
            colorSelectBackground2[] = {0.12, 0.43, 0.44, 1};
        };
        class SearchButton: ctrlButtonSearch {
            idc = IDC_SEARCH_BUTTON;
            x = QUOTE(CENTER_X - GRID_W(73));
            y = QUOTE(CENTER_Y + GRID_H(34));
            w = QUOTE(GRID_W(6));
            h = QUOTE(GRID_H(6));
        };
        class SearchBar: ctrlEdit {
            idc = IDC_SEARCH_BAR;
            x = QUOTE(CENTER_X - GRID_W(66));
            y = QUOTE(CENTER_Y + GRID_H(34));
            w = QUOTE(GRID_W(58));
            h = QUOTE(GRID_H(6));
            sizeEx = QUOTE(GRID_H(4.2));
            colorBackground[] = {0.025, 0.035, 0.045, 0.25};
        };
        class Presets: ctrlButton {
            idc = IDC_PRESETS;
            text = CSTRING(Presets);
            tooltip = CSTRING(PresetsTooltip);
            x = QUOTE(CENTER_X - GRID_W(7));
            y = QUOTE(CENTER_Y + GRID_H(34));
            w = QUOTE(GRID_W(21));
            h = QUOTE(GRID_H(6));
            sizeEx = QUOTE(GRID_H(3.2));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
        };
        class DetailsBackground: ctrlStatic {
            x = QUOTE(CENTER_X + GRID_W(18));
            y = QUOTE(CENTER_Y - GRID_H(48));
            w = QUOTE(GRID_W(55));
            h = QUOTE(GRID_H(88));
            colorBackground[] = {0.075, 0.09, 0.11, 0.20};
        };
        class PictureBackground: ctrlStatic {
            x = QUOTE(CENTER_X + GRID_W(31));
            y = QUOTE(CENTER_Y - GRID_H(45));
            w = QUOTE(GRID_W(29));
            h = QUOTE(GRID_H(29));
            colorBackground[] = {0.035, 0.045, 0.06, 0.35};
        };
        class Picture: ctrlStaticPictureKeepAspect {
            idc = IDC_PICTURE;
            x = QUOTE(CENTER_X + GRID_W(31));
            y = QUOTE(CENTER_Y - GRID_H(45));
            w = QUOTE(GRID_W(29));
            h = QUOTE(GRID_H(29));
        };
        class PictureDefault: Picture {
            idc = IDC_PICTURE_DEFAULT;
            text = QPATHTOF(ui\music_ca.paa);
            colorText[] = {0.22, 0.3, 0.35, 0.8};
        };
        class Name: ctrlStatic {
            idc = IDC_NAME;
            style = ST_CENTER + ST_MULTI + ST_NO_RECT;
            x = QUOTE(CENTER_X + GRID_W(20));
            y = QUOTE(CENTER_Y - GRID_H(14));
            w = QUOTE(GRID_W(51));
            h = QUOTE(GRID_H(12));
            sizeEx = QUOTE(GRID_H(4.2));
            colorText[] = {0.95, 0.97, 1, 1};
            lineSpacing = 1;
        };
        class Description: ctrlStatic {
            idc = IDC_DESCRIPTION;
            style = ST_CENTER + ST_MULTI + ST_NO_RECT;
            x = QUOTE(CENTER_X + GRID_W(20));
            y = QUOTE(CENTER_Y - GRID_H(1));
            w = QUOTE(GRID_W(51));
            h = QUOTE(GRID_H(8));
            sizeEx = QUOTE(GRID_H(3.5));
            colorText[] = {0.66, 0.72, 0.78, 1};
        };
        class CopyTitle: ctrlButton {
            idc = IDC_COPY_TITLE;
            text = "";
            x = QUOTE(CENTER_X + GRID_W(20));
            y = QUOTE(CENTER_Y - GRID_H(1));
            w = QUOTE(GRID_W(51));
            h = QUOTE(GRID_H(8));
            colorBackground[] = {0, 0, 0, 0};
            colorBackgroundActive[] = {0, 0, 0, 0};
            colorBackgroundDisabled[] = {0, 0, 0, 0};
            colorFocused[] = {0, 0, 0, 0};
            colorShadow[] = {0, 0, 0, 0};
            colorBorder[] = {0, 0, 0, 0};
            soundEnter[] = {"", 0, 1};
            soundPush[] = {"", 0, 1};
            soundClick[] = {"", 0, 1};
            soundEscape[] = {"", 0, 1};
        };
        class Status: ctrlStatic {
            idc = IDC_STATUS;
            style = ST_CENTER + ST_MULTI + ST_NO_RECT;
            x = QUOTE(CENTER_X + GRID_W(20));
            y = QUOTE(CENTER_Y + GRID_H(8));
            w = QUOTE(GRID_W(51));
            h = QUOTE(GRID_H(8));
            sizeEx = QUOTE(GRID_H(3.3));
            colorBackground[] = {0.04, 0.055, 0.07, 0.35};
        };
        class Retry: ctrlButton {
            idc = IDC_RETRY;
            text = "$STR_Live_Radio_Interface_Retry";
            tooltip = "$STR_Live_Radio_Interface_RetryTooltip";
            x = QUOTE(CENTER_X + GRID_W(20));
            y = QUOTE(CENTER_Y + GRID_H(17));
            w = QUOTE(GRID_W(51));
            h = QUOTE(GRID_H(6));
            sizeEx = QUOTE(GRID_H(3.4));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
        class Empty: ctrlStatic {
            idc = IDC_EMPTY;
            text = "$STR_Live_Radio_Interface_Empty";
            style = ST_CENTER;
            x = QUOTE(CENTER_X - GRID_W(72));
            y = QUOTE(CENTER_Y - GRID_H(16));
            w = QUOTE(GRID_W(85));
            h = QUOTE(GRID_H(8));
            sizeEx = QUOTE(GRID_H(4));
        };
        class VolumeText: ctrlStatic {
            idc = IDC_VOLUME_TEXT;
            style = ST_RIGHT;
            x = QUOTE(CENTER_X + GRID_W(59));
            y = QUOTE(CENTER_Y + GRID_H(34));
            w = QUOTE(GRID_W(12));
            h = QUOTE(GRID_H(6));
            sizeEx = QUOTE(GRID_H(3.2));
        };
        class Power: ctrlButtonPictureKeepAspect {
            idc = IDC_POWER;
            text = QPATHTOF(ui\power_ca.paa);
            x = QUOTE(CENTER_X + GRID_W(23));
            y = QUOTE(CENTER_Y + GRID_H(24));
            w = QUOTE(GRID_W(10));
            h = QUOTE(GRID_H(10));
            colorFocused[] = {0, 0, 0, 0};
            colorBackground[] = {0, 0, 0, 0};
            colorBackgroundActive[] = {0, 0, 0, 0};
            colorBackgroundDisabled[] = {0, 0, 0, 0};
        };
        class Mute: ctrlButton {
            idc = IDC_MUTE;
            text = "$STR_Live_Radio_Interface_Mute";
            x = QUOTE(CENTER_X + GRID_W(36));
            y = QUOTE(CENTER_Y + GRID_H(25));
            w = QUOTE(GRID_W(35));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
        class VolumeIcon: ctrlStaticPictureKeepAspect {
            idc = IDC_VOLUME_ICON;
            text = QPATHTOF(ui\volume_high_ca.paa);
            x = QUOTE(CENTER_X + GRID_W(21));
            y = QUOTE(CENTER_Y + GRID_H(35));
            w = QUOTE(GRID_W(4));
            h = QUOTE(GRID_H(4));
        };
        class VolumeBarBackground: ctrlStatic {
            x = QUOTE(CENTER_X + GRID_W(27));
            y = QUOTE(CENTER_Y + GRID_H(35.6));
            w = QUOTE(GRID_W(30));
            h = QUOTE(GRID_H(2.8));
            colorBackground[] = {0.025, 0.035, 0.045, 1};
        };
        class VolumeBarFill: VolumeBarBackground {
            idc = IDC_VOLUME_BAR_FILL;
            colorBackground[] = {0.3, 0.8, 0.77, 1};
        };
        class VolumeBarMouse: VolumeBarBackground {
            idc = IDC_VOLUME_BAR_MOUSE;
            style = ST_MULTI;
            colorBackground[] = {0, 0, 0, 0};
        };
        class StreamerStatus: ctrlStatic {
            idc = IDC_STREAMER_STATUS;
            text = "Streamer Mode enabled: False";
            x = QUOTE(CENTER_X - GRID_W(73));
            y = QUOTE(CENTER_Y + GRID_H(47.5));
            w = QUOTE(GRID_W(72));
            h = QUOTE(GRID_H(6));
            sizeEx = QUOTE(GRID_H(3.7));
            colorText[] = {0.8, 0.83, 0.87, 1};
        };
        class StreamerToggle: ctrlButton {
            idc = IDC_STREAMER_TOGGLE;
            text = "$STR_Live_Radio_Interface_ToggleStreamer";
            x = QUOTE(CENTER_X + GRID_W(0));
            y = QUOTE(CENTER_Y + GRID_H(47));
            w = QUOTE(GRID_W(43));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
        class ButtonOK: ctrlButtonOK {
            x = QUOTE(CENTER_X + GRID_W(46));
            y = QUOTE(CENTER_Y + GRID_H(47));
            w = QUOTE(GRID_W(27));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.7));
        };
    };
};
