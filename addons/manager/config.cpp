#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = QUOTE(COMPONENT);
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"live_radio_main"};
        author = "BrettMayson";
        VERSION_CONFIG;
    };
};

#include "CfgEventHandlers.hpp"
