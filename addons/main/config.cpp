#include "script_component.hpp"

class CfgPatches {
  class ADDON {
    name = QUOTE(COMPONENT);
    units[] = {};
    weapons[] = {};
    requiredVersion = REQUIRED_VERSION;
    requiredAddons[] = {"cba_settings"};
    author = "BrettMayson";
    VERSION_CONFIG;
  };
};
