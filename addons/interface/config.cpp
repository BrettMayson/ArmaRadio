#include "script_component.hpp"

class CfgPatches {
    class ADDON {
        name = COMPONENT_NAME;
        units[] = {QGVAR(moduleRadio)};
        weapons[] = {};
        requiredVersion = REQUIRED_VERSION;
        requiredAddons[] = {"live_radio_manager", "A3_Modules_F"};
        author = ECSTRING(main,Author);
        authors[] = {"BrettMayson", "mharis001", "Joncantplay"};
        url = ECSTRING(main,URL);
        VERSION_CONFIG;
    };
};

#include "CfgEventHandlers.hpp"
#include "CfgRadioStations.hpp"
#include "CfgVehicles.hpp"
#include "gui.hpp"

#include "gui_classic.hpp"

class CfgFactionClasses {
    class GVAR(modules) {
        displayName = "Live Radio";
        priority = 2;
        side = 7;
    };
};
#include "gui_presets.hpp"
