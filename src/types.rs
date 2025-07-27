use heapless::String;

#[cfg(feature = "defmt")]
use defmt::*;

#[derive(Default, Debug)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProductType {
    pub firmware_type: u8,
    pub main_revision: u8,
    pub sub_revision: u8,
    pub sensor_id: u32,
    pub product_code: String<16>,
}

#[derive(Default, Clone, Debug)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub single_measurement_mode: u8,
    pub measurement_period: u16,
    pub number_of_samples: u16,
    pub abc_period: u16,
    pub abc_target: u16,
    pub iir_filter: u8,
    pub meter_control: u8,
    pub i2c_address: u8,
    pub nominator: u16,
    pub denominator: u16,
    pub scaled_abc_target: u16,
}

#[derive(Debug, Default)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetData {
    pub abc_time: u16,
    pub abc_par0: u16,
    pub abc_par1: u16,
    pub abc_par2: u16,
    pub abc_par3: u16,
    pub filter_par0: u16,
    pub filter_par1: u16,
    pub filter_par2: u16,
    pub filter_par3: u16,
    pub filter_par4: u16,
    pub filter_par5: u16,
    pub filter_par6: u16,
}

#[derive(Clone, Debug, Default)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Measurement {
    pub measured_filtered_press_comp: i16,
    pub temperature: i16,
    pub measurement_count: u8,
    pub measurement_cycle_time: u16,
    pub measured_unfiltered_press_comp: i16,
    pub measured_filtered: i16,
    pub measured_unfiltered: i16,
    pub scaled_measured: i16,
    pub etc: u32,
}
