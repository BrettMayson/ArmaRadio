class ctrlCheckbox;
class GVAR(presetsDisplay) {
    idd = -1;
    movingEnable = 1;
    enableSimulation = 1;
    onLoad = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(presetsDisplay),_this select 0)]);
    onUnload = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(presetsDisplay),displayNull)]);
    class controls {
        class Background: ctrlStaticBackground {
            x = QUOTE(CENTER_X + GRID_W(-58));
            y = QUOTE(CENTER_Y + GRID_H(-49));
            w = QUOTE(GRID_W(116));
            h = QUOTE(GRID_H(86));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.055, 0.065, 0.08, 0.80};
        };
        class Title: ctrlStaticTitle {
            text = CSTRING(PresetTitle);
            x = QUOTE(CENTER_X + GRID_W(-58));
            y = QUOTE(CENTER_Y + GRID_H(-49));
            w = QUOTE(GRID_W(116));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.1, 0.43, 0.44, 1};
        };
        class List: ctrlListbox {
            idc = 300;
            x = QUOTE(CENTER_X + GRID_W(-55));
            y = QUOTE(CENTER_Y + GRID_H(-39));
            w = QUOTE(GRID_W(49));
            h = QUOTE(GRID_H(62));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.025, 0.035, 0.045, 0.25};
            colorSelectBackground[] = {0.12, 0.43, 0.44, 1};
            colorSelectBackground2[] = {0.12, 0.43, 0.44, 1};
            colorText[] = {0.92, 0.94, 0.97, 1};
            rowHeight = QUOTE(GRID_H(5));
        };
        class NameLabel: ctrlStatic {
            text = CSTRING(PresetName);
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(-39));
            w = QUOTE(GRID_W(57));
            h = QUOTE(GRID_H(5));
            sizeEx = QUOTE(GRID_H(3.5));
        };
        class Name: ctrlEdit {
            idc = 301;
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(-33));
            w = QUOTE(GRID_W(57));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.025, 0.035, 0.045, 0.35};
        };
        class URLLabel: ctrlStatic {
            text = CSTRING(PresetURL);
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(-24));
            w = QUOTE(GRID_W(57));
            h = QUOTE(GRID_H(5));
            sizeEx = QUOTE(GRID_H(3.5));
        };
        class URL: ctrlEdit {
            idc = 302;
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(-18));
            w = QUOTE(GRID_W(57));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            colorBackground[] = {0.025, 0.035, 0.045, 0.35};
        };
        class Approved: ctrlCheckbox {
            idc = 303;
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(-8));
            w = QUOTE(GRID_W(5));
            h = QUOTE(GRID_H(5));
        };
        class ApprovedLabel: ctrlStatic {
            text = CSTRING(PresetApproved);
            x = QUOTE(CENTER_X + GRID_W(5));
            y = QUOTE(CENTER_Y + GRID_H(-8));
            w = QUOTE(GRID_W(50));
            h = QUOTE(GRID_H(6));
            sizeEx = QUOTE(GRID_H(3.5));
            style = ST_MULTI + ST_NO_RECT;
        };
        class Note: ctrlStatic {
            text = CSTRING(PresetNote);
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(0));
            w = QUOTE(GRID_W(57));
            h = QUOTE(GRID_H(13));
            sizeEx = QUOTE(GRID_H(3));
            style = ST_MULTI + ST_NO_RECT;
        };
        class New: ctrlButton {
            text = CSTRING(PresetNew);
            x = QUOTE(CENTER_X + GRID_W(-2));
            y = QUOTE(CENTER_Y + GRID_H(16));
            w = QUOTE(GRID_W(18));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            onButtonClick = QUOTE(['new'] call FUNC(handlePresets));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
        class Save: ctrlButton {
            text = CSTRING(PresetSave);
            x = QUOTE(CENTER_X + GRID_W(18));
            y = QUOTE(CENTER_Y + GRID_H(16));
            w = QUOTE(GRID_W(18));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            onButtonClick = QUOTE(['save'] call FUNC(handlePresets));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
        class Delete: ctrlButton {
            text = CSTRING(PresetDelete);
            x = QUOTE(CENTER_X + GRID_W(38));
            y = QUOTE(CENTER_Y + GRID_H(16));
            w = QUOTE(GRID_W(17));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            onButtonClick = QUOTE(['delete'] call FUNC(handlePresets));
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
        class Close: ctrlButton {
            text = CSTRING(PresetClose);
            x = QUOTE(CENTER_X + GRID_W(30));
            y = QUOTE(CENTER_Y + GRID_H(26));
            w = QUOTE(GRID_W(25));
            h = QUOTE(GRID_H(7));
            sizeEx = QUOTE(GRID_H(3.5));
            onButtonClick = "(ctrlParent (_this select 0)) closeDisplay 2";
            colorBackground[] = {0.14, 0.26, 0.3, 1};
            colorBackgroundActive[] = {0.12, 0.43, 0.44, 1};
        };
    };
};
