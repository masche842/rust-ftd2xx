///
/// FTDI D2XX TypeDefs
/// as defined in WinTypes.h
///
///

extern crate libc;
use libc::{c_int, c_uint, c_ushort, c_uchar, c_ulong, c_ulonglong, c_void};


#[repr(C)]
pub type   DWORD     = c_uint;
pub type   ULONG     = c_uint;
pub type   USHORT    = c_ushort;
pub type   SHORT     = c_ushort;
pub type   UCHAR     = c_uchar;
pub type   WORD      = c_ushort;
pub type   BYTE      = c_uchar;
pub type   LPBYTE    = *mut BYTE;
pub type   BOOL      = uint;
pub type   BOOLEAN   = c_uchar;
pub type   CHAR      = c_uchar;
pub type   LPBOOL    = *mut BOOL;
pub type   PUCHAR    = *mut UCHAR;
pub type   LPCSTR    = *mut char; //const
pub type   PCHAR     = *mut char;
pub type   PVOID     = *mut c_void;
pub type   HANDLE    = *mut c_void;
pub type   LONG      = uint;
pub type   INT       = int;
pub type   UINT      = uint;
pub type   LPSTR     = *mut char;
pub type   LPTSTR    = *mut char;
pub type   LPCTSTR   = *mut char; //const
pub type   LPDWORD   = *mut DWORD;
pub type   LPWORD    = *mut WORD;
pub type   PULONG    = *mut ULONG;
pub type   LPLONG    = *mut LONG;
pub type   LPVOID    = PVOID;
pub type   VOID      = c_void;
pub type   ULONGLONG = c_ulonglong;

pub type  FT_HANDLE = PVOID;
pub type  FT_DEVICE = ULONG;

struct _OVERLAPPED {
  Internal: DWORD,
  InternalHigh: DWORD,
  Offset: DWORD,
  OffsetHigh: DWORD,
  hEvent: HANDLE
}
pub type LPOVERLAPPED = _OVERLAPPED;

// typedef struct _SECURITY_ATTRIBUTES {
//   DWORD nLength;
//   LPVOID lpSecurityDescriptor;
//   BOOL bInheritHandle;
// } SECURITY_ATTRIBUTES , *LPSECURITY_ATTRIBUTES;

// typedef struct timeval SYSTEMTIME;
// typedef struct timeval FILETIME;
// #ifndef TRUE
// #define TRUE  1
// #endif
// #ifndef FALSE
// #define FALSE 0
// #endif
