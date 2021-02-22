#define COMPONENT interface
#define COMPONENT_BEAUTIFIED Interface
#include "\z\live_radio\addons\main\script_mod.hpp"

#define DEBUG_MODE_FULL
#define DISABLE_COMPILE_CACHE
#define ENABLE_PERFORMANCE_COUNTERS

#ifdef DEBUG_ENABLED_INTERFACE
    #define DEBUG_MODE_FULL
#endif

#ifdef DEBUG_SETTINGS_INTERFACE
    #define DEBUG_SETTINGS DEBUG_SETTINGS_INTERFACE
#endif

#include "\z\live_radio\addons\main\script_macros.hpp"

#include "\a3\ui_f\hpp\defineResincl.inc"

#define GRID_W(N) ((N) * pixelW * pixelGrid * 0.5)
#define GRID_H(N) ((N) * pixelH * pixelGrid * 0.5)

#define CENTER_X ((getResolution select 2) * 0.5 * pixelW)
#define CENTER_Y ((getResolution select 3) * 0.5 * pixelH)

#define IDC_LIST 100
#define IDC_SEARCH_BUTTON 110
#define IDC_SEARCH_BAR 120
#define IDC_PICTURE 130
#define IDC_PICTURE_DEFAULT 140
#define IDC_NAME 150
#define IDC_DESCRIPTION 160
#define IDC_POWER 170
#define IDC_VOLUME_ICON 180
#define IDC_VOLUME_BAR_FILL 190
#define IDC_VOLUME_BAR_MOUSE 200

#define MIN_VOLUME 0
#define MAX_VOLUME 2
#define DEFAULT_VOLUME 1

#define ICON_VOLUME_LOW QPATHTOF(ui\volume_low_ca.paa)
#define ICON_VOLUME_MEDIUM QPATHTOF(ui\volume_medium_ca.paa)
#define ICON_VOLUME_HIGH QPATHTOF(ui\volume_high_ca.paa)
