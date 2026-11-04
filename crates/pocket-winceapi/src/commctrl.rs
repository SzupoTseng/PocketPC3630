//! Common Controls library (`commctrl.dll`).
//!
//! Pocket PC apps that draw their own UI still call into commctrl
//! for [`InitCommonControls`], [`InitCommonControlsEx`] and a few
//! [`ImageList_*`] helpers. The library on PPC2003 exports
//! everything by ordinal only (no name table), so we register both
//! the friendly name (looked up via `data/commctrl-ordinals.json`,
//! when present) and the bare ordinal form.

use pocket_kernel::{DispatchOutcome, KernelError, StatusBar};

use crate::{coredll::FAKE_STATUSBAR_HWND, CallCtx, WinCeDispatcher};

pub fn register(d: &mut WinCeDispatcher) {
    let dll = "commctrl.dll";

    // Named exports — present in the WM5/WM6 SDK headers even when
    // the lib stripped names — that succeed without doing anything. The
    // image list calls we model come from `register_image_lists`.
    for f in [
        "InitCommonControls",
        "InitCommonControlsEx",
        "ImageList_ReplaceIcon",
        "_TrackMouseEvent",
    ] {
        d.register_handler(dll, f, ok);
    }
    crate::coredll::register_image_lists(d, dll);
    crate::coredll::register_command_bars(d, dll);

    // #2, #4 and #5 are successful no-op common-control calls in the
    // PPC2003 ordinal space used by Cubis.
    d.register_handler(dll, "ord:2", ok);
    d.register_handler(dll, "ord:4", ok);
    d.register_handler(dll, "ord:5", ok);

    // The Pocket PC 2000 command bar calls, identified from Pocket Slay's
    // arguments: #3 (hInst, hwndParent, id) is CommandBar_Create, #8
    // (hwndCB, hInst, idMenu, iButton) CommandBar_InsertMenubar, #9
    // (hwndCB, iButton) CommandBar_GetMenu and #10 (hwndCB, flags, 0)
    // CommandBar_AddAdornments. As plain successes they handed back 1 as
    // both the bar's window and its menu.
    crate::coredll::register_command_bar_ordinals(d, dll);

    // #17 is `CreateStatusWindowW` in the PPC2002 ordinal space.
    // PPC2002 Solitaire calls it as
    //   (0x50000003, NULL, 0xdead0001, 45)
    // = (WS_CHILD|WS_VISIBLE|CCS_BOTTOM, no initial text, hwndParent,
    //   control id 45), then drives it with SB_SETPARTS(2) and two
    // SB_SETTEXTW calls carrying "Time: %d " and "  Score: ". The app
    // imports no text-output API at all, so unless we both create the
    // control and draw it, the status strip can never appear.
    d.register_handler(dll, "ord:17", create_status_window);
    for f in ["CreateStatusWindow", "CreateStatusWindowW"] {
        d.register_handler(dll, f, create_status_window);
    }

    // Every other ordinal from 1 to 80 succeeds without doing anything.
    // The most-frequent culprits for "unimplemented call ->
    // commctrl.dll!#N" on Pocket PC are:
    //
    //   #1  InitCommonControls
    //   #2  InitCommonControlsEx
    //   #6  CreateUpDownControl
    //
    // Returning success leaves the control invisible but doesn't crash
    // anything downstream.
    for ord in 1u16..=80 {
        let name = format!("ord:{ord}");
        if !d.has_handler(dll, &name) {
            d.register_handler(dll, &name, ok);
        }
    }
}

/// `CreateStatusWindowW(style, lpszText, hwndParent, wID)`.
fn create_status_window(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let style = ctx.arg_u32(0)?;
    let text = ctx.arg_u32(1)?;
    let parent = ctx.arg_u32(2)?;
    let id = ctx.arg_u32(3)?;

    let mut bar = StatusBar {
        parent,
        height: StatusBar::DEFAULT_HEIGHT,
        ..Default::default()
    };
    // A non-NULL lpszText seeds part 0 — CreateStatusWindow is
    // documented as equivalent to creating the control and sending it
    // one SB_SETTEXT.
    if text != 0 {
        let s = String::from_utf16_lossy(&read_status_text(ctx, text));
        if !s.is_empty() {
            bar.set_part_text(0, s);
        }
    }
    ctx.kernel.status_bar = Some(bar);
    log::debug!(
        "CreateStatusWindowW(style=0x{style:08x}, parent=0x{parent:08x}, id={id}) \
         -> hwnd=0x{FAKE_STATUSBAR_HWND:08x}"
    );
    Ok(DispatchOutcome::ReturnedR0(FAKE_STATUSBAR_HWND))
}

/// Read a NUL-terminated UTF-16 string, bounded so a bogus pointer
/// can't spin.
fn read_status_text(ctx: &mut CallCtx<'_>, p: u32) -> Vec<u16> {
    let mut out = Vec::new();
    for i in 0..256u32 {
        match ctx.cpu.read_u32_le(p + i * 2) {
            Ok(v) => {
                let c = (v & 0xffff) as u16;
                if c == 0 {
                    break;
                }
                out.push(c);
            }
            Err(_) => break,
        }
    }
    out
}

fn ok(_ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    Ok(DispatchOutcome::ReturnedR0(1))
}
