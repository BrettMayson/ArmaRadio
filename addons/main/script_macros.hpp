#include "\x\cba\addons\main\script_macros_common.hpp"

#define DFUNC(var1) TRIPLES(ADDON,fnc,var1)

#ifdef DISABLE_COMPILE_CACHE
  #undef PREP
  #define PREP(fncName) DFUNC(fncName) = compile preprocessFileLineNumbers QPATHTOF(functions\DOUBLES(fnc,fncName).sqf)
#else
  #undef PREP
  #define PREP(fncName) [QPATHTOF(functions\DOUBLES(fnc,fncName).sqf), QFUNC(fncName)] call CBA_fnc_compileFunction
#endif

#define SIZEX ((safeZoneW / safeZoneH) min 1.2)
#define SIZEY (SIZEX / 1.2)
#define W_PART(num) (num * (SIZEX / 40))
#define H_PART(num) (num * (SIZEY / 25))
#define X_PART(num) (W_PART(num) + (safeZoneX + (safeZoneW - SIZEX) / 2))
#define Y_PART(num) (H_PART(num) + (safeZoneY + (safeZoneH - SIZEY) / 2))

// remoteExec
#define REMOTE_GLOBAL   0
#define REMOTE_SERVER   ([2,([ACE_player,2] select isDedicated)] select isServer)
#define REMOTE_CLIENTS  ([0,-2] select isDedicated)
