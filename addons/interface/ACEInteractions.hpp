class LandVehicle;
class Car: LandVehicle {
	class ACE_SelfActions {
		class GVAR(openInterface) {
			displayName = "FM Radio";
			condition = QUOTE(true);
			statement = QUOTE([_target] call FUNC(openInterface));
		};
	};
};

class Items_base_F;
class Land_FMradio_F: Items_base_F {
	class ACE_Actions {
		class ACE_MainActions {
			selection = "interaction_point";
			distance = 5;
			condition = "(true)";
			class GVAR(openInterface) {
				displayName = "FM Radio";
				condition = QUOTE(true);
				statement = QUOTE([_target] call FUNC(openInterface));
			};
		};
	};
};
