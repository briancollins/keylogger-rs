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

// CGEventRef loggerCallback(CGEventTapProxy proxy, CGEventType type, CGEventRef event, void* context)

pub type CGEventTapCallBack = extern fn(CGEventTapProxy, CGEventType, CGEventRef, *const libc::c_void) -> CGEventRef;

extern {
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;

    pub static kCFRunLoopDefaultMode: CFStringRef;
    pub static kCFRunLoopCommonModes: CFStringRef;
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CGEventTapCreate(tap: libc::uint32_t, place: libc::uint32_t, options: libc::uint32_t, eventsOfInterest: CGEventMask, callback: CGEventTapCallBack, userInfo: *const libc::c_void ) -> CFMachPortRef;
    pub fn CFMachPortCreateRunLoopSource(allocator: *const libc::c_void, port: CFMachPortRef,
                                         order: libc::uint64_t) -> CFRunLoopSourceRef;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: CFBooleanRef);
    pub fn CFRunLoopRun();
}

extern fn logger_callback(proxy: CGEventTapProxy, eventType: CGEventType, event: CGEventRef, context: *const libc::c_void) -> CGEventRef {
    println!("keystroke detected");
    event
}


fn main() {
    let key_down = 1 << 10;
    let tap = unsafe { CGEventTapCreate(0, 0, 0, key_down, logger_callback, std::ptr::null()) };
    if tap.is_null() {
        panic!("You need to run as root");
    }

    let source = unsafe { CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0) };
    let run_loop = unsafe { CFRunLoopGetCurrent() };
    unsafe {
        CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode);
        CGEventTapEnable(tap, kCFBooleanTrue);
        CFRunLoopRun();
    }
}
