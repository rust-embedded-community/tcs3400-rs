use crate::{
    BitFlags, Error, Register, RgbCGain, RgbCInterruptPersistence, Tcs3400, DEVICE_ADDRESS,
};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

impl<I2C, E> Tcs3400<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Enable the device (Power ON).
    ///
    /// The device goes to idle state.
    #[cfg(not(feature = "async"))]
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::POWER_ON)
    }

    /// Enable the device (Power ON).
    ///
    /// The device goes to idle state.
    #[cfg(feature = "async")]
    pub async fn enable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::POWER_ON).await
    }

    /// Disable the device (sleep).
    #[cfg(not(feature = "async"))]
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::POWER_ON)
    }

    /// Disable the device (sleep).
    #[cfg(feature = "async")]
    pub async fn disable(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::POWER_ON).await
    }

    /// Enable the RGB converter.
    #[cfg(not(feature = "async"))]
    pub fn enable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_EN)
    }

    /// Enable the RGB converter.
    #[cfg(feature = "async")]
    pub async fn enable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_EN).await
    }

    /// Disable the RGB converter.
    #[cfg(not(feature = "async"))]
    pub fn disable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_EN)
    }

    /// Disable the RGB converter.
    #[cfg(feature = "async")]
    pub async fn disable_rgbc(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_EN).await
    }

    /// Enable the RGB converter interrupt generation.
    #[cfg(not(feature = "async"))]
    pub fn enable_rgbc_interrupts(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_INT_EN)
    }

    /// Enable the RGB converter interrupt generation.
    #[cfg(feature = "async")]
    pub async fn enable_rgbc_interrupts(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::RGBC_INT_EN).await
    }

    /// Disable the RGB converter interrupt generation.
    #[cfg(not(feature = "async"))]
    pub fn disable_rgbc_interrupts(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_INT_EN)
    }

    /// Disable the RGB converter interrupt generation.
    #[cfg(feature = "async")]
    pub async fn disable_rgbc_interrupts(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::RGBC_INT_EN).await
    }

    /// Enable the wait feature (wait timer).
    #[cfg(not(feature = "async"))]
    pub fn enable_wait(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::WAIT_EN)
    }

    /// Enable the wait feature (wait timer).
    #[cfg(feature = "async")]
    pub async fn enable_wait(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable | BitFlags::WAIT_EN).await
    }

    /// Disable the wait feature (wait timer).
    #[cfg(not(feature = "async"))]
    pub fn disable_wait(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::WAIT_EN)
    }

    /// Disable the wait feature (wait timer).
    #[cfg(feature = "async")]
    pub async fn disable_wait(&mut self) -> Result<(), Error<E>> {
        let enable = self.enable;
        self.write_enable(enable & !BitFlags::WAIT_EN).await
    }

    #[cfg(not(feature = "async"))]
    fn write_enable(&mut self, enable: u8) -> Result<(), Error<E>> {
        self.write_register(Register::ENABLE, enable)?;
        self.enable = enable;
        Ok(())
    }

    #[cfg(feature = "async")]
    async fn write_enable(&mut self, enable: u8) -> Result<(), Error<E>> {
        self.write_register(Register::ENABLE, enable).await?;
        self.enable = enable;
        Ok(())
    }

    /// Set the number of wait time cycles  (1-256).
    ///
    /// The actual wait time depends on the "*wait long*" setting.
    /// - If *wait long* is disabled, then the wait time corresponds to:
    ///   `number_of_cycles * 2.78ms`.
    /// - If *wait long* is enabled, then the wait time is increased by a
    ///   factor of 12 and therefore corresponds to aproximately:
    ///   `number_of_cycles * 0.03s`.
    /// See [`enable_wait_long()`](#method.enable_wait_long) and
    ///  [`disable_wait_long()`](#method.disable_wait_long).
    #[cfg(not(feature = "async"))]
    pub fn set_wait_cycles(&mut self, cycles: u16) -> Result<(), Error<E>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::WTIME, (256 - cycles as u16) as u8)
    }

    /// Set the number of wait time cycles  (1-256).
    ///
    /// The actual wait time depends on the "*wait long*" setting.
    /// - If *wait long* is disabled, then the wait time corresponds to:
    ///   `number_of_cycles * 2.78ms`.
    /// - If *wait long* is enabled, then the wait time is increased by a
    ///   factor of 12 and therefore corresponds to aproximately:
    ///   `number_of_cycles * 0.03s`.
    /// See [`enable_wait_long()`](#method.enable_wait_long) and
    ///  [`disable_wait_long()`](#method.disable_wait_long).
    #[cfg(feature = "async")]
    pub async fn set_wait_cycles(&mut self, cycles: u16) -> Result<(), Error<E>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::WTIME, (256 - cycles as u16) as u8)
            .await
    }

    /// Enable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is increased by a
    /// factor of 12. See [`set_wait_cycles()`](#method.set_wait_cycles).
    #[cfg(not(feature = "async"))]
    pub fn enable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, BitFlags::WLONG)
    }

    /// Enable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is increased by a
    /// factor of 12. See [`set_wait_cycles()`](#method.set_wait_cycles).
    #[cfg(feature = "async")]
    pub async fn enable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, BitFlags::WLONG).await
    }

    /// Disable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is used without
    /// multiplication factor. See [`set_wait_cycles()`](#method.set_wait_cycles).
    #[cfg(not(feature = "async"))]
    pub fn disable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, 0)
    }

    /// Disable the *wait long* setting.
    ///
    /// The wait time configured with `set_wait_cycles()` is used without
    /// multiplication factor. See [`set_wait_cycles()`](#method.set_wait_cycles).
    #[cfg(feature = "async")]
    pub async fn disable_wait_long(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::CONFIG, 0).await
    }

    /// Set the RGB converter gain.
    #[cfg(not(feature = "async"))]
    pub fn set_rgbc_gain(&mut self, gain: RgbCGain) -> Result<(), Error<E>> {
        // Register field: AGAIN
        match gain {
            RgbCGain::_1x => self.write_register(Register::CONTROL, 0),
            RgbCGain::_4x => self.write_register(Register::CONTROL, 1),
            RgbCGain::_16x => self.write_register(Register::CONTROL, 2),
            RgbCGain::_60x => self.write_register(Register::CONTROL, 3),
        }
    }

    /// Set the RGB converter gain.
    #[cfg(feature = "async")]
    pub async fn set_rgbc_gain(&mut self, gain: RgbCGain) -> Result<(), Error<E>> {
        // Register field: AGAIN
        match gain {
            RgbCGain::_1x => self.write_register(Register::CONTROL, 0).await,
            RgbCGain::_4x => self.write_register(Register::CONTROL, 1).await,
            RgbCGain::_16x => self.write_register(Register::CONTROL, 2).await,
            RgbCGain::_60x => self.write_register(Register::CONTROL, 3).await,
        }
    }

    /// Set the number of integration cycles (1-256).
    ///
    /// The actual integration time corresponds to: `number_of_cycles * 2.78ms`.
    #[cfg(not(feature = "async"))]
    pub fn set_integration_cycles(&mut self, cycles: u16) -> Result<(), Error<E>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::ATIME, (256 - cycles as u16) as u8)
    }

    /// Set the number of integration cycles (1-256).
    ///
    /// The actual integration time corresponds to: `number_of_cycles * 2.78ms`.
    #[cfg(feature = "async")]
    pub async fn set_integration_cycles(&mut self, cycles: u16) -> Result<(), Error<E>> {
        if cycles > 256 || cycles == 0 {
            return Err(Error::InvalidInputData);
        }
        // the value is stored as a two's complement
        self.write_register(Register::ATIME, (256 - cycles as u16) as u8)
            .await
    }

    /// Set the RGB converter interrupt clear channel low threshold.
    #[cfg(not(feature = "async"))]
    pub fn set_rgbc_interrupt_low_threshold(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_register(Register::AILTL, threshold as u8)?;
        self.write_register(Register::AILTH, (threshold >> 8) as u8)
    }

    /// Set the RGB converter interrupt clear channel low threshold.
    #[cfg(feature = "async")]
    pub async fn set_rgbc_interrupt_low_threshold(
        &mut self,
        threshold: u16,
    ) -> Result<(), Error<E>> {
        self.write_register(Register::AILTL, threshold as u8)
            .await?;
        self.write_register(Register::AILTH, (threshold >> 8) as u8)
            .await
    }

    /// Set the RGB converter interrupt clear channel high threshold.
    #[cfg(not(feature = "async"))]
    pub fn set_rgbc_interrupt_high_threshold(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_register(Register::AIHTL, threshold as u8)?;
        self.write_register(Register::AIHTH, (threshold >> 8) as u8)
    }

    /// Set the RGB converter interrupt clear channel high threshold.
    #[cfg(feature = "async")]
    pub async fn set_rgbc_interrupt_high_threshold(
        &mut self,
        threshold: u16,
    ) -> Result<(), Error<E>> {
        self.write_register(Register::AIHTL, threshold as u8)
            .await?;
        self.write_register(Register::AIHTH, (threshold >> 8) as u8)
            .await
    }

    /// Set the RGB converter interrupt persistence.
    ///
    /// This controls the RGB converter interrupt generation rate.
    #[cfg(not(feature = "async"))]
    pub fn set_rgbc_interrupt_persistence(
        &mut self,
        persistence: RgbCInterruptPersistence,
    ) -> Result<(), Error<E>> {
        use crate::RgbCInterruptPersistence as IP;
        match persistence {
            IP::Every => self.write_register(Register::APERS, 0),
            IP::Any => self.write_register(Register::APERS, 1),
            IP::_2 => self.write_register(Register::APERS, 2),
            IP::_3 => self.write_register(Register::APERS, 3),
            IP::_5 => self.write_register(Register::APERS, 4),
            IP::_10 => self.write_register(Register::APERS, 5),
            IP::_15 => self.write_register(Register::APERS, 6),
            IP::_20 => self.write_register(Register::APERS, 7),
            IP::_25 => self.write_register(Register::APERS, 8),
            IP::_30 => self.write_register(Register::APERS, 9),
            IP::_35 => self.write_register(Register::APERS, 10),
            IP::_40 => self.write_register(Register::APERS, 11),
            IP::_45 => self.write_register(Register::APERS, 12),
            IP::_50 => self.write_register(Register::APERS, 13),
            IP::_55 => self.write_register(Register::APERS, 14),
            IP::_60 => self.write_register(Register::APERS, 15),
        }
    }

    /// Set the RGB converter interrupt persistence.
    ///
    /// This controls the RGB converter interrupt generation rate.
    #[cfg(feature = "async")]
    pub async fn set_rgbc_interrupt_persistence(
        &mut self,
        persistence: RgbCInterruptPersistence,
    ) -> Result<(), Error<E>> {
        use crate::RgbCInterruptPersistence as IP;
        match persistence {
            IP::Every => self.write_register(Register::APERS, 0).await,
            IP::Any => self.write_register(Register::APERS, 1).await,
            IP::_2 => self.write_register(Register::APERS, 2).await,
            IP::_3 => self.write_register(Register::APERS, 3).await,
            IP::_5 => self.write_register(Register::APERS, 4).await,
            IP::_10 => self.write_register(Register::APERS, 5).await,
            IP::_15 => self.write_register(Register::APERS, 6).await,
            IP::_20 => self.write_register(Register::APERS, 7).await,
            IP::_25 => self.write_register(Register::APERS, 8).await,
            IP::_30 => self.write_register(Register::APERS, 9).await,
            IP::_35 => self.write_register(Register::APERS, 10).await,
            IP::_40 => self.write_register(Register::APERS, 11).await,
            IP::_45 => self.write_register(Register::APERS, 12).await,
            IP::_50 => self.write_register(Register::APERS, 13).await,
            IP::_55 => self.write_register(Register::APERS, 14).await,
            IP::_60 => self.write_register(Register::APERS, 15).await,
        }
    }

    #[cfg(not(feature = "async"))]
    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEVICE_ADDRESS, &[register, value])
            .map_err(Error::I2C)
    }

    #[cfg(feature = "async")]
    async fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(DEVICE_ADDRESS, &[register, value])
            .await
            .map_err(Error::I2C)
    }
}
