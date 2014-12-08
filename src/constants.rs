///
/// FTDI D2XX Driver Constants
/// as defined in ftd2xx.h
///
/// Implements "Show" trait for FT_STATUS
/// in order to format Errors nicely
///
extern crate core;
use core::fmt;
use typedefs::*;


#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum FT_STATUS {
  FT_OK,
  FT_INVALID_HANDLE,
  FT_DEVICE_NOT_FOUND,
  FT_DEVICE_NOT_OPENED,
  FT_IO_ERROR,
  FT_INSUFFICIENT_RESOURCES,
  FT_INVALID_PARAMETER,
  FT_INVALID_BAUD_RATE,
  FT_DEVICE_NOT_OPENED_FOR_ERASE,
  FT_DEVICE_NOT_OPENED_FOR_WRITE,
  FT_FAILED_TO_WRITE_DEVICE,
  FT_EEPROM_READ_FAILED,
  FT_EEPROM_WRITE_FAILED,
  FT_EEPROM_ERASE_FAILED,
  FT_EEPROM_NOT_PRESENT,
  FT_EEPROM_NOT_PROGRAMMED,
  FT_INVALID_ARGS,
  FT_NOT_SUPPORTED,
  FT_OTHER_ERROR,
  FT_DEVICE_LIST_NOT_READY,
}

#[allow(dead_code)]
impl core::fmt::Show for FT_STATUS {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      FT_STATUS::FT_OK                          => "FT_OK",
      FT_STATUS::FT_INVALID_HANDLE              => "FT_INVALID_HANDLE",
      FT_STATUS::FT_DEVICE_NOT_FOUND            => "FT_DEVICE_NOT_FOUND",
      FT_STATUS::FT_DEVICE_NOT_OPENED           => "FT_DEVICE_NOT_OPENED",
      FT_STATUS::FT_IO_ERROR                    => "FT_IO_ERROR",
      FT_STATUS::FT_INSUFFICIENT_RESOURCES      => "FT_INSUFFICIENT_RESOURCES",
      FT_STATUS::FT_INVALID_PARAMETER           => "FT_INVALID_PARAMETER",
      FT_STATUS::FT_INVALID_BAUD_RATE           => "FT_INVALID_BAUD_RATE",
      FT_STATUS::FT_DEVICE_NOT_OPENED_FOR_ERASE => "FT_DEVICE_NOT_OPENED_FOR_ERASE",
      FT_STATUS::FT_DEVICE_NOT_OPENED_FOR_WRITE => "FT_DEVICE_NOT_OPENED_FOR_WRITE",
      FT_STATUS::FT_FAILED_TO_WRITE_DEVICE      => "FT_FAILED_TO_WRITE_DEVICE",
      FT_STATUS::FT_EEPROM_READ_FAILED          => "FT_EEPROM_READ_FAILED",
      FT_STATUS::FT_EEPROM_WRITE_FAILED         => "FT_EEPROM_WRITE_FAILED",
      FT_STATUS::FT_EEPROM_ERASE_FAILED         => "FT_EEPROM_ERASE_FAILED",
      FT_STATUS::FT_EEPROM_NOT_PRESENT          => "FT_EEPROM_NOT_PRESENT",
      FT_STATUS::FT_EEPROM_NOT_PROGRAMMED       => "FT_EEPROM_NOT_PROGRAMMED",
      FT_STATUS::FT_INVALID_ARGS                => "FT_INVALID_ARGS",
      FT_STATUS::FT_NOT_SUPPORTED               => "FT_NOT_SUPPORTED",
      FT_STATUS::FT_OTHER_ERROR                 => "FT_OTHER_ERROR",
      FT_STATUS::FT_DEVICE_LIST_NOT_READY       => "FT_DEVICE_LIST_NOT_READY",
    })
  }
}


pub struct FT_DEVICE_LIST_INFO_NODE {
   Flags: ULONG,
   Type: ULONG,
   ID: ULONG,
   LocId: DWORD,
   SerialNumber: [CHAR, ..16],
   Description: [CHAR, ..64],
   ftHandle: FT_HANDLE,
}

// Device information flags
pub enum DEVICE_INFORMATION_FLAGS {
  FT_FLAGS_OPENED = 1,
  FT_FLAGS_HISPEED = 2
}

// #define FT_SUCCESS(status) ((status) == FT_OK)

//
// FT_OpenEx Flags
//
pub static FT_OPEN_BY_SERIAL_NUMBER:  u8 = 1;
pub static FT_OPEN_BY_DESCRIPTION:    u8 = 2;
pub static FT_OPEN_BY_LOCATION:       u8 = 4;

// pub static FT_OPEN_MASK: (FT_OPEN_BY_SERIAL_NUMBER | \;
//                       FT_OPEN_BY_DESCRIPTION | \
//                       FT_OPEN_BY_LOCATION)

//
// FT_ListDevices Flags (used in conjunction with FT_OpenEx Flags
//

pub static FT_LIST_NUMBER_ONLY:   u32 = 0x80000000;
pub static FT_LIST_BY_INDEX:      u32 = 0x40000000;
pub static FT_LIST_ALL:           u32 = 0x20000000;

// pub static FT_LIST_MASK: (FT_LIST_NUMBER_ONLY|FT_LIST_BY_INDEX|FT_LIST_ALL);

//
// Baud Rates
//

pub static FT_BAUD_300:    u32 = 300;
pub static FT_BAUD_600:    u32 = 600;
pub static FT_BAUD_1200:   u32 = 1200;
pub static FT_BAUD_2400:   u32 = 2400;
pub static FT_BAUD_4800:   u32 = 4800;
pub static FT_BAUD_9600:   u32 = 9600;
pub static FT_BAUD_14400:  u32 = 14400;
pub static FT_BAUD_19200:  u32 = 19200;
pub static FT_BAUD_38400:  u32 = 38400;
pub static FT_BAUD_57600:  u32 = 57600;
pub static FT_BAUD_115200: u32 = 115200;
pub static FT_BAUD_230400: u32 = 230400;
pub static FT_BAUD_460800: u32 = 460800;
pub static FT_BAUD_921600: u32 = 921600;

//
// Word Lengths
//

pub static FT_BITS_8:     u8 = 8;
pub static FT_BITS_7:     u8 = 7;

//
// Stop Bits
//

pub static FT_STOP_BITS_1:    u8 = 0;
pub static FT_STOP_BITS_2:    u8 = 2;

//
// Parity
//

pub static FT_PARITY_NONE:    u8 = 0;
pub static FT_PARITY_ODD:     u8 = 1;
pub static FT_PARITY_EVEN:    u8 = 2;
pub static FT_PARITY_MARK:    u8 = 3;
pub static FT_PARITY_SPACE:   u8 = 4;

//
// Flow Control
//

pub static FT_FLOW_NONE:      u16 = 0x0000;
pub static FT_FLOW_RTS_CTS:   u16 = 0x0100;
pub static FT_FLOW_DTR_DSR:   u16 = 0x0200;
pub static FT_FLOW_XON_XOFF:  u16 = 0x0400;

//
// Purge rx and tx buffers
//
pub static FT_PURGE_RX:     u8 = 1;
pub static FT_PURGE_TX:     u8 = 2;

//
// Events
//

// typedef void (*PFT_EVENT_HANDLER)(DWORD,DWORD);

pub static FT_EVENT_RXCHAR:       u8 = 1;
pub static FT_EVENT_MODEM_STATUS: u8 = 2;
pub static FT_EVENT_LINE_STATUS:  u8 = 4;

//
// Timeouts
//

pub static FT_DEFAULT_RX_TIMEOUT: u16 = 300;
pub static FT_DEFAULT_TX_TIMEOUT: u16 = 300;

//
// Device types
//

// typedef ULONG FT_DEVICE;

pub enum DEVICE_TYPES {
  FT_DEVICE_BM,
  FT_DEVICE_AM,
  FT_DEVICE_100AX,
  FT_DEVICE_UNKNOWN,
  FT_DEVICE_2232C,
  FT_DEVICE_232R,
  FT_DEVICE_2232H,
  FT_DEVICE_4232H,
  FT_DEVICE_232H,
  FT_DEVICE_X_SERIES
}

//
// Bit Modes
//

pub static FT_BITMODE_RESET:          u8 = 0x00;
pub static FT_BITMODE_ASYNC_BITBANG:  u8 = 0x01;
pub static FT_BITMODE_MPSSE:          u8 = 0x02;
pub static FT_BITMODE_SYNC_BITBANG:   u8 = 0x04;
pub static FT_BITMODE_MCU_HOST:       u8 = 0x08;
pub static FT_BITMODE_FAST_SERIAL:    u8 = 0x10;
pub static FT_BITMODE_CBUS_BITBANG:   u8 = 0x20;
pub static FT_BITMODE_SYNC_FIFO:      u8 = 0x40;

//
// FT232R CBUS Options EEPROM values
//

pub static FT_232R_CBUS_TXDEN:          u8 = 0x00;  //  Tx Data Enable
pub static FT_232R_CBUS_PWRON:          u8 = 0x01;  //  Power On
pub static FT_232R_CBUS_RXLED:          u8 = 0x02;  //  Rx LED
pub static FT_232R_CBUS_TXLED:          u8 = 0x03;  //  Tx LED
pub static FT_232R_CBUS_TXRXLED:        u8 = 0x04;  //  Tx and Rx LED
pub static FT_232R_CBUS_SLEEP:          u8 = 0x05;  //  Sleep
pub static FT_232R_CBUS_CLK48:          u8 = 0x06;  //  48MHz clock
pub static FT_232R_CBUS_CLK24:          u8 = 0x07;  //  24MHz clock
pub static FT_232R_CBUS_CLK12:          u8 = 0x08;  //  12MHz clock
pub static FT_232R_CBUS_CLK6:           u8 = 0x09;  //  6MHz clock
pub static FT_232R_CBUS_IOMODE:         u8 = 0x0A;  //  IO Mode for CBUS bit-bang
pub static FT_232R_CBUS_BITBANG_WR:     u8 = 0x0B;  //  Bit-bang write strobe
pub static FT_232R_CBUS_BITBANG_RD:     u8 = 0x0C;  //  Bit-bang read strobe

//
// FT232H CBUS Options EEPROM values
//

pub static FT_232H_CBUS_TRISTATE:      u8 = 0x00;  //  Tristate
pub static FT_232H_CBUS_TXLED:         u8 = 0x01;  //  Tx LED
pub static FT_232H_CBUS_RXLED:         u8 = 0x02;  //  Rx LED
pub static FT_232H_CBUS_TXRXLED:       u8 = 0x03;  //  Tx and Rx LED
pub static FT_232H_CBUS_PWREN:         u8 = 0x04;  //  Power Enable
pub static FT_232H_CBUS_SLEEP:         u8 = 0x05;  //  Sleep
pub static FT_232H_CBUS_DRIVE_0:       u8 = 0x06;  //  Drive pin to logic 0
pub static FT_232H_CBUS_DRIVE_1:       u8 = 0x07;  //  Drive pin to logic 1
pub static FT_232H_CBUS_IOMODE:        u8 = 0x08;  //  IO Mode for CBUS bit-bang
pub static FT_232H_CBUS_TXDEN:         u8 = 0x09;  //  Tx Data Enable
pub static FT_232H_CBUS_CLK30:         u8 = 0x0A;  //  30MHz clock
pub static FT_232H_CBUS_CLK15:         u8 = 0x0B;  //  15MHz clock
pub static FT_232H_CBUS_CLK7_5:        u8 = 0x0C;  //  7.5MHz clock

//
// FT X Series CBUS Options EEPROM values
//

pub static FT_X_SERIES_CBUS_TRISTATE:      u8 = 0x00;  //  Tristate;
pub static FT_X_SERIES_CBUS_RXLED:         u8 = 0x01;  //  Tx LED;
pub static FT_X_SERIES_CBUS_TXLED:         u8 = 0x02;  //  Rx LED;
pub static FT_X_SERIES_CBUS_TXRXLED:       u8 = 0x03;  //  Tx and Rx LED;
pub static FT_X_SERIES_CBUS_PWREN:         u8 = 0x04;  //  Power Enable;
pub static FT_X_SERIES_CBUS_SLEEP:         u8 = 0x05;  //  Sleep;
pub static FT_X_SERIES_CBUS_DRIVE_0:       u8 = 0x06;  //  Drive pin to logic 0
pub static FT_X_SERIES_CBUS_DRIVE_1:       u8 = 0x07;  //  Drive pin to logic 1
pub static FT_X_SERIES_CBUS_IOMODE:        u8 = 0x08;  //  IO Mode for CBUS bit-bang;
pub static FT_X_SERIES_CBUS_TXDEN:         u8 = 0x09;  //  Tx Data Enable;
pub static FT_X_SERIES_CBUS_CLK24:         u8 = 0x0A;  //  24MHz clock
pub static FT_X_SERIES_CBUS_CLK12:         u8 = 0x0B;  //  12MHz clock
pub static FT_X_SERIES_CBUS_CLK6:          u8 = 0x0C;  //  6MHz clock
pub static FT_X_SERIES_CBUS_BCD_CHARGER:   u8 = 0x0D;  //  Battery charger detected;
pub static FT_X_SERIES_CBUS_BCD_CHARGER_N: u8 = 0x0E;  //  Battery charger detected inverted;
pub static FT_X_SERIES_CBUS_I2C_TXE:       u8 = 0x0F;  //  I2C Tx empty
pub static FT_X_SERIES_CBUS_I2C_RXF:       u8 = 0x10;  //  I2C Rx full
pub static FT_X_SERIES_CBUS_VBUS_SENSE:    u8 = 0x11;  //  Detect VBUS;
pub static FT_X_SERIES_CBUS_BITBANG_WR:    u8 = 0x12;  //  Bit-bang write strobe;
pub static FT_X_SERIES_CBUS_BITBANG_RD:    u8 = 0x13;  //  Bit-bang read strobe;
pub static FT_X_SERIES_CBUS_TIMESTAMP:     u8 = 0x14;  //  Toggle output when a USB SOF token is received;
pub static FT_X_SERIES_CBUS_KEEP_AWAKE:    u8 = 0x15;  //  ;


// Driver types
pub static FT_DRIVER_TYPE_D2XX:   u8 = 0;
pub static FT_DRIVER_TYPE_VCP:    u8 = 1;

// TODO: EEPROM Data structures

// From Wintypes.h

pub static MAX_NUM_DEVICES: u8 = 50;

//
// Modem Status Flags
//
// #define MS_CTS_ON           ((DWORD)0x0010)
// #define MS_DSR_ON           ((DWORD)0x0020)
// #define MS_RING_ON          ((DWORD)0x0040)
// #define MS_RLSD_ON          ((DWORD)0x0080)

//
// Error Flags
//
// #define CE_RXOVER           0x0001  // Receive Queue overflow
// #define CE_OVERRUN          0x0002  // Receive Overrun Error
// #define CE_RXPARITY         0x0004  // Receive Parity Error
// #define CE_FRAME            0x0008  // Receive Framing error
// #define CE_BREAK            0x0010  // Break Detected
// #define CE_TXFULL           0x0100  // TX Queue is full
// #define CE_PTO              0x0200  // LPTx Timeout
// #define CE_IOE              0x0400  // LPTx I/O Error
// #define CE_DNS              0x0800  // LPTx Device not selected
// #define CE_OOP              0x1000  // LPTx Out-Of-Paper
// #define CE_MODE             0x8000  // Requested mode unsupported

//
// Events
//
// #define EV_RXCHAR           0x0001  // Any Character received
// #define EV_RXFLAG           0x0002  // Received certain character
// #define EV_TXEMPTY          0x0004  // Transmit Queue Empty
// #define EV_CTS              0x0008  // CTS changed state
// #define EV_DSR              0x0010  // DSR changed state
// #define EV_RLSD             0x0020  // RLSD changed state
// #define EV_BREAK            0x0040  // BREAK received
// #define EV_ERR              0x0080  // Line status error occurred
// #define EV_RING             0x0100  // Ring signal detected
// #define EV_PERR             0x0200  // Printer error occured
// #define EV_RX80FULL         0x0400  // Receive buffer is 80 percent full
// #define EV_EVENT1           0x0800  // Provider specific event 1
// #define EV_EVENT2           0x1000  // Provider specific event 2

//
// Escape Functions
//
// #define SETXOFF             1       // Simulate XOFF received
// #define SETXON              2       // Simulate XON received
// #define SETRTS              3       // Set RTS high
// #define CLRRTS              4       // Set RTS low
// #define SETDTR              5       // Set DTR high
// #define CLRDTR              6       // Set DTR low
// #define RESETDEV            7       // Reset device if possible
// #define SETBREAK            8       // Set the device break line.
// #define CLRBREAK            9       // Clear the device break line.

//
// PURGE function flags.
//
// #define PURGE_TXABORT       0x0001  // Kill the pending/current writes to the comm port.
// #define PURGE_RXABORT       0x0002  // Kill the pending/current reads to the comm port.
// #define PURGE_TXCLEAR       0x0004  // Kill the transmit queue if there.
// #define PURGE_RXCLEAR       0x0008  // Kill the typeahead buffer if there.

// #define INVALID_HANDLE_VALUE 0xFFFFFFFF
