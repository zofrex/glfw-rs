/**
 * Common constants for glfw. You shouldn't have to access these directly as each module
 * publicly exports them.
 */

use core::libc::c_int;

/* GLFW version */
pub static VERSION_MAJOR                : c_int = 3;
pub static VERSION_MINOR                : c_int = 0;
pub static VERSION_REVISION             : c_int = 0;

/* Not actually defined in GLFW, but very useful */
pub static FALSE                        : c_int = 0;
pub static TRUE                         : c_int = 1;

/* Key and button state/action definitions */
pub static RELEASE                      : c_int = 0;
pub static PRESS                        : c_int = 1;
pub static REPEAT                       : c_int = 2;

/* Printable keys */
pub static KEY_SPACE                    : c_int = 32;
pub static KEY_APOSTROPHE               : c_int = 39;
pub static KEY_COMMA                    : c_int = 44;
pub static KEY_MINUS                    : c_int = 45;
pub static KEY_PERIOD                   : c_int = 46;
pub static KEY_SLASH                    : c_int = 47;
pub static KEY_0                        : c_int = 48;
pub static KEY_1                        : c_int = 49;
pub static KEY_2                        : c_int = 50;
pub static KEY_3                        : c_int = 51;
pub static KEY_4                        : c_int = 52;
pub static KEY_5                        : c_int = 53;
pub static KEY_6                        : c_int = 54;
pub static KEY_7                        : c_int = 55;
pub static KEY_8                        : c_int = 56;
pub static KEY_9                        : c_int = 57;
pub static KEY_SEMICOLON                : c_int = 59;
pub static KEY_EQUAL                    : c_int = 61;
pub static KEY_A                        : c_int = 65;
pub static KEY_B                        : c_int = 66;
pub static KEY_C                        : c_int = 67;
pub static KEY_D                        : c_int = 68;
pub static KEY_E                        : c_int = 69;
pub static KEY_F                        : c_int = 70;
pub static KEY_G                        : c_int = 71;
pub static KEY_H                        : c_int = 72;
pub static KEY_I                        : c_int = 73;
pub static KEY_J                        : c_int = 74;
pub static KEY_K                        : c_int = 75;
pub static KEY_L                        : c_int = 76;
pub static KEY_M                        : c_int = 77;
pub static KEY_N                        : c_int = 78;
pub static KEY_O                        : c_int = 79;
pub static KEY_P                        : c_int = 80;
pub static KEY_Q                        : c_int = 81;
pub static KEY_R                        : c_int = 82;
pub static KEY_S                        : c_int = 83;
pub static KEY_T                        : c_int = 84;
pub static KEY_U                        : c_int = 85;
pub static KEY_V                        : c_int = 86;
pub static KEY_W                        : c_int = 87;
pub static KEY_X                        : c_int = 88;
pub static KEY_Y                        : c_int = 89;
pub static KEY_Z                        : c_int = 90;
pub static KEY_LEFT_BRACKET             : c_int = 91;
pub static KEY_BACKSLASH                : c_int = 92;
pub static KEY_RIGHT_BRACKET            : c_int = 93;
pub static KEY_GRAVE_ACCENT             : c_int = 96;
pub static KEY_WORLD_1                  : c_int = 161;
pub static KEY_WORLD_2                  : c_int = 162;

/* Function keys */
pub static KEY_ESCAPE                   : c_int = 256;
pub static KEY_ENTER                    : c_int = 257;
pub static KEY_TAB                      : c_int = 258;
pub static KEY_BACKSPACE                : c_int = 259;
pub static KEY_INSERT                   : c_int = 260;
pub static KEY_DELETE                   : c_int = 261;
pub static KEY_RIGHT                    : c_int = 262;
pub static KEY_LEFT                     : c_int = 263;
pub static KEY_DOWN                     : c_int = 264;
pub static KEY_UP                       : c_int = 265;
pub static KEY_PAGE_UP                  : c_int = 266;
pub static KEY_PAGE_DOWN                : c_int = 267;
pub static KEY_HOME                     : c_int = 268;
pub static KEY_END                      : c_int = 269;
pub static KEY_CAPS_LOCK                : c_int = 280;
pub static KEY_SCROLL_LOCK              : c_int = 281;
pub static KEY_NUM_LOCK                 : c_int = 282;
pub static KEY_PRINT_SCREEN             : c_int = 283;
pub static KEY_PAUSE                    : c_int = 284;
pub static KEY_F1                       : c_int = 290;
pub static KEY_F2                       : c_int = 291;
pub static KEY_F3                       : c_int = 292;
pub static KEY_F4                       : c_int = 293;
pub static KEY_F5                       : c_int = 294;
pub static KEY_F6                       : c_int = 295;
pub static KEY_F7                       : c_int = 296;
pub static KEY_F8                       : c_int = 297;
pub static KEY_F9                       : c_int = 298;
pub static KEY_F10                      : c_int = 299;
pub static KEY_F11                      : c_int = 300;
pub static KEY_F12                      : c_int = 301;
pub static KEY_F13                      : c_int = 302;
pub static KEY_F14                      : c_int = 303;
pub static KEY_F15                      : c_int = 304;
pub static KEY_F16                      : c_int = 305;
pub static KEY_F17                      : c_int = 306;
pub static KEY_F18                      : c_int = 307;
pub static KEY_F19                      : c_int = 308;
pub static KEY_F20                      : c_int = 309;
pub static KEY_F21                      : c_int = 310;
pub static KEY_F22                      : c_int = 311;
pub static KEY_F23                      : c_int = 312;
pub static KEY_F24                      : c_int = 313;
pub static KEY_F25                      : c_int = 314;
pub static KEY_KP_0                     : c_int = 320;
pub static KEY_KP_1                     : c_int = 321;
pub static KEY_KP_2                     : c_int = 322;
pub static KEY_KP_3                     : c_int = 323;
pub static KEY_KP_4                     : c_int = 324;
pub static KEY_KP_5                     : c_int = 325;
pub static KEY_KP_6                     : c_int = 326;
pub static KEY_KP_7                     : c_int = 327;
pub static KEY_KP_8                     : c_int = 328;
pub static KEY_KP_9                     : c_int = 329;
pub static KEY_KP_DECIMAL               : c_int = 330;
pub static KEY_KP_DIVIDE                : c_int = 331;
pub static KEY_KP_MULTIPLY              : c_int = 332;
pub static KEY_KP_SUBTRACT              : c_int = 333;
pub static KEY_KP_ADD                   : c_int = 334;
pub static KEY_KP_ENTER                 : c_int = 335;
pub static KEY_KP_EQUAL                 : c_int = 336;
pub static KEY_LEFT_SHIFT               : c_int = 340;
pub static KEY_LEFT_CONTROL             : c_int = 341;
pub static KEY_LEFT_ALT                 : c_int = 342;
pub static KEY_LEFT_SUPER               : c_int = 343;
pub static KEY_RIGHT_SHIFT              : c_int = 344;
pub static KEY_RIGHT_CONTROL            : c_int = 345;
pub static KEY_RIGHT_ALT                : c_int = 346;
pub static KEY_RIGHT_SUPER              : c_int = 347;
pub static KEY_MENU                     : c_int = 348;
pub static KEY_LAST                     : c_int = KEY_MENU;

/* Mouse buttons */
pub static MOUSE_BUTTON_1               : c_int = 0;
pub static MOUSE_BUTTON_2               : c_int = 1;
pub static MOUSE_BUTTON_3               : c_int = 2;
pub static MOUSE_BUTTON_4               : c_int = 3;
pub static MOUSE_BUTTON_5               : c_int = 4;
pub static MOUSE_BUTTON_6               : c_int = 5;
pub static MOUSE_BUTTON_7               : c_int = 6;
pub static MOUSE_BUTTON_8               : c_int = 7;
pub static MOUSE_BUTTON_LEFT            : c_int = MOUSE_BUTTON_1;
pub static MOUSE_BUTTON_RIGHT           : c_int = MOUSE_BUTTON_2;
pub static MOUSE_BUTTON_MIDDLE          : c_int = MOUSE_BUTTON_3;
pub static MOUSE_BUTTON_LAST            : c_int = MOUSE_BUTTON_8;

/* Joysticks */
pub static JOYSTICK_1                   : c_int = 0;
pub static JOYSTICK_2                   : c_int = 1;
pub static JOYSTICK_3                   : c_int = 2;
pub static JOYSTICK_4                   : c_int = 3;
pub static JOYSTICK_5                   : c_int = 4;
pub static JOYSTICK_6                   : c_int = 5;
pub static JOYSTICK_7                   : c_int = 6;
pub static JOYSTICK_8                   : c_int = 7;
pub static JOYSTICK_9                   : c_int = 8;
pub static JOYSTICK_10                  : c_int = 9;
pub static JOYSTICK_11                  : c_int = 10;
pub static JOYSTICK_12                  : c_int = 11;
pub static JOYSTICK_13                  : c_int = 12;
pub static JOYSTICK_14                  : c_int = 13;
pub static JOYSTICK_15                  : c_int = 14;
pub static JOYSTICK_16                  : c_int = 15;
pub static JOYSTICK_LAST                : c_int = JOYSTICK_16;

/* glfwGetWindowParam tokens */
pub static FOCUSED                      : c_int = 0x00020001;
pub static ICONIFIED                    : c_int = 0x00020002;
pub static CONTEXT_REVISION             : c_int = 0x00020004;

/* glfwWindowHint tokens */
pub static RED_BITS                     : c_int = 0x00021000;
pub static GREEN_BITS                   : c_int = 0x00021001;
pub static BLUE_BITS                    : c_int = 0x00021002;
pub static ALPHA_BITS                   : c_int = 0x00021003;
pub static DEPTH_BITS                   : c_int = 0x00021004;
pub static STENCIL_BITS                 : c_int = 0x00021005;
pub static ACCUM_RED_BITS               : c_int = 0x00021006;
pub static ACCUM_GREEN_BITS             : c_int = 0x00021007;
pub static ACCUM_BLUE_BITS              : c_int = 0x00021008;
pub static ACCUM_ALPHA_BITS             : c_int = 0x00021009;
pub static AUX_BUFFERS                  : c_int = 0x0002100A;
pub static STEREO                       : c_int = 0x0002100B;
pub static SAMPLES                      : c_int = 0x0002100C;
pub static SRGB_CAPABLE                 : c_int = 0x0002100D;

/* Used with both glfwGetWindowParam and glfwWindowHint */
pub static CLIENT_API                   : c_int = 0x00022000;
pub static CONTEXT_VERSION_MAJOR        : c_int = 0x00022001;
pub static CONTEXT_VERSION_MINOR        : c_int = 0x00022002;
pub static CONTEXT_ROBUSTNESS           : c_int = 0x00022003;
pub static OPENGL_FORWARD_COMPAT        : c_int = 0x00022004;
pub static OPENGL_DEBUG_CONTEXT         : c_int = 0x00022005;
pub static OPENGL_PROFILE               : c_int = 0x00022006;
pub static RESIZABLE                    : c_int = 0x00022007;
pub static VISIBLE                      : c_int = 0x00022008;
pub static UNDECORATED                  : c_int = 0x00022009;

/* GLFW_CLIENT_API tokens */
pub static OPENGL_API                   : c_int = 0x00000001;
pub static OPENGL_ES_API                : c_int = 0x00000002;

/* GLFW_CONTEXT_ROBUSTNESS mode tokens */
pub static NO_ROBUSTNESS                : c_int = 0x00000000;
pub static NO_RESET_NOTIFICATION        : c_int = 0x00000001;
pub static LOSE_CONTEXT_ON_RESET        : c_int = 0x00000002;

/* GLFW_OPENGL_PROFILE bit tokens */
pub static OPENGL_NO_PROFILE            : c_int = 0x00000000;
pub static OPENGL_CORE_PROFILE          : c_int = 0x00000001;
pub static OPENGL_COMPAT_PROFILE        : c_int = 0x00000002;

/* glfwGetInputMode/glfwSetInputMode tokens */
pub static CURSOR_MODE                  : c_int = 0x00030001;
pub static STICKY_KEYS                  : c_int = 0x00030002;
pub static STICKY_MOUSE_BUTTONS         : c_int = 0x00030003;

/* GLFW_CURSOR_MODE values */
pub static CURSOR_NORMAL                : c_int = 0x00040001;
pub static CURSOR_HIDDEN                : c_int = 0x00040002;
pub static CURSOR_CAPTURED              : c_int = 0x00040003;

/* glfwGetJoystickParam tokens */
pub static PRESENT                      : c_int = 0x00050001;
pub static AXES                         : c_int = 0x00050002;
pub static BUTTONS                      : c_int = 0x00050003;

/* glfwGetError/glfwErrorString tokens */
pub static NOT_INITIALIZED              : c_int = 0x00070001;
pub static NO_CURRENT_CONTEXT           : c_int = 0x00070002;
pub static INVALID_ENUM                 : c_int = 0x00070003;
pub static INVALID_VALUE                : c_int = 0x00070004;
pub static OUT_OF_MEMORY                : c_int = 0x00070005;
pub static API_UNAVAILABLE              : c_int = 0x00070006;
pub static VERSION_UNAVAILABLE          : c_int = 0x00070007;
pub static PLATFORM_ERROR               : c_int = 0x00070008;
pub static FORMAT_UNAVAILABLE           : c_int = 0x00070009;

/* Gamma ramps */
pub static GAMMA_RAMP_SIZE              : c_int = 256;

/* Monitor event tokens */
pub static CONNECTED                    : c_int = 0x00061000;
pub static DISCONNECTED                 : c_int = 0x00061001;