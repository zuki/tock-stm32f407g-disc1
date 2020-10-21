//! Driver for the MEMS Lis3dshSpi motion sensor, 3 axis digital output
//! accelerometer and temperature sensor.
//!
//! May be used with NineDof and Temperature
//!
//! SPI Interface
//!
//! <https://www.st.com/resource/en/datasheet/lis3dsh.pdf>
//!
//!
//! Syscall Interface
//! -----------------
//!
//! ### Command
//!
//! All commands are asynchronous, they return a one shot callback when done
//! Only one command can be issued at a time.
//!
//! #### command num
//! - `0`: Returns SUCCESS
//!   - `data`: Unused.
//!   - Return: 0
//! - `1`: Is Present
//!   - `data`: unused
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//! - `2`: Set Power mode
//!   - `data`: 0 to 9
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//! - `3`: Set Full scale selection and anti-aliasing filter bandwidth
//!   - `data1`: 0, 1, 2, 3 or 4
//!   - `data2`: 0, 1, 2 or 3
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//! - `4`: Read XYZ
//!   - `data`: unused
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//! - `5`: Read Temperature
//!   - `data`: unused
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//!
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//! - `6`: Read Temperature
//!   - `data`: 0 - 5 (OUTX_L + data)
//!   - Return: `SUCCESS` if no other command is in progress, `EBUSY` otherwise.
//! 
//!
//! ### Subscribe
//!
//! All commands call this callback when done, usually subscribes
//! should be one time functions
//!
//! #### subscribe num
//! - `0`: Done callback
//!   - 'data1`: depends on command
//!     - `1` - 1 for is present, 0 for not present
//!     - `4` - X rotation
//!     - `5` - temperature in deg C
//!     - `6` - accelleration value
//!   - 'data2`: depends on command
//!     - `4` - Y rotation
//!   - 'data3`: depends on command
//!     - `4` - Z rotation
//!
//! Usage
//! -----
//!
//! ```rust
//! let mux_spi = components::spi::SpiMuxComponent::new(&stm32f407vg::spi::SPI1)
//!     .finalize(components::spi_mux_component_helper!(stm32f407vg::spi::Spi));
//!
//! let lis3dsh = my_components::lis3dsh::Lis3dshSpiComponent::new()
//!     .finalize(my_components::lis3dsh_spi_component_helper!(stm32f407vg::spi::Spi, stm32f407vg::gpio::PinId::PE03, mux_spi));
//!
//! lis3dsh.configure(
//!     lis3dsh::Lis3dshDataRate::DataRate100Hz,
//!     lis3dsh::Lis3dshScale::Scale2G,
//!     lis3dsh::Lis3dshFilter::Filter800Hz,
//! );
//! ```
//!
//! NineDof Example
//!
//! ```rust
//! let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
//! let grant_ninedof = board_kernel.create_grant(&grant_cap);
//!
//! let ninedof = components::ninedof::NineDofComponent::new(board_kernel)
//!     .finalize(components::ninedof_component_helper!(lis3dsh));
//!
//! ```
//!
//! Temperature Example
//!
//! ```rust
//! let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
//! let grant_temp = board_kernel.create_grant(&grant_cap);
//!
//! l3gd20.set_power_mode();
//! let temp = static_init!(
//!     capsules::temperature::TemperatureSensor<'static>,
//!     capsules::temperature::TemperatureSensor::new(l3gd20, grant_temperature));
//! kernel::hil::sensors::TemperatureDriver::set_client(l3gd20, temp);
//!
//! ```
//!
//! Author: Keiji Suzuki
//!

#![allow(non_camel_case_types)]

use core::cell::Cell;
use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::common::registers::register_bitfields;
use kernel::hil::sensors;
use kernel::hil::spi;
use kernel::ReturnCode;
use kernel::{AppId, Callback, Driver};

register_bitfields![u8,
    CTRL_REG4 [
        // Output data rate
        ODR OFFSET(4) NUMBITS(4) [],
        // Block data update
        BDU OFFSET(3) NUMBITS(1) [],
        // Z-axis enable
        ZEN OFFSET(2) NUMBITS(1) [],
        // Y-axis enable
        YEN OFFSET(1) NUMBITS(1) [],
        // X-axis enable
        XEN OFFSET(0) NUMBITS(1) []
    ],
    CTRL_REG5 [
        // Anti aliasing filter bandwidth
        BW OFFSET(6) NUMBITS(2) [],
        // Full scale selection
        FSCALE OFFSET(3) NUMBITS(3) [],
        // Self test enable
        ST OFFSET(1) NUMBITS(2) [],
        // SPI serial interface
        SIM OFFSET(0) NUMBITS(1) []
    ]
];

//use capsules::driver;
pub const DRIVER_NUM: usize = 0x70223; // 暫定的: driver::NUM::Lis3dsh as usize;

/* Identification number */
const LIS3DSH_WHO_AM_I: u8 = 0x3F;

/* Registers addresses */
const LIS3DSH_REG_OUT_T: u8 = 0x0C;         // r:   -    温度出力
/*
const LIS3DSH_REG_INFO1: u8 = 0x0D;         // r:   0x31 情報レジスタ1
const LIS3DSH_REG_INFO2: u8 = 0x0E;         // r:   0x00 情報レジスタ2
*/
const LIS3DSH_REG_WHO_AM_I: u8 = 0x0F;      // r:   0x3F Who I am ID
/*
const LIS3DSH_REG_OFF_X: u8 = 0x10;         // r/w: 0x00 X軸オフセット修正
const LIS3DSH_REG_OFF_Y: u8 = 0x11;         // r/w: 0x00 Y軸オフセット修正
const LIS3DSH_REG_OFF_Z: u8 = 0x12;         // r/w: 0x00 Z軸オフセット修正
const LIS3DSH_REG_CS_X: u8 = 0x13;          // r/w: 0x00 定数シフトX
const LIS3DSH_REG_CS_Y: u8 = 0x14;          // r/w: 0x00 定数シフトY
const LIS3DSH_REG_CS_Z: u8 = 0x15;          // r/w: 0x00 定数シフトZ
const LIS3DSH_REG_LC_L: u8 = 0x16;          // r/w: 0x01 長いカウンタ（低位）
const LIS3DSH_REG_LC_H: u8 = 0x17;          // r/w: 0x00 長いカウンタ（高位)
const LIS3DSH_REG_STAT: u8 = 0x18;          // r:   -    割り込み同期
const LIS3DSH_REG_PEAK1: u8 = 0x19;         // r:   -    ピーク値
const LIS3DSH_REG_PEAK2: u8 = 0x1A;         // r:   -    ピーク値
const LIS3DSH_REG_VFC_1: u8 = 0x1B;         // r/w: -    ベクタフィルタ係数1
const LIS3DSH_REG_VFC_2: u8 = 0x1C;         // r/w: -    ベクタフィルタ係数2
const LIS3DSH_REG_VFC_3: u8 = 0x1D;         // r/w: -    ベクタフィルタ係数3
const LIS3DSH_REG_VFC_4: u8 = 0x1E;         // r/w: -    ベクタフィルタ係数4
const LIS3DSH_REG_THRS3: u8 = 0x1F;         // r/w: -    閾値3
*/
const LIS3DSH_REG_CTRL_REG4: u8 = 0x20;     // r/w: 0x07 制御レジスタ
/*
const LIS3DSH_REG_CTRL_REG1: u8 = 0x21;     // r/w: 0x00 SM1制御レジスタ
const LIS3DSH_REG_CTRL_REG2: u8 = 0x22;     // r/w: 0x00 SM2制御レジスタ
const LIS3DSH_REG_CTRL_REG3: u8 = 0x23;     // r/w: 0x00 制御レジスタ
*/
const LIS3DSH_REG_CTRL_REG5: u8 = 0x24;     // r/w: 0x00 制御レジスタ
/*
const LIS3DSH_REG_CTRL_REG6: u8 = 0x25;     // r/w: 0x10 制御レジスタ
const LIS3DSH_REG_STATUS: u8 = 0x27;        // r:   -    状態データレジスタ
*/
const LIS3DSH_REG_OUT_X_L: u8 = 0x28;       // r:   0x00 出力レジスタ（X低位）
/*
const LIS3DSH_REG_OUT_X_H: u8 = 0x29;       // r:        出力レジスタ（X高位）
const LIS3DSH_REG_OUT_Y_L: u8 = 0x2A;       // r:        出力レジスタ（Y低位）
const LIS3DSH_REG_OUT_Y_H: u8 = 0x2B;       // r:        出力レジスタ（Y高位）
const LIS3DSH_REG_OUT_Z_L: u8 = 0x2C;       // r:        出力レジスタ（Z低位）
const LIS3DSH_REG_OUT_Z_H: u8 = 0x2D;       // r:        出力レジスタ（Z高位）
const LIS3DSH_REG_FIFO_CTRL: u8 = 0x2E;     // r/w: 0x00 FIFOレジスタ
const LIS3DSH_REG_FIFO_SRC: u8 = 0x2F;      // r:   -    FIFOレジスタ
const LIS3DSH_REG_ST1_1: u8 = 0x40;         // w:   -    SM1コードレジスタ-1
const LIS3DSH_REG_ST1_2: u8 = 0x41;         // w:   -    SM1コードレジスタ-2
const LIS3DSH_REG_ST1_3: u8 = 0x42;         // w:   -    SM1コードレジスタ-3
const LIS3DSH_REG_ST1_4: u8 = 0x43;         // w:   -    SM1コードレジスタ-4
const LIS3DSH_REG_ST1_5: u8 = 0x44;         // w:   -    SM1コードレジスタ-5
const LIS3DSH_REG_ST1_6: u8 = 0x45;         // w:   -    SM1コードレジスタ-6
const LIS3DSH_REG_ST1_7: u8 = 0x46;         // w:   -    SM1コードレジスタ-7
const LIS3DSH_REG_ST1_8: u8 = 0x47;         // w:   -    SM1コードレジスタ-8
const LIS3DSH_REG_ST1_9: u8 = 0x48;         // w:   -    SM1コードレジスタ-9
const LIS3DSH_REG_ST1_10: u8 = 0x49;        // w:   -    SM1コードレジスタ-10
const LIS3DSH_REG_ST1_11: u8 = 0x4A;        // w:   -    SM1コードレジスタ-11
const LIS3DSH_REG_ST1_12: u8 = 0x4B;        // w:   -    SM1コードレジスタ-12
const LIS3DSH_REG_ST1_13: u8 = 0x4C;        // w:   -    SM1コードレジスタ-13
const LIS3DSH_REG_ST1_14: u8 = 0x4D;        // w:   -    SM1コードレジスタ-14
const LIS3DSH_REG_ST1_15: u8 = 0x4E;        // w:   -    SM1コードレジスタ-15
const LIS3DSH_REG_ST1_16: u8 = 0x4F;        // w:   -    SM1コードレジスタ-16
const LIS3DSH_REG_TIM4_1: u8 = 0x50;        // w:   -    SM1 8-bit汎用タイマー
const LIS3DSH_REG_TIM3_1: u8 = 0x51;        // w:   -    SM1 8-bit汎用タイマー
const LIS3DSH_REG_TIM2_1L: u8 = 0x52;       // w:   -    SM1 16-bit汎用タイマー（低位）
const LIS3DSH_REG_TIM2_1H: u8 = 0x53;       // w:   -    SM1 16-bit汎用タイマー（高位）
const LIS3DSH_REG_TIM1_1L: u8 = 0x54;       // w:   -    SM1 16-bit汎用タイマー（低位）
const LIS3DSH_REG_TIM1_1H: u8 = 0x55;       // w:   -    SM1 16-bit汎用タイマー（高位）
const LIS3DSH_REG_THRS2_1: u8 = 0x56;       // w:   -    SM1閾値1
const LIS3DSH_REG_THRS1_1: u8 = 0x57;       // w:   -    SM1閾値2
const LIS3DSH_REG_MASK1_B: u8 = 0x59;       // w:   -    SM1軸符号マスク
const LIS3DSH_REG_MASK1_A: u8 = 0x5A;       // w:   -    SM1軸符号マスク
const LIS3DSH_REG_SETT1: u8 = 0x5B;         // w:   -    SM1検知設定
const LIS3DSH_REG_PR1: u8 = 0x5C;           // r:   -    SM1プログラムリセットポインタ
const LIS3DSH_REG_TC1: u16 = 0x5D;          // r:   -    SM1タイマーカウンタ
const LIS3DSH_REG_OUTS1: u8 = 0x5F;         // r:   -    SM1メイン設定フラグ
const LIS3DSH_REG_ST2_1: u8 = 0x60;         // w:   -    SM２コードレジスタ-1
const LIS3DSH_REG_ST2_2: u8 = 0x61;         // w:   -    SM２コードレジスタ-2
const LIS3DSH_REG_ST2_3: u8 = 0x62;         // w:   -    SM２コードレジスタ-3
const LIS3DSH_REG_ST2_4: u8 = 0x63;         // w:   -    SM２コードレジスタ-4
const LIS3DSH_REG_ST2_5: u8 = 0x64;         // w:   -    SM２コードレジスタ-5
const LIS3DSH_REG_ST2_6: u8 = 0x65;         // w:   -    SM２コードレジスタ-6
const LIS3DSH_REG_ST2_7: u8 = 0x66;         // w:   -    SM２コードレジスタ-7
const LIS3DSH_REG_ST2_8: u8 = 0x67;         // w:   -    SM２コードレジスタ-8
const LIS3DSH_REG_ST2_9: u8 = 0x68;         // w:   -    SM２コードレジスタ-9
const LIS3DSH_REG_ST2_10: u8 = 0x69;        // w:   -    SM２コードレジスタ-10
const LIS3DSH_REG_ST2_11: u8 = 0x6A;        // w:   -    SM２コードレジスタ-11
const LIS3DSH_REG_ST2_12: u8 = 0x6B;        // w:   -    SM２コードレジスタ-12
const LIS3DSH_REG_ST2_13: u8 = 0x6C;        // w:   -    SM２コードレジスタ-13
const LIS3DSH_REG_ST2_14: u8 = 0x6D;        // w:   -    SM２コードレジスタ-14
const LIS3DSH_REG_ST2_15: u8 = 0x6E;        // w:   -    SM２コードレジスタ-15
const LIS3DSH_REG_ST2_16: u8 = 0x6F;        // w:   -    SM２コードレジスタ-16
const LIS3DSH_REG_TIM4_2: u8 = 0x70;        // w:   -    SM2 8-bit汎用タイマー
const LIS3DSH_REG_TIM3_2: u8 = 0x71;        // w:   -    SM3 8-bit汎用タイマー
const LIS3DSH_REG_TIM2_2L: u8 = 0x72;       // w:   -    SM2 16-bit汎用タイマー（低位）
const LIS3DSH_REG_TIM2_2H: u8 = 0x72;       // w:   -    SM2 16-bit汎用タイマー（高位）
const LIS3DSH_REG_TIM1_2L: u8 = 0x74;       // w:   -    SM2 16-bit汎用タイマー（低位）
const LIS3DSH_REG_TIM1_2H: u8 = 0x74;       // w:   -    SM2 16-bit汎用タイマー（高位）
const LIS3DSH_REG_THRS2_2: u8 = 0x76;       // w:   -    SM2閾値1
const LIS3DSH_REG_THRS1_2: u8 = 0x77;       // w:   -    SM2閾値2
const LIS3DSH_REG_DES2: u8 = 0x78;          // w:   -    SM2小数ファクター
const LIS3DSH_REG_MASK2_B: u8 = 0x79;       // w:   -    SM２軸符号マスク
const LIS3DSH_REG_MASK2_A: u8 = 0x7A;       // w:   -    SM２軸符号マスク
const LIS3DSH_REG_SETT2: u8 = 0x7B;         // w:   -    SM２検知設定
const LIS3DSH_REG_PR2: u8 = 0x7C;           // r:   -    SM2プログラムリセットポインタ
const LIS3DSH_REG_TC2: u16 = 0x7D;          // r:   -    SM2タイマーカウンタ
const LIS3DSH_REG_OUTS2: u8 = 0x7F;         // r:   -    SM2メイン設定フラグ
*/

// buffer size
pub const LIS3DSH_TX_SIZE: usize = 10;
pub const LIS3DSH_RX_SIZE: usize = 10;
// buffers for read and write
pub static mut TXBUFFER: [u8; LIS3DSH_TX_SIZE] = [0; LIS3DSH_TX_SIZE];
pub static mut RXBUFFER: [u8; LIS3DSH_RX_SIZE] = [0; LIS3DSH_RX_SIZE];

// Manual page Table 55, page 39
enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Lis3dshDataRate {
        Off = 0,
        DataRate3_125Hz = 1,
        DataRate6_26Hz = 2,
        DataRate12_5Hz = 3,
        DataRate25Hz = 4,
        DataRate50Hz = 5,
        DataRate100Hz = 6,
        DataRate400Hz = 7,
        DataRate800Hz = 8,
        DataRate1600Hz = 9,
    }
}

// Manual page 41
enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Lis3dshFilter {
        Filter800Hz = 0,
        Filter400Hz = 1,
        Filter200Hz = 2,
        Filter50Hz = 3,
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Lis3dshScale {
        Scale2G = 0,
        Scale4G = 1,
        Scale6G = 2,
        Scale8G = 3,
        Scale16G = 4
    }
}

// Manual page 41
const SCALE_FACTOR: [u32; 5] = [60, 120, 180, 240, 730]; // ug/digit

#[derive(Copy, Clone, PartialEq)]
enum Lis3dshStatus {
    Idle,
    IsPresent,
    SetPowerMode,
    SetScaleAndFilter,
    ReadXYZ,
    ReadTemperature,
    ReadValue,
}

pub struct Lis3dshSpi<'a> {
    spi: &'a dyn spi::SpiMasterDevice,
    txbuffer: TakeCell<'static, [u8]>,
    rxbuffer: TakeCell<'static, [u8]>,
    status: Cell<Lis3dshStatus>,
    data_rate: Cell<Lis3dshDataRate>,
    scale: Cell<Lis3dshScale>,
    filter: Cell<Lis3dshFilter>,
    callback: OptionalCell<Callback>,
    nine_dof_client: OptionalCell<&'a dyn sensors::NineDofClient>,
    temperature_client: OptionalCell<&'a dyn sensors::TemperatureClient>,
}

impl<'a> Lis3dshSpi<'a> {
    pub fn new(
        spi: &'a dyn spi::SpiMasterDevice,
        txbuffer: &'static mut [u8; LIS3DSH_TX_SIZE],
        rxbuffer: &'static mut [u8; LIS3DSH_RX_SIZE],
    ) -> Lis3dshSpi<'a> {
        // setup and return struct
        Lis3dshSpi {
            spi: spi,
            txbuffer: TakeCell::new(txbuffer),
            rxbuffer: TakeCell::new(rxbuffer),
            status: Cell::new(Lis3dshStatus::Idle),
            data_rate: Cell::new(Lis3dshDataRate::DataRate100Hz),
            scale: Cell::new(Lis3dshScale::Scale2G),
            filter: Cell::new(Lis3dshFilter::Filter800Hz),
            callback: OptionalCell::empty(),
            nine_dof_client: OptionalCell::empty(),
            temperature_client: OptionalCell::empty(),
        }
    }

    pub fn configure(
        &self,
        data_rate: Lis3dshDataRate,
        scale: Lis3dshScale,
        filter: Lis3dshFilter,
    ) {
        if self.status.get() == Lis3dshStatus::Idle {
            self.data_rate.set(data_rate);
            self.scale.set(scale);
            self.filter.set(filter);
            self.spi.configure(
                spi::ClockPolarity::IdleHigh,
                spi::ClockPhase::SampleTrailing,
                1_000_000,
            );
            
            self.set_power_mode(data_rate);
        }
    }

    pub fn is_present(&self) {
        self.status.set(Lis3dshStatus::IsPresent);
        self.txbuffer.take().map(|buf| {
            buf[0] = LIS3DSH_REG_WHO_AM_I | 0x80;   // 読み込み
            buf[1] = 0x00;
            self.spi.read_write_bytes(buf, self.rxbuffer.take(), 2);
        });
    }

    pub fn set_power_mode(&self, data_rate: Lis3dshDataRate) {
        self.status.set(Lis3dshStatus::SetPowerMode);
        self.data_rate.set(data_rate);
        self.txbuffer.take().map(|buf| {
            buf[0] = LIS3DSH_REG_CTRL_REG4;
            buf[1] = (CTRL_REG4::ODR.val(data_rate as u8)
                + CTRL_REG4::BDU::CLEAR
                + CTRL_REG4::ZEN::SET
                + CTRL_REG4::YEN::SET
                + CTRL_REG4::XEN::SET)
                .value;
            self.spi.read_write_bytes(buf, None, 2);
        });
    }

    fn set_scale_and_filter(&self, scale: Lis3dshScale, filter: Lis3dshFilter) {
        self.status.set(Lis3dshStatus::SetScaleAndFilter);
        self.scale.set(scale);
        self.filter.set(filter);
        self.txbuffer.take().map(|buf| {
            buf[0] = LIS3DSH_REG_CTRL_REG5;
            buf[1] = (CTRL_REG5::BW.val(filter as u8)
                + CTRL_REG5::FSCALE.val(scale as u8))
                .value;
            self.spi.read_write_bytes(buf, None, 2);
        });
    }

    fn read_xyz(&self) {
        self.status.set(Lis3dshStatus::ReadXYZ);
        self.txbuffer.take().map(|buf| {            // 連続Read時にアドレスを自動加算
            buf[0] = LIS3DSH_REG_OUT_X_L | 0x80;    //  CTRL_REG6[4] = 1 (Default)
            buf[1] = 0x00;
            buf[2] = 0x00;
            buf[3] = 0x00;
            buf[4] = 0x00;
            buf[5] = 0x00;
            buf[6] = 0x00;
            self.spi.read_write_bytes(buf, self.rxbuffer.take(), 7);
        });
    }

    fn read_value(&self, offset: u8) {
        self.status.set(Lis3dshStatus::ReadValue);
        self.txbuffer.take().map(|buf| {
            buf[0] = (LIS3DSH_REG_OUT_X_L + offset) | 0x80;
            buf[1] = 0x00;
            self.spi.read_write_bytes(buf, self.rxbuffer.take(), 2);
        });
    }

    fn read_temperature(&self) {
        self.status.set(Lis3dshStatus::ReadTemperature);
        self.txbuffer.take().map(|buf| {
            buf[0] = LIS3DSH_REG_OUT_T | 0x80;
            buf[1] = 0x00;
            self.spi.read_write_bytes(buf, self.rxbuffer.take(), 2);
        });
    }
}

impl Driver for Lis3dshSpi<'_> {
    fn command(&self, command_num: usize, data1: usize, data2: usize, _: AppId) -> ReturnCode {
        match command_num {
            0 => ReturnCode::SUCCESS,
            // センサが正しく接続されているかチェックする
            1 => {
                if self.status.get() == Lis3dshStatus::Idle {
                    self.is_present();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // 出力データレート設定
            2 => {
                if self.status.get() == Lis3dshStatus::Idle {
                    if let Some(data_rate) = Lis3dshDataRate::from_usize(data1) {
                        self.set_power_mode(data_rate);
                        ReturnCode::SUCCESS
                    } else {
                        ReturnCode::EINVAL
                    }
                } else {
                    ReturnCode::EBUSY
                }
            }
            // スケール, フィルター設定
            3 => {
                if self.status.get() == Lis3dshStatus::Idle {
                    let scale = Lis3dshScale::from_usize(data1);
                    let filter = Lis3dshFilter::from_usize(data2);
                    if scale.is_some() && filter.is_some() {
                        self.set_scale_and_filter(scale.unwrap(), filter.unwrap());
                        ReturnCode::SUCCESS
                    } else {
                        ReturnCode::EINVAL
                    }                  
                } else {
                    ReturnCode::EBUSY
                }
            }
            // XYZ読み取り
            4 => {
                if self.status.get() == Lis3dshStatus::Idle {
                    self.read_xyz();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // 温度読み取り
            5 => {
                if self.status.get() == Lis3dshStatus::Idle {
                    self.read_temperature();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // 任意の軸の1バイトを読み取り
            6 => {
                if self.status.get() == Lis3dshStatus::Idle {
                    self.read_value(data1 as u8);
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }           
            // 未定義
            _ => ReturnCode::ENOSUPPORT,
        }
    }

    fn subscribe(
        &self,
        subscribe_num: usize,
        callback: Option<Callback>,
        _app_id: AppId,
    ) -> ReturnCode {
        match subscribe_num {
            // ワンショットコールバックを設定
            0 => {
                self.callback.insert(callback);
                ReturnCode::SUCCESS
            },
            // 未定義
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}

impl spi::SpiMasterClient for Lis3dshSpi<'_> {
    fn read_write_done(
        &self,
        write_buffer: &'static mut [u8],
        read_buffer: Option<&'static mut [u8]>,
        len: usize,
    ) {
        self.status.set(match self.status.get() {
            Lis3dshStatus::IsPresent => {
                let id: usize;
                let present = if let Some(ref buf) = read_buffer {
                    if buf[1] == LIS3DSH_WHO_AM_I {
                        id = buf[1] as usize;
                        true
                    } else {
                        id = (((buf[0] as u16) << 8) | (buf[1] as u16)) as usize; 
                        false
                    }
                } else {
                    id = 0;
                    false
                };
                self.callback.map(|callback| {
                    callback.schedule(if present { 1 } else { 0 }, id, 0);
                });
                Lis3dshStatus::Idle
            }

            Lis3dshStatus::SetPowerMode => {
                self.callback.map(|callback| {
                    callback.schedule(0, 0, 0);
                });
                Lis3dshStatus::Idle              
            }

            Lis3dshStatus::SetScaleAndFilter => {
                self.callback.map(|callback| {
                    callback.schedule(0, 0, 0);
                });
                Lis3dshStatus::Idle    
            }

            Lis3dshStatus::ReadXYZ => {
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut z: usize = 0;
                let values = if let Some(ref buf) = read_buffer {
                    if len >= 7 {
                        self.nine_dof_client.map(|client| {
                            // 整数のみを使って計算（単位はmg）
                            let scale_factor = self.scale.get() as usize;
                            let x: usize = ((buf[1] as i16 | ((buf[2] as i16) << 8)) as i32
                                * (SCALE_FACTOR[scale_factor] as i32)
                                / 1000) as usize;   // unit = mg
                            let y: usize = ((buf[3] as i16 | ((buf[4] as i16) << 8)) as i32
                                * (SCALE_FACTOR[scale_factor] as i32)
                                / 1000) as usize;
                            let z: usize = ((buf[5] as i16 | ((buf[6] as i16) << 8)) as i32
                                * (SCALE_FACTOR[scale_factor] as i32)
                                / 1000) as usize;
                            client.callback(x, y, z);
                        });
                        x = (buf[1] as i16 | ((buf[2] as i16) << 8)) as usize;
                        y = (buf[3] as i16 | ((buf[4] as i16) << 8)) as usize;
                        z = (buf[5] as i16 | ((buf[6] as i16) << 8)) as usize;
                        true
                    } else {
                        self.nine_dof_client.map(|client| {
                            client.callback(0, 0, 0);
                        });
                        false
                    }
                } else {
                    false
                };
                if values {
                    self.callback.map(|callback| {
                        callback.schedule(x, y, z);
                    });
                } else {
                    self.callback.map(|callback| {
                        callback.schedule(0, 0, 0);
                    });
                }
                Lis3dshStatus::Idle
            }

            Lis3dshStatus::ReadTemperature => {
                let mut temperature: usize = 0;
                let value = if let Some(ref buf) = read_buffer {
                    if len >= 2 {
                        temperature = (buf[1] as i8) as usize;
                        self.temperature_client.map(|client| {
                            client.callback(temperature * 100);  // divide 100 in sensors app
                        });
                        true
                    } else {
                        self.temperature_client.map(|client| {
                            client.callback(0);
                        });
                        false
                    }
                } else {
                    false
                };
                if value {
                    self.callback.map(|callback| {
                        callback.schedule(temperature, 0, 0);
                    });
                } else {
                    self.callback.map(|callback| {
                        callback.schedule(0, 0, 0);
                    });
                }
                Lis3dshStatus::Idle
            }

            Lis3dshStatus::ReadValue => {
                let mut v: usize = 0;
                if let Some(ref buf) = read_buffer {
                    if len >= 2 {
                        v = buf[1] as usize;
                    }
                }
                self.callback.map(|callback| {
                    callback.schedule(v, 0, 0);
                });
                Lis3dshStatus::Idle
            }

            _ => {
                self.callback.map(|callback| {
                    callback.schedule(0, 0, 0);
                });
                Lis3dshStatus::Idle
            }
        });
        self.txbuffer.replace(write_buffer);
        if let Some(buf) = read_buffer {
            self.rxbuffer.replace(buf);
        }
    }
}

impl<'a> sensors::NineDof<'a> for Lis3dshSpi<'a> {
    fn set_client(&self, nine_dof_client: &'a dyn sensors::NineDofClient) {
        self.nine_dof_client.replace(nine_dof_client);
    }

    fn read_accelerometer(&self) -> ReturnCode {
        if self.status.get() == Lis3dshStatus::Idle {
            self.read_xyz();
            ReturnCode::SUCCESS
        } else {
            ReturnCode::EBUSY
        }
    }
}

impl<'a> sensors::TemperatureDriver<'a> for Lis3dshSpi<'a> {
    fn set_client(&self, temperature_client: &'a dyn sensors::TemperatureClient) {
        self.temperature_client.replace(temperature_client);
    }

    fn read_temperature(&self) -> ReturnCode {
        if self.status.get() == Lis3dshStatus::Idle {
            self.read_temperature();
            ReturnCode::SUCCESS
        } else {
            ReturnCode::EBUSY
        }
    }
}