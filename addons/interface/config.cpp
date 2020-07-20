#include "script_component.hpp"

class CfgPatches {
  class ADDON {
    name = QUOTE(COMPONENT);
    units[] = {};
    weapons[] = {};
    requiredVersion = REQUIRED_VERSION;
    requiredAddons[] = {"radio_main"};
    author = "SynixeBrett";
    VERSION_CONFIG;
  };
};

#include "CfgEventHandlers.hpp"
#include "CfgRadioStations.hpp"
#include "RscAttributes.hpp"

class CfgVehicles {
  #include "ACEInteractions.hpp"
};
