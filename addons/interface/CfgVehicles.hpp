class CfgVehicles {
    class LandVehicle;
    class Car: LandVehicle {
        class ACE_SelfActions {
            class GVAR(open) {
                displayName = CSTRING(DisplayName);
                statement = QUOTE(_target call FUNC(open));
                condition = QUOTE(_target call FUNC(canOpen));
            };
        };
    };

    class Items_base_F;
    class Land_FMradio_F: Items_base_F {
        class ACE_Actions {
            class ACE_MainActions {
                selection = "interaction_point";
                distance = 5;
                class GVAR(open) {
                    displayName = CSTRING(DisplayName);
                    statement = QUOTE(_target call FUNC(open));
                };
            };
        };
    };
};
