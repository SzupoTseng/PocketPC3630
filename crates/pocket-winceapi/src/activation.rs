//! Activation, focus and synchronous message chains for windows with
//! handles of their own.
//!
//! On a device, `ShowWindow`, `SetActiveWindow`, `SetForegroundWindow`,
//! `SetFocus`, `EnableWindow` and `DestroyWindow` send their messages
//! (`WM_NCACTIVATE`, `WM_ACTIVATE`, `WM_KILLFOCUS`, `WM_SETFOCUS`,
//! `WM_ENABLE`, `WM_DESTROY`) to the windows' procedures before they
//! return. The orders here are the ones Device Emulator shows for
//! `tools/wince-window-test` (its log is in that folder's `reference/`).
//!
//! It applies where every window has a handle of its own (MFC programs, or
//! any program with `POCKETHLE_SEPARATE_WINDOWS=1`). Everything else keeps
//! the single-window model.
//!
//! A chain runs like the `WM_CREATE` that `CreateWindowExW` sends: the API
//! handler jumps to the first window procedure with LR pointing back at its
//! own thunk, so when that procedure returns the same handler runs again,
//! sees the chain on top (SP back where the call started) and sends the next
//! message. A call to another API from inside a procedure runs deeper on
//! the stack and is a call of its own.

use std::collections::VecDeque;

use pocket_cpu::regs::ArmReg;
use pocket_kernel::{DispatchOutcome, GuestCallFrame, KernelError};

use crate::coredll::{separate_windows, FAKE_HWND, WS_CHILD_STYLE, WS_VISIBLE_STYLE};
use crate::CallCtx;

const WM_DESTROY: u32 = 0x0002;
const WM_ACTIVATE: u32 = 0x0006;
const WM_SETFOCUS: u32 = 0x0007;
const WM_KILLFOCUS: u32 = 0x0008;
const WM_ENABLE: u32 = 0x000A;
const WM_NCACTIVATE: u32 = 0x0086;
const WA_INACTIVE: u32 = 0;
const WA_ACTIVE: u32 = 1;

/// What a chain does once its last message has been answered.
enum Finish {
    /// Return this value to the caller.
    Return(u32),
    /// Remove these destroyed windows, then return TRUE.
    Destroy(Vec<u32>),
}

/// One API call delivering its messages.
struct Chain {
    /// The thunk of the API that started it: only that API continues it.
    thunk_va: u32,
    frame: GuestCallFrame,
    messages: VecDeque<(u32, u32, u32, u32)>,
    finish: Finish,
}

thread_local! {
    /// The chains in flight, innermost last.
    static CHAINS: std::cell::RefCell<Vec<Chain>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Whether windows here follow a device's activation and focus.
pub(crate) fn applies(ctx: &CallCtx<'_>, hwnd: u32) -> bool {
    separate_windows(ctx) && (hwnd == FAKE_HWND || ctx.kernel.guest_windows.contains_key(&hwnd))
}

/// If a chain started by this API is waiting on a procedure that has just
/// returned, carry on with it.
pub(crate) fn resume(ctx: &mut CallCtx<'_>) -> Result<Option<DispatchOutcome>, KernelError> {
    let sp = ctx.cpu.read_reg(ArmReg::Sp)?;
    let thunk_va = ctx.thunk.thunk_va;
    let ours = CHAINS.with(|c| {
        c.borrow()
            .last()
            .is_some_and(|chain| chain.thunk_va == thunk_va && sp >= chain.frame.sp)
    });
    if !ours {
        return Ok(None);
    }
    deliver(ctx).map(Some)
}

/// Start a chain for the API being called: send `messages` in order, then
/// `finish`.
fn start(
    ctx: &mut CallCtx<'_>,
    messages: Vec<(u32, u32, u32, u32)>,
    finish: Finish,
) -> Result<DispatchOutcome, KernelError> {
    let frame = GuestCallFrame {
        args: [
            ctx.cpu.read_reg(ArmReg::R0)?,
            ctx.cpu.read_reg(ArmReg::R1)?,
            ctx.cpu.read_reg(ArmReg::R2)?,
            ctx.cpu.read_reg(ArmReg::R3)?,
        ],
        lr: ctx.cpu.read_reg(ArmReg::Lr)?,
        sp: ctx.cpu.read_reg(ArmReg::Sp)?,
    };
    let chain = Chain {
        thunk_va: ctx.thunk.thunk_va,
        frame,
        messages: messages.into(),
        finish,
    };
    CHAINS.with(|c| c.borrow_mut().push(chain));
    deliver(ctx)
}

/// The procedure answering messages for `hwnd`, if it has one.
fn procedure(ctx: &CallCtx<'_>, hwnd: u32) -> u32 {
    if hwnd == FAKE_HWND {
        ctx.kernel
            .window_procs
            .get(&hwnd)
            .copied()
            .unwrap_or(ctx.kernel.wnd_proc)
    } else {
        ctx.kernel.window_procs.get(&hwnd).copied().unwrap_or(0)
    }
}

/// Send the innermost chain's next message, or finish it.
fn deliver(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    loop {
        let next = CHAINS.with(|c| {
            c.borrow_mut()
                .last_mut()
                .and_then(|ch| ch.messages.pop_front())
        });
        let Some((hwnd, message, wparam, lparam)) = next else {
            break;
        };
        // A window destroyed meanwhile hears nothing more.
        if hwnd != FAKE_HWND && !ctx.kernel.guest_windows.contains_key(&hwnd) {
            continue;
        }
        let proc_ = procedure(ctx, hwnd);
        if proc_ == 0 {
            continue;
        }
        ctx.cpu.write_reg(ArmReg::R0, hwnd)?;
        ctx.cpu.write_reg(ArmReg::R1, message)?;
        ctx.cpu.write_reg(ArmReg::R2, wparam)?;
        ctx.cpu.write_reg(ArmReg::R3, lparam)?;
        ctx.cpu.write_reg(ArmReg::Lr, ctx.thunk.thunk_va)?;
        return Ok(DispatchOutcome::JumpTo(proc_));
    }
    let chain = CHAINS
        .with(|c| c.borrow_mut().pop())
        .expect("a chain in flight");
    let result = match chain.finish {
        Finish::Return(value) => value,
        Finish::Destroy(doomed) => {
            for hwnd in &doomed {
                if *hwnd == FAKE_HWND {
                    ctx.kernel.main_window_destroyed = true;
                    continue;
                }
                ctx.kernel.guest_windows.remove(hwnd);
                ctx.kernel.window_procs.remove(hwnd);
                ctx.kernel.window_classes.remove(hwnd);
                ctx.kernel.disabled_windows.remove(hwnd);
            }
            ctx.kernel
                .posted_messages
                .retain(|(hwnd, ..)| *hwnd == FAKE_HWND || !doomed.contains(hwnd));
            1
        }
    };
    let frame = chain.frame;
    ctx.cpu.write_reg(ArmReg::Sp, frame.sp)?;
    ctx.cpu.write_reg(ArmReg::R1, frame.args[1])?;
    ctx.cpu.write_reg(ArmReg::R2, frame.args[2])?;
    ctx.cpu.write_reg(ArmReg::R3, frame.args[3])?;
    ctx.cpu.write_reg(ArmReg::Lr, frame.lr)?;
    Ok(DispatchOutcome::ReturnedR0(result))
}

// ---- State ------------------------------------------------------------------

/// The active window: the main window until another is activated.
pub(crate) fn active(ctx: &CallCtx<'_>) -> u32 {
    if ctx.kernel.active_window != 0 {
        ctx.kernel.active_window
    } else if ctx.kernel.main_window_destroyed {
        0
    } else {
        FAKE_HWND
    }
}

/// The focused window: the main window until another takes the focus.
pub(crate) fn focus(ctx: &CallCtx<'_>) -> u32 {
    if ctx.kernel.focus_window != 0 {
        ctx.kernel.focus_window
    } else if ctx.kernel.main_window_destroyed || ctx.kernel.active_window != 0 {
        // Activation always sets the focus; 0 here means it was taken away.
        0
    } else {
        FAKE_HWND
    }
}

/// The top-level window `hwnd` sits in.
fn top_level(ctx: &CallCtx<'_>, mut hwnd: u32) -> u32 {
    for _ in 0..64 {
        match ctx.kernel.guest_windows.get(&hwnd) {
            Some(win) if win.style & WS_CHILD_STYLE != 0 && win.parent != 0 => hwnd = win.parent,
            _ => break,
        }
    }
    hwnd
}

/// The messages that make `new` the active window and give it the focus,
/// and the state to match. Nothing when it is active already.
fn activate(ctx: &mut CallCtx<'_>, new: u32) -> Vec<(u32, u32, u32, u32)> {
    let old = active(ctx);
    if old == new {
        return Vec::new();
    }
    // Pin down who has the focus now: it is worked out from the active
    // window while nothing has taken the focus yet.
    ctx.kernel.focus_window = focus(ctx);
    let mut messages = Vec::new();
    if old != 0 {
        messages.push((old, WM_NCACTIVATE, 0, 0));
        messages.push((old, WM_ACTIVATE, WA_INACTIVE, new));
    }
    messages.push((new, WM_NCACTIVATE, 1, 0));
    messages.push((new, WM_ACTIVATE, WA_ACTIVE, old));
    ctx.kernel.active_window = new;
    messages.extend(move_focus(ctx, new));
    messages
}

/// The messages that move the focus to `new`, and the state to match.
fn move_focus(ctx: &mut CallCtx<'_>, new: u32) -> Vec<(u32, u32, u32, u32)> {
    let old = focus(ctx);
    if old == new {
        return Vec::new();
    }
    let mut messages = Vec::new();
    if old != 0 {
        messages.push((old, WM_KILLFOCUS, new, 0));
    }
    ctx.kernel.focus_window = new;
    messages.push((new, WM_SETFOCUS, old, 0));
    messages
}

// ---- The calls --------------------------------------------------------------

/// `SetActiveWindow(hwnd)`: activate it; answers the window active before.
pub(crate) fn set_active_window(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
) -> Result<DispatchOutcome, KernelError> {
    let before = active(ctx);
    let top = top_level(ctx, hwnd);
    let messages = activate(ctx, top);
    start(ctx, messages, Finish::Return(before))
}

/// `SetForegroundWindow(hwnd)`: activate it; answers TRUE.
pub(crate) fn set_foreground_window(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
) -> Result<DispatchOutcome, KernelError> {
    let top = top_level(ctx, hwnd);
    let messages = activate(ctx, top);
    start(ctx, messages, Finish::Return(1))
}

/// `SetFocus(hwnd)`: activate its top-level window if need be, then move
/// the focus to it; answers the window focused before.
pub(crate) fn set_focus(ctx: &mut CallCtx<'_>, hwnd: u32) -> Result<DispatchOutcome, KernelError> {
    let before = focus(ctx);
    let top = top_level(ctx, hwnd);
    let mut messages = activate(ctx, top);
    messages.extend(move_focus(ctx, hwnd));
    start(ctx, messages, Finish::Return(before))
}

/// `ShowWindow` making a top-level window visible activates it, as on a
/// device; `None` when there is nothing to send.
pub(crate) fn shown(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
    was_visible: bool,
) -> Result<Option<DispatchOutcome>, KernelError> {
    let top_level_window = ctx
        .kernel
        .guest_windows
        .get(&hwnd)
        .is_some_and(|win| win.style & WS_CHILD_STYLE == 0);
    if !top_level_window && hwnd != FAKE_HWND {
        return Ok(None);
    }
    let messages = activate(ctx, hwnd);
    if messages.is_empty() {
        return Ok(None);
    }
    start(ctx, messages, Finish::Return(u32::from(was_visible))).map(Some)
}

/// A top-level window created visible is activated before
/// `CreateWindowExW` returns its handle.
pub(crate) fn created(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
) -> Result<Option<DispatchOutcome>, KernelError> {
    let visible_top_level =
        ctx.kernel.guest_windows.get(&hwnd).is_some_and(|win| {
            win.style & WS_CHILD_STYLE == 0 && win.style & WS_VISIBLE_STYLE != 0
        });
    if !visible_top_level {
        return Ok(None);
    }
    let messages = activate(ctx, hwnd);
    if messages.is_empty() {
        return Ok(None);
    }
    start(ctx, messages, Finish::Return(hwnd)).map(Some)
}

/// `EnableWindow(hwnd, enable)`: sends WM_ENABLE when the state changes;
/// answers whether the window was disabled before.
pub(crate) fn enable_window(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
    enable: bool,
) -> Result<DispatchOutcome, KernelError> {
    let was_disabled = ctx.kernel.disabled_windows.contains(&hwnd);
    if was_disabled == !enable {
        return Ok(DispatchOutcome::ReturnedR0(u32::from(was_disabled)));
    }
    if enable {
        ctx.kernel.disabled_windows.remove(&hwnd);
    } else {
        ctx.kernel.disabled_windows.insert(hwnd);
    }
    start(
        ctx,
        vec![(hwnd, WM_ENABLE, u32::from(enable), 0)],
        Finish::Return(u32::from(was_disabled)),
    )
}

/// Whether `hwnd` is being destroyed by a `DestroyWindow` still running.
fn being_destroyed(hwnd: u32) -> bool {
    CHAINS.with(|c| {
        c.borrow().iter().any(|chain| match &chain.finish {
            Finish::Destroy(doomed) => doomed.contains(&hwnd),
            Finish::Return(_) => false,
        })
    })
}

/// The windows `DestroyWindow(target)` destroys, in the order they hear
/// WM_DESTROY on a device: the windows it owns first, each with everything
/// of its own, then the window itself, then its children, newest first.
fn destroy_order(ctx: &CallCtx<'_>, target: u32, out: &mut Vec<u32>) {
    if out.contains(&target) || out.len() > 512 {
        return;
    }
    let newest_first = |pick: &dyn Fn(&pocket_kernel::GuestWindow) -> bool| {
        let mut found: Vec<u32> = ctx
            .kernel
            .guest_windows
            .iter()
            .filter(|(_, win)| pick(win))
            .map(|(hwnd, _)| *hwnd)
            .collect();
        found.sort_unstable_by(|a, b| b.cmp(a));
        found
    };
    for owned in newest_first(&|win| win.owner == target) {
        destroy_order(ctx, owned, out);
    }
    out.push(target);
    for child in newest_first(&|win| win.parent == target) {
        destroy_order(ctx, child, out);
    }
}

/// `DestroyWindow(target)`: the active window is deactivated and loses the
/// focus first, then WM_DESTROY goes to everything destroyed.
pub(crate) fn destroy_window(
    ctx: &mut CallCtx<'_>,
    target: u32,
) -> Result<DispatchOutcome, KernelError> {
    if being_destroyed(target) {
        return Ok(DispatchOutcome::ReturnedR0(1));
    }
    let mut doomed = Vec::new();
    destroy_order(ctx, target, &mut doomed);
    doomed.retain(|hwnd| !being_destroyed(*hwnd));
    for hwnd in &doomed {
        ctx.kernel.controls.destroy_children_of(*hwnd);
    }
    let mut messages = Vec::new();
    let was_active = active(ctx);
    if doomed.contains(&was_active) {
        // Activation passes to the destroyed window's owner, or else the
        // main window; with neither left, nothing is active.
        let owner = ctx.kernel.guest_windows.get(&target).map_or(0, |w| w.owner);
        let next = [owner, FAKE_HWND]
            .into_iter()
            .find(|w| {
                *w != 0
                    && !doomed.contains(w)
                    && (*w != FAKE_HWND || !ctx.kernel.main_window_destroyed)
            })
            .unwrap_or(0);
        if doomed.contains(&FAKE_HWND) {
            ctx.kernel.main_window_destroyed = true;
        }
        if next != 0 {
            messages.extend(activate(ctx, next));
        } else {
            messages.push((was_active, WM_NCACTIVATE, 0, 0));
            messages.push((was_active, WM_ACTIVATE, WA_INACTIVE, 0));
            let had_focus = focus(ctx);
            if had_focus != 0 {
                messages.push((had_focus, WM_KILLFOCUS, 0, 0));
            }
            ctx.kernel.active_window = 0;
            ctx.kernel.focus_window = 0;
        }
    } else if doomed.contains(&focus(ctx)) {
        ctx.kernel.focus_window = 0;
    }
    messages.extend(doomed.iter().map(|hwnd| (*hwnd, WM_DESTROY, 0, 0)));
    start(ctx, messages, Finish::Destroy(doomed))
}
