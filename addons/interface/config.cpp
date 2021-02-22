#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"live_radio_manager"};
        author = ECSTRING(main,Author);
        authors[] = {"BrettMayson", "mharis001"};
        url = ECSTRING(main,URL);
        VERSION_CONFIG;
    };
};

#include "CfgEventHandlers.hpp"
#include "CfgRadioStations.hpp"
#include "CfgVehicles.hpp"
#include "gui.hpp"
