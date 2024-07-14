//! C definitions used by libnative that don't belong in liblibc

#![allow(nonstandard_style)]
#![cfg_attr(test, allow(dead_code))]
#![unstable(issue = "none", feature = "windows_c")]
#![allow(clippy::style)]

use crate::ffi::CStr;
use crate::mem;
pub use crate::os::raw::c_int;
use crate::os::raw::{c_char, c_uint, c_ulong, c_ushort, c_void};
use crate::os::windows::io::{AsRawHandle, BorrowedHandle};
use crate::ptr;

pub(super) mod windows_targets;

mod windows_sys;
pub use windows_sys::*;

pub type WCHAR = u16;

pub type socklen_t = c_int;
pub type ADDRESS_FAMILY = c_ushort;
pub use FD_SET as fd_set;
pub use LINGER as linger;
pub use TIMEVAL as timeval;

pub const INVALID_HANDLE_VALUE: HANDLE = ::core::ptr::without_provenance_mut(-1i32 as _);

// https://learn.microsoft.com/en-us/cpp/c-runtime-library/exit-success-exit-failure?view=msvc-170
pub const EXIT_SUCCESS: u32 = 0;
pub const EXIT_FAILURE: u32 = 1;

#[cfg(target_vendor = "win7")]
pub const CONDITION_VARIABLE_INIT: CONDITION_VARIABLE = CONDITION_VARIABLE { Ptr: ptr::null_mut() };
#[cfg(target_vendor = "win7")]
pub const SRWLOCK_INIT: SRWLOCK = SRWLOCK { Ptr: ptr::null_mut() };
#[cfg(not(target_thread_local))]
pub const INIT_ONCE_STATIC_INIT: INIT_ONCE = INIT_ONCE { Ptr: ptr::null_mut() };

// Some windows_sys types have different signs than the types we use.
pub const OBJ_DONT_REPARSE: u32 = windows_sys::OBJ_DONT_REPARSE as u32;
pub const FRS_ERR_SYSVOL_POPULATE_TIMEOUT: u32 =
    windows_sys::FRS_ERR_SYSVOL_POPULATE_TIMEOUT as u32;
pub const AF_INET: c_int = windows_sys::AF_INET as c_int;
pub const AF_INET6: c_int = windows_sys::AF_INET6 as c_int;

#[repr(C)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_uint,
}

// Equivalent to the `NT_SUCCESS` C preprocessor macro.
// See: https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/using-ntstatus-values
pub fn nt_success(status: NTSTATUS) -> bool {
    status >= 0
}

impl UNICODE_STRING {
    pub fn from_ref(slice: &[u16]) -> Self {
        let len = mem::size_of_val(slice);
        Self { Length: len as _, MaximumLength: len as _, Buffer: slice.as_ptr() as _ }
    }
}

impl Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        Self {
            Length: mem::size_of::<Self>() as _,
            RootDirectory: ptr::null_mut(),
            ObjectName: ptr::null_mut(),
            Attributes: 0,
            SecurityDescriptor: ptr::null_mut(),
            SecurityQualityOfService: ptr::null_mut(),
        }
    }
}

impl IO_STATUS_BLOCK {
    pub const PENDING: Self =
        IO_STATUS_BLOCK { Anonymous: IO_STATUS_BLOCK_0 { Status: STATUS_PENDING }, Information: 0 };
    pub fn status(&self) -> NTSTATUS {
        // SAFETY: If `self.Anonymous.Status` was set then this is obviously safe.
        // If `self.Anonymous.Pointer` was set then this is the equivalent to converting
        // the pointer to an integer, which is also safe.
        // Currently the only safe way to construct `IO_STATUS_BLOCK` outside of
        // this module is to call the `default` method, which sets the `Status`.
        unsafe { self.Anonymous.Status }
    }
}

/// NB: Use carefully! In general using this as a reference is likely to get the
/// provenance wrong for the `rest` field!
#[repr(C)]
pub struct REPARSE_DATA_BUFFER {
    pub ReparseTag: c_uint,
    pub ReparseDataLength: c_ushort,
    pub Reserved: c_ushort,
    pub rest: (),
}

/// NB: Use carefully! In general using this as a reference is likely to get the
/// provenance wrong for the `PathBuffer` field!
#[repr(C)]
pub struct SYMBOLIC_LINK_REPARSE_BUFFER {
    pub SubstituteNameOffset: c_ushort,
    pub SubstituteNameLength: c_ushort,
    pub PrintNameOffset: c_ushort,
    pub PrintNameLength: c_ushort,
    pub Flags: c_ulong,
    pub PathBuffer: WCHAR,
}

#[repr(C)]
pub struct MOUNT_POINT_REPARSE_BUFFER {
    pub SubstituteNameOffset: c_ushort,
    pub SubstituteNameLength: c_ushort,
    pub PrintNameOffset: c_ushort,
    pub PrintNameLength: c_ushort,
    pub PathBuffer: WCHAR,
}

#[repr(C)]
pub struct SOCKADDR_STORAGE_LH {
    pub ss_family: ADDRESS_FAMILY,
    pub __ss_pad1: [c_char; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [c_char; 112],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: ADDRESS_FAMILY,
    pub sin_port: c_ushort,
    pub sin_addr: in_addr,
    pub sin_zero: [c_char; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: ADDRESS_FAMILY,
    pub sin6_port: c_ushort,
    pub sin6_flowinfo: c_ulong,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

// Desktop specific functions & types
cfg_if::cfg_if! {
if #[cfg(not(target_vendor = "uwp"))] {
    pub const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
}
}

pub unsafe extern "system" fn WriteFileEx(
    hFile: BorrowedHandle<'_>,
    lpBuffer: *mut ::core::ffi::c_void,
    nNumberOfBytesToWrite: u32,
    lpOverlapped: *mut OVERLAPPED,
    lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
) -> BOOL {
    windows_sys::WriteFileEx(
        hFile.as_raw_handle(),
        lpBuffer.cast::<u8>(),
        nNumberOfBytesToWrite,
        lpOverlapped,
        lpCompletionRoutine,
    )
}

pub unsafe extern "system" fn ReadFileEx(
    hFile: BorrowedHandle<'_>,
    lpBuffer: *mut ::core::ffi::c_void,
    nNumberOfBytesToRead: u32,
    lpOverlapped: *mut OVERLAPPED,
    lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE,
) -> BOOL {
    windows_sys::ReadFileEx(
        hFile.as_raw_handle(),
        lpBuffer.cast::<u8>(),
        nNumberOfBytesToRead,
        lpOverlapped,
        lpCompletionRoutine,
    )
}

// POSIX compatibility shims.
pub unsafe fn recv(socket: SOCKET, buf: *mut c_void, len: c_int, flags: c_int) -> c_int {
    windows_sys::recv(socket, buf.cast::<u8>(), len, flags)
}
pub unsafe fn send(socket: SOCKET, buf: *const c_void, len: c_int, flags: c_int) -> c_int {
    windows_sys::send(socket, buf.cast::<u8>(), len, flags)
}
pub unsafe fn recvfrom(
    socket: SOCKET,
    buf: *mut c_void,
    len: c_int,
    flags: c_int,
    addr: *mut SOCKADDR,
    addrlen: *mut c_int,
) -> c_int {
    windows_sys::recvfrom(socket, buf.cast::<u8>(), len, flags, addr, addrlen)
}
pub unsafe fn sendto(
    socket: SOCKET,
    buf: *const c_void,
    len: c_int,
    flags: c_int,
    addr: *const SOCKADDR,
    addrlen: c_int,
) -> c_int {
    windows_sys::sendto(socket, buf.cast::<u8>(), len, flags, addr, addrlen)
}
pub unsafe fn getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const ADDRINFOA,
    res: *mut *mut ADDRINFOA,
) -> c_int {
    windows_sys::getaddrinfo(node.cast::<u8>(), service.cast::<u8>(), hints, res)
}

cfg_if::cfg_if! {
if #[cfg(not(target_vendor = "uwp"))] {
pub unsafe fn NtReadFile(
    filehandle: BorrowedHandle<'_>,
    event: HANDLE,
    apcroutine: PIO_APC_ROUTINE,
    apccontext: *mut c_void,
    iostatusblock: &mut IO_STATUS_BLOCK,
    buffer: *mut crate::mem::MaybeUninit<u8>,
    length: u32,
    byteoffset: Option<&i64>,
    key: Option<&u32>,
) -> NTSTATUS {
    windows_sys::NtReadFile(
        filehandle.as_raw_handle(),
        event,
        apcroutine,
        apccontext,
        iostatusblock,
        buffer.cast::<c_void>(),
        length,
        byteoffset.map(|o| o as *const i64).unwrap_or(ptr::null()),
        key.map(|k| k as *const u32).unwrap_or(ptr::null()),
    )
}
pub unsafe fn NtWriteFile(
    filehandle: BorrowedHandle<'_>,
    event: HANDLE,
    apcroutine: PIO_APC_ROUTINE,
    apccontext: *mut c_void,
    iostatusblock: &mut IO_STATUS_BLOCK,
    buffer: *const u8,
    length: u32,
    byteoffset: Option<&i64>,
    key: Option<&u32>,
) -> NTSTATUS {
    windows_sys::NtWriteFile(
        filehandle.as_raw_handle(),
        event,
        apcroutine,
        apccontext,
        iostatusblock,
        buffer.cast::<c_void>(),
        length,
        byteoffset.map(|o| o as *const i64).unwrap_or(ptr::null()),
        key.map(|k| k as *const u32).unwrap_or(ptr::null()),
    )
}
}
}

// Use raw-dylib to import ProcessPrng as we can't rely on there being an import library.
cfg_if::cfg_if! {
if #[cfg(not(target_vendor = "win7"))] {
    #[cfg(target_arch = "x86")]
    #[link(name = "bcryptprimitives", kind = "raw-dylib", import_name_type = "undecorated")]
    extern "system" {
        pub fn ProcessPrng(pbdata: *mut u8, cbdata: usize) -> BOOL;
    }
    #[cfg(not(target_arch = "x86"))]
    #[link(name = "bcryptprimitives", kind = "raw-dylib")]
    extern "system" {
        pub fn ProcessPrng(pbdata: *mut u8, cbdata: usize) -> BOOL;
    }
}}

// Functions that aren't available on every version of Windows that we support,
// but we still use them and just provide some form of a fallback implementation.
compat_fn_with_fallback! {
    pub static KERNEL32: &CStr = c"kernel32";

    // >= Win10 1607
    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreaddescription
    pub fn SetThreadDescription(hthread: HANDLE, lpthreaddescription: PCWSTR) -> HRESULT {
        SetLastError(ERROR_CALL_NOT_IMPLEMENTED as u32); E_NOTIMPL
    }

    // >= Win10 1607
    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreaddescription
    pub fn GetThreadDescription(hthread: HANDLE, lpthreaddescription: *mut PWSTR) -> HRESULT {
        SetLastError(ERROR_CALL_NOT_IMPLEMENTED as u32); E_NOTIMPL
    }

    // >= Win8 / Server 2012
    // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimepreciseasfiletime
    #[cfg(target_vendor = "win7")]
    pub fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut FILETIME) -> () {
        GetSystemTimeAsFileTime(lpsystemtimeasfiletime)
    }

    // >= Win11 / Server 2022
    // https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-gettemppath2a
    pub fn GetTempPath2W(bufferlength: u32, buffer: PWSTR) -> u32 {
        GetTempPathW(bufferlength, buffer)
    }
}

#[cfg(not(target_vendor = "win7"))]
// Use raw-dylib to import synchronization functions to workaround issues with the older mingw import library.
#[cfg_attr(
    target_arch = "x86",
    link(
        name = "api-ms-win-core-synch-l1-2-0",
        kind = "raw-dylib",
        import_name_type = "undecorated"
    )
)]
#[cfg_attr(
    not(target_arch = "x86"),
    link(name = "api-ms-win-core-synch-l1-2-0", kind = "raw-dylib")
)]
extern "system" {
    pub fn WaitOnAddress(
        address: *const c_void,
        compareaddress: *const c_void,
        addresssize: usize,
        dwmilliseconds: u32,
    ) -> BOOL;
    pub fn WakeByAddressSingle(address: *const c_void);
    pub fn WakeByAddressAll(address: *const c_void);
}

#[cfg(target_vendor = "win7")]
compat_fn_optional! {
    crate::sys::compat::load_synch_functions();
    pub fn WaitOnAddress(
        address: *const ::core::ffi::c_void,
        compareaddress: *const ::core::ffi::c_void,
        addresssize: usize,
        dwmilliseconds: u32
    ) -> BOOL;
    pub fn WakeByAddressSingle(address: *const ::core::ffi::c_void);
}

#[cfg(any(target_vendor = "win7", target_vendor = "uwp"))]
compat_fn_with_fallback! {
    pub static NTDLL: &CStr = c"ntdll";

    #[cfg(target_vendor = "win7")]
    pub fn NtCreateKeyedEvent(
        KeyedEventHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut c_void,
        Flags: u32
    ) -> NTSTATUS {
        panic!("keyed events not available")
    }
    #[cfg(target_vendor = "win7")]
    pub fn NtReleaseKeyedEvent(
        EventHandle: HANDLE,
        Key: *mut c_void,
        Alertable: BOOLEAN,
        Timeout: *mut c_longlong
    ) -> NTSTATUS {
        panic!("keyed events not available")
    }
    #[cfg(target_vendor = "win7")]
    pub fn NtWaitForKeyedEvent(
        EventHandle: HANDLE,
        Key: *mut c_void,
        Alertable: BOOLEAN,
        Timeout: *mut c_longlong
    ) -> NTSTATUS {
        panic!("keyed events not available")
    }

    // These functions are available on UWP when lazily loaded. They will fail WACK if loaded statically.
    #[cfg(target_vendor = "uwp")]
    pub fn NtCreateFile(
        filehandle: *mut HANDLE,
        desiredaccess: FILE_ACCESS_RIGHTS,
        objectattributes: *const OBJECT_ATTRIBUTES,
        iostatusblock: *mut IO_STATUS_BLOCK,
        allocationsize: *const i64,
        fileattributes: FILE_FLAGS_AND_ATTRIBUTES,
        shareaccess: FILE_SHARE_MODE,
        createdisposition: NTCREATEFILE_CREATE_DISPOSITION,
        createoptions: NTCREATEFILE_CREATE_OPTIONS,
        eabuffer: *const ::core::ffi::c_void,
        ealength: u32
    ) -> NTSTATUS {
        STATUS_NOT_IMPLEMENTED
    }
    #[cfg(target_vendor = "uwp")]
    pub fn NtReadFile(
        filehandle: BorrowedHandle<'_>,
        event: HANDLE,
        apcroutine: PIO_APC_ROUTINE,
        apccontext: *mut c_void,
        iostatusblock: &mut IO_STATUS_BLOCK,
        buffer: *mut crate::mem::MaybeUninit<u8>,
        length: u32,
        byteoffset: Option<&i64>,
        key: Option<&u32>
    ) -> NTSTATUS {
        STATUS_NOT_IMPLEMENTED
    }
    #[cfg(target_vendor = "uwp")]
    pub fn NtWriteFile(
        filehandle: BorrowedHandle<'_>,
        event: HANDLE,
        apcroutine: PIO_APC_ROUTINE,
        apccontext: *mut c_void,
        iostatusblock: &mut IO_STATUS_BLOCK,
        buffer: *const u8,
        length: u32,
        byteoffset: Option<&i64>,
        key: Option<&u32>
    ) -> NTSTATUS {
        STATUS_NOT_IMPLEMENTED
    }
    #[cfg(target_vendor = "uwp")]
    pub fn RtlNtStatusToDosError(Status: NTSTATUS) -> u32 {
        Status as u32
    }
}

// # Arm32 shim
//
// AddVectoredExceptionHandler and WSAStartup use platform-specific types.
// However, Microsoft no longer supports thumbv7a so definitions for those targets
// are not included in the win32 metadata. We work around that by defining them here.
//
// Where possible, these definitions should be kept in sync with https://docs.rs/windows-sys
cfg_if::cfg_if! {
if #[cfg(not(target_vendor = "uwp"))] {
    #[link(name = "kernel32")]
    extern "system" {
        pub fn AddVectoredExceptionHandler(
            first: u32,
            handler: PVECTORED_EXCEPTION_HANDLER,
        ) -> *mut c_void;
    }
    pub type PVECTORED_EXCEPTION_HANDLER = Option<
        unsafe extern "system" fn(exceptioninfo: *mut EXCEPTION_POINTERS) -> i32,
    >;
    #[repr(C)]
    pub struct EXCEPTION_POINTERS {
        pub ExceptionRecord: *mut EXCEPTION_RECORD,
        pub ContextRecord: *mut CONTEXT,
    }
    #[cfg(target_arch = "arm")]
    pub enum CONTEXT {}
}}
// WSAStartup is only redefined here so that we can override WSADATA for Arm32
windows_targets::link!("ws2_32.dll" "system" fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSADATA) -> i32);
#[cfg(target_arch = "arm")]
#[repr(C)]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [u8; 257],
    pub szSystemStatus: [u8; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: PSTR,
}
