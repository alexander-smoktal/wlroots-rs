/// Backend includes
#include <wlr/backend.h>
#include <wlr/backend/drm.h>
#include <wlr/backend/interface.h>
#include <wlr/backend/libinput.h>
#include <wlr/backend/multi.h>
#include <wlr/backend/session.h>
#include <wlr/backend/wayland.h>
#include <wlr/backend/x11.h>
#include <wlr/backend/session/interface.h>

/// Render includes
#include <wlr/render.h>
#include <wlr/render/gles2.h>
#include <wlr/render/interface.h>
#include <wlr/render/matrix.h>

/// Type includes
#include <wlr/types/wlr_box.h>
#include <wlr/types/wlr_compositor.h>
#include <wlr/types/wlr_cursor.h>
#include <wlr/types/wlr_data_device.h>
#include <wlr/types/wlr_gamma_control.h>
#include <wlr/types/wlr_input_device.h>
#include <wlr/types/wlr_keyboard.h>
#include <wlr/types/wlr_output.h>
#include <wlr/types/wlr_output_layout.h>
#include <wlr/types/wlr_pointer.h>
#include <wlr/types/wlr_region.h>
#include <wlr/types/wlr_server_decoration.h>
#include <wlr/types/wlr_screenshooter.h>
#include <wlr/types/wlr_seat.h>
#include <wlr/types/wlr_surface.h>
#include <wlr/types/wlr_tablet_pad.h>
#include <wlr/types/wlr_tablet_tool.h>
#include <wlr/types/wlr_touch.h>
#include <wlr/types/wlr_wl_shell.h>
#include <wlr/types/wlr_xdg_shell_v6.h>

/// Util includes
#include <wlr/util/log.h>

/// Misc includes
#include <xcursor.h>
#include <xwayland.h>
#include <xkbcommon/xkbcommon.h>
