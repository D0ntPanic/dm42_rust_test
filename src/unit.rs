use crate::error::{Error, Result};
use crate::functions::Function;
use crate::layout::Layout;
use crate::menu::{Menu, MenuItem, MenuItemFunction};
use crate::number::{Number, ToNumber};
use crate::screen::Screen;
use crate::state::State;
use crate::storage::{DeserializeInput, SerializeOutput, StorageObject, StorageRefSerializer};
use crate::value::Value;
use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use intel_dfp::Decimal;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AngleUnit {
	Degrees,
	Radians,
	Gradians,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AreaUnit {
	Hectares,
	Acres,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DistanceUnit {
	Nanometers,
	Micrometers,
	Millimeters,
	Centimeters,
	Meters,
	Kilometers,
	Inches,
	Feet,
	Yards,
	Miles,
	NauticalMiles,
	AstronomicalUnits,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EnergyUnit {
	Joules,
	Millijoules,
	Kilojoules,
	Megajoules,
	Calories,
	Kilocalories,
	BTU,
	FootPounds,
	FootPoundals,
	WattHours,
	KilowattHours,
	Erg,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ForceUnit {
	Newton,
	Kilonewton,
	Dyne,
	KilogramForce,
	PoundForce,
	Poundal,
	Kip,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MassUnit {
	Grams,
	Milligrams,
	Kilograms,
	MetricTons,
	Pounds,
	Ounces,
	Stones,
	Tons,
	UKTons,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PowerUnit {
	Watts,
	Milliwatts,
	Kilowatts,
	Megawatts,
	Gigawatts,
	MechanicalHorsepower,
	MetricHorsepower,
	ElectricalHorsepower,
	TonsOfRefrigeration,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PressureUnit {
	Pascals,
	Kilopascals,
	Bars,
	Millibars,
	Atmospheres,
	InchesOfMercury,
	MillimetersOfMercury,
	InchesOfWater,
	MillimetersOfWater,
	PoundsPerSquareInch,
	Torr,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TemperatureUnit {
	Celsius,
	Fahrenheit,
	Kelvin,
	Rankine,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TimeUnit {
	Nanoseconds,
	Microseconds,
	Milliseconds,
	Seconds,
	Minutes,
	Hours,
	Days,
	Years,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VolumeUnit {
	Litre,
	Millilitre,
	Gallons,
	Quarts,
	Pints,
	Cups,
	FluidOunces,
	ImperialGallons,
	ImperialQuarts,
	ImperialPints,
	ImperialOunces,
	Tablespoons,
	Teaspoons,
	UKTablespoons,
	UKTeaspoons,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Unit {
	Angle(AngleUnit),
	Area(AreaUnit),
	Distance(DistanceUnit),
	Energy(EnergyUnit),
	Force(ForceUnit),
	Mass(MassUnit),
	Power(PowerUnit),
	Pressure(PressureUnit),
	Temperature(TemperatureUnit),
	Time(TimeUnit),
	Volume(VolumeUnit),
}

impl AngleUnit {
	pub fn to_str(&self) -> String {
		match self {
			AngleUnit::Degrees => "°".to_string(),
			AngleUnit::Radians => "rad".to_string(),
			AngleUnit::Gradians => "grad".to_string(),
		}
	}
}

impl AreaUnit {
	pub fn to_str(&self) -> String {
		match self {
			AreaUnit::Hectares => "ha".to_string(),
			AreaUnit::Acres => "acre".to_string(),
		}
	}

	fn to_square_meters(&self, value: &Number) -> Number {
		value * &self.multiplier_to_standard() * 10_000.to_number()
	}

	fn from_square_meters(&self, value: &Number) -> Number {
		value / &(self.multiplier_to_standard() * 10_000.to_number())
	}

	fn to_square_meters_with_power(&self, value: &Number, power: i32) -> Number {
		if power < 0 {
			let mut result = value.clone();
			for _ in 0..-power {
				result = self.from_square_meters(&result);
			}
			result
		} else if power > 0 {
			let mut result = value.clone();
			for _ in 0..power {
				result = self.to_square_meters(&result);
			}
			result
		} else {
			value.clone()
		}
	}

	fn from_square_meters_with_power(&self, value: &Number, power: i32) -> Number {
		if power < 0 {
			let mut result = value.clone();
			for _ in 0..-power {
				result = self.to_square_meters(&result);
			}
			result
		} else if power > 0 {
			let mut result = value.clone();
			for _ in 0..power {
				result = self.from_square_meters(&result);
			}
			result
		} else {
			value.clone()
		}
	}
}

impl DistanceUnit {
	pub fn to_str(&self) -> String {
		match self {
			DistanceUnit::Nanometers => "nm".to_string(),
			DistanceUnit::Micrometers => "μm".to_string(),
			DistanceUnit::Millimeters => "mm".to_string(),
			DistanceUnit::Centimeters => "cm".to_string(),
			DistanceUnit::Meters => "m".to_string(),
			DistanceUnit::Kilometers => "km".to_string(),
			DistanceUnit::Inches => "in".to_string(),
			DistanceUnit::Feet => "ft".to_string(),
			DistanceUnit::Yards => "yd".to_string(),
			DistanceUnit::Miles => "mi".to_string(),
			DistanceUnit::NauticalMiles => "nmi".to_string(),
			DistanceUnit::AstronomicalUnits => "au".to_string(),
		}
	}
}

impl EnergyUnit {
	pub fn to_str(&self) -> String {
		match self {
			EnergyUnit::Joules => "J".to_string(),
			EnergyUnit::Millijoules => "mJ".to_string(),
			EnergyUnit::Kilojoules => "kJ".to_string(),
			EnergyUnit::Megajoules => "MJ".to_string(),
			EnergyUnit::Calories => "cal".to_string(),
			EnergyUnit::Kilocalories => "kcal".to_string(),
			EnergyUnit::BTU => "BTU".to_string(),
			EnergyUnit::FootPounds => "ftlbf".to_string(),
			EnergyUnit::FootPoundals => "ftpdl".to_string(),
			EnergyUnit::WattHours => "Wh".to_string(),
			EnergyUnit::KilowattHours => "kWh".to_string(),
			EnergyUnit::Erg => "erg".to_string(),
		}
	}
}

impl ForceUnit {
	pub fn to_str(&self) -> String {
		match self {
			ForceUnit::Newton => "N".to_string(),
			ForceUnit::Kilonewton => "kN".to_string(),
			ForceUnit::Dyne => "dyn".to_string(),
			ForceUnit::KilogramForce => "kgf".to_string(),
			ForceUnit::PoundForce => "lbf".to_string(),
			ForceUnit::Poundal => "pdl".to_string(),
			ForceUnit::Kip => "kip".to_string(),
		}
	}
}

impl MassUnit {
	pub fn to_str(&self) -> String {
		match self {
			MassUnit::Grams => "g".to_string(),
			MassUnit::Milligrams => "mg".to_string(),
			MassUnit::Kilograms => "kg".to_string(),
			MassUnit::MetricTons => "t".to_string(),
			MassUnit::Pounds => "lb".to_string(),
			MassUnit::Ounces => "oz".to_string(),
			MassUnit::Stones => "st".to_string(),
			MassUnit::Tons => "ton".to_string(),
			MassUnit::UKTons => "UK ton".to_string(),
		}
	}
}

impl PowerUnit {
	pub fn to_str(&self) -> String {
		match self {
			PowerUnit::Watts => "W".to_string(),
			PowerUnit::Milliwatts => "mW".to_string(),
			PowerUnit::Kilowatts => "kW".to_string(),
			PowerUnit::Megawatts => "MW".to_string(),
			PowerUnit::Gigawatts => "GW".to_string(),
			PowerUnit::MechanicalHorsepower => "hp".to_string(),
			PowerUnit::MetricHorsepower => "hpM".to_string(),
			PowerUnit::ElectricalHorsepower => "hpE".to_string(),
			PowerUnit::TonsOfRefrigeration => "RT".to_string(),
		}
	}
}

impl PressureUnit {
	pub fn to_str(&self) -> String {
		match self {
			PressureUnit::Pascals => "Pa".to_string(),
			PressureUnit::Kilopascals => "kPa".to_string(),
			PressureUnit::Bars => "bar".to_string(),
			PressureUnit::Millibars => "mbar".to_string(),
			PressureUnit::Atmospheres => "atm".to_string(),
			PressureUnit::InchesOfMercury => "inHg".to_string(),
			PressureUnit::MillimetersOfMercury => "mmHg".to_string(),
			PressureUnit::InchesOfWater => "inH₂O".to_string(),
			PressureUnit::MillimetersOfWater => "mmH₂O".to_string(),
			PressureUnit::PoundsPerSquareInch => "psi".to_string(),
			PressureUnit::Torr => "Torr".to_string(),
		}
	}
}

impl TemperatureUnit {
	pub fn to_str(&self) -> String {
		match self {
			TemperatureUnit::Celsius => "°C".to_string(),
			TemperatureUnit::Fahrenheit => "°F".to_string(),
			TemperatureUnit::Kelvin => "K".to_string(),
			TemperatureUnit::Rankine => "°R".to_string(),
		}
	}

	fn to_celsius<'a>(&self, value: &'a Number) -> Cow<'a, Number> {
		match self {
			TemperatureUnit::Celsius => Cow::Borrowed(value),
			TemperatureUnit::Fahrenheit => {
				Cow::Owned((value - &32.to_number()) * 5.to_number() / 9.to_number())
			}
			TemperatureUnit::Kelvin => Cow::Owned(value - &(5463.to_number() / 20.to_number())),
			TemperatureUnit::Rankine => Cow::Owned(
				(value - &(49_167.to_number() / 100.to_number())) * 5.to_number() / 9.to_number(),
			),
		}
	}

	fn from_celsius<'a>(&self, value: &'a Number) -> Cow<'a, Number> {
		match self {
			TemperatureUnit::Celsius => Cow::Borrowed(value),
			TemperatureUnit::Fahrenheit => {
				Cow::Owned((value * &9.to_number() / 5.to_number()) + 32.to_number())
			}
			TemperatureUnit::Kelvin => Cow::Owned(value + &(5463.to_number() / 20.to_number())),
			TemperatureUnit::Rankine => Cow::Owned(
				(value * &9.to_number() / 5.to_number()) + (49_167.to_number() / 100.to_number()),
			),
		}
	}
}

impl TimeUnit {
	pub fn to_str(&self) -> String {
		match self {
			TimeUnit::Nanoseconds => "ns".to_string(),
			TimeUnit::Microseconds => "μs".to_string(),
			TimeUnit::Milliseconds => "ms".to_string(),
			TimeUnit::Seconds => "sec".to_string(),
			TimeUnit::Minutes => "min".to_string(),
			TimeUnit::Hours => "hr".to_string(),
			TimeUnit::Days => "day".to_string(),
			TimeUnit::Years => "yr".to_string(),
		}
	}
}

impl VolumeUnit {
	pub fn to_str(&self) -> String {
		match self {
			VolumeUnit::Litre => "L".to_string(),
			VolumeUnit::Millilitre => "mL".to_string(),
			VolumeUnit::Gallons => "gal".to_string(),
			VolumeUnit::Quarts => "qt".to_string(),
			VolumeUnit::Pints => "pt".to_string(),
			VolumeUnit::Cups => "cup".to_string(),
			VolumeUnit::FluidOunces => "fl oz".to_string(),
			VolumeUnit::ImperialGallons => "UK gal".to_string(),
			VolumeUnit::ImperialQuarts => "UK qt".to_string(),
			VolumeUnit::ImperialPints => "UK pt".to_string(),
			VolumeUnit::ImperialOunces => "UK oz".to_string(),
			VolumeUnit::Tablespoons => "tbsp".to_string(),
			VolumeUnit::Teaspoons => "tsp".to_string(),
			VolumeUnit::UKTablespoons => "UK tbsp".to_string(),
			VolumeUnit::UKTeaspoons => "UK tsp".to_string(),
		}
	}

	fn to_cubic_meters(&self, value: &Number) -> Number {
		value * &self.multiplier_to_standard() / 1000.to_number()
	}

	fn from_cubic_meters(&self, value: &Number) -> Number {
		value / &(self.multiplier_to_standard() / 1000.to_number())
	}

	fn to_cubic_meters_with_power(&self, value: &Number, power: i32) -> Number {
		if power < 0 {
			let mut result = value.clone();
			for _ in 0..-power {
				result = self.from_cubic_meters(&result);
			}
			result
		} else if power > 0 {
			let mut result = value.clone();
			for _ in 0..power {
				result = self.to_cubic_meters(&result);
			}
			result
		} else {
			value.clone()
		}
	}

	fn from_cubic_meters_with_power(&self, value: &Number, power: i32) -> Number {
		if power < 0 {
			let mut result = value.clone();
			for _ in 0..-power {
				result = self.to_cubic_meters(&result);
			}
			result
		} else if power > 0 {
			let mut result = value.clone();
			for _ in 0..power {
				result = self.from_cubic_meters(&result);
			}
			result
		} else {
			value.clone()
		}
	}
}

impl Unit {
	pub fn to_str(&self) -> String {
		match self {
			Unit::Angle(unit) => unit.to_str(),
			Unit::Area(unit) => unit.to_str(),
			Unit::Distance(unit) => unit.to_str(),
			Unit::Energy(unit) => unit.to_str(),
			Unit::Force(unit) => unit.to_str(),
			Unit::Mass(unit) => unit.to_str(),
			Unit::Power(unit) => unit.to_str(),
			Unit::Pressure(unit) => unit.to_str(),
			Unit::Temperature(unit) => unit.to_str(),
			Unit::Time(unit) => unit.to_str(),
			Unit::Volume(unit) => unit.to_str(),
		}
	}

	pub fn to_u16(&self) -> u16 {
		match self {
			Unit::Angle(AngleUnit::Degrees) => 0x0000,
			Unit::Angle(AngleUnit::Radians) => 0x0001,
			Unit::Angle(AngleUnit::Gradians) => 0x0002,
			Unit::Area(AreaUnit::Hectares) => 0x0100,
			Unit::Area(AreaUnit::Acres) => 0x0101,
			Unit::Distance(DistanceUnit::Nanometers) => 0x0200,
			Unit::Distance(DistanceUnit::Micrometers) => 0x0201,
			Unit::Distance(DistanceUnit::Millimeters) => 0x0202,
			Unit::Distance(DistanceUnit::Centimeters) => 0x0203,
			Unit::Distance(DistanceUnit::Meters) => 0x0204,
			Unit::Distance(DistanceUnit::Kilometers) => 0x0205,
			Unit::Distance(DistanceUnit::Inches) => 0x0210,
			Unit::Distance(DistanceUnit::Feet) => 0x0211,
			Unit::Distance(DistanceUnit::Yards) => 0x0212,
			Unit::Distance(DistanceUnit::Miles) => 0x0213,
			Unit::Distance(DistanceUnit::NauticalMiles) => 0x0214,
			Unit::Distance(DistanceUnit::AstronomicalUnits) => 0x0220,
			Unit::Energy(EnergyUnit::Joules) => 0x0300,
			Unit::Energy(EnergyUnit::Millijoules) => 0x0301,
			Unit::Energy(EnergyUnit::Kilojoules) => 0x0302,
			Unit::Energy(EnergyUnit::Megajoules) => 0x0303,
			Unit::Energy(EnergyUnit::Calories) => 0x0304,
			Unit::Energy(EnergyUnit::Kilocalories) => 0x0305,
			Unit::Energy(EnergyUnit::BTU) => 0x0306,
			Unit::Energy(EnergyUnit::FootPounds) => 0x0307,
			Unit::Energy(EnergyUnit::FootPoundals) => 0x0308,
			Unit::Energy(EnergyUnit::WattHours) => 0x0309,
			Unit::Energy(EnergyUnit::KilowattHours) => 0x030a,
			Unit::Energy(EnergyUnit::Erg) => 0x030b,
			Unit::Force(ForceUnit::Newton) => 0x0400,
			Unit::Force(ForceUnit::Kilonewton) => 0x0401,
			Unit::Force(ForceUnit::Dyne) => 0x0402,
			Unit::Force(ForceUnit::KilogramForce) => 0x0403,
			Unit::Force(ForceUnit::PoundForce) => 0x0404,
			Unit::Force(ForceUnit::Poundal) => 0x0405,
			Unit::Force(ForceUnit::Kip) => 0x0406,
			Unit::Mass(MassUnit::Grams) => 0x0500,
			Unit::Mass(MassUnit::Milligrams) => 0x0501,
			Unit::Mass(MassUnit::Kilograms) => 0x0502,
			Unit::Mass(MassUnit::MetricTons) => 0x0503,
			Unit::Mass(MassUnit::Pounds) => 0x0504,
			Unit::Mass(MassUnit::Ounces) => 0x0505,
			Unit::Mass(MassUnit::Stones) => 0x0506,
			Unit::Mass(MassUnit::Tons) => 0x0507,
			Unit::Mass(MassUnit::UKTons) => 0x0508,
			Unit::Power(PowerUnit::Watts) => 0x0600,
			Unit::Power(PowerUnit::Milliwatts) => 0x0601,
			Unit::Power(PowerUnit::Kilowatts) => 0x0602,
			Unit::Power(PowerUnit::Megawatts) => 0x0603,
			Unit::Power(PowerUnit::Gigawatts) => 0x0604,
			Unit::Power(PowerUnit::MechanicalHorsepower) => 0x0605,
			Unit::Power(PowerUnit::MetricHorsepower) => 0x0606,
			Unit::Power(PowerUnit::ElectricalHorsepower) => 0x0607,
			Unit::Power(PowerUnit::TonsOfRefrigeration) => 0x0608,
			Unit::Pressure(PressureUnit::Pascals) => 0x0700,
			Unit::Pressure(PressureUnit::Kilopascals) => 0x0701,
			Unit::Pressure(PressureUnit::Bars) => 0x0702,
			Unit::Pressure(PressureUnit::Millibars) => 0x0703,
			Unit::Pressure(PressureUnit::Atmospheres) => 0x0704,
			Unit::Pressure(PressureUnit::InchesOfMercury) => 0x0705,
			Unit::Pressure(PressureUnit::MillimetersOfMercury) => 0x0706,
			Unit::Pressure(PressureUnit::InchesOfWater) => 0x0707,
			Unit::Pressure(PressureUnit::MillimetersOfWater) => 0x0708,
			Unit::Pressure(PressureUnit::PoundsPerSquareInch) => 0x0709,
			Unit::Pressure(PressureUnit::Torr) => 0x070a,
			Unit::Temperature(TemperatureUnit::Celsius) => 0x0800,
			Unit::Temperature(TemperatureUnit::Fahrenheit) => 0x0801,
			Unit::Temperature(TemperatureUnit::Kelvin) => 0x0802,
			Unit::Temperature(TemperatureUnit::Rankine) => 0x0803,
			Unit::Time(TimeUnit::Nanoseconds) => 0x0900,
			Unit::Time(TimeUnit::Microseconds) => 0x0901,
			Unit::Time(TimeUnit::Milliseconds) => 0x0902,
			Unit::Time(TimeUnit::Seconds) => 0x0903,
			Unit::Time(TimeUnit::Minutes) => 0x0904,
			Unit::Time(TimeUnit::Hours) => 0x0905,
			Unit::Time(TimeUnit::Days) => 0x0906,
			Unit::Time(TimeUnit::Years) => 0x0907,
			Unit::Volume(VolumeUnit::Litre) => 0x0a00,
			Unit::Volume(VolumeUnit::Millilitre) => 0x0a01,
			Unit::Volume(VolumeUnit::Gallons) => 0x0a02,
			Unit::Volume(VolumeUnit::Quarts) => 0x0a03,
			Unit::Volume(VolumeUnit::Pints) => 0x0a04,
			Unit::Volume(VolumeUnit::Cups) => 0x0a05,
			Unit::Volume(VolumeUnit::FluidOunces) => 0x0a06,
			Unit::Volume(VolumeUnit::ImperialGallons) => 0x0a07,
			Unit::Volume(VolumeUnit::ImperialQuarts) => 0x0a08,
			Unit::Volume(VolumeUnit::ImperialPints) => 0x0a09,
			Unit::Volume(VolumeUnit::ImperialOunces) => 0x0a0a,
			Unit::Volume(VolumeUnit::Tablespoons) => 0x0a0b,
			Unit::Volume(VolumeUnit::Teaspoons) => 0x0a0c,
			Unit::Volume(VolumeUnit::UKTablespoons) => 0x0a0d,
			Unit::Volume(VolumeUnit::UKTeaspoons) => 0x0a0e,
		}
	}

	pub fn from_u16(value: u16) -> Option<Self> {
		match value {
			0x0000 => Some(Unit::Angle(AngleUnit::Degrees)),
			0x0001 => Some(Unit::Angle(AngleUnit::Radians)),
			0x0002 => Some(Unit::Angle(AngleUnit::Gradians)),
			0x0100 => Some(Unit::Area(AreaUnit::Hectares)),
			0x0101 => Some(Unit::Area(AreaUnit::Acres)),
			0x0200 => Some(Unit::Distance(DistanceUnit::Nanometers)),
			0x0201 => Some(Unit::Distance(DistanceUnit::Micrometers)),
			0x0202 => Some(Unit::Distance(DistanceUnit::Millimeters)),
			0x0203 => Some(Unit::Distance(DistanceUnit::Centimeters)),
			0x0204 => Some(Unit::Distance(DistanceUnit::Meters)),
			0x0205 => Some(Unit::Distance(DistanceUnit::Kilometers)),
			0x0210 => Some(Unit::Distance(DistanceUnit::Inches)),
			0x0211 => Some(Unit::Distance(DistanceUnit::Feet)),
			0x0212 => Some(Unit::Distance(DistanceUnit::Yards)),
			0x0213 => Some(Unit::Distance(DistanceUnit::Miles)),
			0x0214 => Some(Unit::Distance(DistanceUnit::NauticalMiles)),
			0x0220 => Some(Unit::Distance(DistanceUnit::AstronomicalUnits)),
			0x0300 => Some(Unit::Energy(EnergyUnit::Joules)),
			0x0301 => Some(Unit::Energy(EnergyUnit::Millijoules)),
			0x0302 => Some(Unit::Energy(EnergyUnit::Kilojoules)),
			0x0303 => Some(Unit::Energy(EnergyUnit::Megajoules)),
			0x0304 => Some(Unit::Energy(EnergyUnit::Calories)),
			0x0305 => Some(Unit::Energy(EnergyUnit::Kilocalories)),
			0x0306 => Some(Unit::Energy(EnergyUnit::BTU)),
			0x0307 => Some(Unit::Energy(EnergyUnit::FootPounds)),
			0x0308 => Some(Unit::Energy(EnergyUnit::FootPoundals)),
			0x0309 => Some(Unit::Energy(EnergyUnit::WattHours)),
			0x030a => Some(Unit::Energy(EnergyUnit::KilowattHours)),
			0x030b => Some(Unit::Energy(EnergyUnit::Erg)),
			0x0400 => Some(Unit::Force(ForceUnit::Newton)),
			0x0401 => Some(Unit::Force(ForceUnit::Kilonewton)),
			0x0402 => Some(Unit::Force(ForceUnit::Dyne)),
			0x0403 => Some(Unit::Force(ForceUnit::KilogramForce)),
			0x0404 => Some(Unit::Force(ForceUnit::PoundForce)),
			0x0405 => Some(Unit::Force(ForceUnit::Poundal)),
			0x0406 => Some(Unit::Force(ForceUnit::Kip)),
			0x0500 => Some(Unit::Mass(MassUnit::Grams)),
			0x0501 => Some(Unit::Mass(MassUnit::Milligrams)),
			0x0502 => Some(Unit::Mass(MassUnit::Kilograms)),
			0x0503 => Some(Unit::Mass(MassUnit::MetricTons)),
			0x0504 => Some(Unit::Mass(MassUnit::Pounds)),
			0x0505 => Some(Unit::Mass(MassUnit::Ounces)),
			0x0506 => Some(Unit::Mass(MassUnit::Stones)),
			0x0507 => Some(Unit::Mass(MassUnit::Tons)),
			0x0508 => Some(Unit::Mass(MassUnit::UKTons)),
			0x0600 => Some(Unit::Power(PowerUnit::Watts)),
			0x0601 => Some(Unit::Power(PowerUnit::Milliwatts)),
			0x0602 => Some(Unit::Power(PowerUnit::Kilowatts)),
			0x0603 => Some(Unit::Power(PowerUnit::Megawatts)),
			0x0604 => Some(Unit::Power(PowerUnit::Gigawatts)),
			0x0605 => Some(Unit::Power(PowerUnit::MechanicalHorsepower)),
			0x0606 => Some(Unit::Power(PowerUnit::MetricHorsepower)),
			0x0607 => Some(Unit::Power(PowerUnit::ElectricalHorsepower)),
			0x0608 => Some(Unit::Power(PowerUnit::TonsOfRefrigeration)),
			0x0700 => Some(Unit::Pressure(PressureUnit::Pascals)),
			0x0701 => Some(Unit::Pressure(PressureUnit::Kilopascals)),
			0x0702 => Some(Unit::Pressure(PressureUnit::Bars)),
			0x0703 => Some(Unit::Pressure(PressureUnit::Millibars)),
			0x0704 => Some(Unit::Pressure(PressureUnit::Atmospheres)),
			0x0705 => Some(Unit::Pressure(PressureUnit::InchesOfMercury)),
			0x0706 => Some(Unit::Pressure(PressureUnit::MillimetersOfMercury)),
			0x0707 => Some(Unit::Pressure(PressureUnit::InchesOfWater)),
			0x0708 => Some(Unit::Pressure(PressureUnit::MillimetersOfWater)),
			0x0709 => Some(Unit::Pressure(PressureUnit::PoundsPerSquareInch)),
			0x070a => Some(Unit::Pressure(PressureUnit::Torr)),
			0x0800 => Some(Unit::Temperature(TemperatureUnit::Celsius)),
			0x0801 => Some(Unit::Temperature(TemperatureUnit::Fahrenheit)),
			0x0802 => Some(Unit::Temperature(TemperatureUnit::Kelvin)),
			0x0803 => Some(Unit::Temperature(TemperatureUnit::Rankine)),
			0x0900 => Some(Unit::Time(TimeUnit::Nanoseconds)),
			0x0901 => Some(Unit::Time(TimeUnit::Microseconds)),
			0x0902 => Some(Unit::Time(TimeUnit::Milliseconds)),
			0x0903 => Some(Unit::Time(TimeUnit::Seconds)),
			0x0904 => Some(Unit::Time(TimeUnit::Minutes)),
			0x0905 => Some(Unit::Time(TimeUnit::Hours)),
			0x0906 => Some(Unit::Time(TimeUnit::Days)),
			0x0907 => Some(Unit::Time(TimeUnit::Years)),
			0x0a00 => Some(Unit::Volume(VolumeUnit::Litre)),
			0x0a01 => Some(Unit::Volume(VolumeUnit::Millilitre)),
			0x0a02 => Some(Unit::Volume(VolumeUnit::Gallons)),
			0x0a03 => Some(Unit::Volume(VolumeUnit::Quarts)),
			0x0a04 => Some(Unit::Volume(VolumeUnit::Pints)),
			0x0a05 => Some(Unit::Volume(VolumeUnit::Cups)),
			0x0a06 => Some(Unit::Volume(VolumeUnit::FluidOunces)),
			0x0a07 => Some(Unit::Volume(VolumeUnit::ImperialGallons)),
			0x0a08 => Some(Unit::Volume(VolumeUnit::ImperialQuarts)),
			0x0a09 => Some(Unit::Volume(VolumeUnit::ImperialPints)),
			0x0a0a => Some(Unit::Volume(VolumeUnit::ImperialOunces)),
			0x0a0b => Some(Unit::Volume(VolumeUnit::Tablespoons)),
			0x0a0c => Some(Unit::Volume(VolumeUnit::Teaspoons)),
			0x0a0d => Some(Unit::Volume(VolumeUnit::UKTablespoons)),
			0x0a0e => Some(Unit::Volume(VolumeUnit::UKTeaspoons)),
			_ => None,
		}
	}
}

pub trait UnitConversion: Eq {
	/// Converts a value from this unit to a target unit
	fn to_unit(&self, value: &Number, target_unit: &Self) -> Number;

	/// Converts a value from this unit to a target unit when the unit is inverted (for
	/// example, the seconds in meters per second)
	fn to_unit_inv(&self, value: &Number, target_unit: &Self) -> Number {
		target_unit.to_unit(value, self)
	}

	/// Converts a value from this unit to a target unit with the unit raised to
	/// the given power
	fn to_unit_with_power(&self, value: &Number, target_unit: &Self, power: i32) -> Number {
		if self == target_unit {
			return value.clone();
		}
		if power < 0 {
			let mut result = value.clone();
			for _ in 0..-power {
				result = self.to_unit_inv(&result, target_unit);
			}
			result
		} else if power > 0 {
			let mut result = value.clone();
			for _ in 0..power {
				result = self.to_unit(&result, target_unit);
			}
			result
		} else {
			value.clone()
		}
	}
}

pub trait MultiplierUnitConversion: UnitConversion {
	/// Gets the conversion factor from this unit to the standard unit of this type
	fn multiplier_to_standard(&self) -> Number;
}

impl<T: MultiplierUnitConversion> UnitConversion for T {
	/// Converts a value from this unit to a target unit
	fn to_unit(&self, value: &Number, target_unit: &Self) -> Number {
		if self == target_unit {
			return value.clone();
		}
		let value = value * &self.multiplier_to_standard();
		value / target_unit.multiplier_to_standard()
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UnitType {
	Angle,
	Area,
	Distance,
	Energy,
	Force,
	Mass,
	Power,
	Pressure,
	Temperature,
	Time,
	Volume,
}

impl UnitType {
	pub fn to_str(&self) -> String {
		match self {
			UnitType::Angle => "Angle".to_string(),
			UnitType::Area => "Area".to_string(),
			UnitType::Distance => "Dist".to_string(),
			UnitType::Energy => "Energy".to_string(),
			UnitType::Force => "Force".to_string(),
			UnitType::Mass => "Mass".to_string(),
			UnitType::Power => "Power".to_string(),
			UnitType::Pressure => "Pressure".to_string(),
			UnitType::Temperature => "Temp".to_string(),
			UnitType::Time => "Time".to_string(),
			UnitType::Volume => "Volume".to_string(),
		}
	}
}

#[derive(Clone)]
pub struct CompositeUnit {
	pub units: BTreeMap<UnitType, (Unit, i32)>,
}

impl MultiplierUnitConversion for AngleUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			AngleUnit::Degrees => 1.to_number(),
			AngleUnit::Radians => {
				Decimal::from_str("57.29577951308232087679815481410517").to_number()
			}
			AngleUnit::Gradians => 9.to_number() / 10.to_number(),
		}
	}
}

impl MultiplierUnitConversion for AreaUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			AreaUnit::Hectares => 1.to_number(),
			AreaUnit::Acres => 158_080_329.to_number() / 390_625_000.to_number(),
		}
	}
}

impl MultiplierUnitConversion for DistanceUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			DistanceUnit::Nanometers => 1.to_number() / 1_000_000_000.to_number(),
			DistanceUnit::Micrometers => 1.to_number() / 1_000_000.to_number(),
			DistanceUnit::Millimeters => 1.to_number() / 1000.to_number(),
			DistanceUnit::Centimeters => 1.to_number() / 100.to_number(),
			DistanceUnit::Meters => 1.to_number(),
			DistanceUnit::Kilometers => 1000.to_number(),
			DistanceUnit::Inches => 127.to_number() / 5000.to_number(),
			DistanceUnit::Feet => 381.to_number() / 1250.to_number(),
			DistanceUnit::Yards => 1143.to_number() / 1250.to_number(),
			DistanceUnit::Miles => 201168.to_number() / 125.to_number(),
			DistanceUnit::NauticalMiles => 1852.to_number(),
			DistanceUnit::AstronomicalUnits => 149_597_870_700u64.to_number(),
		}
	}
}

impl MultiplierUnitConversion for EnergyUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			EnergyUnit::Joules => 1.to_number(),
			EnergyUnit::Millijoules => 1.to_number() / 1000.to_number(),
			EnergyUnit::Kilojoules => 1000.to_number(),
			EnergyUnit::Megajoules => 1_000_000.to_number(),
			EnergyUnit::Calories => 523.to_number() / 125.to_number(),
			EnergyUnit::Kilocalories => 4184.to_number(),
			EnergyUnit::BTU => 23_722_880_951i64.to_number() / 22_500_000i64.to_number(),
			EnergyUnit::FootPounds => {
				3_389_544_870_828_501i64.to_number() / 2_500_000_000_000_000i64.to_number()
			}
			EnergyUnit::FootPoundals => {
				6_584_392_202_157i64.to_number() / 156_250_000_000_000i64.to_number()
			}
			EnergyUnit::WattHours => 3600.to_number(),
			EnergyUnit::KilowattHours => 3_600_000.to_number(),
			EnergyUnit::Erg => 1.to_number() / 10_000_000.to_number(),
		}
	}
}

impl MultiplierUnitConversion for ForceUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			ForceUnit::Newton => 1.to_number(),
			ForceUnit::Kilonewton => 1000.to_number(),
			ForceUnit::Dyne => 1.to_number() / 100_000.to_number(),
			ForceUnit::KilogramForce => 196_133.to_number() / 20_000.to_number(),
			ForceUnit::PoundForce => {
				8_896_443_230_521i64.to_number() / 2_000_000_000_000i64.to_number()
			}
			ForceUnit::Poundal => 17_281_869_297i64.to_number() / 125_000_000_000i64.to_number(),
			ForceUnit::Kip => 8_896_443_230_521i64.to_number() / 2_000_000_000i64.to_number(),
		}
	}
}

impl MultiplierUnitConversion for MassUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			MassUnit::Grams => 1.to_number(),
			MassUnit::Milligrams => 1.to_number() / 1000.to_number(),
			MassUnit::Kilograms => 1000.to_number(),
			MassUnit::MetricTons => 1_000_000.to_number(),
			MassUnit::Pounds => 45_359_237.to_number() / 100_000.to_number(),
			MassUnit::Ounces => 45_359_237.to_number() / 1_600_000.to_number(),
			MassUnit::Stones => 317_514_659.to_number() / 50_000.to_number(),
			MassUnit::Tons => 45_359_237.to_number() / 50.to_number(),
			MassUnit::UKTons => 635_029_318.to_number() / 625.to_number(),
		}
	}
}

impl MultiplierUnitConversion for PowerUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			PowerUnit::Watts => 1.to_number(),
			PowerUnit::Milliwatts => 1.to_number() / 1000.to_number(),
			PowerUnit::Kilowatts => 1000.to_number(),
			PowerUnit::Megawatts => 1_000_000.to_number(),
			PowerUnit::Gigawatts => 1_000_000_000.to_number(),
			PowerUnit::MechanicalHorsepower => {
				37_284_993_579_113_511i64.to_number() / 50_000_000_000_000i64.to_number()
			}
			PowerUnit::MetricHorsepower => 588_399.to_number() / 800.to_number(),
			PowerUnit::ElectricalHorsepower => 746.to_number(),
			PowerUnit::TonsOfRefrigeration => {
				52_752_792_631i64.to_number() / 15_000_000i64.to_number()
			}
		}
	}
}

impl MultiplierUnitConversion for PressureUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			PressureUnit::Pascals => 1.to_number(),
			PressureUnit::Kilopascals => 1000.to_number(),
			PressureUnit::Bars => 100_000.to_number(),
			PressureUnit::Millibars => 100.to_number(),
			PressureUnit::Atmospheres => 101_325.to_number(),
			PressureUnit::InchesOfMercury => {
				3_386_388_640_341i64.to_number() / 1_000_000_000i64.to_number()
			}
			PressureUnit::MillimetersOfMercury => {
				26_664_477_483i64.to_number() / 200_000_000i64.to_number()
			}
			PressureUnit::InchesOfWater => 24_908_891.to_number() / 100_000.to_number(),
			PressureUnit::MillimetersOfWater => 196_133.to_number() / 20_000.to_number(),
			PressureUnit::PoundsPerSquareInch => {
				8_896_443_230_521i64.to_number() / 1_290_320_000i64.to_number()
			}
			PressureUnit::Torr => 20_265.to_number() / 152.to_number(),
		}
	}
}

impl UnitConversion for TemperatureUnit {
	fn to_unit(&self, value: &Number, target_unit: &Self) -> Number {
		if self == target_unit {
			return value.clone();
		}
		target_unit
			.from_celsius(&*self.to_celsius(value))
			.into_owned()
	}
}

impl MultiplierUnitConversion for TimeUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			TimeUnit::Nanoseconds => 1.to_number() / 1_000_000_000.to_number(),
			TimeUnit::Microseconds => 1.to_number() / 1_000_000.to_number(),
			TimeUnit::Milliseconds => 1.to_number() / 1000.to_number(),
			TimeUnit::Seconds => 1.to_number(),
			TimeUnit::Minutes => 60.to_number(),
			TimeUnit::Hours => 3600.to_number(),
			TimeUnit::Days => (3600 * 24).to_number(),
			TimeUnit::Years => 31556952.to_number(), // Average length of year over 400 years
		}
	}
}

impl MultiplierUnitConversion for VolumeUnit {
	fn multiplier_to_standard(&self) -> Number {
		match self {
			VolumeUnit::Litre => 1.to_number(),
			VolumeUnit::Millilitre => 1.to_number() / 1000.to_number(),
			VolumeUnit::Gallons => 473_176_473.to_number() / 125_000_000.to_number(),
			VolumeUnit::Quarts => 473_176_473.to_number() / 500_000_000.to_number(),
			VolumeUnit::Pints => 473_176_473.to_number() / 1_000_000_000.to_number(),
			VolumeUnit::Cups => 473_176_473.to_number() / 2_000_000_000.to_number(),
			VolumeUnit::FluidOunces => 473_176_473i64.to_number() / 16_000_000_000i64.to_number(),
			VolumeUnit::ImperialGallons => 454_609.to_number() / 100_000.to_number(),
			VolumeUnit::ImperialQuarts => 454_609.to_number() / 400_000.to_number(),
			VolumeUnit::ImperialPints => 454_609.to_number() / 800_000.to_number(),
			VolumeUnit::ImperialOunces => 454_609.to_number() / 16_000_000.to_number(),
			VolumeUnit::Tablespoons => 473_176_473i64.to_number() / 32_000_000_000i64.to_number(),
			VolumeUnit::Teaspoons => 473_176_473i64.to_number() / 96_000_000_000i64.to_number(),
			VolumeUnit::UKTablespoons => 3.to_number() / 200.to_number(),
			VolumeUnit::UKTeaspoons => 1.to_number() / 200.to_number(),
		}
	}
}

impl Unit {
	pub fn unit_type(&self) -> UnitType {
		match self {
			Unit::Angle(_) => UnitType::Angle,
			Unit::Area(_) => UnitType::Area,
			Unit::Distance(_) => UnitType::Distance,
			Unit::Energy(_) => UnitType::Energy,
			Unit::Force(_) => UnitType::Force,
			Unit::Mass(_) => UnitType::Mass,
			Unit::Power(_) => UnitType::Power,
			Unit::Pressure(_) => UnitType::Pressure,
			Unit::Temperature(_) => UnitType::Temperature,
			Unit::Time(_) => UnitType::Time,
			Unit::Volume(_) => UnitType::Volume,
		}
	}
}

impl From<AngleUnit> for Unit {
	fn from(unit: AngleUnit) -> Self {
		Unit::Angle(unit)
	}
}

impl From<AreaUnit> for Unit {
	fn from(unit: AreaUnit) -> Self {
		Unit::Area(unit)
	}
}

impl From<DistanceUnit> for Unit {
	fn from(unit: DistanceUnit) -> Self {
		Unit::Distance(unit)
	}
}

impl From<EnergyUnit> for Unit {
	fn from(unit: EnergyUnit) -> Self {
		Unit::Energy(unit)
	}
}

impl From<ForceUnit> for Unit {
	fn from(unit: ForceUnit) -> Self {
		Unit::Force(unit)
	}
}

impl From<MassUnit> for Unit {
	fn from(unit: MassUnit) -> Self {
		Unit::Mass(unit)
	}
}

impl From<PowerUnit> for Unit {
	fn from(unit: PowerUnit) -> Self {
		Unit::Power(unit)
	}
}

impl From<PressureUnit> for Unit {
	fn from(unit: PressureUnit) -> Self {
		Unit::Pressure(unit)
	}
}

impl From<TemperatureUnit> for Unit {
	fn from(unit: TemperatureUnit) -> Self {
		Unit::Temperature(unit)
	}
}

impl From<TimeUnit> for Unit {
	fn from(unit: TimeUnit) -> Self {
		Unit::Time(unit)
	}
}

impl From<VolumeUnit> for Unit {
	fn from(unit: VolumeUnit) -> Self {
		Unit::Volume(unit)
	}
}

impl CompositeUnit {
	pub fn new() -> Self {
		CompositeUnit {
			units: BTreeMap::new(),
		}
	}

	pub fn single_unit(unit: Unit) -> Self {
		let mut units = BTreeMap::new();
		units.insert(unit.unit_type(), (unit, 1));
		CompositeUnit { units }
	}

	pub fn single_unit_inv(unit: Unit) -> Self {
		let mut units = BTreeMap::new();
		units.insert(unit.unit_type(), (unit, -1));
		CompositeUnit { units }
	}

	pub fn ratio_unit(numer: Unit, denom: Unit) -> Self {
		let mut units = BTreeMap::new();
		units.insert(numer.unit_type(), (numer, 1));
		units.insert(denom.unit_type(), (denom, -1));
		CompositeUnit { units }
	}

	pub fn unitless(&self) -> bool {
		self.units.len() == 0
	}

	fn convert_value_of_unit(
		value: &Number,
		from_unit: &Unit,
		to_unit: &Unit,
		power: i32,
	) -> Result<Number> {
		match from_unit {
			Unit::Angle(from) => match to_unit {
				Unit::Angle(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Area(from) => match to_unit {
				Unit::Area(to) => Ok(from.to_unit_with_power(value, to, power)),
				Unit::Distance(to) => Ok(DistanceUnit::Meters.to_unit_with_power(
					&from.to_square_meters_with_power(value, power),
					to,
					power * 2,
				)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Distance(from) => match to_unit {
				Unit::Area(to) => {
					if power % 2 == 0 {
						Ok(to.from_square_meters_with_power(
							&from.to_unit_with_power(value, &DistanceUnit::Meters, power),
							power / 2,
						))
					} else {
						Err(Error::IncompatibleUnits)
					}
				}
				Unit::Volume(to) => {
					if power % 3 == 0 {
						Ok(to.from_cubic_meters_with_power(
							&from.to_unit_with_power(value, &DistanceUnit::Meters, power),
							power / 3,
						))
					} else {
						Err(Error::IncompatibleUnits)
					}
				}
				Unit::Distance(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Energy(from) => match to_unit {
				Unit::Energy(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Force(from) => match to_unit {
				Unit::Force(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Mass(from) => match to_unit {
				Unit::Mass(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Power(from) => match to_unit {
				Unit::Power(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Pressure(from) => match to_unit {
				Unit::Pressure(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Temperature(from) => match to_unit {
				Unit::Temperature(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Time(from) => match to_unit {
				Unit::Time(to) => Ok(from.to_unit_with_power(value, to, power)),
				_ => Err(Error::IncompatibleUnits),
			},
			Unit::Volume(from) => match to_unit {
				Unit::Volume(to) => Ok(from.to_unit_with_power(value, to, power)),
				Unit::Distance(to) => Ok(DistanceUnit::Meters.to_unit_with_power(
					&from.to_cubic_meters_with_power(value, power),
					to,
					power * 3,
				)),
				_ => Err(Error::IncompatibleUnits),
			},
		}
	}

	fn collapse_composite_unit_types(&mut self, value: Number) -> Number {
		let mut value = value;

		// Check for area unit alongside distance unit
		if self.units.contains_key(&UnitType::Distance) && self.units.contains_key(&UnitType::Area)
		{
			// Collapse area unit into distance unit
			let area_unit = self.units.get(&UnitType::Area).unwrap().clone();
			self.units.remove(&UnitType::Area);
			let distance_unit = self.units.get_mut(&UnitType::Distance).unwrap();
			value =
				Self::convert_value_of_unit(&value, &area_unit.0, &distance_unit.0, area_unit.1)
					.unwrap();
			distance_unit.1 += area_unit.1 * 2;
			if distance_unit.1 == 0 {
				self.units.remove(&UnitType::Distance);
			}
		}

		// Check for volume unit alongside distance unit
		if self.units.contains_key(&UnitType::Distance)
			&& self.units.contains_key(&UnitType::Volume)
		{
			// Collapse volume unit into distance unit
			let volume_unit = self.units.get(&UnitType::Volume).unwrap().clone();
			self.units.remove(&UnitType::Volume);
			let distance_unit = self.units.get_mut(&UnitType::Distance).unwrap();
			value = Self::convert_value_of_unit(
				&value,
				&volume_unit.0,
				&distance_unit.0,
				volume_unit.1,
			)
			.unwrap();
			distance_unit.1 += volume_unit.1 * 3;
			if distance_unit.1 == 0 {
				self.units.remove(&UnitType::Distance);
			}
		}

		value
	}

	pub fn add_unit(&mut self, value: &Number, unit: Unit) -> Number {
		let unit_type = unit.unit_type();
		let new_value = if let Some(existing_unit) = self.units.get_mut(&unit_type) {
			let value =
				Self::convert_value_of_unit(value, &existing_unit.0, &unit, existing_unit.1)
					.unwrap();
			existing_unit.0 = unit;
			existing_unit.1 += 1;
			if existing_unit.1 == 0 {
				self.units.remove(&unit_type);
			}
			value
		} else {
			self.units.insert(unit_type, (unit, 1));
			value.clone()
		};
		self.collapse_composite_unit_types(new_value)
	}

	pub fn add_unit_inv(&mut self, value: &Number, unit: Unit) -> Number {
		let unit_type = unit.unit_type();
		let new_value = if let Some(existing_unit) = self.units.get_mut(&unit_type) {
			let value =
				Self::convert_value_of_unit(value, &existing_unit.0, &unit, existing_unit.1)
					.unwrap();
			existing_unit.0 = unit;
			existing_unit.1 -= 1;
			if existing_unit.1 == 0 {
				self.units.remove(&unit_type);
			}
			value
		} else {
			self.units.insert(unit_type, (unit, -1));
			value.clone()
		};
		self.collapse_composite_unit_types(new_value)
	}

	pub fn inverse(&self) -> Self {
		let mut units = BTreeMap::new();
		for (unit_type, unit) in self.units.iter() {
			units.insert(*unit_type, (unit.0, -unit.1));
		}
		CompositeUnit { units }
	}

	pub fn convert_single_unit(&mut self, value: &Number, target_unit: Unit) -> Result<Number> {
		let unit_type = target_unit.unit_type();
		if let Some(existing_unit) = self.units.get_mut(&unit_type) {
			let value = Self::convert_value_of_unit(
				value,
				&existing_unit.0,
				&target_unit,
				existing_unit.1,
			)?;
			existing_unit.0 = target_unit;
			Ok(value)
		} else if unit_type == UnitType::Area {
			if let Some(existing_unit) = self.units.get_mut(&UnitType::Distance) {
				let value = Self::convert_value_of_unit(
					value,
					&existing_unit.0,
					&target_unit,
					existing_unit.1,
				)?;
				let distance_power = existing_unit.1;
				self.units.remove(&UnitType::Distance);
				self.units
					.insert(UnitType::Area, (target_unit, distance_power / 2));
				Ok(value)
			} else {
				Err(Error::IncompatibleUnits)
			}
		} else if unit_type == UnitType::Volume {
			if let Some(existing_unit) = self.units.get_mut(&UnitType::Distance) {
				let value = Self::convert_value_of_unit(
					value,
					&existing_unit.0,
					&target_unit,
					existing_unit.1,
				)?;
				let distance_power = existing_unit.1;
				self.units.remove(&UnitType::Distance);
				self.units
					.insert(UnitType::Volume, (target_unit, distance_power / 3));
				Ok(value)
			} else {
				Err(Error::IncompatibleUnits)
			}
		} else if unit_type == UnitType::Distance {
			if let Some(existing_unit) = self.units.get_mut(&UnitType::Area) {
				let value = Self::convert_value_of_unit(
					value,
					&existing_unit.0,
					&target_unit,
					existing_unit.1,
				)?;
				let area_power = existing_unit.1;
				self.units.remove(&UnitType::Area);
				self.units
					.insert(UnitType::Distance, (target_unit, area_power * 2));
				Ok(value)
			} else if let Some(existing_unit) = self.units.get_mut(&UnitType::Volume) {
				let value = Self::convert_value_of_unit(
					value,
					&existing_unit.0,
					&target_unit,
					existing_unit.1,
				)?;
				let volume_power = existing_unit.1;
				self.units.remove(&UnitType::Volume);
				self.units
					.insert(UnitType::Distance, (target_unit, volume_power * 3));
				Ok(value)
			} else {
				Err(Error::IncompatibleUnits)
			}
		} else {
			Err(Error::IncompatibleUnits)
		}
	}

	pub fn coerce_to_other(&self, value: &Number, target_units: &CompositeUnit) -> Result<Number> {
		// First convert composite unit types (like area) into the base unit types
		let mut result = value.clone();
		let mut collapsed_units = self.clone();
		if collapsed_units.units.contains_key(&UnitType::Area) {
			result = collapsed_units
				.convert_single_unit(&result, Unit::Distance(DistanceUnit::Meters))?;
		}
		if collapsed_units.units.contains_key(&UnitType::Volume) {
			result = collapsed_units
				.convert_single_unit(&result, Unit::Distance(DistanceUnit::Meters))?;
		}

		let mut collapsed_target_units = target_units.clone();
		if collapsed_target_units.units.contains_key(&UnitType::Area) {
			// Collapse area unit into distance unit
			let area_unit = collapsed_target_units
				.units
				.get(&UnitType::Area)
				.unwrap()
				.clone();
			collapsed_target_units.units.remove(&UnitType::Area);
			if let Some(distance_unit) = collapsed_target_units.units.get_mut(&UnitType::Distance) {
				distance_unit.1 += area_unit.1 * 2;
				if distance_unit.1 == 0 {
					collapsed_target_units.units.remove(&UnitType::Distance);
				}
			} else {
				collapsed_target_units.units.insert(
					UnitType::Distance,
					(Unit::Distance(DistanceUnit::Meters), area_unit.1 * 2),
				);
			}
		}
		if collapsed_target_units.units.contains_key(&UnitType::Volume) {
			// Collapse volume unit into distance unit
			let volume_unit = collapsed_target_units
				.units
				.get(&UnitType::Volume)
				.unwrap()
				.clone();
			collapsed_target_units.units.remove(&UnitType::Volume);
			if let Some(distance_unit) = collapsed_target_units.units.get_mut(&UnitType::Distance) {
				distance_unit.1 += volume_unit.1 * 3;
				if distance_unit.1 == 0 {
					collapsed_target_units.units.remove(&UnitType::Distance);
				}
			} else {
				collapsed_target_units.units.insert(
					UnitType::Distance,
					(Unit::Distance(DistanceUnit::Meters), volume_unit.1 * 3),
				);
			}
		}

		// Check units to make sure they are compatible. There must be the same set of
		// unit types and each unit type must be the same power.
		for (unit_type, unit) in collapsed_units.units.iter() {
			if let Some(target) = collapsed_target_units.units.get(&unit_type) {
				if unit.1 != target.1 {
					return Err(Error::IncompatibleUnits);
				}
			} else {
				return Err(Error::IncompatibleUnits);
			}
		}
		for (unit_type, unit) in collapsed_target_units.units.iter() {
			if let Some(target) = collapsed_units.units.get(&unit_type) {
				if unit.1 != target.1 {
					return Err(Error::IncompatibleUnits);
				}
			} else {
				return Err(Error::IncompatibleUnits);
			}
		}

		// Convert units to the target unit
		for (_, value) in collapsed_target_units.units.iter() {
			result = collapsed_units.convert_single_unit(&result, value.0)?;
		}

		// Convert any composite unit types collapsed earlier back to the target unit
		if let Some(area_unit) = target_units.units.get(&UnitType::Area) {
			result = collapsed_units.convert_single_unit(&result, area_unit.0)?;
		}
		if let Some(volume_unit) = target_units.units.get(&UnitType::Volume) {
			result = collapsed_units.convert_single_unit(&result, volume_unit.0)?;
		}

		Ok(result)
	}

	pub fn combine(&mut self, value: &Number, target_units: &CompositeUnit) -> Number {
		let mut result = value.clone();
		for (unit_type, unit) in target_units.units.iter() {
			if let Some(target) = self.units.get_mut(&unit_type) {
				result =
					Self::convert_value_of_unit(&result, &target.0, &unit.0, target.1).unwrap();
				target.0 = unit.0;
				target.1 += unit.1;
				if target.1 == 0 {
					self.units.remove(&unit_type);
				}
			} else {
				self.units.insert(*unit_type, unit.clone());
			}
		}
		self.collapse_composite_unit_types(result)
	}
}

impl StorageObject for CompositeUnit {
	fn serialize<Ref: StorageRefSerializer, Out: SerializeOutput>(
		&self,
		output: &mut Out,
		_: &Ref,
	) -> Result<()> {
		output.write_u32(self.units.len() as u32)?;
		for (_, unit) in &self.units {
			output.write_u16(unit.0.to_u16())?;
			output.write_i32(unit.1)?;
		}
		Ok(())
	}

	unsafe fn deserialize<T: StorageRefSerializer>(
		input: &mut DeserializeInput,
		_: &T,
	) -> Result<Self> {
		let count = input.read_u32()?;
		let mut result = CompositeUnit::new();
		for _ in 0..count {
			let unit = match Unit::from_u16(input.read_u16()?) {
				Some(unit) => unit,
				None => return Err(Error::CorruptData),
			};
			let power = input.read_i32()?;
			let unit_type = unit.unit_type();
			result.units.insert(unit_type, (unit, power));
		}
		Ok(result)
	}
}

fn value_layout<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Layout {
	let value_layout = value.render(&state.format, &None, screen.width());
	let mut layout_items = Vec::new();
	layout_items.push(Layout::HorizontalRule);
	layout_items.push(value_layout);
	Layout::Vertical(layout_items)
}

pub fn unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	items.push(MenuItem {
		layout: MenuItem::string_layout("Angle".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Angle)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Area".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Area)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Distance".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Distance)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Energy".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Energy)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Force".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Force)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Mass".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Mass)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Power".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Power)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Pressure".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Pressure)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Temp".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Temperature)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Time".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Time)),
	});
	items.push(MenuItem {
		layout: MenuItem::string_layout("Volume".to_string()),
		function: MenuItemFunction::Action(Function::UnitMenu(UnitType::Volume)),
	});

	let mut menu = Menu::new_with_bottom(
		"Units".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

pub fn unit_menu_of_type<ScreenT: Screen>(
	state: &State,
	screen: &ScreenT,
	value: &Value,
	unit_type: UnitType,
) -> Menu {
	match unit_type {
		UnitType::Angle => angle_unit_menu(state, screen, value),
		UnitType::Area => area_unit_menu(state, screen, value),
		UnitType::Distance => distance_unit_menu(state, screen, value),
		UnitType::Energy => energy_unit_menu(state, screen, value),
		UnitType::Force => force_unit_menu(state, screen, value),
		UnitType::Mass => mass_unit_menu(state, screen, value),
		UnitType::Power => power_unit_menu(state, screen, value),
		UnitType::Pressure => pressure_unit_menu(state, screen, value),
		UnitType::Temperature => temperature_unit_menu(state, screen, value),
		UnitType::Time => time_unit_menu(state, screen, value),
		UnitType::Volume => volume_unit_menu(state, screen, value),
	}
}

fn angle_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[AngleUnit::Degrees, AngleUnit::Radians, AngleUnit::Gradians] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Angle(*unit), UnitType::Angle),
				Function::AddInvUnit(Unit::Angle(*unit), UnitType::Angle),
				Function::ConvertToUnit(Unit::Angle(*unit), UnitType::Angle),
			),
		});
	}

	Menu::new_with_bottom(
		"Angle (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	)
}

fn area_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[AreaUnit::Acres, AreaUnit::Hectares] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Area(*unit), UnitType::Area),
				Function::AddInvUnit(Unit::Area(*unit), UnitType::Area),
				Function::ConvertToUnit(Unit::Area(*unit), UnitType::Area),
			),
		});
	}

	for unit in &[
		DistanceUnit::Meters,
		DistanceUnit::Millimeters,
		DistanceUnit::Centimeters,
		DistanceUnit::Kilometers,
		DistanceUnit::Inches,
		DistanceUnit::Feet,
		DistanceUnit::Yards,
		DistanceUnit::Miles,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str() + "²"),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnitSquared(Unit::Distance(*unit), UnitType::Area),
				Function::AddInvUnitSquared(Unit::Distance(*unit), UnitType::Area),
				Function::ConvertToUnit(Unit::Distance(*unit), UnitType::Area),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Area (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

fn distance_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		DistanceUnit::Meters,
		DistanceUnit::Nanometers,
		DistanceUnit::Micrometers,
		DistanceUnit::Millimeters,
		DistanceUnit::Centimeters,
		DistanceUnit::Kilometers,
		DistanceUnit::Inches,
		DistanceUnit::Feet,
		DistanceUnit::Yards,
		DistanceUnit::Miles,
		DistanceUnit::NauticalMiles,
		DistanceUnit::AstronomicalUnits,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Distance(*unit), UnitType::Distance),
				Function::AddInvUnit(Unit::Distance(*unit), UnitType::Distance),
				Function::ConvertToUnit(Unit::Distance(*unit), UnitType::Distance),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Distance (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

fn energy_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		EnergyUnit::Joules,
		EnergyUnit::Millijoules,
		EnergyUnit::Kilojoules,
		EnergyUnit::Megajoules,
		EnergyUnit::Calories,
		EnergyUnit::Kilocalories,
		EnergyUnit::BTU,
		EnergyUnit::FootPounds,
		EnergyUnit::FootPoundals,
		EnergyUnit::WattHours,
		EnergyUnit::KilowattHours,
		EnergyUnit::Erg,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Energy(*unit), UnitType::Energy),
				Function::AddInvUnit(Unit::Energy(*unit), UnitType::Energy),
				Function::ConvertToUnit(Unit::Energy(*unit), UnitType::Energy),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Energy (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

fn force_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		ForceUnit::Newton,
		ForceUnit::Kilonewton,
		ForceUnit::Dyne,
		ForceUnit::KilogramForce,
		ForceUnit::PoundForce,
		ForceUnit::Poundal,
		ForceUnit::Kip,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Force(*unit), UnitType::Force),
				Function::AddInvUnit(Unit::Force(*unit), UnitType::Force),
				Function::ConvertToUnit(Unit::Force(*unit), UnitType::Force),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Force (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(2);
	menu
}

fn mass_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		MassUnit::Kilograms,
		MassUnit::Grams,
		MassUnit::Milligrams,
		MassUnit::MetricTons,
		MassUnit::Pounds,
		MassUnit::Ounces,
		MassUnit::Stones,
		MassUnit::Tons,
		MassUnit::UKTons,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Mass(*unit), UnitType::Mass),
				Function::AddInvUnit(Unit::Mass(*unit), UnitType::Mass),
				Function::ConvertToUnit(Unit::Mass(*unit), UnitType::Mass),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Mass (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

fn power_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		PowerUnit::Watts,
		PowerUnit::Milliwatts,
		PowerUnit::Kilowatts,
		PowerUnit::Megawatts,
		PowerUnit::Gigawatts,
		PowerUnit::MechanicalHorsepower,
		PowerUnit::MetricHorsepower,
		PowerUnit::ElectricalHorsepower,
		PowerUnit::TonsOfRefrigeration,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Power(*unit), UnitType::Power),
				Function::AddInvUnit(Unit::Power(*unit), UnitType::Power),
				Function::ConvertToUnit(Unit::Power(*unit), UnitType::Power),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Power (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

fn pressure_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		PressureUnit::Pascals,
		PressureUnit::Kilopascals,
		PressureUnit::Bars,
		PressureUnit::Millibars,
		PressureUnit::Atmospheres,
		PressureUnit::InchesOfMercury,
		PressureUnit::MillimetersOfMercury,
		PressureUnit::InchesOfWater,
		PressureUnit::MillimetersOfWater,
		PressureUnit::PoundsPerSquareInch,
		PressureUnit::Torr,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Pressure(*unit), UnitType::Pressure),
				Function::AddInvUnit(Unit::Pressure(*unit), UnitType::Pressure),
				Function::ConvertToUnit(Unit::Pressure(*unit), UnitType::Pressure),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Pressure (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(3);
	menu
}

fn temperature_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		TemperatureUnit::Celsius,
		TemperatureUnit::Fahrenheit,
		TemperatureUnit::Kelvin,
		TemperatureUnit::Rankine,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Temperature(*unit), UnitType::Temperature),
				Function::AddInvUnit(Unit::Temperature(*unit), UnitType::Temperature),
				Function::ConvertToUnit(Unit::Temperature(*unit), UnitType::Temperature),
			),
		});
	}

	Menu::new_with_bottom(
		"Temp (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	)
}

fn time_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		TimeUnit::Seconds,
		TimeUnit::Nanoseconds,
		TimeUnit::Microseconds,
		TimeUnit::Milliseconds,
		TimeUnit::Minutes,
		TimeUnit::Hours,
		TimeUnit::Days,
		TimeUnit::Years,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Time(*unit), UnitType::Time),
				Function::AddInvUnit(Unit::Time(*unit), UnitType::Time),
				Function::ConvertToUnit(Unit::Time(*unit), UnitType::Time),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Time (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(2);
	menu
}

fn volume_unit_menu<ScreenT: Screen>(state: &State, screen: &ScreenT, value: &Value) -> Menu {
	let mut items = Vec::new();
	for unit in &[
		VolumeUnit::Litre,
		VolumeUnit::Millilitre,
		VolumeUnit::Gallons,
		VolumeUnit::Quarts,
		VolumeUnit::Pints,
		VolumeUnit::Cups,
		VolumeUnit::FluidOunces,
		VolumeUnit::ImperialGallons,
		VolumeUnit::ImperialQuarts,
		VolumeUnit::ImperialPints,
		VolumeUnit::ImperialOunces,
		VolumeUnit::Tablespoons,
		VolumeUnit::Teaspoons,
		VolumeUnit::UKTablespoons,
		VolumeUnit::UKTeaspoons,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout_small(unit.to_str()),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnit(Unit::Volume(*unit), UnitType::Volume),
				Function::AddInvUnit(Unit::Volume(*unit), UnitType::Volume),
				Function::ConvertToUnit(Unit::Volume(*unit), UnitType::Volume),
			),
		});
	}

	for unit in &[
		DistanceUnit::Meters,
		DistanceUnit::Centimeters,
		DistanceUnit::Inches,
		DistanceUnit::Feet,
	] {
		items.push(MenuItem {
			layout: MenuItem::string_layout_small(unit.to_str() + "³"),
			function: MenuItemFunction::ConversionAction(
				Function::AddUnitCubed(Unit::Distance(*unit), UnitType::Volume),
				Function::AddInvUnitCubed(Unit::Distance(*unit), UnitType::Volume),
				Function::ConvertToUnit(Unit::Distance(*unit), UnitType::Volume),
			),
		});
	}

	let mut menu = Menu::new_with_bottom(
		"Volume (×,÷ Assign; x≷y Convert)".to_string(),
		items,
		value_layout(state, screen, value),
	);
	menu.set_columns(4);
	menu
}
