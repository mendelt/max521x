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
    pub fn new(spi: SPI, mut chip_select: CS) -> Self {
        chip_select.set_high().ok();
        Self { spi, chip_select }
    }

    fn send_spi(&mut self, data: &[u8; 2]) -> Result<(), E> {
        self.chip_select.set_high().ok();
        let result = self.spi.write(data);
        self.chip_select.set_low().ok();
        result
    }

    /// Set power down mode
    pub fn power_down(&mut self, mode: PowerDownMode) -> Result<(), E> {
        self.send_spi(&[ControlBits::PowerDown as u8 | mode as u8, 0x0u8])
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

pub struct MAX5216<SPI, CS> {
    spi: SPI,
    chip_select: CS,
}

impl<SPI, CS, E> MAX5216<SPI, CS>
where
    SPI: Write<u8, Error = E>,
    CS: OutputPin,
{
    /// Construct a new MAX566x driver
    pub fn new(spi: SPI, mut chip_select: CS) -> Self {
        chip_select.set_high().ok();
        Self { spi, chip_select }
    }

    fn send_spi(&mut self, data: &[u8; 3]) -> Result<(), E> {
        self.chip_select.set_high().ok();
        let result = self.spi.write(data);
        self.chip_select.set_low().ok();
        result
    }

    /// Set power down mode
    pub fn power_down(&mut self, mode: PowerDownMode) -> Result<(), E> {
        self.send_spi(&[ControlBits::PowerDown as u8 | mode as u8, 0x0u8, 0x0u8])
    }

    /// Write data to the dac
    pub fn write_through(&mut self, data: u16) -> Result<(), E> {
        self.send_spi(&[
            ControlBits::WriteThrough as u8 | ((data >> 10) as u8),
            (data >> 2) as u8,
            (data << 6) as u8,
        ])
    }

    /// Destroy the driver and return the wrapped SPI driver and chip select pin to be re-used
    pub fn destroy(self) -> (SPI, CS) {
        (self.spi, self.chip_select)
    }
}

#[cfg(test)]
mod test_helpers {
    use embedded_hal_mock::{pin, spi};

    /// Helper method to set up mock spi and corresponding chip select pin
    pub(crate) fn setup_mock_spi() -> (spi::Mock, pin::Mock) {
        let spi = spi::Mock::new(&[]);

        // Default cs expectations, new sets high, sending command toggles low, then high
        let chip_select = pin::Mock::new(&[
            pin::Transaction::set(pin::State::High),
            pin::Transaction::set(pin::State::Low),
            pin::Transaction::set(pin::State::High),
        ]);

        (spi, chip_select)
    }
}

#[cfg(test)]
mod test_max5214 {
    use super::test_helpers::*;
    use super::*;
    use embedded_hal_mock::pin;

    #[test]
    pub fn should_init_chip_select_high() {
        let (spi, mut chip_select) = setup_mock_spi();

        chip_select.expect(&[pin::Transaction::set(pin::State::High)]);

        let _dac = MAX5214::new(spi, chip_select);
    }
}

#[cfg(test)]
mod test_max5216 {
    use super::*;
    use embedded_hal_mock::{pin, spi};

    /// Helper method to set up mock spi and corresponding chip select pin
    fn setup_mock_spi() -> (spi::Mock, pin::Mock) {
        let spi = spi::Mock::new(&[]);

        // Default cs expectations, new sets high, sending command toggles low, then high
        let chip_select = pin::Mock::new(&[
            pin::Transaction::set(pin::State::High),
            pin::Transaction::set(pin::State::Low),
            pin::Transaction::set(pin::State::High),
        ]);

        (spi, chip_select)
    }

    #[test]
    pub fn should_init_chip_select_high() {
        let (spi, mut chip_select) = setup_mock_spi();

        chip_select.expect(&[pin::Transaction::set(pin::State::High)]);

        let _dac = MAX5216::new(spi, chip_select);
    }
}
