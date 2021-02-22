#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles clicking the search bar with a mouse button.
 *
 * Arguments:
 * 0: Search Bar <CONTROL>
 * 1: Button <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, 0] call live_radiointerface_fnc_handleSearchClick
 *
 * Public: No
 */

params ["_ctrlSearchBar", "_button"];

if (_button != 1) exitWith {};

_ctrlSearchBar ctrlSetText "";
ctrlSetFocus _ctrlSearchBar;

private _display = ctrlParent _ctrlSearchBar;
[_display] call FUNC(updateList);
