use crate::{AllChannelMeasurement, BitFlags, Error, Register, Tcs3400, DEVICE_ADDRESS};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

impl<I2C, E> Tcs3400<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Check whether the RGB converter status is valid.
    ///
    /// Indicates that the RGBC channels have completed an integration cycle.
    #[allow(clippy::wrong_self_convention)]
    #[cfg(not(feature = "async"))]
    pub fn is_rgbc_status_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok((status & BitFlags::RGBC_VALID) != 0)
    }

    /// Check whether the RGB converter status is valid.
    ///
    /// Indicates that the RGBC channels have completed an integration cycle.
    #[allow(clippy::wrong_self_convention)]
    #[cfg(feature = "async")]
    pub async fn is_rgbc_status_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS).await?;
        Ok((status & BitFlags::RGBC_VALID) != 0)
    }

    /// Read the clear (unfiltered) channel measurement data.
    #[cfg(not(feature = "async"))]
    pub fn read_clear_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::CDATAL)
    }

    /// Read the clear (unfiltered) channel measurement data.
    #[cfg(feature = "async")]
    pub async fn read_clear_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::CDATAL).await
    }

    /// Read the red channel measurement data.
    #[cfg(not(feature = "async"))]
    pub fn read_red_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::RDATAL)
    }

    /// Read the red channel measurement data.
    #[cfg(feature = "async")]
    pub async fn read_red_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::RDATAL).await
    }

    /// Read the green channel measurement data.
    #[cfg(not(feature = "async"))]
    pub fn read_green_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::GDATAL)
    }

    /// Read the green channel measurement data.
    #[cfg(feature = "async")]
    pub async fn read_green_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::GDATAL).await
    }

    /// Read the blue channel measurement data.
    #[cfg(not(feature = "async"))]
    pub fn read_blue_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::BDATAL)
    }

    /// Read the blue channel measurement data.
    #[cfg(feature = "async")]
    pub async fn read_blue_channel(&mut self) -> Result<u16, Error<E>> {
        self.read_channel(Register::BDATAL).await
    }

    #[cfg(not(feature = "async"))]
    fn read_channel(&mut self, first_register: u8) -> Result<u16, Error<E>> {
        let mut cdata = [0; 2];
        self.read_registers(first_register, &mut cdata)?;
        Ok((u16::from(cdata[1])) << 8 | u16::from(cdata[0]))
    }

    #[cfg(feature = "async")]
    async fn read_channel(&mut self, first_register: u8) -> Result<u16, Error<E>> {
        let mut cdata = [0; 2];
        self.read_registers(first_register, &mut cdata).await?;
        Ok((u16::from(cdata[1])) << 8 | u16::from(cdata[0]))
    }

    /// Read the measurement data of all channels at once.
    #[cfg(not(feature = "async"))]
    pub fn read_all_channels(&mut self) -> Result<AllChannelMeasurement, Error<E>> {
        let mut data = [0; 8];
        self.read_registers(Register::CDATAL, &mut data)?;
        Ok(AllChannelMeasurement {
            clear: u16::from(data[1]) << 8 | u16::from(data[0]),
            red: u16::from(data[3]) << 8 | u16::from(data[2]),
            green: u16::from(data[5]) << 8 | u16::from(data[4]),
            blue: u16::from(data[7]) << 8 | u16::from(data[6]),
        })
    }

    /// Read the measurement data of all channels at once.
    #[cfg(feature = "async")]
    pub async fn read_all_channels(&mut self) -> Result<AllChannelMeasurement, Error<E>> {
        let mut data = [0; 8];
        self.read_registers(Register::CDATAL, &mut data).await?;
        Ok(AllChannelMeasurement {
            clear: u16::from(data[1]) << 8 | u16::from(data[0]),
            red: u16::from(data[3]) << 8 | u16::from(data[2]),
            green: u16::from(data[5]) << 8 | u16::from(data[4]),
            blue: u16::from(data[7]) << 8 | u16::from(data[6]),
        })
    }

    /// Read the device ID.
    ///
    /// The value returned corresponds to the part number identification:
    /// - `0x90` => `TCS34001 & TCS34005`
    /// - `0x93` => `TCS34003 & TCS34007`
    #[cfg(not(feature = "async"))]
    pub fn read_device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::ID)
    }

    /// Read the device ID.
    ///
    /// The value returned corresponds to the part number identification:
    /// - `0x90` => `TCS34001 & TCS34005`
    /// - `0x93` => `TCS34003 & TCS34007`
    #[cfg(feature = "async")]
    pub async fn read_device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::ID).await
    }

    #[cfg(not(feature = "async"))]
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0])
    }

    #[cfg(feature = "async")]
    async fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .await
            .map_err(Error::I2C)?;
        Ok(data[0])
    }

    #[cfg(not(feature = "async"))]
    fn read_registers(&mut self, first_register: u8, mut data: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(DEVICE_ADDRESS, &[first_register], &mut data)
            .map_err(Error::I2C)
    }

    #[cfg(feature = "async")]
    async fn read_registers(
        &mut self,
        first_register: u8,
        mut data: &mut [u8],
    ) -> Result<(), Error<E>> {
        self.i2c
            .write_read(DEVICE_ADDRESS, &[first_register], &mut data)
            .await
            .map_err(Error::I2C)
    }
}
