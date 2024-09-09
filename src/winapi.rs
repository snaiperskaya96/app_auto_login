use std::ffi::c_void;
use std::ffi::c_int;

pub type HWND = *mut c_void;
pub type LPARAM = *mut usize;
pub type LPRECT = *mut Rect;
pub type LPPOINT = *mut Point;

#[link(name = "user32")]
extern "system" {
    pub fn SetFocus(hWnd: HWND) -> HWND;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: *mut u32) -> u32;
    pub fn EnumWindows(lpEnumFunc: extern "system" fn(HWND, LPARAM) -> c_int, lParam: LPARAM) -> c_int;
    pub fn SetForegroundWindow(hWnd: HWND) -> c_int;
    pub fn SetActiveWindow(hWnd: HWND) -> HWND;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> c_int;
    pub fn ClientToScreen(hWnd: HWND, lpPoint: LPPOINT) -> c_int;
}

#[repr(C)]
#[derive(Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn is_zero(&self) -> bool {
        return self.coords_is_zero() && self.right == 0 && self.bottom == 0;
    }

    pub fn coords_is_zero(&self) -> bool {
        return self.left == 0 && self.top == 0;
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}