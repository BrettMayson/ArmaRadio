#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles the key up event for the search bar.
 *
 * Arguments:
 * 0: Search Bar <CONTROL>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL] call radio_interface_fnc_handleSearchKeyUp
 *
 * Public: No
 */

params ["_ctrlSearchBar"];

private _display = ctrlParent _ctrlSearchBar;
[_display] call FUNC(updateList);
