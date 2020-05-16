#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::{blocking::spi::Write, digital::v2::OutputPin};

pub struct MAX521x<SPI, CS> {
    spi: SPI,
    chip_select: CS,
}

impl<SPI, CS, E> MAX521x<SPI, CS>
where
    SPI: Write<u8, Error = E>,
    CS: OutputPin,
{
    /// Construct a new MAX566x driver
    pub fn new(spi: SPI, chip_select: CS) -> Self {
        Self { spi, chip_select }
    }

    /// Destroy the driver and return the wrapped SPI driver and chip select pin to be re-used
    pub fn destroy(self) -> (SPI, CS) {
        (self.spi, self.chip_select)
    }
}
