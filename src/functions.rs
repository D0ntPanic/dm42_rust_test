use crate::error::{Error, Result};
use crate::font::SANS_13;
use crate::input::InputEvent;
use crate::number::{
	IntegerMode, Number, NumberDecimalPointMode, NumberFormat, NumberFormatMode, ToNumber,
	MAX_INTEGER_BITS,
};
use crate::screen::{Color, Rect, Screen};
use crate::state::State;
use crate::time::Now;
use crate::unit::{CompositeUnit, DistanceUnit, TimeUnit};
use crate::value::Value;
use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use core::cell::RefCell;
use core::convert::TryFrom;
use num_bigint::ToBigInt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Function {
	Input(InputEvent),
	NormalFormat,
	RationalFormat,
	ScientificFormat,
	EngineeringFormat,
	AlternateHex,
	AlternateFloat,
	ThousandsSeparatorOff,
	ThousandsSeparatorOn,
	DecimalPointPeriod,
	DecimalPointComma,
	Float,
	SignedInteger,
	UnsignedInteger,
	BigInteger,
	Signed8Bit,
	Signed16Bit,
	Signed32Bit,
	Signed64Bit,
	Signed128Bit,
	Unsigned8Bit,
	Unsigned16Bit,
	Unsigned32Bit,
	Unsigned64Bit,
	Unsigned128Bit,
	And,
	Or,
	Xor,
	Not,
	ShiftLeft,
	ShiftRight,
	RotateLeft,
	RotateRight,
	Hex,
	Octal,
	Decimal,
	ConstCatalog,
	SpeedOfLight,
	TimeCatalog,
	Now,
	Date,
	Time,
	TimeUnits,
	Nanoseconds,
	Microseconds,
	Milliseconds,
	Seconds,
	Minutes,
	Hours,
	Days,
	Years,
	InverseTimeUnits,
	InverseNanoseconds,
	InverseMicroseconds,
	InverseMilliseconds,
	InverseSeconds,
	InverseMinutes,
	InverseHours,
	InverseDays,
	InverseYears,
	ToTimeUnits,
	ToNanoseconds,
	ToMicroseconds,
	ToMilliseconds,
	ToSeconds,
	ToMinutes,
	ToHours,
	ToDays,
	ToYears,
	DistanceUnits,
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
	InverseDistanceUnits,
	InverseNanometers,
	InverseMicrometers,
	InverseMillimeters,
	InverseCentimeters,
	InverseMeters,
	InverseKilometers,
	InverseInches,
	InverseFeet,
	InverseYards,
	InverseMiles,
	InverseNauticalMiles,
	InverseAstronomicalUnits,
	ToDistanceUnits,
	ToNanometers,
	ToMicrometers,
	ToMillimeters,
	ToCentimeters,
	ToMeters,
	ToKilometers,
	ToInches,
	ToFeet,
	ToYards,
	ToMiles,
	ToNauticalMiles,
	ToAstronomicalUnits,
}

impl Function {
	pub fn to_str(&self, state: &State) -> String {
		match self {
			Function::Input(input) => input.to_str(),
			Function::NormalFormat => {
				if state.format.mode == NumberFormatMode::Normal {
					"▪Norm".to_string()
				} else {
					"Norm".to_string()
				}
			}
			Function::RationalFormat => {
				if state.format.mode == NumberFormatMode::Rational {
					"▪Frac".to_string()
				} else {
					"Frac".to_string()
				}
			}
			Function::ScientificFormat => {
				if state.format.mode == NumberFormatMode::Scientific {
					"▪Sci".to_string()
				} else {
					"Sci".to_string()
				}
			}
			Function::EngineeringFormat => {
				if state.format.mode == NumberFormatMode::Engineering {
					"▪Eng".to_string()
				} else {
					"Eng".to_string()
				}
			}
			Function::AlternateHex => {
				if state.format.show_alt_hex {
					"▪↓Hex".to_string()
				} else {
					"↓Hex".to_string()
				}
			}
			Function::AlternateFloat => {
				if state.format.show_alt_float {
					"▪↓Flt".to_string()
				} else {
					"↓Flt".to_string()
				}
			}
			Function::ThousandsSeparatorOff => {
				if state.format.thousands {
					"1000".to_string()
				} else {
					"▪1000".to_string()
				}
			}
			Function::ThousandsSeparatorOn => {
				if state.format.thousands {
					"▪1,000".to_string()
				} else {
					"1,000".to_string()
				}
			}
			Function::DecimalPointPeriod => {
				if state.format.decimal_point == NumberDecimalPointMode::Period {
					"▪0.5".to_string()
				} else {
					"0.5".to_string()
				}
			}
			Function::DecimalPointComma => {
				if state.format.decimal_point == NumberDecimalPointMode::Comma {
					"▪0,5".to_string()
				} else {
					"0,5".to_string()
				}
			}
			Function::Float => {
				if state.format.integer_mode == IntegerMode::Float {
					"▪float".to_string()
				} else {
					"float".to_string()
				}
			}
			Function::SignedInteger => match state.format.integer_mode {
				IntegerMode::BigInteger | IntegerMode::SizedInteger(_, true) => "▪int".to_string(),
				_ => "int".to_string(),
			},
			Function::UnsignedInteger => match state.format.integer_mode {
				IntegerMode::SizedInteger(_, false) => "▪uint".to_string(),
				_ => "uint".to_string(),
			},
			Function::BigInteger => {
				if state.format.integer_mode == IntegerMode::BigInteger {
					"▪int∞".to_string()
				} else {
					"int∞".to_string()
				}
			}
			Function::Signed8Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(8, true) {
					"▪i8".to_string()
				} else {
					"i8".to_string()
				}
			}
			Function::Signed16Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(16, true) {
					"▪i16".to_string()
				} else {
					"i16".to_string()
				}
			}
			Function::Signed32Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(32, true) {
					"▪i32".to_string()
				} else {
					"i32".to_string()
				}
			}
			Function::Signed64Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(64, true) {
					"▪i64".to_string()
				} else {
					"i64".to_string()
				}
			}
			Function::Signed128Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(128, true) {
					"▪i128".to_string()
				} else {
					"i128".to_string()
				}
			}
			Function::Unsigned8Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(8, false) {
					"▪u8".to_string()
				} else {
					"u8".to_string()
				}
			}
			Function::Unsigned16Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(16, false) {
					"▪u16".to_string()
				} else {
					"u16".to_string()
				}
			}
			Function::Unsigned32Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(32, false) {
					"▪u32".to_string()
				} else {
					"u32".to_string()
				}
			}
			Function::Unsigned64Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(64, false) {
					"▪u64".to_string()
				} else {
					"u64".to_string()
				}
			}
			Function::Unsigned128Bit => {
				if state.format.integer_mode == IntegerMode::SizedInteger(128, false) {
					"▪u128".to_string()
				} else {
					"u128".to_string()
				}
			}
			Function::And => "and".to_string(),
			Function::Or => "or".to_string(),
			Function::Xor => "xor".to_string(),
			Function::Not => "not".to_string(),
			Function::ShiftLeft => "<<".to_string(),
			Function::ShiftRight => ">>".to_string(),
			Function::RotateLeft => "rol".to_string(),
			Function::RotateRight => "ror".to_string(),
			Function::Hex => {
				if state.format.integer_radix == 16 {
					"▪Hex".to_string()
				} else {
					"Hex".to_string()
				}
			}
			Function::Octal => {
				if state.format.integer_radix == 8 {
					"▪Oct".to_string()
				} else {
					"Oct".to_string()
				}
			}
			Function::Decimal => {
				if state.format.integer_radix == 10 {
					"▪Dec".to_string()
				} else {
					"Dec".to_string()
				}
			}
			Function::ConstCatalog => "Const".to_string(),
			Function::SpeedOfLight => "c".to_string(),
			Function::TimeCatalog => "Time".to_string(),
			Function::Now => "Now".to_string(),
			Function::Date => "Date".to_string(),
			Function::Time => "Time".to_string(),
			Function::TimeUnits => "Time".to_string(),
			Function::Nanoseconds => "ns".to_string(),
			Function::Microseconds => "μs".to_string(),
			Function::Milliseconds => "ms".to_string(),
			Function::Seconds => "sec".to_string(),
			Function::Minutes => "min".to_string(),
			Function::Hours => "hr".to_string(),
			Function::Days => "day".to_string(),
			Function::Years => "yr".to_string(),
			Function::InverseTimeUnits => "/Time".to_string(),
			Function::InverseNanoseconds => "/ns".to_string(),
			Function::InverseMicroseconds => "/μs".to_string(),
			Function::InverseMilliseconds => "/ms".to_string(),
			Function::InverseSeconds => "/sec".to_string(),
			Function::InverseMinutes => "/min".to_string(),
			Function::InverseHours => "/hr".to_string(),
			Function::InverseDays => "/day".to_string(),
			Function::InverseYears => "/yr".to_string(),
			Function::ToTimeUnits => "▸Time".to_string(),
			Function::ToNanoseconds => "▸ns".to_string(),
			Function::ToMicroseconds => "▸μs".to_string(),
			Function::ToMilliseconds => "▸ms".to_string(),
			Function::ToSeconds => "▸sec".to_string(),
			Function::ToMinutes => "▸min".to_string(),
			Function::ToHours => "▸hr".to_string(),
			Function::ToDays => "▸day".to_string(),
			Function::ToYears => "▸yr".to_string(),
			Function::DistanceUnits => "Dist".to_string(),
			Function::Meters => "m".to_string(),
			Function::Inches => "in".to_string(),
			Function::Feet => "ft".to_string(),
			Function::Yards => "yd".to_string(),
			Function::Miles => "mi".to_string(),
			Function::NauticalMiles => "nmi".to_string(),
			Function::Nanometers => "nm".to_string(),
			Function::Micrometers => "μm".to_string(),
			Function::Millimeters => "mm".to_string(),
			Function::Centimeters => "cm".to_string(),
			Function::Kilometers => "km".to_string(),
			Function::AstronomicalUnits => "au".to_string(),
			Function::InverseDistanceUnits => "/Dist".to_string(),
			Function::InverseMeters => "/m".to_string(),
			Function::InverseInches => "/in".to_string(),
			Function::InverseFeet => "/ft".to_string(),
			Function::InverseYards => "/yd".to_string(),
			Function::InverseMiles => "/mi".to_string(),
			Function::InverseNauticalMiles => "/nmi".to_string(),
			Function::InverseNanometers => "/nm".to_string(),
			Function::InverseMicrometers => "/μm".to_string(),
			Function::InverseMillimeters => "/mm".to_string(),
			Function::InverseCentimeters => "/cm".to_string(),
			Function::InverseKilometers => "/km".to_string(),
			Function::InverseAstronomicalUnits => "/au".to_string(),
			Function::ToDistanceUnits => "▸Dist".to_string(),
			Function::ToMeters => "▸m".to_string(),
			Function::ToInches => "▸in".to_string(),
			Function::ToFeet => "▸ft".to_string(),
			Function::ToYards => "▸yd".to_string(),
			Function::ToMiles => "▸mi".to_string(),
			Function::ToNauticalMiles => "▸nmi".to_string(),
			Function::ToNanometers => "▸nm".to_string(),
			Function::ToMicrometers => "▸μm".to_string(),
			Function::ToMillimeters => "▸mm".to_string(),
			Function::ToCentimeters => "▸cm".to_string(),
			Function::ToKilometers => "▸km".to_string(),
			Function::ToAstronomicalUnits => "▸au".to_string(),
		}
	}

	pub fn execute(&self, state: &mut State) -> Result<()> {
		match self {
			Function::Input(input) => {
				state.handle_input(*input)?;
			}
			Function::NormalFormat => {
				state.format.mode = NumberFormatMode::Normal;
				state.stack.end_edit();
			}
			Function::RationalFormat => {
				state.format.mode = NumberFormatMode::Rational;
				state.stack.end_edit();
			}
			Function::ScientificFormat => {
				state.format.mode = NumberFormatMode::Scientific;
				state.stack.end_edit();
			}
			Function::EngineeringFormat => {
				state.format.mode = NumberFormatMode::Engineering;
				state.stack.end_edit();
			}
			Function::AlternateHex => {
				state.format.show_alt_hex = !state.format.show_alt_hex;
			}
			Function::AlternateFloat => {
				state.format.show_alt_float = !state.format.show_alt_float;
			}
			Function::ThousandsSeparatorOff => {
				state.format.thousands = false;
			}
			Function::ThousandsSeparatorOn => {
				state.format.thousands = true;
			}
			Function::DecimalPointPeriod => {
				state.format.decimal_point = NumberDecimalPointMode::Period;
			}
			Function::DecimalPointComma => {
				state.format.decimal_point = NumberDecimalPointMode::Comma;
			}
			Function::Float => {
				if state.format.integer_radix == 10 {
					state.format.integer_mode = IntegerMode::Float;
					state.stack.end_edit();
				} else {
					return Err(Error::FloatRequiresDecimalMode);
				}
			}
			Function::SignedInteger => {
				state.function_keys.show_menu(FunctionMenu::SignedInteger);
			}
			Function::UnsignedInteger => {
				state.function_keys.show_menu(FunctionMenu::UnsignedInteger);
			}
			Function::BigInteger => {
				state.format.integer_mode = IntegerMode::BigInteger;
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Signed8Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(8, true);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Signed16Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(16, true);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Signed32Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(32, true);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Signed64Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(64, true);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Signed128Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(128, true);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Unsigned8Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(8, false);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Unsigned16Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(16, false);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Unsigned32Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(32, false);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Unsigned64Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(64, false);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::Unsigned128Bit => {
				state.format.integer_mode = IntegerMode::SizedInteger(128, false);
				state.default_integer_format = state.format.integer_mode;
				state.stack.end_edit();
			}
			Function::And => {
				let value = Value::Number(Number::Integer(
					&*state.stack.entry(1)?.to_int()? & &*state.stack.entry(0)?.to_int()?,
				));
				state.replace_entries(2, value)?;
			}
			Function::Or => {
				let value = Value::Number(Number::Integer(
					&*state.stack.entry(1)?.to_int()? | &*state.stack.entry(0)?.to_int()?,
				));
				state.replace_entries(2, value)?;
			}
			Function::Xor => {
				let value = Value::Number(Number::Integer(
					&*state.stack.entry(1)?.to_int()? ^ &*state.stack.entry(0)?.to_int()?,
				));
				state.replace_entries(2, value)?;
			}
			Function::Not => {
				let value = Number::Integer(!&*state.stack.top().to_int()?);
				state.set_top(Value::Number(value))?;
			}
			Function::ShiftLeft => {
				let x = state.stack.entry(0)?;
				let mut x = x.to_int()?;
				if let IntegerMode::SizedInteger(size, _) = state.format.integer_mode {
					if size.is_power_of_two() {
						x = Cow::Owned(&*x & &(size - 1).to_bigint().unwrap());
					}
				}
				let x = u32::try_from(&*x)?;
				let y = state.stack.entry(1)?;
				let y = y.to_int()?;
				if (y.bits() + x as u64) > MAX_INTEGER_BITS {
					return Err(Error::ValueOutOfRange);
				}
				let value = Value::Number(Number::Integer(&*y << x));
				state.replace_entries(2, value)?;
			}
			Function::ShiftRight => {
				let x = state.stack.entry(0)?;
				let mut x = x.to_int()?;
				if let IntegerMode::SizedInteger(size, _) = state.format.integer_mode {
					if size.is_power_of_two() {
						x = Cow::Owned(&*x & (size - 1).to_bigint().unwrap());
					}
				}
				let x = u32::try_from(&*x)?;
				let y = state.stack.entry(1)?;
				let y = y.to_int()?;
				let value = Value::Number(Number::Integer(&*y >> x));
				state.replace_entries(2, value)?;
			}
			Function::RotateLeft => {
				if let IntegerMode::SizedInteger(size, _) = state.format.integer_mode {
					let x = state.stack.entry(0)?;
					let mut x = x.to_int()?;
					if size.is_power_of_two() {
						x = Cow::Owned(&*x & (size - 1).to_bigint().unwrap());
					}
					if let Ok(x) = u32::try_from(&*x) {
						let y = state.stack.entry(1)?;
						let y = y.to_int()?;
						let value = (&*y << x) | (&*y >> ((size as u32) - x));
						state.replace_entries(2, Value::Number(Number::Integer(value)))?;
					}
				} else {
					return Err(Error::RequiresSizedIntegerMode);
				}
			}
			Function::RotateRight => {
				if let IntegerMode::SizedInteger(size, _) = state.format.integer_mode {
					let x = state.stack.entry(0)?;
					let mut x = x.to_int()?;
					if size.is_power_of_two() {
						x = Cow::Owned(&*x & (size - 1).to_bigint().unwrap());
					}
					if let Ok(x) = u32::try_from(&*x) {
						let y = state.stack.entry(1)?;
						let y = y.to_int()?;
						let value = (&*y >> x) | (&*y << ((size as u32) - x));
						state.replace_entries(2, Value::Number(Number::Integer(value)))?;
					}
				} else {
					return Err(Error::RequiresSizedIntegerMode);
				}
			}
			Function::Hex => {
				if state.format.integer_radix == 10 {
					state.prev_decimal_integer_mode = state.format.integer_mode;
					state.format.integer_mode = state.default_integer_format;
				}
				state.format.integer_radix = 16;
				state.stack.end_edit();
			}
			Function::Octal => {
				if state.format.integer_radix == 10 {
					state.prev_decimal_integer_mode = state.format.integer_mode;
					state.format.integer_mode = state.default_integer_format;
				}
				state.format.integer_radix = 8;
				state.stack.end_edit();
			}
			Function::Decimal => {
				if state.format.integer_radix != 10 {
					state.format.integer_mode = state.prev_decimal_integer_mode;
				}
				state.format.integer_radix = 10;
				state.stack.end_edit();
			}
			Function::ConstCatalog => state.function_keys.show_menu(FunctionMenu::ConstCatalog),
			Function::SpeedOfLight => {
				state.stack.input_value(Value::NumberWithUnit(
					299_792_458.to_number(),
					CompositeUnit::ratio_unit(
						DistanceUnit::Meters.into(),
						TimeUnit::Seconds.into(),
					),
				))?;
			}
			Function::TimeCatalog => state.function_keys.show_menu(FunctionMenu::TimeCatalog),
			Function::Now => {
				state
					.stack
					.input_value(Value::DateTime(NaiveDateTime::now()))?;
			}
			Function::Date => {
				if let Value::DateTime(dt) = state.stack.top() {
					let date = dt.date();
					state.stack.set_top(Value::Date(date))?;
				} else {
					let year = i32::try_from(&*state.stack.entry(2)?.to_int()?)?;
					let month = u8::try_from(&*state.stack.entry(1)?.to_int()?)?;
					let day = u8::try_from(&*state.stack.entry(0)?.to_int()?)?;
					let date = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
						.ok_or(Error::InvalidDate)?;
					{
						state.stack.replace_entries(3, Value::Date(date))?;
					}
				}
			}
			Function::Time => {
				if let Value::DateTime(dt) = state.stack.top() {
					let time = dt.time();
					state.stack.set_top(Value::Time(time))?;
				} else {
					let nano = (state.stack.entry(0)?
						* Value::Number(Number::Integer(1_000_000_000.to_bigint().unwrap())))?;
					let hr = u8::try_from(&*state.stack.entry(2)?.to_int()?)?;
					let min = u8::try_from(&*state.stack.entry(1)?.to_int()?)?;
					let sec = u64::try_from(&*nano.to_int()?)?;
					let nsec = (sec % 1_000_000_000) as u32;
					let sec = (sec / 1_000_000_000) as u32;
					let time = NaiveTime::from_hms_nano_opt(hr as u32, min as u32, sec, nsec)
						.ok_or(Error::InvalidTime)?;
					state.stack.replace_entries(3, Value::Time(time))?;
				}
			}
			Function::TimeUnits => state.function_keys.show_menu(FunctionMenu::TimeUnit),
			Function::Nanoseconds => {
				let value = state.stack.top().add_unit(TimeUnit::Nanoseconds.into())?;
				state.set_top(value)?;
			}
			Function::Microseconds => {
				let value = state.stack.top().add_unit(TimeUnit::Microseconds.into())?;
				state.set_top(value)?;
			}
			Function::Milliseconds => {
				let value = state.stack.top().add_unit(TimeUnit::Milliseconds.into())?;
				state.set_top(value)?;
			}
			Function::Seconds => {
				let value = state.stack.top().add_unit(TimeUnit::Seconds.into())?;
				state.set_top(value)?;
			}
			Function::Minutes => {
				let value = state.stack.top().add_unit(TimeUnit::Minutes.into())?;
				state.set_top(value)?;
			}
			Function::Hours => {
				let value = state.stack.top().add_unit(TimeUnit::Hours.into())?;
				state.set_top(value)?;
			}
			Function::Days => {
				let value = state.stack.top().add_unit(TimeUnit::Days.into())?;
				state.set_top(value)?;
			}
			Function::Years => {
				let value = state.stack.top().add_unit(TimeUnit::Years.into())?;
				state.set_top(value)?;
			}
			Function::InverseTimeUnits => {
				state.function_keys.show_menu(FunctionMenu::InverseTimeUnit)
			}
			Function::InverseNanoseconds => {
				let value = state
					.stack
					.top()
					.add_unit_inv(TimeUnit::Nanoseconds.into())?;
				state.set_top(value)?;
			}
			Function::InverseMicroseconds => {
				let value = state
					.stack
					.top()
					.add_unit_inv(TimeUnit::Microseconds.into())?;
				state.set_top(value)?;
			}
			Function::InverseMilliseconds => {
				let value = state
					.stack
					.top()
					.add_unit_inv(TimeUnit::Milliseconds.into())?;
				state.set_top(value)?;
			}
			Function::InverseSeconds => {
				let value = state.stack.top().add_unit_inv(TimeUnit::Seconds.into())?;
				state.set_top(value)?;
			}
			Function::InverseMinutes => {
				let value = state.stack.top().add_unit_inv(TimeUnit::Minutes.into())?;
				state.set_top(value)?;
			}
			Function::InverseHours => {
				let value = state.stack.top().add_unit_inv(TimeUnit::Hours.into())?;
				state.set_top(value)?;
			}
			Function::InverseDays => {
				let value = state.stack.top().add_unit_inv(TimeUnit::Days.into())?;
				state.set_top(value)?;
			}
			Function::InverseYears => {
				let value = state.stack.top().add_unit_inv(TimeUnit::Years.into())?;
				state.set_top(value)?;
			}
			Function::ToTimeUnits => state.function_keys.show_menu(FunctionMenu::ToTimeUnit),
			Function::ToNanoseconds => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Nanoseconds.into())?;
				state.set_top(value)?;
			}
			Function::ToMicroseconds => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Microseconds.into())?;
				state.set_top(value)?;
			}
			Function::ToMilliseconds => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Milliseconds.into())?;
				state.set_top(value)?;
			}
			Function::ToSeconds => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Seconds.into())?;
				state.set_top(value)?;
			}
			Function::ToMinutes => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Minutes.into())?;
				state.set_top(value)?;
			}
			Function::ToHours => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Hours.into())?;
				state.set_top(value)?;
			}
			Function::ToDays => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Days.into())?;
				state.set_top(value)?;
			}
			Function::ToYears => {
				let value = state
					.stack
					.top()
					.convert_single_unit(TimeUnit::Years.into())?;
				state.set_top(value)?;
			}
			Function::DistanceUnits => state.function_keys.show_menu(FunctionMenu::DistanceUnit),
			Function::Nanometers => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::Nanometers.into())?;
				state.set_top(value)?;
			}
			Function::Micrometers => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::Micrometers.into())?;
				state.set_top(value)?;
			}
			Function::Millimeters => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::Millimeters.into())?;
				state.set_top(value)?;
			}
			Function::Centimeters => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::Centimeters.into())?;
				state.set_top(value)?;
			}
			Function::Meters => {
				let value = state.stack.top().add_unit(DistanceUnit::Meters.into())?;
				state.set_top(value)?;
			}
			Function::Kilometers => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::Kilometers.into())?;
				state.set_top(value)?;
			}
			Function::Inches => {
				let value = state.stack.top().add_unit(DistanceUnit::Inches.into())?;
				state.set_top(value)?;
			}
			Function::Feet => {
				let value = state.stack.top().add_unit(DistanceUnit::Feet.into())?;
				state.set_top(value)?;
			}
			Function::Yards => {
				let value = state.stack.top().add_unit(DistanceUnit::Yards.into())?;
				state.set_top(value)?;
			}
			Function::Miles => {
				let value = state.stack.top().add_unit(DistanceUnit::Miles.into())?;
				state.set_top(value)?;
			}
			Function::NauticalMiles => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::NauticalMiles.into())?;
				state.set_top(value)?;
			}
			Function::AstronomicalUnits => {
				let value = state
					.stack
					.top()
					.add_unit(DistanceUnit::AstronomicalUnits.into())?;
				state.set_top(value)?;
			}
			Function::InverseDistanceUnits => state
				.function_keys
				.show_menu(FunctionMenu::InverseDistanceUnit),
			Function::InverseNanometers => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Nanometers.into())?;
				state.set_top(value)?;
			}
			Function::InverseMicrometers => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Micrometers.into())?;
				state.set_top(value)?;
			}
			Function::InverseMillimeters => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Millimeters.into())?;
				state.set_top(value)?;
			}
			Function::InverseCentimeters => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Centimeters.into())?;
				state.set_top(value)?;
			}
			Function::InverseMeters => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Meters.into())?;
				state.set_top(value)?;
			}
			Function::InverseKilometers => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Kilometers.into())?;
				state.set_top(value)?;
			}
			Function::InverseInches => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::Inches.into())?;
				state.set_top(value)?;
			}
			Function::InverseFeet => {
				let value = state.stack.top().add_unit_inv(DistanceUnit::Feet.into())?;
				state.set_top(value)?;
			}
			Function::InverseYards => {
				let value = state.stack.top().add_unit_inv(DistanceUnit::Yards.into())?;
				state.set_top(value)?;
			}
			Function::InverseMiles => {
				let value = state.stack.top().add_unit_inv(DistanceUnit::Miles.into())?;
				state.set_top(value)?;
			}
			Function::InverseNauticalMiles => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::NauticalMiles.into())?;
				state.set_top(value)?;
			}
			Function::InverseAstronomicalUnits => {
				let value = state
					.stack
					.top()
					.add_unit_inv(DistanceUnit::AstronomicalUnits.into())?;
				state.set_top(value)?;
			}
			Function::ToDistanceUnits => {
				state.function_keys.show_menu(FunctionMenu::ToDistanceUnit)
			}
			Function::ToNanometers => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Nanometers.into())?;
				state.set_top(value)?;
			}
			Function::ToMicrometers => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Micrometers.into())?;
				state.set_top(value)?;
			}
			Function::ToMillimeters => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Millimeters.into())?;
				state.set_top(value)?;
			}
			Function::ToCentimeters => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Centimeters.into())?;
				state.set_top(value)?;
			}
			Function::ToMeters => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Meters.into())?;
				state.set_top(value)?;
			}
			Function::ToKilometers => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Kilometers.into())?;
				state.set_top(value)?;
			}
			Function::ToInches => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Inches.into())?;
				state.set_top(value)?;
			}
			Function::ToFeet => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Feet.into())?;
				state.set_top(value)?;
			}
			Function::ToYards => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Yards.into())?;
				state.set_top(value)?;
			}
			Function::ToMiles => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::Miles.into())?;
				state.set_top(value)?;
			}
			Function::ToNauticalMiles => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::NauticalMiles.into())?;
				state.set_top(value)?;
			}
			Function::ToAstronomicalUnits => {
				let value = state
					.stack
					.top()
					.convert_single_unit(DistanceUnit::AstronomicalUnits.into())?;
				state.set_top(value)?;
			}
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FunctionMenu {
	Disp,
	Base,
	SignedInteger,
	UnsignedInteger,
	Logic,
	Catalog,
	ConstCatalog,
	TimeCatalog,
	Units,
	TimeUnit,
	InverseTimeUnit,
	ToTimeUnit,
	DistanceUnit,
	InverseDistanceUnit,
	ToDistanceUnit,
}

impl FunctionMenu {
	pub fn functions(&self) -> Vec<Option<Function>> {
		match self {
			FunctionMenu::Disp => [
				Some(Function::NormalFormat),
				Some(Function::RationalFormat),
				Some(Function::ScientificFormat),
				Some(Function::EngineeringFormat),
				Some(Function::AlternateHex),
				Some(Function::AlternateFloat),
				Some(Function::ThousandsSeparatorOff),
				Some(Function::ThousandsSeparatorOn),
				Some(Function::DecimalPointPeriod),
				Some(Function::DecimalPointComma),
			]
			.to_vec(),
			FunctionMenu::Base => [
				Some(Function::Decimal),
				Some(Function::Octal),
				Some(Function::Hex),
				Some(Function::Float),
				Some(Function::SignedInteger),
				Some(Function::UnsignedInteger),
			]
			.to_vec(),
			FunctionMenu::SignedInteger => [
				Some(Function::BigInteger),
				Some(Function::Signed8Bit),
				Some(Function::Signed16Bit),
				Some(Function::Signed32Bit),
				Some(Function::Signed64Bit),
				Some(Function::Signed128Bit),
			]
			.to_vec(),
			FunctionMenu::UnsignedInteger => [
				Some(Function::BigInteger),
				Some(Function::Unsigned8Bit),
				Some(Function::Unsigned16Bit),
				Some(Function::Unsigned32Bit),
				Some(Function::Unsigned64Bit),
				Some(Function::Unsigned128Bit),
			]
			.to_vec(),
			FunctionMenu::Logic => [
				Some(Function::And),
				Some(Function::Or),
				Some(Function::Xor),
				Some(Function::Not),
				Some(Function::ShiftLeft),
				Some(Function::ShiftRight),
				Some(Function::RotateLeft),
				Some(Function::RotateRight),
			]
			.to_vec(),
			FunctionMenu::Catalog => {
				[Some(Function::ConstCatalog), Some(Function::TimeCatalog)].to_vec()
			}
			FunctionMenu::ConstCatalog => [Some(Function::SpeedOfLight)].to_vec(),
			FunctionMenu::TimeCatalog => [
				Some(Function::Now),
				Some(Function::Date),
				Some(Function::Time),
			]
			.to_vec(),
			FunctionMenu::Units => [
				Some(Function::DistanceUnits),
				Some(Function::TimeUnits),
				None,
				None,
				None,
				None,
				Some(Function::ToDistanceUnits),
				Some(Function::ToTimeUnits),
				None,
				None,
				None,
				None,
				Some(Function::InverseDistanceUnits),
				Some(Function::InverseTimeUnits),
				None,
				None,
				None,
				None,
			]
			.to_vec(),
			FunctionMenu::TimeUnit => [
				Some(Function::Seconds),
				Some(Function::Minutes),
				Some(Function::Hours),
				Some(Function::Days),
				Some(Function::Years),
				None,
				Some(Function::Milliseconds),
				Some(Function::Microseconds),
				Some(Function::Nanoseconds),
			]
			.to_vec(),
			FunctionMenu::InverseTimeUnit => [
				Some(Function::InverseSeconds),
				Some(Function::InverseMinutes),
				Some(Function::InverseHours),
				Some(Function::InverseDays),
				Some(Function::InverseYears),
				None,
				Some(Function::InverseMilliseconds),
				Some(Function::InverseMicroseconds),
				Some(Function::InverseNanoseconds),
			]
			.to_vec(),
			FunctionMenu::ToTimeUnit => [
				Some(Function::ToSeconds),
				Some(Function::ToMinutes),
				Some(Function::ToHours),
				Some(Function::ToDays),
				Some(Function::ToYears),
				None,
				Some(Function::ToMilliseconds),
				Some(Function::ToMicroseconds),
				Some(Function::ToNanoseconds),
			]
			.to_vec(),
			FunctionMenu::DistanceUnit => [
				Some(Function::Meters),
				Some(Function::Kilometers),
				Some(Function::Feet),
				Some(Function::Yards),
				Some(Function::Miles),
				Some(Function::NauticalMiles),
				Some(Function::Nanometers),
				Some(Function::Micrometers),
				Some(Function::Millimeters),
				Some(Function::Centimeters),
				Some(Function::Inches),
				Some(Function::AstronomicalUnits),
			]
			.to_vec(),
			FunctionMenu::InverseDistanceUnit => [
				Some(Function::InverseMeters),
				Some(Function::InverseKilometers),
				Some(Function::InverseFeet),
				Some(Function::InverseYards),
				Some(Function::InverseMiles),
				Some(Function::InverseNauticalMiles),
				Some(Function::InverseNanometers),
				Some(Function::InverseMicrometers),
				Some(Function::InverseMillimeters),
				Some(Function::InverseCentimeters),
				Some(Function::InverseInches),
				Some(Function::InverseAstronomicalUnits),
			]
			.to_vec(),
			FunctionMenu::ToDistanceUnit => [
				Some(Function::ToMeters),
				Some(Function::ToKilometers),
				Some(Function::ToFeet),
				Some(Function::ToYards),
				Some(Function::ToMiles),
				Some(Function::ToNauticalMiles),
				Some(Function::ToNanometers),
				Some(Function::ToMicrometers),
				Some(Function::ToMillimeters),
				Some(Function::ToCentimeters),
				Some(Function::ToInches),
				Some(Function::ToAstronomicalUnits),
			]
			.to_vec(),
		}
	}
}

pub struct FunctionKeyState {
	menu: Option<FunctionMenu>,
	functions: Vec<Option<Function>>,
	page: usize,
	menu_stack: Vec<(Option<FunctionMenu>, usize)>,
	quick_functions: Vec<Option<Function>>,
	menu_strings: RefCell<Vec<String>>,
}

impl FunctionKeyState {
	pub fn new() -> Self {
		FunctionKeyState {
			menu: None,
			functions: Vec::new(),
			page: 0,
			menu_stack: Vec::new(),
			quick_functions: Vec::new(),
			menu_strings: RefCell::new(Vec::new()),
		}
	}

	pub fn function(&self, idx: u8) -> Option<Function> {
		if let Some(func) = self.functions.get(self.page * 6 + (idx as usize - 1)) {
			func.clone()
		} else {
			None
		}
	}

	fn quick_functions(&self, format: &NumberFormat) -> Vec<Option<Function>> {
		let mut result = Vec::new();
		if format.integer_radix == 16 {
			result.push(Some(Function::Input(InputEvent::Character('A'))));
			result.push(Some(Function::Input(InputEvent::Character('B'))));
			result.push(Some(Function::Input(InputEvent::Character('C'))));
			result.push(Some(Function::Input(InputEvent::Character('D'))));
			result.push(Some(Function::Input(InputEvent::Character('E'))));
			result.push(Some(Function::Input(InputEvent::Character('F'))));
		}
		result.append(&mut self.quick_functions.clone());
		result
	}

	pub fn update(&mut self, format: &NumberFormat) {
		// Update function list from current menu
		if let Some(menu) = self.menu {
			self.functions = menu.functions();
		} else {
			self.functions = self.quick_functions(format);
		}

		// Ensure current page is within bounds
		if self.functions.len() == 0 {
			self.page = 0;
		} else {
			let max_page = (self.functions.len() + 5) / 6;
			if self.page >= max_page {
				self.page = max_page - 1;
			}
		}
	}

	pub fn update_menu_strings(&self, state: &State) -> bool {
		let mut strings = Vec::new();
		for i in 0..6 {
			if let Some(function) = self.function((i + 1) as u8) {
				strings.push(function.to_str(state));
			} else {
				strings.push("".to_string());
			}
		}
		if strings != *self.menu_strings.borrow() {
			*self.menu_strings.borrow_mut() = strings;
			true
		} else {
			false
		}
	}

	pub fn exit_menu(&mut self, format: &NumberFormat) {
		// Set menu state from previous stack entry and update the function list
		if let Some((menu, page)) = self.menu_stack.pop() {
			self.menu = menu;
			self.page = page;
			self.update(format);
		}
	}

	pub fn show_menu(&mut self, menu: FunctionMenu) {
		self.menu_stack.push((self.menu, self.page));
		self.menu = Some(menu);
		self.functions = menu.functions();
		self.page = 0;
	}

	pub fn show_toplevel_menu(&mut self, menu: FunctionMenu) {
		self.menu_stack.clear();
		self.menu_stack.push((None, 0));
		self.menu = Some(menu);
		self.functions = menu.functions();
		self.page = 0;
	}

	pub fn prev_page(&mut self) {
		if self.page == 0 {
			let page_count = (self.functions.len() + 5) / 6;
			if page_count > 1 {
				self.page = page_count - 1;
			}
		} else {
			self.page -= 1;
		}
	}

	pub fn next_page(&mut self) {
		let page_count = (self.functions.len() + 5) / 6;
		if (self.page + 1) < page_count {
			self.page += 1;
		} else {
			self.page = 0;
		}
	}

	pub fn multiple_pages(&self) -> bool {
		self.functions.len() > 6
	}

	pub fn render<ScreenT: Screen>(&self, screen: &mut ScreenT) {
		let top = screen.height() - SANS_13.height;

		// Clear menu area
		screen.fill(
			Rect {
				x: 0,
				y: top - 1,
				w: screen.width(),
				h: SANS_13.height + 1,
			},
			Color::ContentBackground,
		);

		// Render each function key display
		for i in 0..6 {
			let min_x = (screen.width() - 1) * i / 6;
			let max_x = (screen.width() - 1) * (i + 1) / 6;

			// Render key background
			screen.fill(
				Rect {
					x: min_x + 1,
					y: top,
					w: max_x - min_x - 1,
					h: SANS_13.height,
				},
				Color::MenuBackground,
			);
			screen.set_pixel(min_x + 1, top, Color::ContentBackground);
			screen.set_pixel(max_x - 1, top, Color::ContentBackground);

			// Render key text if there is one
			if let Some(string) = self.menu_strings.borrow().get(i as usize) {
				let mut string = string.clone();

				// Trim string until it fits
				let mut width = SANS_13.width(&string);
				while string.len() > 1 {
					if width > max_x - min_x {
						string.pop();
						width = SANS_13.width(&string);
					} else {
						break;
					}
				}

				// Draw key text centered in button
				SANS_13.draw(
					screen,
					(min_x + max_x) / 2 - (width / 2),
					top,
					&string,
					Color::MenuText,
				);
			}
		}
	}

	pub fn height(&self) -> i32 {
		SANS_13.height + 1
	}
}
