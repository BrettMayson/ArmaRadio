#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        units[] = {};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"radio_manager"};
        author = ECSTRING(main,Author);
        authors[] = {"SynixeBrett", "mharis001"};
        url = ECSTRING(main,URL);
        VERSION_CONFIG;
    };
};

#include "CfgEventHandlers.hpp"
#include "CfgRadioStations.hpp"
#include "CfgVehicles.hpp"
#include "gui.hpp"
