use embedded_hal::{digital::*, spi};

pub struct SoftSpi<Out, InOut>
where
    Out: OutputPin,
    InOut: InputPin + OutputPin,
{
    sck: Out,
    sda: InOut,
}

impl<Out, InOut> SoftSpi<Out, InOut>
where
    Out: OutputPin,
    InOut: InputPin + OutputPin,
{
    /// **sda needs to be pullup**
    pub fn new(sck: Out, sda: InOut) -> Self {
        let mut this = SoftSpi { sck, sda };
        let _ = this.sck.set_high();
        this
    }
}
impl<Out, InOut> spi::ErrorType for SoftSpi<Out, InOut>
where
    Out: OutputPin,
    InOut: InputPin + OutputPin,
{
    type Error = spi::ErrorKind;
}

impl<Out, InOut> spi::SpiBus for SoftSpi<Out, InOut>
where
    Out: OutputPin,
    InOut: InputPin + OutputPin,
{
    /// Read `words` from the slave.
    ///
    /// The word value sent on MOSI during reading is implementation-defined,
    /// typically `0x00`, `0xFF`, or configurable.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.sda.set_high().ok();
        for bytes in 0..words.len() {
            for _ in 0..8 {
                // Clock high then low
                self.sck.set_low().ok();
                self.sck.set_high().ok();
                // Read the MISO pin after clocking the data in
                words[bytes] <<= 1;
                if Some(true) == self.sda.is_high().ok() {
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
                //self.sda.set_state((bit != 0).into()).ok(); // Set the MOSI pin to the current bit value
                self.sda.set_state((bit != 0).into()).ok();
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
    fn transfer(&mut self, _: &mut [u8], _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write and read simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Implementations are allowed to return before the operation is
    /// complete. See the [module-level documentation](self) for details.
    fn transfer_in_place(&mut self, _: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Wait until all operations have completed and the bus is idle.
    ///
    /// See the [module-level documentation](self) for important usage information.
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
