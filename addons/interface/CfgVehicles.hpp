#define ADD_RADIO(BASE,PARENT) class BASE: PARENT { \
    GVAR(hasRadio) = 1; \
    class ACE_Actions { \
        class ACE_MainActions { \
            selection = "interaction_point"; \
            distance = 5; \
            class GVAR(open) { \
                displayName = CSTRING(DisplayName); \
                statement = QUOTE(_target call FUNC(open)); \
            }; \
        }; \
    }; \
}

class CfgVehicles {
    class Items_base_F;
    ADD_RADIO(Land_FMradio_F,Items_base_F);
    ADD_RADIO(Land_PortableSpeakers_01_F,Items_base_F);
    ADD_RADIO(Land_SurvivalRadio_F,Items_base_F);

    class AllVehicles;
    class Air: AllVehicles {
        GVAR(hasRadio) = 1;
    };
    class LandVehicle;
    class Car: LandVehicle {
        GVAR(hasRadio) = 1;
    };
    class Tank: LandVehicle {
        GVAR(hasRadio) = 1;
    };
    class Ship_F;
    class Boat_F: Ship_F {
        GVAR(hasRadio) = 1;
    };
};
