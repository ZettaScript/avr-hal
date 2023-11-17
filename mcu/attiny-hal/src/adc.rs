#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter

use crate::port;
pub use avr_hal_generic::adc::{AdcChannel, AdcOps, ClockDivider};

/// Select the voltage reference for the ADC peripheral
///
/// The internal voltage reference options may not be used if an external reference voltage is
/// being applied to the AREF pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReferenceVoltage {
    /// Voltage applied to AREF pin.
    #[cfg(any(
        feature = "attiny85",
        feature = "attiny167",
    ))]
    Aref,
    /// Default reference voltage (default).
    AVcc,
    /// Internal 0.55V reference.
    #[cfg(feature = "attiny402")]
    Internal0_55,
    /// Internal 1.1V reference.
    Internal1_1,
    /// Internal 1.5V reference.
    #[cfg(feature = "attiny402")]
    Internal1_5,
    /// Internal 2.5V reference.
    #[cfg(feature = "attiny402")]
    Internal2_5,
    /// Internal 2.56V reference.
    #[cfg(any(
        feature = "attiny85",
        feature = "attiny167",
    ))]
    Internal2_56,
    /// Internal 4.3V reference.
    #[cfg(feature = "attiny402")]
    Internal4_3,
}

impl Default for ReferenceVoltage {
    fn default() -> Self {
        Self::AVcc
    }
}

/// Configuration for the ADC peripheral.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdcSettings {
    pub clock_divider: ClockDivider,
    pub ref_voltage: ReferenceVoltage,
}

/// Check the [`avr_hal_generic::adc::Adc`] documentation.
#[cfg(not(feature = "attiny402"))]
pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<crate::Attiny, crate::pac::ADC, CLOCK>;

/// Check the [`avr_hal_generic::adc::Adc`] documentation.
#[cfg(feature = "attiny402")]
pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<crate::Attiny, crate::pac::ADC0, CLOCK>;

/// Check the [`avr_hal_generic::adc::Channel`] documentation.
#[cfg(not(feature = "attiny402"))]
pub type Channel = avr_hal_generic::adc::Channel<crate::Attiny, crate::pac::ADC>;

/// Check the [`avr_hal_generic::adc::Channel`] documentation.
#[cfg(feature = "attiny402")]
pub type Channel = avr_hal_generic::adc::Channel<crate::Attiny, crate::pac::ADC0>;

/// Additional channels
///
/// Some channels are not directly connected to pins.  This module provides types which can be used
/// to access them.
///
/// # Example
/// ```
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let mut adc = atmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let value = adc.read_blocking(&channel::Vbg);
/// ```
pub mod channel {
    #[cfg(feature = "attiny167")]
    pub struct AVcc_4;
    pub struct Vbg;
    pub struct Gnd;
    pub struct Temperature;
    #[cfg(feature = "attiny402")]
    pub struct InternalReference;
}

#[cfg(not(feature = "attiny402"))]
fn apply_clock(peripheral: &crate::pac::ADC, settings: AdcSettings) {
    peripheral.adcsra.write(|w| {
        w.aden().set_bit();
        match settings.clock_divider {
            ClockDivider::Factor2 => w.adps().prescaler_2(),
            ClockDivider::Factor4 => w.adps().prescaler_4(),
            ClockDivider::Factor8 => w.adps().prescaler_8(),
            ClockDivider::Factor16 => w.adps().prescaler_16(),
            ClockDivider::Factor32 => w.adps().prescaler_32(),
            ClockDivider::Factor64 => w.adps().prescaler_64(),
            ClockDivider::Factor128 => w.adps().prescaler_128(),
        }
    });
}

#[cfg(feature = "attiny402")]
fn apply_clock(peripheral: &crate::pac::ADC0, settings: AdcSettings) {
    peripheral.ctrlc.write(|w| {
        match settings.clock_divider {
            ClockDivider::Factor2 => w.presc().div2(),
            ClockDivider::Factor4 => w.presc().div4(),
            ClockDivider::Factor8 => w.presc().div8(),
            ClockDivider::Factor16 => w.presc().div16(),
            ClockDivider::Factor32 => w.presc().div32(),
            ClockDivider::Factor64 => w.presc().div64(),
            ClockDivider::Factor128 => w.presc().div128(),
        }
    });
}


#[cfg(feature = "attiny85")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        peripheral.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refs().aref(),
            ReferenceVoltage::AVcc => w.refs().vcc(),
            ReferenceVoltage::Internal1_1 => w.refs().internal().refs2().clear_bit(),
            ReferenceVoltage::Internal2_56 => w.refs().internal().refs2().set_bit(),
        });
    },
    channel_id: crate::pac::adc::admux::MUX_A,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().variant(id));
    },
    pins: {
        port::PB5: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        port::PB2: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        port::PB4: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        port::PB3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
    },
    channels: {
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}


#[cfg(feature = "attiny88")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        peripheral.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::AVcc => w.refs0().avcc(),
            ReferenceVoltage::Internal1_1 => w.refs0().internal(),
        });
    },
    channel_id: crate::pac::adc::admux::MUX_A,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().variant(id));
    },
    pins: {
        port::PC0: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        port::PC1: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        port::PC2: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        port::PC3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        port::PC4: (crate::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        port::PC5: (crate::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        port::PA0: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
        port::PA1: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
    },
    channels: {
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}


#[cfg(feature = "attiny167")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        peripheral.amiscr.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.arefen().set_bit(),
            _ => w.arefen().clear_bit(),
        });
        peripheral.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refs().avcc(),
            ReferenceVoltage::AVcc => w.refs().avcc(),
            ReferenceVoltage::Internal1_1 => w.refs().internal_11(),
            ReferenceVoltage::Internal2_56 => w.refs().internal_256(),
        });
    },
    channel_id: crate::pac::adc::admux::MUX_A,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().variant(id));
    },
    pins: {
        port::PA0: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        port::PA1: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        port::PA2: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        port::PA3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        port::PA4: (crate::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        port::PA5: (crate::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        port::PA6: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
        port::PA7: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        port::PB5: (crate::pac::adc::admux::MUX_A::ADC8, didr1::adc8d),
        port::PB6: (crate::pac::adc::admux::MUX_A::ADC9, didr1::adc9d),
        port::PB7: (crate::pac::adc::admux::MUX_A::ADC10, didr1::adc10d),
    },
    channels: {
        channel::AVcc_4: crate::pac::adc::admux::MUX_A::ADC_AVCC_4,
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}


#[cfg(feature = "attiny402")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC0,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        if settings.ref_voltage == ReferenceVoltage::AVcc {
            peripheral.ctrlc.write(|w| w.refsel().vddref());
        } else {
            peripheral.VREF.ctrla.write(|w| match settings.ref_voltage {
                ReferenceVoltage::AVcc => unreachable!(),
                ReferenceVoltage::Internal0_55 => w.adc0refsel()._0v55(),
                ReferenceVoltage::Internal1_1 => w.adc0refsel()._1v1(),
                ReferenceVoltage::Internal1_5 => w.adc0refsel()._1v5(),
                ReferenceVoltage::Internal2_5 => w.adc0refsel()._2v5(),
                ReferenceVoltage::Internal4_3 => w.adc0refsel()._4v34(),
            });
            peripheral.ctrlc.write(|w| w.refsel().intref());
        }
    },
    channel_id: crate::pac::adc0::muxpos::MUXPOS_A,
    set_channel: |peripheral, id| {
        peripheral.muxpos.modify(|_, w| w.muxpos().variant(id));
    },
    pins: {
        port::PA0: (crate::pac::adc0::muxpos::MUXPOS_A::AIN0),
        port::PA1: (crate::pac::adc0::muxpos::MUXPOS_A::AIN1),
        port::PA2: (crate::pac::adc0::muxpos::MUXPOS_A::AIN2),
        port::PA3: (crate::pac::adc0::muxpos::MUXPOS_A::AIN3),
        port::PA6: (crate::pac::adc0::muxpos::MUXPOS_A::AIN6),
        port::PA7: (crate::pac::adc0::muxpos::MUXPOS_A::AIN7),
    },
    channels: {
        channel::Gnd: crate::pac::adc0::muxpos::MUXPOS_A::GND,
        channel::Temperature: crate::pac::adc0::muxpos::MUXPOS_A::TEMPSENSE,
        channel::InternalReference: crate::pac::adc0::muxpos::MUXPOS_A::INTREF,
    },
}
