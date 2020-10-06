//! Components for the LIS3DSH sensor.
//!
//! SPI Interface
//!
//! Usage
//! -----
//! ```rust
//! let lis3dsh = components::lis3dsh::Lis3dshSpiComponent::new().finalize(
//!     components::lis3dsh_spi_component_helper!(
//!         // spi type
//!         stm32f407vg::spi::Spi,
//!         // chip select
//!         stm32f407vg::gpio::PinId::PE03,
//!         // spi mux
//!         spi_mux
//!     )
//! );
//! ```
use my_capsules::lis3dsh::Lis3dshSpi;
use capsules::virtual_spi::VirtualSpiMasterDevice;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::spi;
use kernel::static_init_half;

// オブジェクト用の静的空間を設定
#[macro_export]
macro_rules! lis3dsh_spi_component_helper {
    ($A:ty, $select: expr, $spi_mux: expr) => {{
        use my_capsules::lis3dsh::Lis3dshSpi;
        use capsules::virtual_spi::VirtualSpiMasterDevice;
        use core::mem::MaybeUninit;
        let mut lis3dsh_spi: &'static capsules::virtual_spi::VirtualSpiMasterDevice<'static, $A> =
            components::spi::SpiComponent::new($spi_mux, $select)
                .finalize(components::spi_component_helper!($A));
        static mut lis3dshspi: MaybeUninit<Lis3dshSpi<'static>> = MaybeUninit::uninit();
        (&mut lis3dsh_spi, &mut lis3dshspi)
    };};
}

pub struct Lis3dshSpiComponent<S: 'static + spi::SpiMaster> {
    _select: PhantomData<S>,
}

impl<S: 'static + spi::SpiMaster> Lis3dshSpiComponent<S> {
    pub fn new() -> Lis3dshSpiComponent<S> {
        Lis3dshSpiComponent {
            _select: PhantomData,
        }
    }
}

impl<S: 'static + spi::SpiMaster> Component for Lis3dshSpiComponent<S> {
    type StaticInput = (
        &'static VirtualSpiMasterDevice<'static, S>,
        &'static mut MaybeUninit<Lis3dshSpi<'static>>,
    );
    type Output = &'static Lis3dshSpi<'static>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let lis3dsh = static_init_half!(
            static_buffer.1,
            Lis3dshSpi<'static>,
            Lis3dshSpi::new(
                static_buffer.0,
                &mut my_capsules::lis3dsh::TXBUFFER,
                &mut my_capsules::lis3dsh::RXBUFFER
            )
        );
        static_buffer.0.set_client(lis3dsh);
        lis3dsh.configure();

        lis3dsh
    }
}