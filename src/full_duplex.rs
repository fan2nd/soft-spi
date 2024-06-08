use embedded_hal::{digital::*, spi};

pub struct SoftSpi<In: InputPin, Out: OutputPin> {
    sck: Out,
    miso: In,
    mosi: Out,
}

impl<In: InputPin, Out: OutputPin> SoftSpi<In, Out> {
    pub fn new(sck: Out, miso: In, mosi: Out) -> SoftSpi<In, Out> {
        let mut this = SoftSpi { sck, miso, mosi };
        let _ = this.sck.set_high();
        this
    }
}
impl<In: InputPin, Out: OutputPin> spi::ErrorType for SoftSpi<In, Out> {
    type Error = spi::ErrorKind;
}

impl<In: InputPin, Out: OutputPin> spi::SpiBus for SoftSpi<In, Out> {
    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for bytes in 0..words.len() {
            for _ in 0..8 {
                // Clock high then low
                self.sck.set_low().ok();
                self.sck.set_high().ok();
                // Read the MISO pin after clocking the data in
                words[bytes] <<= 1;
                if Some(true) == self.miso.is_high().ok() {
                    words[bytes] += 1;
                }
            }
        }
        Ok(())
    }

    /// Write `words` to the slave, ignoring all the incoming words.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for bytes in 0..words.len() {
            for bits in 0..8 {
                let bit = (words[bytes] << bits) & 0x80;
                self.mosi.set_state((bit != 0).into()).ok(); // Set the MOSI pin to the current bit value
                                                             // Clock high then low
                self.sck.set_low().ok();
                self.sck.set_high().ok();
            }
        }
        Ok(())
    }

    /// Write and read simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        assert_eq!(
            read.len(),
            write.len(),
            "Read and write buffers must be the same length"
        );

        for bytes in 0..read.len() {
            for bits in 0..8 {
                let bit = (write[bytes] << bits) & 0x80;
                self.mosi.set_state((bit != 0).into()).ok(); // Set the MOSI pin to the current bit value
                                                             // Clock high then low
                self.sck.set_low().ok();
                self.sck.set_high().ok();

                // Read the MISO pin after clocking the data in
                read[bytes] <<= 1;
                if Some(true) == self.miso.is_high().ok() {
                    read[bytes] = read[bytes] + 1;
                }
            }
        }
        Ok(())
    }

    /// Write and read simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for bytes in 0..words.len() {
            let mut read_byte = 0 as u8;
            for bits in 0..8 {
                let bit = (words[bytes] << bits) & 0x80;
                self.mosi.set_high().ok();
                self.mosi.set_state((bit != 0).into()).ok(); // Set the MOSI pin to the current bit value

                // Clock high then low
                self.sck.set_low().ok();
                self.sck.set_high().ok();

                // Read the MISO pin after clocking the data in
                read_byte <<= 1;
                if Some(true) == self.miso.is_high().ok() {
                    read_byte += 1;
                }
            }
            words[bytes] = read_byte;
        }
        Ok(())
    }

    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See the [module-level documentation](self) for important usage information.
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
