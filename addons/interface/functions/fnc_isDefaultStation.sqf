#include "script_component.hpp"
// Author: Joncantplay
params ["_station", "_root"];
if (_root isNotEqualTo configFile) exitWith {false};
if (getNumber (_station >> "defaultStation") != 1) exitWith {false};
// An expansion overriding a bundled class is treated as external configuration.
private _sources = configSourceAddonList _station;
(_sources findIf {toLower _x != "live_radio_interface"}) == -1
