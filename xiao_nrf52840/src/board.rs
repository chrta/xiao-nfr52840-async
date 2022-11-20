use embassy_nrf::{
    gpio::{AnyPin, Level, Output, OutputDrive, Pin},
    peripherals::{
        P0_02, P0_03, P0_04, P0_05, P0_13, P0_14, P0_17, P0_28, P0_29, P0_31,
        P1_11, P1_12, P1_13, P1_14, P1_15, PPI_CH0, PPI_CH1, PWM0, RNG, SAADC, TIMER0, TWISPI0,
        UARTE0,
    },
};

pub use embassy_nrf::{config::Config, interrupt::Priority};

/// LED
pub type Led = Output<'static, AnyPin>;

/// Represents all the peripherals and pins available for the XIAO nRF52840.
pub struct XiaoNrf52840 {
    /// UART0 peripheral
    pub uarte0: UARTE0,
    /// TIMER0 peripheral
    pub timer0: TIMER0,

    /// D0 pin
    pub d0: P0_02,
    /// D1 pin
    pub d1: P0_03,
    /// D2 pin
    pub d2: P0_28,
    /// D3 pin
    pub d3: P0_29,
    /// D4 pin
    pub d4: P0_04,
    /// D5 pin
    pub d5: P0_05,
    /// D6 pin
    pub d6: P1_11,
    /// D7 pin
    pub d7: P1_12,
    /// D8 pin
    pub d8: P1_13,
    /// D9 pin
    pub d9: P1_14,
    /// D10 pin
    pub d10: P1_15,

    /// RGB LED red channel
    pub led_red: Led,
    /// RGB LED green channel
    pub led_green: Led,
    /// RGB LED blue channel
    pub led_blue: Led,

    /// Only enable output and set to low to enable reading battery voltage
    pub read_bat: P0_14,
    /// Connected to voltage divider of battery voltage (between 1M to 510k R)
    pub v_bat: P0_31,

    /// BQ25100 high charge
    pub bq25100_hichg: P0_13,
    /// BQ25100 no charge
    pub bq25100_not_chg: P0_17,

    /// SPI/I2C peripheral
    pub twispi0: TWISPI0,
    /// PWM0 peripheral
    pub pwm0: PWM0,
    /// PPI channel 0
    pub ppi_ch0: PPI_CH0,
    /// PPI channel 1
    pub ppi_ch1: PPI_CH1,
    /// Random number generator
    pub rng: RNG,
    /// Analog digital converter
    pub saadc: SAADC,
}

impl Default for XiaoNrf52840 {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl XiaoNrf52840 {
    /// Create a new instance based on HAL configuration
    pub fn new(config: embassy_nrf::config::Config) -> Self {
        let p = embassy_nrf::init(config);

        Self {
            uarte0: p.UARTE0,
            timer0: p.TIMER0,
            d0: p.P0_02,
            d1: p.P0_03,
            d2: p.P0_28,
            d3: p.P0_29,
            d4: p.P0_04,
            d5: p.P0_05,
            d6: p.P1_11,
            d7: p.P1_12,
            d8: p.P1_13,
            d9: p.P1_14,
            d10: p.P1_15,

            led_red: Output::new(p.P0_26.degrade(), Level::Low, OutputDrive::Standard),
            led_green: Output::new(p.P0_30.degrade(), Level::Low, OutputDrive::Standard),
            led_blue: Output::new(p.P0_06.degrade(), Level::Low, OutputDrive::Standard),

            read_bat: p.P0_14,
            v_bat: p.P0_31,

            bq25100_hichg: p.P0_13,
            bq25100_not_chg: p.P0_17,

            ppi_ch0: p.PPI_CH0,
            ppi_ch1: p.PPI_CH1,
            twispi0: p.TWISPI0,
            pwm0: p.PWM0,
            rng: p.RNG,
            saadc: p.SAADC,
        }
    }
}
