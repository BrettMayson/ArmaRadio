class CfgVehicles {
    class LandVehicle;
    class Car: LandVehicle {
        class ACE_SelfActions {
            class GVAR(open) {
                displayName = CSTRING(DisplayName);
                statement = QUOTE(_target call FUNC(open));
                condition = QUOTE([ARR_2(_target,false)] call FUNC(canOpen));
            };
        };
        class ACE_Actions {
            class ACE_MainActions {
                class GVAR(open) {
                    displayName = CSTRING(DisplayName);
                    statement = QUOTE(_target call FUNC(open));
                    condition = QUOTE([ARR_2(_target,true)] call FUNC(canOpen));
                };
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
