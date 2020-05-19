#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::{blocking::spi::Write, digital::v2::OutputPin};

pub struct MAX521x<SPI, CS> {
    spi: SPI,
    chip_select: CS,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum ControlBits {
    PowerDown = 0b10000000u8,
    WriteThrough = 0b01000000u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
/// Power down modes for the DAC outputs
pub enum PowerDownMode {
    Normal = 0b0000u8,
    HighImpedance = 0b0100u8,
    Gnd100KOhm = 0b1000u8,
    Gnd1KOhm = 0b1100u8,
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
