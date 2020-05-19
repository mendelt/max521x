#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::{blocking::spi::Write, digital::v2::OutputPin};

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

pub struct MAX5214<SPI, CS> {
    spi: SPI,
    chip_select: CS,
}

impl<SPI, CS, E> MAX5214<SPI, CS>
where
    SPI: Write<u8, Error = E>,
    CS: OutputPin,
{
    /// Construct a new MAX566x driver
    pub fn new(spi: SPI, chip_select: CS) -> Self {
        Self { spi, chip_select }
    }

    fn send_spi(&mut self, data: &[u8;2]) -> Result<(), E> {
        self.chip_select.set_high().ok();
        let result = self.spi.write(data);
        self.chip_select.set_low().ok();
        result
    }

    /// Set power down mode
    pub fn power_down(&mut self, mode: PowerDownMode) -> Result<(), E> {
        self.send_spi(&[
            ControlBits::PowerDown as u8 | mode as u8,
            0x0u8
        ])
    }

    /// Write data to the dac
    pub fn write_through(&mut self, data: u16) -> Result<(), E> {
        self.send_spi(&[
            ControlBits::WriteThrough as u8 | ((data >> 8) as u8 & 0x3fu8),
            data as u8,
        ])
    }

    /// Destroy the driver and return the wrapped SPI driver and chip select pin to be re-used
    pub fn destroy(self) -> (SPI, CS) {
        (self.spi, self.chip_select)
    }
}
