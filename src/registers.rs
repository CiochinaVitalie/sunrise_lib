#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Registers {
    ErrorStatus = 0x00,
    MeasuredFilteredPc = 0x06,
    Temperature = 0x08,
    MeasurementCount = 0x0D,
    MeasurCycleTime = 0x0E,
    MeasUnfPressCompens = 0x10,
    MeasFilPressCompens = 0x12,
    MeasuredUnfiltered = 0x14,
    FirmwareType = 0x2F,
    FirmwareVer = 0x38,
    SensorId = 0x3A,
    ProductCode = 0x70,
    CalibrationStatus = 0x81,
    CalibrationCommand = 0x82,
    CalibrationTarget = 0x84,
    MeasurementModeEe = 0x95,
    MeasurementPeriodEe = 0x96,
    NumberOfSamplesEe = 0x98,
    AbcPeriodEe = 0x9A,
    AbcTargetEe = 0x9E,
    StaticIIRFilterEe = 0xA1,
    MeterControlEe = 0xA5,
    I2cAddressEe = 0xA7,
    NominatorEe = 0xA8,
    DenominatorEe = 0xAA,
    ScaleAbcTarget = 0xB0,
    StartMesurement = 0xC3,
    PressureValue = 0xDC,
    AbcTime = 0xC4,
    AbcPar0 = 0xC6,
    AbcPar1 = 0xC8,
    AbcPar2 = 0xCA,
    AbcPar3 = 0xCC,
    FilterPar0 = 0xCE,
    FilterPar1 = 0xD0,
    FilterPar2 = 0xD2,
    FilterPar3 = 0xD4,
    FilterPar4 = 0xD6,
    FilterPar5 = 0xD8,
    FilterPar6 = 0xDA,
    ClearErrorStatus = 0x9D,
}

#[derive(Debug)]
pub enum ErrorStatus<E> {
    I2c(E),
    LowInternalRegulatedVoltage,
    MeasurementTimeout,
    AbnormalSignalLevel,
    ScaleFactorError,
    FatalError,
    I2cError,
    AlgoritmError,
    CalibrationError,
    SelfDiagnosticsError,
    OutOfRange,
    MemoryError,
    NoMeasurementCompleted,
}

impl<E> ErrorStatus<E> {
    /// Create an [`ErrorStatus`] from the raw sensor error bits.
    pub fn from_bits(bits: u16) -> Option<Self> {
        match bits {
            x if x & (1 << 15) != 0 => Some(ErrorStatus::LowInternalRegulatedVoltage),
            x if x & (1 << 14) != 0 => Some(ErrorStatus::MeasurementTimeout),
            x if x & (1 << 13) != 0 => Some(ErrorStatus::AbnormalSignalLevel),
            x if x & (1 << 8) != 0 => Some(ErrorStatus::ScaleFactorError),
            x if x & (1 << 7) != 0 => Some(ErrorStatus::FatalError),
            x if x & (1 << 6) != 0 => Some(ErrorStatus::I2cError),
            x if x & (1 << 5) != 0 => Some(ErrorStatus::AlgoritmError),
            x if x & (1 << 4) != 0 => Some(ErrorStatus::CalibrationError),
            x if x & (1 << 3) != 0 => Some(ErrorStatus::SelfDiagnosticsError),
            x if x & (1 << 2) != 0 => Some(ErrorStatus::OutOfRange),
            x if x & (1 << 1) != 0 => Some(ErrorStatus::MemoryError),
            x if x & (1 << 0) != 0 => Some(ErrorStatus::NoMeasurementCompleted),
            _ => None,
        }
    }
}
