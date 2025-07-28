// #![deny(missing_docs)]

use core::option::Option;
use core::result::Result;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use heapless::String;
use heapless::Vec;

use crate::registers::{ErrorStatus, Registers};
use crate::time::*;
use crate::types::{Config, Measurement, ProductType, SetData};

const EXPECT_MSG: &str = "Vec was not large enough";
const ADDRESS: u8 = 0x68;
const RAM_DELAY: u32 = 1;
const EEPROM_DELAY: u32 = 25;

/// Represents the Sunrise sensor module with its associated components and configuration.
///
/// # Type Parameters
/// - `T`: The timer type used for managing delays or time-based operations.
/// - `I2C`: The I2C interface type used for communication with the sensor.
/// - `EN`: The type of the enable pin, which is optional.
/// - `NRDY`: The type of the not-ready pin, used to check the sensor's readiness.
///
/// # Fields
/// - `i2c`: The I2C interface instance for communication with the sensor.
/// - `timer`: The timer instance for handling delays or timing operations.
/// - `en_pin`: An optional enable pin used to control the sensor's power state.
/// - `n_rdy_pin`: The not-ready pin used to determine the sensor's readiness state.
/// - `address`: The I2C address of the sensor.
/// - `state_buf`: A buffer used to store the sensor's state data.
/// - `config`: The configuration settings for the sensor.
/// - `product_type`: The type of the product, representing the specific Sunrise sensor model.
/// - `mesurement`: The measurement data obtained from the sensor.
/// - `set_data`: The data used to configure or control the sensor.
pub struct Sunrise<T, I2C, EN, NRDY> {
    i2c: I2C,
    timer: T,
    en_pin: Option<EN>,
    n_rdy_pin: NRDY,
    address: u8,
    state_buf: [u8; 24],
    config: Config,
    product_type: ProductType,
    mesurement: Measurement,
    set_data: SetData,
}
/**
 * # Sunrise
 *
 * Abstraction for interacting with an I2C device of type Sunrise.
 *
 * ## Parameters:
 * - `T`: A timer that implements the `Timer` trait.
 * - `I2C`: An I2C interface that implements the following traits:
 *   - `Read`
 *   - `Write`
 *   - `WriteRead`
 * - `EN`: An enable pin that implements `OutputPin`.
 * - `NRDY`: A pin that indicates the readiness of the device, implementing `InputPin`.
 *
 * ## Fields:
 * - `i2c`: The I2C interface for device communication.
 * - `timer`: Timer for delays.
 * - `en_pin`: Optional enable pin.
 * - `n_rdy_pin`: Readiness indicator pin.
 * - `address`: The I2C device address.
 * - `state_buf`: Buffer for state storage.
 * - `config`: Device configuration.
 * - `product_type`: The product type of the device.
 * - `measurement`: Object that holds measurement data.
 * - `set_data`: Configuration settings for the device.
 */
impl<T, I2C, E, EN, NRDY> Sunrise<T, I2C, EN, NRDY>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    T: DelayMs<u32>,
    EN: OutputPin,
    NRDY: InputPin,
{
    /**
     * ## new
     * Constructor to create an instance of `Sunrise`.
     *
     * ### Parameters:
     * - `i2c`: The I2C interface for data communication.
     * - `timer`: Timer for handling time delays.
     * - `en_pin`: Optional enable pin for power control.
     * - `nrdy_pin`: Pin indicating the readiness of the device.
     *
     * ### Returns:
     * - An instance of `Sunrise`.
     */
    pub fn new(i2c: I2C, timer: T, en_pin: Option<EN>, nrdy_pin: NRDY) -> Self {
        Sunrise {
            i2c: i2c,
            timer: timer,
            en_pin: en_pin,
            n_rdy_pin: nrdy_pin,
            address: ADDRESS,
            state_buf: [0x00; 24],
            config: Config::default(),
            product_type: ProductType::default(),
            mesurement: Measurement::default(),
            set_data: SetData::default(),
        }
    }
    /**
     * ## delay_ms
     * Creates a delay in milliseconds using the timer.
     *
     * ### Parameters:
     * - `ms`: The number of milliseconds to wait.
     *
     * ### Example:
     * ```
     * sunrise.delay_ms(500); // Waits for 500 milliseconds
     * ```
     */
    fn delay_ms(&mut self, ms: u32) {
        self.timer.delay_ms(ms);
    }
    /**
     * ## read_register
     * Reads data from the specified register.
     *
     * ### Parameters:
     * - `register`: The register address to read from.
     * - `buf`: A mutable buffer to store the read data.
     *
     * ### Returns:
     * - `Ok(())` if the read operation is successful.
     * - `Err(E)` if there is an I2C communication error.
     *
     * ### Example:
     * ```
     * let mut buffer = [0u8; 8];
     * sunrise.read_register(Registers::Status, &mut buffer).unwrap();
     * ```
     */
    fn read_register(&mut self, register: Registers, buf: &mut [u8]) -> Result<(), E> {
        self.i2c
            .write_read(self.address, &[register as u8], buf)
            .or_else(|_| self.i2c.write_read(self.address, &[register as u8], buf))?;
        Ok(())
    }
    /**
     * ## write_register
     * Writes data to the specified register with an optional delay.
     *
     * ### Parameters:
     * - `register`: The register address to write to.
     * - `data`: A slice of bytes to be written.
     * - `delay`: An optional delay (in milliseconds) after the write operation.
     *
     * ### Returns:
     * - `Ok(())` if the write operation is successful.
     * - `Err(E)` if there is an I2C communication error.
     *
     * ### Example:
     * ```
     * sunrise.write_register(Registers::Config, &[0x01, 0x02], 100).unwrap();
     * ```
     */
    fn write_register(&mut self, register: Registers, data: &[u8], delay: u32) -> Result<(), E> {
        // Создаем новый heapless::Vec с фиксированным размером (например, 32 байта)
        let mut buf: Vec<u8, 32> = Vec::new();

        // Пытаемся добавить адрес регистра
        buf.push(register as u8).expect(EXPECT_MSG);

        // Добавляем данные в буфер
        buf.extend_from_slice(data).expect(EXPECT_MSG);

        // Отправляем данные через I2C
        self.i2c
            .write_read(self.address, &[0], &mut [0])
            .or_else(|_| self.i2c.write(self.address, &buf))?;

        self.delay_ms(delay);

        Ok(())
    }
    /**
     * ## product_type_get
     * Reads the product type information from the device registers.
     *
     * ### Returns:
     * - `Ok(())` if the read operations are successful.
     * - `Err(E)` if there is an I2C communication error.
     *
     * ### Example:
     * ```
     * sunrise.product_type_get().unwrap();
     * println!("Product Code: {}", sunrise.product_type.product_code);
     * ```
     */
    fn product_type_get(&mut self) -> Result<(), E> {
        let mut vec: Vec<u8, 16> = Vec::new();
        let mut buf = [0u8; 16];

        self.read_register(Registers::FirmwareType, &mut buf[..1])?;
        self.product_type.firmware_type = buf[0];

        self.read_register(Registers::FirmwareVer, &mut buf[..2])?;
        self.product_type.main_revision = buf[0];
        self.product_type.sub_revision = buf[1];

        self.read_register(Registers::SensorId, &mut buf[..4])?;
        self.product_type.sensor_id = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);

        if self.product_type.main_revision >= 4 && self.product_type.sub_revision >= 8 {
            vec.extend_from_slice(&mut buf).expect(EXPECT_MSG);
            self.product_type.product_code = String::from_utf8(vec).unwrap();
        } else {
            self.product_type.product_code = String::try_from("No Supporte").unwrap();
        }

        Ok(())
    }

    /**
     * ## check_sensor_error
     *
     * This method checks the bitmask value from the sensor's error register and matches it to known error states.
     *
     * ### Parameters:
     * - `sensor_err`: A `u16` bitmask representing the current error status from the sensor.
     *
     * ### Returns:
     * - `Option<ErrorStatus<E>>`:  
     *   - `Some(ErrorStatus::...)` — if a known error bit is set.
     *   - `None` — if no error bits are detected.
     *
     * ### Error Mapping:
     * The bit positions in the `sensor_err` bitmask correspond to specific error states:
     *
     * | Bit Position | Error Type                            |
     * |--------------|--------------------------------------|
     * | 15           | `LowInternalRegulatedVoltage`        |
     * | 14           | `MeasurementTimeout`                 |
     * | 13           | `AbnormalSignalLevel`                |
     * | 8            | `ScaleFactorError`                   |
     * | 7            | `FatalError`                         |
     * | 6            | `I2cError`                           |
     * | 5            | `AlgorithmError`                     |
     * | 4            | `CalibrationError`                   |
     * | 3            | `SelfDiagnosticsError`               |
     * | 2            | `OutOfRange`                         |
     * | 1            | `MemoryError`                        |
     * | 0            | `NoMeasurementCompleted`             |
     *
     * ### Example:
     * ```rust
     * let error_code: u16 = 0b1000000000000000; // Bit 15 is set
     * if let Some(error) = sunrise.check_sensor_error(error_code) {
     *     println!("Sensor Error Detected: {:?}", error);
     * } else {
     *     println!("No Errors Detected.");
     * }
     * ```
     *
     * ### Explanation:
     * - The method uses bitwise AND (`&`) to check if specific bits are set.
     * - If a bit matches an error type, it returns `Some(ErrorStatus::...)`.
     * - If no bits are set, it returns `None`.
     *
     */
    fn check_sensor_error(&mut self, sensor_err: u16) -> Option<ErrorStatus<E>> {
        ErrorStatus::from_bits(sensor_err)
    }

    /**
     * ## clear_error_status
     *
     * Clears the current error status of the sensor by writing to the `ClearErrorStatus` register.
     *
     * This method sends a write command to the sensor's specified register to reset any error flags that may have been set during operation.
     *
     * ### Returns:
     * - `Ok(())`: If the write operation to the register was successful.
     * - `Err(E)`: If there was an I2C communication error during the write process.
     *
     * ### Example:
     * ```rust
     * match sunrise.clear_error_status() {
     *     Ok(_) => println!("Error status cleared successfully."),
     *     Err(e) => println!("Failed to clear error status: {:?}", e),
     * }
     * ```
     *
     * ### Explanation:
     * This method attempts to clear any existing error flags by writing `0x00` to the `ClearErrorStatus` register of the sensor.
     * The operation uses a pre-defined constant `RAM_DELAY` to handle the necessary wait time after the operation.
     */
    fn clear_error_status(&mut self) -> Result<(), E> {
        self.write_register(Registers::ClearErrorStatus, &[0x00u8], RAM_DELAY)
    }
    /**
     * ## sensor_state_data_set
     *
     * Writes the internal configuration state data from the `set_data` structure to the sensor's registers.
     *
     * This method sequentially writes the values of the parameters from `self.set_data` to the corresponding registers
     * on the sensor. It ensures that each value is converted to big-endian byte order before transmission, as expected
     * by the hardware.
     *
     * ### Returns:
     * - `Ok(())`: If all write operations to the registers are successful.
     * - `Err(E)`: If any I2C write operation fails during the process.
     *
     * ### Registers Written:
     * | **Register Name** | **Field in `set_data`**       |
     * |--------------------|-------------------------------|
     * | `AbcTime`         | `abc_time`                    |
     * | `AbcPar0`         | `abc_par0`                    |
     * | `AbcPar1`         | `abc_par1`                    |
     * | `AbcPar2`         | `abc_par2`                    |
     * | `AbcPar3`         | `abc_par3`                    |
     * | `FilterPar0`      | `filter_par0`                 |
     * | `FilterPar1`      | `filter_par1`                 |
     * | `FilterPar2`      | `filter_par2`                 |
     * | `FilterPar3`      | `filter_par3`                 |
     * | `FilterPar4`      | `filter_par4`                 |
     * | `FilterPar5`      | `filter_par5`                 |
     * | `FilterPar6`      | `filter_par6`                 |
     *
     * ### Example:
     * ```rust
     * if let Err(e) = sunrise.sensor_state_data_set() {
     *     println!("Failed to set sensor state data: {:?}", e);
     * } else {
     *     println!("Sensor state data successfully written to registers.");
     * }
     * ```
     *
     * ### Explanation:
     * The method:
     * 1. Iterates over each parameter in the `set_data` structure.
     * 2. Converts each parameter to big-endian format using `.to_be_bytes()`.
     * 3. Writes the bytes to the corresponding register with a specified `RAM_DELAY`.
     * 4. If any write operation fails, the function returns `Err(E)`.
     *
     */
    fn sensor_state_data_set(&mut self) -> Result<(), E> {
        self.write_register(
            Registers::AbcTime,
            &mut self.set_data.abc_time.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::AbcPar0,
            &mut self.set_data.abc_par0.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::AbcPar1,
            &mut self.set_data.abc_par1.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::AbcPar2,
            &mut self.set_data.abc_par2.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::AbcPar3,
            &mut self.set_data.abc_par3.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar0,
            &mut self.set_data.filter_par0.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar1,
            &mut self.set_data.filter_par1.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar2,
            &mut self.set_data.filter_par2.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar3,
            &mut self.set_data.filter_par3.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar4,
            &mut self.set_data.filter_par4.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar5,
            &mut self.set_data.filter_par5.to_be_bytes(),
            RAM_DELAY,
        )?;
        self.write_register(
            Registers::FilterPar6,
            &mut self.set_data.filter_par6.to_be_bytes(),
            RAM_DELAY,
        )?;
        Ok(())
    }

    /**
     * ## sensor_state_data_get
     *
     * Reads the state configuration parameters from the sensor's registers and updates the internal
     * `set_data` structure. This method sequentially accesses each register, retrieves the 16-bit
     * big-endian value, and stores it into the corresponding field in `set_data`.
     *
     * ### Returns:
     * - `Ok(())`: If all register reads were successful.
     * - `Err(E)`: If any I2C communication error occurred during the read operations.
     *
     * ### Registers Read:
     * | **Register Name** | **Field in `set_data`**       |
     * |--------------------|-------------------------------|
     * | `AbcTime`         | `abc_time`                    |
     * | `AbcPar0`         | `abc_par0`                    |
     * | `AbcPar1`         | `abc_par1`                    |
     * | `AbcPar2`         | `abc_par2`                    |
     * | `AbcPar3`         | `abc_par3`                    |
     * | `FilterPar0`      | `filter_par0`                 |
     * | `FilterPar1`      | `filter_par1`                 |
     * | `FilterPar2`      | `filter_par2`                 |
     * | `FilterPar3`      | `filter_par3`                 |
     * | `FilterPar4`      | `filter_par4`                 |
     * | `FilterPar5`      | `filter_par5`                 |
     * | `FilterPar6`      | `filter_par6`                 |
     *
     * ### Example:
     * ```rust
     * if let Err(e) = sunrise.sensor_state_data_get() {
     *     println!("Failed to read sensor state data: {:?}", e);
     * } else {
     *     println!("Sensor state data successfully read and updated.");
     * }
     * ```
     *
     * ### Explanation:
     * - The method initializes a 2-byte buffer (`state_buf`) to store temporary read data.
     * - It iterates through all the relevant registers:
     *   - Reads 2 bytes of data from each register.
     *   - Converts the big-endian data into a `u16` using `from_be_bytes`.
     *   - Updates the corresponding field in `self.set_data`.
     * - If any read operation fails, it immediately returns `Err(E)`.
     */
    fn sensor_state_data_get(&mut self) -> Result<(), E> {
        let mut state_buf: [u8; 2] = [0; 2];

        self.read_register(Registers::AbcTime, &mut state_buf)?;
        self.set_data.abc_time = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::AbcPar0, &mut state_buf)?;
        self.set_data.abc_par0 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::AbcPar1, &mut state_buf)?;
        self.set_data.abc_par1 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::AbcPar2, &mut state_buf)?;
        self.set_data.abc_par2 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::AbcPar3, &mut state_buf)?;
        self.set_data.abc_par3 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar0, &mut state_buf)?;
        self.set_data.filter_par0 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar1, &mut state_buf)?;
        self.set_data.filter_par1 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar2, &mut state_buf)?;
        self.set_data.filter_par2 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar3, &mut state_buf)?;
        self.set_data.filter_par3 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar4, &mut state_buf)?;
        self.set_data.filter_par4 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar5, &mut state_buf)?;
        self.set_data.filter_par5 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        self.read_register(Registers::FilterPar6, &mut state_buf)?;
        self.set_data.filter_par6 = u16::from_be_bytes([state_buf[0], state_buf[1]]);
        Ok(())
    }
    /// Sets the enable pin high to power on the sensor.
    fn en_pin_set(&mut self) {
        if let Some(ref mut en_pin) = self.en_pin {
            en_pin.set_high().ok();
            /// Wait for minimum 35ms for sensor start-up and stabilisation
            self.delay_ms(35);
        }
    }
    /// Resets the enable pin to power off the sensor.
    fn en_pin_reset(&mut self) {
        if let Some(ref mut en_pin) = self.en_pin {
            en_pin.set_low().ok();
        }
    }

    /// Powers on the sensor using the optional enable pin.
    pub fn power_on(&mut self) {
        self.en_pin_set();
    }

    /// Powers off the sensor by resetting the optional enable pin.
    pub fn power_off(&mut self) {
        self.en_pin_reset();
    }

    /// Issues a start measurement command to the sensor.
    pub fn start_measurement(&mut self) -> Result<(), E> {
        self.write_register(Registers::StartMesurement, &[0x01], RAM_DELAY)
    }

    /// Performs a soft reset by writing `0xFF` to the SCR register.
    pub fn reset(&mut self) -> Result<(), E> {
        self.write_register(Registers::Scr, &[0xFF], RAM_DELAY)
    }

    /// Reads the scaled measured concentration from the sensor.
    pub fn scaled_measurement_get(&mut self) -> Result<i16, E> {
        let mut buf = [0u8; 2];
        self.read_register(Registers::ScaledMeasured, &mut buf)?;
        Ok(i16::from_be_bytes([buf[0], buf[1]]))
    }

    /// Reads the elapsed time counter (ETC) from the sensor.
    pub fn elapsed_time_get(&mut self) -> Result<u32, E> {
        let mut buf = [0u8; 4];
        self.read_register(Registers::Etc, &mut buf)?;
        Ok(u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]))
    }

    /// Retrieves the sensor's configuration values from the sensor's EEPROM registers.
    pub fn get_config(&mut self) -> Result<Config, E> {
        let mut buf = [0u8; 2];

        self.read_register(Registers::MeasurementModeEe, &mut buf)?;
        self.config.single_measurement_mode = buf[0];
        self.read_register(Registers::MeasurementPeriodEe, &mut buf)?;
        self.config.measurement_period = u16::from_be_bytes([buf[0], buf[1]]);
        self.read_register(Registers::NumberOfSamplesEe, &mut buf)?;
        self.config.number_of_samples = u16::from_be_bytes([buf[0], buf[1]]);
        self.read_register(Registers::AbcPeriodEe, &mut buf)?;
        self.config.abc_period = u16::from_be_bytes([buf[0], buf[1]]);
        self.read_register(Registers::AbcTargetEe, &mut buf)?;
        self.config.abc_target = u16::from_be_bytes([buf[0], buf[1]]);
        self.read_register(Registers::StaticIIRFilterEe, &mut buf)?;
        self.config.iir_filter = buf[0];
        self.read_register(Registers::MeterControlEe, &mut buf)?;
        self.config.meter_control = buf[0];
        self.read_register(Registers::I2cAddressEe, &mut buf)?;
        self.config.i2c_address = buf[0];
        self.read_register(Registers::NominatorEe, &mut buf)?;
        self.config.nominator = u16::from_be_bytes([buf[0], buf[1]]);
        self.read_register(Registers::DenominatorEe, &mut buf)?;
        self.config.denominator = u16::from_be_bytes([buf[0], buf[1]]);
        self.read_register(Registers::ScaleAbcTarget, &mut buf)?;
        self.config.scaled_abc_target = u16::from_be_bytes([buf[0], buf[1]]);

        let read_config = self.config.clone();

        Ok(read_config)
    }
    /// Sets the sensor's configuration values in the sensor's EEPROM registers.
    fn set_config(&mut self, config: Config) -> Result<(), E> {
        if config.single_measurement_mode != self.config.single_measurement_mode {
            self.write_register(
                Registers::MeterControlEe,
                &[config.single_measurement_mode],
                EEPROM_DELAY,
            )?;
        }
        if config.measurement_period != self.config.measurement_period {
            self.write_register(
                Registers::MeasurementPeriodEe,
                &config.measurement_period.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.abc_period != self.config.abc_period {
            self.write_register(
                Registers::AbcPeriodEe,
                &config.abc_period.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.abc_target != self.config.abc_target {
            self.write_register(
                Registers::AbcTargetEe,
                &config.abc_target.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.denominator != self.config.denominator {
            self.write_register(
                Registers::DenominatorEe,
                &config.denominator.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.nominator != self.config.nominator {
            self.write_register(
                Registers::NominatorEe,
                &config.nominator.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.number_of_samples != self.config.number_of_samples {
            self.write_register(
                Registers::NumberOfSamplesEe,
                &config.number_of_samples.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.scaled_abc_target != self.config.scaled_abc_target {
            self.write_register(
                Registers::ScaleAbcTarget,
                &config.scaled_abc_target.to_be_bytes(),
                EEPROM_DELAY,
            )?;
        }
        if config.i2c_address != self.config.i2c_address {
            self.write_register(Registers::I2cAddressEe, &[config.i2c_address], EEPROM_DELAY)?;
        }
        if config.iir_filter != self.config.iir_filter {
            self.write_register(
                Registers::StaticIIRFilterEe,
                &[config.iir_filter],
                EEPROM_DELAY,
            )?;
        }

        Ok(())
    }

    pub fn background_calibration(&mut self) -> Result<(), ErrorStatus<E>> {
        let mut buf = [0u8; 1];

        self.write_register(Registers::CalibrationStatus, &[0x00], RAM_DELAY)
            .map_err(ErrorStatus::I2c)?;

        self.write_register(Registers::CalibrationCommand, &[0x07, 0xC6], RAM_DELAY)
            .map_err(ErrorStatus::I2c)?;

        if self.config.single_measurement_mode == 0x01 {
            self.sensor_state_data_set().map_err(ErrorStatus::I2c)?;

            loop {
                if let Ok(true) = self.n_rdy_pin.is_low() {
                    break;
                }
            }
        }

        self.read_register(Registers::CalibrationStatus, &mut buf)
            .map_err(ErrorStatus::I2c)?;

        if let 0x20 = buf[0] {
            if self.config.single_measurement_mode == 0x01 {
                self.sensor_state_data_get().map_err(ErrorStatus::I2c)?;
            }
            return Ok(());
        } else {
            return Err(ErrorStatus::CalibrationError);
        }
    }

    /// Sets the target calibration value for the sensor.
    pub fn target_calibration(&mut self, value: u16) -> Result<(), E> {
        self.write_register(
            Registers::CalibrationTarget,
            &value.to_be_bytes(),
            RAM_DELAY,
        )?;

        Ok(())
    }

    /// Initiates a CO2 measurement and retrieves the result.
    pub fn init(&mut self, config_sensor: Option<Config>) -> Result<(), E> {
        ///set sensor state data
        self.sensor_state_data_get()?;
        ///
        self.product_type_get()?;
        self.get_config()?;
        ///if config is none
        if let Some(config) = config_sensor {
            self.set_config(config)?;
        }

        Ok(())
    }

    /// Retrieves a CO2 measurement from the sensor.
    pub fn co2_measurement_get(
        &mut self,
        pressure: Option<u16>,
    ) -> Result<&Measurement, ErrorStatus<E>> {
        let mut buf = [0u8; 2];

        self.clear_error_status().map_err(ErrorStatus::I2c)?;
        self.sensor_state_data_set().map_err(ErrorStatus::I2c)?;

        if let Some(value) = pressure {
            self.write_register(Registers::PressureValue, &value.to_be_bytes(), RAM_DELAY)
                .map_err(ErrorStatus::I2c)?;
        }

        loop {
            if let Ok(true) = self.n_rdy_pin.is_low() {
                break;
            }
        }
        // self.delay_ms(2400);

        let sensor_err = u16::from_be_bytes([buf[0], buf[1]]);

        if let Some(err) = self.check_sensor_error(sensor_err) {
            return Err(err);
        }

        self.read_register(Registers::ErrorStatus, &mut buf[..2])
            .map_err(ErrorStatus::I2c)?;

        self.read_register(Registers::MeasuredFilteredPc, &mut buf)
            .map_err(ErrorStatus::I2c)?;
        self.mesurement.measured_filtered_press_comp = i16::from_be_bytes([buf[0], buf[1]]);

        self.read_register(Registers::Temperature, &mut buf)
            .map_err(ErrorStatus::I2c)?;
        self.mesurement.temperature = i16::from_be_bytes([buf[0], buf[1]]);

        self.read_register(Registers::MeasurementCount, &mut buf)
            .map_err(ErrorStatus::I2c)?;
        self.mesurement.measurement_count = buf[0];

        self.read_register(Registers::MeasurCycleTime, &mut buf)
            .map_err(ErrorStatus::I2c)?;

        self.mesurement.measurement_cycle_time = u16::from_be_bytes([buf[0], buf[1]]);

        self.read_register(Registers::MeasUnfPressCompens, &mut buf)
            .map_err(ErrorStatus::I2c)?;

        self.mesurement.measured_unfiltered_press_comp = i16::from_be_bytes([buf[0], buf[1]]);

        self.read_register(Registers::MeasFilPressCompens, &mut buf)
            .map_err(ErrorStatus::I2c)?;

        self.mesurement.measured_filtered = i16::from_be_bytes([buf[0], buf[1]]);

        self.read_register(Registers::MeasuredUnfiltered, &mut buf)
            .map_err(ErrorStatus::I2c)?;

        self.mesurement.measured_unfiltered = i16::from_be_bytes([buf[0], buf[1]]);

        // Read scaled measured concentration if available
        self.read_register(Registers::ScaledMeasured, &mut buf)
            .map_err(ErrorStatus::I2c)?;
        self.mesurement.scaled_measured = i16::from_be_bytes([buf[0], buf[1]]);

        // Read elapsed time counter
        let mut etc_buf = [0u8; 4];
        self.read_register(Registers::Etc, &mut etc_buf)
            .map_err(ErrorStatus::I2c)?;
        self.mesurement.etc =
            u32::from_be_bytes([etc_buf[0], etc_buf[1], etc_buf[2], etc_buf[3]]);

        self.sensor_state_data_get().map_err(ErrorStatus::I2c)?;

        Ok(&self.mesurement)
    }
}
