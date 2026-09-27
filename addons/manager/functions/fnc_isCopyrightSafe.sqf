#include "script_component.hpp"
/*
 * Author: BrettMayson
 * Checks if a radio station is marked as copyright-safe (noCopyright = 1).
 *
 * Arguments:
 * 0: Station URL <STRING>
 *
 * Return Value:
 * Copyright Safe <BOOLEAN>
 *
 * Example:
 * ["https://example.com/stream"] call live_radio_manager_fnc_isCopyrightSafe
 *
 * Public: No
 */

params ["_url"];

GVAR(copyrightSafe) getOrDefault [_url, false]
