#include "\a3\ui_f\hpp\definecommongrids.inc"

class RscButtonMenuOK;
class RscListbox;
class RscPicture;
class RscSlider;

class A3R_RscStations {
	idd = 10000;
	name = "A3R_RscStations";
	onload = QUOTE(_this call FUNC(ui_stations));
	class controls {
		class Background: RscPicture {
			text = "#(argb,8,8,3)color(0,0,0,0.7)";
			x = X_PART(1.5);
			y = Y_PART(1.5);
			w = W_PART(37);
			h = H_PART(20.5);
		};
		class Stations: RscListbox {
			idc = 16189;
			x = X_PART(2);
			y = Y_PART(2);
			w = W_PART(36);
			h = H_PART(19.5);
		};
		class Ok: RscButtonMenuOK {
			x = X_PART(32.5);
			y = Y_PART(22.5);
			w = W_PART(6);
			h = H_PART(1);
		};
		class SliderBackground: RscPicture {
			text = "#(argb,8,8,3)color(0,0,0,0.7)";
			x = X_PART(1.5);
			y = Y_PART(22.475);
			w = W_PART(30);
			h = H_PART(1.25);
		};
		class Volume: RscSlider {
			idc = 13589;
			x = X_PART(1.5);
			y = Y_PART(22.60);
			w = W_PART(30);
			h = H_PART(1);
		};
	};
};
