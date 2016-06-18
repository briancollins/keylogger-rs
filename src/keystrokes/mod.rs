#![allow(non_upper_case_globals, improper_ctypes)]
extern crate libc;

#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "CoreGraphics", kind = "framework")]

pub type CFRunLoopRef = *const libc::c_void;
pub type CGEventType = libc::c_uint;
pub type CGEventTapProxy = *const libc::c_void;
pub type CGEventRef = *const libc::c_void;
pub type CFMachPortRef = *const libc::c_void;
pub type CGEventMask = libc::uint64_t;
pub type CFRunLoopSourceRef = *const libc::c_void;
pub type CFStringRef = *const libc::c_void;

pub type Boolean = u8;

pub struct __CFBoolean(libc::c_void);

pub type CFBooleanRef = *const __CFBoolean;
pub struct CFBoolean(CFBooleanRef);
pub static kCGEventKeyDown: CGEventType = 10;
pub static kCGEventMouseMoved: CGEventType = 5;


pub type CGEventTapCallBack = extern fn(CGEventTapProxy, CGEventType,
                                        CGEventRef, *const libc::c_void)
    -> CGEventRef;

extern {
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;

    pub static kCFRunLoopDefaultMode: CFStringRef;
    pub static kCFRunLoopCommonModes: CFStringRef;
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CGEventTapCreate(tap: libc::uint32_t, place: libc::uint32_t,
                            options: libc::uint32_t, events: CGEventMask,
                            callback: CGEventTapCallBack,
                            user_info: *const libc::c_void ) -> CFMachPortRef;
    pub fn CFMachPortCreateRunLoopSource(allocator: *const libc::c_void,
                                         port: CFMachPortRef,
                                         order: libc::uint64_t)
        -> CFRunLoopSourceRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef,
                              mode: CFStringRef);
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: CFBooleanRef);
    pub fn CFRunLoopRun();
}

extern fn logger_callback(_: CGEventTapProxy, event_type: CGEventType,
                          event: CGEventRef, _: *const libc::c_void)
    -> CGEventRef {
    if event_type == kCGEventKeyDown {
        println!("Key down!");
    } else if event_type == kCGEventMouseMoved {
        println!("Mouse moved!");
    }
    event
}

pub fn listen() {
    let key_down = 1 << kCGEventKeyDown;
    let mouse_moved = 1 << kCGEventMouseMoved;
    unsafe {
        let tap = CGEventTapCreate(0, 0, 0, key_down | mouse_moved,
                                   logger_callback, ::std::ptr::null());

        if tap.is_null() {
            panic!("This program needs to run as root");
        }

        let source = CFMachPortCreateRunLoopSource(::std::ptr::null(), tap, 0);
        let run_loop = CFRunLoopGetCurrent();
        CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode);
        CGEventTapEnable(tap, kCFBooleanTrue);
        CFRunLoopRun();
    }
}
