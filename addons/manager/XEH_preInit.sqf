#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(active) = call CBA_fnc_createNamespace;
GVAR(jips) = [];

// Make sure the extension has been loaded once
EXT callExtension ""; 
