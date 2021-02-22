#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles clicking the search button.
 *
 * Arguments:
 * 0: Search Button <CONTROL>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL] call live_radiointerface_fnc_handleSearchButton
 *
 * Public: No
 */

params ["_ctrlSearchButton"];

private _display = ctrlParent _ctrlSearchButton;

private _ctrlSearchBar = _display displayCtrl IDC_SEARCH_BAR;
_ctrlSearchBar ctrlSetText "";
ctrlSetFocus _ctrlSearchBar;

[_display] call FUNC(updateList);
