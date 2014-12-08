#![feature(globs)]
#![link(name = "ftd2xx",
       vers = "0.0.1",
       uuid = "",
       url = "")]
#![comment = "FTDI d2xx driver bindings"]
#![license = "MIT"]
#![crate_type = "lib"]

extern crate libc;
extern crate core;

use core::fmt;
use libc::{c_int, c_uint, c_void, c_ulong, malloc, free};
use std::string;
use std::c_vec;

use constants::*;
use typedefs::*;

mod constants;
mod typedefs;

#[allow(dead_code)]
#[link(name = "ftd2xx")]
extern {
  fn FT_Open(deviceNumber: c_int , pHandle: *mut FT_HANDLE) -> FT_STATUS;
  fn FT_OpenEx( pArg1: PVOID, Flags: DWORD, pHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_ListDevices( pArg1: PVOID, pArg2: PVOID, Flags: DWORD) -> FT_STATUS;
  fn FT_Close(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_Read(ftHandle: FT_HANDLE, lpBuffer: LPVOID, dwBytesToRead: DWORD, lpBytesReturned: LPDWORD) -> FT_STATUS;
  fn FT_Write(ftHandle: FT_HANDLE, lpBuffer: LPVOID, dwBytesToWrite: DWORD, lpBytesWritten: LPDWORD) -> FT_STATUS;
  fn FT_IoCtl(ftHandle: FT_HANDLE, dwIoControlCode: DWORD, lpInBuf: LPVOID, nInBufSize: DWORD, lpOutBuf: LPVOID, nOutBufSize: DWORD, lpBytesReturned: LPDWORD, lpOverlapped: LPOVERLAPPED) -> FT_STATUS;
  fn FT_SetBaudRate(ftHandle: FT_HANDLE, BaudRate: ULONG) -> FT_STATUS;
  fn FT_SetDivisor(ftHandle: FT_HANDLE, Divisor: USHORT) -> FT_STATUS;
  fn FT_SetDataCharacteristics(ftHandle: FT_HANDLE, WordLength: UCHAR, StopBits: UCHAR, Parity: UCHAR) -> FT_STATUS;
  fn FT_SetFlowControl(ftHandle: FT_HANDLE, FlowControl: USHORT, XonChar: UCHAR, XoffChar: UCHAR) -> FT_STATUS;
  fn FT_ResetDevice(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_SetDtr(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_ClrDtr(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_SetRts(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_ClrRts(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_GetModemStatus(ftHandle: FT_HANDLE, pModemStatus: *mut ULONG) -> FT_STATUS;
  fn FT_SetChars(ftHandle: FT_HANDLE, EventChar: UCHAR, EventCharEnabled: UCHAR, ErrorChar: UCHAR, ErrorCharEnabled: UCHAR) -> FT_STATUS;
  fn FT_Purge(ftHandle: FT_HANDLE, Mask: ULONG) -> FT_STATUS;
  fn FT_SetTimeouts(ftHandle: FT_HANDLE, ReadTimeout: ULONG, WriteTimeout: ULONG) -> FT_STATUS;
  fn FT_GetQueueStatus(ftHandle: FT_HANDLE, dwRxBytes: *mut DWORD) -> FT_STATUS;
  fn FT_SetEventNotification(ftHandle: FT_HANDLE, Mask: DWORD, Param: PVOID) -> FT_STATUS;
  fn FT_GetStatus(ftHandle: FT_HANDLE, dwRxBytes: *mut DWORD, dwTxBytes: *mut DWORD, dwEventDWord: *mut DWORD) -> FT_STATUS;
  fn FT_SetBreakOn(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_SetBreakOff(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_SetWaitMask(ftHandle: FT_HANDLE, Mask: DWORD) -> FT_STATUS;
  fn FT_WaitOnMask(ftHandle: FT_HANDLE, Mask: *mut DWORD) -> FT_STATUS;
  fn FT_GetEventStatus(ftHandle: FT_HANDLE, dwEventDWord: *mut DWORD) -> FT_STATUS;
  fn FT_ReadEE(ftHandle: FT_HANDLE, dwWordOffset: DWORD, lpwValue: LPWORD) -> FT_STATUS;
  fn FT_WriteEE(ftHandle: FT_HANDLE, dwWordOffset: DWORD, wValue: WORD) -> FT_STATUS;
  fn FT_EraseEE(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_EEPROM_Read(ftHandle: FT_HANDLE, eepromData: *mut c_void, eepromDataSize: DWORD, Manufacturer: *mut char, ManufacturerId: *mut char, Description: *mut char, SerialNumber: *mut char) -> FT_STATUS;
  fn FT_EEPROM_Program(ftHandle: FT_HANDLE, eepromData: *mut c_void, eepromDataSize: DWORD, Manufacturer: *mut char, ManufacturerId: *mut char, Description: *mut char, SerialNumber: *mut char) -> FT_STATUS;
  fn FT_SetLatencyTimer(ftHandle: FT_HANDLE, ucLatency: UCHAR) -> FT_STATUS;
  fn FT_GetLatencyTimer(ftHandle: FT_HANDLE, pucLatency: PUCHAR) -> FT_STATUS;
  fn FT_SetBitMode(ftHandle: FT_HANDLE, ucMask: UCHAR, ucEnable: UCHAR) -> FT_STATUS;
  fn FT_GetBitMode(ftHandle: FT_HANDLE, pucMode: PUCHAR) -> FT_STATUS;
  fn FT_SetUSBParameters(ftHandle: FT_HANDLE, ulInTransferSize: ULONG, ulOutTransferSize: ULONG) -> FT_STATUS;
  fn FT_SetDeadmanTimeout(ftHandle: FT_HANDLE, ulDeadmanTimeout: ULONG) -> FT_STATUS;
  fn FT_SetVIDPID(dwVID: DWORD, dwPID: DWORD) -> FT_STATUS;
  fn FT_GetVIDPID( pdwVID: *mut DWORD, pdwPID: *mut DWORD) -> FT_STATUS;
  fn FT_GetDeviceLocId(ftHandle: FT_HANDLE, lpdwLocId: LPDWORD) -> FT_STATUS;
  fn FT_GetDeviceInfo(ftHandle: FT_HANDLE, lpftDevice: *mut FT_DEVICE, lpdwID: LPDWORD, SerialNumber: PCHAR, Description: PCHAR, Dummy: LPVOID) -> FT_STATUS;
  fn FT_StopInTask(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_RestartInTask(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_SetResetPipeRetryCount(ftHandle: FT_HANDLE, dwCount: DWORD) -> FT_STATUS;
  fn FT_ResetPort(ftHandle: FT_HANDLE) -> FT_STATUS;
  fn FT_CyclePort(ftHandle: FT_HANDLE) -> FT_STATUS;
  //
  // Device information
  //
  fn FT_CreateDeviceInfoList(lpdwNumDevs: LPDWORD) -> FT_STATUS;
  fn FT_GetDeviceInfoList(pDest: *mut FT_DEVICE_LIST_INFO_NODE, lpdwNumDevs: LPDWORD) -> FT_STATUS;
  fn FT_GetDeviceInfoDetail(dwIndex: DWORD, lpdwFlags: LPDWORD, lpdwType: LPDWORD, lpdwID: LPDWORD, lpdwLocId: LPDWORD, lpSerialNumber: LPVOID, lpDescription: LPVOID, pftHandle: FT_HANDLE) -> FT_STATUS;
  //
  // Version information
  //
  fn FT_GetDriverVersion(ftHandle: FT_HANDLE, lpdwVersion: LPDWORD) -> FT_STATUS;
  fn FT_GetLibraryVersion(lpdwVersion: LPDWORD) -> FT_STATUS;
  fn FT_Rescan() -> FT_STATUS;
  fn FT_Reload(wVid: WORD, wPid: WORD ) -> FT_STATUS;
  fn FT_GetComPortNumber(ftHandle: FT_HANDLE, lpdwComPortNumber: LPLONG) -> FT_STATUS;
}

pub fn get_library_version() -> Result<u32, constants::FT_STATUS> {
  let status: FT_STATUS;
  let version: LPDWORD = &mut 0;
  unsafe {
    status = FT_GetLibraryVersion(version);
    match status {
      constants::FT_STATUS::FT_OK => Ok(*version),
      _ => Err(status)
    }
  }
}

pub fn get_driver_version() -> Result<u32, constants::FT_STATUS> {
  let status: FT_STATUS;
  let version: LPDWORD = &mut 0;
  let mut handle: FT_DEVICE = 0;
  // TODO
  let handle_ptr: FT_HANDLE = &mut handle as *mut _ as *mut c_void;
  unsafe {
    status = FT_GetDriverVersion(handle_ptr, version);
    match status {
      constants::FT_STATUS::FT_OK => Ok(*version),
      _ => Err(status)
    }
  }
}


pub fn create_device_info_list() -> Result<u32, constants::FT_STATUS> {
  let status: FT_STATUS;
  let num_devs: LPDWORD = &mut 0;
  unsafe {
    status = FT_CreateDeviceInfoList(num_devs);
    match status {
      constants::FT_STATUS::FT_OK => Ok(*num_devs),
      _ => Err(status)
    }
  }
}


#[test]
fn it_creates_device_info_list() {
  let result = create_device_info_list();
  match result {
    Ok(1) => println!("pass!"),
    Ok(n) => panic!("Number of devices does not match expectation: {}", n),
    Err(e) => panic!("Got FT_STATUS Error: {}", e)
  }
}


#[test]
fn it_returns_library_version() {
  let result = get_library_version();
  match result {
    Ok(0x10202) => println!("pass!"),
    Ok(v) => panic!("Library Version {:x} does not match expectation", v),
    Err(e) => panic!("Got FT_STATUS Error: {}", e)
  }
}

#[test]
fn it_returns_driver_version() {
  let result = get_driver_version();
  match result {
    Ok(0x10202) => println!("pass!"),
    Ok(v) => panic!("Driver Version {:x} does not match expectation", v),
    Err(e) => panic!("Got FT_STATUS Error: {}", e)
  }
}
