#include "script_component.hpp"
// Author: Joncantplay

// Read and persist the same client setting used by CBA Addon Options.
private _enabled = !([QEGVAR(manager,streamerMode), "client"] call CBA_settings_fnc_get);
[QEGVAR(manager,streamerMode), _enabled, 0, "client", true] call CBA_settings_fnc_set;
