class ctrlStatic;
class ctrlStaticTitle;
class ctrlStaticBackground;
class ctrlStaticFooter;
class ctrlStaticFrame;
class ctrlStaticPictureKeepAspect;
class ctrlEdit;
class ctrlListbox;
class ctrlProgress;
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
            x = CENTER_X - GRID_W(140/2);
            y = CENTER_Y - GRID_H(110/2);
            w = GRID_W(140);
            h = GRID_H(110);
        };
        class BackgroundButtons: ctrlStaticFooter {
            x = CENTER_X - GRID_W(140/2);
            y = CENTER_Y + GRID_H(110/2 - 7);
            w = GRID_W(140);
            h = GRID_H(7);
        };
        class Title: ctrlStaticTitle {
            text = CSTRING(DisplayName);
            x = CENTER_X - GRID_W(140/2);
            y = CENTER_Y - GRID_H(110/2);
            w = GRID_W(140);
            h = GRID_H(5);
        };
        class List: ctrlListbox {
            idc = IDC_LIST;
            x = CENTER_X - GRID_W(140/2 - 1);
            y = CENTER_Y - GRID_H(110/2 - 5 - 1);
            w = GRID_W(79);
            h = GRID_H(110 - 5 - 7 - 6 - 2);
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class ListFrame: ctrlStaticFrame {
            x = CENTER_X - GRID_W(140/2 - 1);
            y = CENTER_Y - GRID_H(110/2 - 5 - 1);
            w = GRID_W(79);
            h = GRID_H(110 - 5 - 7 - 6 - 2);
            colorText[] = {0, 0, 0, 1};
        };
        class ButtonSearch: ctrlButtonSearch {
            idc = IDC_SEARCH_BUTTON;
            x = CENTER_X - GRID_W(140/2 - 1);
            y = CENTER_Y + GRID_H(110/2 - 5 - 7 - 1);
            w = GRID_W(5);
            h = GRID_H(5);
        };
        class SearchBar: ctrlEdit {
            idc = IDC_SEARCH_BAR;
            x = CENTER_X - GRID_W(140/2 - 1 - 6);
            y = CENTER_Y + GRID_H(110/2 - 5 - 7 - 1);
            w = GRID_W(73);
            h = GRID_H(5);
        };
        class PictureBackground: ctrlStatic {
            x = CENTER_X + GRID_W(140/2 - 55);
            y = CENTER_Y - GRID_H(110/2 - 10);
            w = GRID_W(50);
            h = GRID_H(50);
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class Picture: ctrlStaticPictureKeepAspect {
            idc = IDC_PICTURE;
            x = CENTER_X + GRID_W(140/2 - 55);
            y = CENTER_Y - GRID_H(110/2 - 10);
            w = GRID_W(50);
            h = GRID_H(50);
        };
        class PictureDefault: Picture {
            idc = IDC_PICTURE_DEFAULT;
            text = QPATHTOF(ui\music_ca.paa);
            colorText[] = {0.2, 0.2, 0.2, 0.5};
        };
        class Name: ctrlStatic {
            idc = IDC_NAME;
            style = ST_CENTER;
            x = CENTER_X + GRID_W(140/2 - 59);
            y = CENTER_Y - GRID_H(110/2 - 62);
            w = GRID_W(58);
            h = GRID_H(7);
            sizeEx = GRID_H(7);
        };
        class Description: ctrlStatic {
            idc = IDC_DESCRIPTION;
            style = ST_CENTER + ST_MULTI + ST_NO_RECT;
            x = CENTER_X + GRID_W(140/2 - 59);
            y = CENTER_Y - GRID_H(110/2 - 70);
            w = GRID_W(58);
            h = GRID_H(13);
        };
        class Power: ctrlButtonPictureKeepAspect {
            idc = IDC_POWER;
            text = QPATHTOF(ui\power_ca.paa);
            x = CENTER_X + GRID_W(140/2 - 34);
            y = CENTER_Y + GRID_H(110/2 - 5 - 7 - 13);
            w = GRID_W(10);
            h = GRID_H(10);
            colorFocused[] = {0, 0, 0, 0};
            colorBackground[] = {0, 0, 0, 0};
            colorBackgroundActive[] = {0, 0, 0, 0};
            colorBackgroundDisabled[] = {0, 0, 0, 0};
        };
        class VolumeIcon: ctrlStaticPictureKeepAspect {
            idc = IDC_VOLUME_ICON;
            text = QPATHTOF(ui\volume_high_ca.paa);
            x = CENTER_X + GRID_W(140/2 - 57);
            y = CENTER_Y + GRID_H(110/2 - 5 - 7 - 1);
            w = GRID_W(5);
            h = GRID_H(5);
        };
        class VolumeBarBackground: ctrlStatic {
            x = CENTER_X + GRID_W(140/2 - 51);
            y = CENTER_Y + GRID_H(110/2 - 5 - 7 - 0.5);
            w = GRID_W(48);
            h = GRID_H(4);
            colorBackground[] = {0, 0, 0, 0.3};
        };
        class VolumeBarFill: VolumeBarBackground {
            idc = IDC_VOLUME_BAR_FILL;
            colorBackground[] = {1, 1, 1, 1};
        };
        class VolumeBarMouse: VolumeBarBackground {
            idc = IDC_VOLUME_BAR_MOUSE;
            style = ST_MULTI;
            colorBackground[] = {0, 0, 0, 0};
        };
        class ButtonOK: ctrlButtonOK {
            x = CENTER_X + GRID_W(140/2 - 25 - 1);
            y = CENTER_Y + GRID_H(110/2 - 5 - 1);
            w = GRID_W(25);
            h = GRID_H(5);
        };
    };
};
