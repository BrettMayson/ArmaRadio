// COMPONENT should be defined in the script_component.hpp and included BEFORE this hpp

#define MAINPREFIX z
#define PREFIX live_radio

#define EXT "live_radio"

#include "script_version.hpp"

#define VERSION     MAJOR.MINOR
#define VERSION_STR MAJOR.MINOR.PATCH.BUILD
#define VERSION_AR  MAJOR,MINOR,PATCH,BUILD

// MINIMAL required version for the Mod. Components can specify others..
#define REQUIRED_VERSION 2.02

#ifdef COMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(Live Radio - COMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(Live Radio - COMPONENT)
#endif
