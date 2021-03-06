#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[repr(C)]
#[repr(align(16))]
#[derive(Clone)]
pub struct Decimal {
	parts: [u64; 2],
}

pub trait ToDecimal {
	fn to_decimal(self) -> Decimal;
}

#[repr(C)]
#[allow(dead_code)]
enum Class {
	SignalingNaN,
	QuietNaN,
	NegativeInfinity,
	NegativeNormal,
	NegativeSubnormal,
	NegativeZero,
	PositiveZero,
	PositiveSubnormal,
	PositiveNormal,
	PositiveInfinity,
}

extern "C" {
	fn __bid128_from_int32(result: *mut Decimal, n: &i32);
	fn __bid128_from_uint32(result: *mut Decimal, n: &u32);
	fn __bid128_from_int64(result: *mut Decimal, n: &i64);
	fn __bid128_from_uint64(result: *mut Decimal, n: &u64);
	fn __binary32_to_bid128(result: *mut Decimal, n: &f32);
	fn __binary64_to_bid128(result: *mut Decimal, n: &f64);
	fn __bid128_to_string(dest: *mut u8, n: &Decimal);
	fn __bid128_from_string(dest: *mut Decimal, string: *const u8);
	fn __bid128_add(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_sub(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_mul(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_div(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_fmod(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_modf(result: *mut Decimal, x: &Decimal, int: *mut Decimal);
	fn __bid128_fma(result: *mut Decimal, x: &Decimal, y: &Decimal, z: &Decimal);
	fn __bid128_exp(result: *mut Decimal, x: &Decimal);
	fn __bid128_log(result: *mut Decimal, x: &Decimal);
	fn __bid128_pow(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_atan2(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_hypot(result: *mut Decimal, x: &Decimal, y: &Decimal);
	fn __bid128_sin(result: *mut Decimal, x: &Decimal);
	fn __bid128_cos(result: *mut Decimal, x: &Decimal);
	fn __bid128_tan(result: *mut Decimal, x: &Decimal);
	fn __bid128_atan(result: *mut Decimal, x: &Decimal);
	fn __bid128_log1p(result: *mut Decimal, x: &Decimal);
	fn __bid128_expm1(result: *mut Decimal, x: &Decimal);
	fn __bid128_log10(result: *mut Decimal, x: &Decimal);
	fn __bid128_log2(result: *mut Decimal, x: &Decimal);
	fn __bid128_exp10(result: *mut Decimal, x: &Decimal);
	fn __bid128_exp2(result: *mut Decimal, x: &Decimal);
	fn __bid128_erf(result: *mut Decimal, x: &Decimal);
	fn __bid128_erfc(result: *mut Decimal, x: &Decimal);
	fn __bid128_tgamma(result: *mut Decimal, x: &Decimal);
	fn __bid128_lgamma(result: *mut Decimal, x: &Decimal);
	fn __bid128_cbrt(result: *mut Decimal, x: &Decimal);
	fn __bid128_abs(result: *mut Decimal, x: &Decimal);
	fn __bid128_negate(result: *mut Decimal, x: &Decimal);
	fn __bid128_class(result: *mut Class, x: &Decimal);
	fn __bid128_isSigned(result: *mut i32, x: &Decimal);
	fn __bid128_isNormal(result: *mut i32, x: &Decimal);
	fn __bid128_isFinite(result: *mut i32, x: &Decimal);
	fn __bid128_isInf(result: *mut i32, x: &Decimal);
	fn __bid128_isNaN(result: *mut i32, x: &Decimal);
	fn __bid128_quiet_equal(result: *mut i32, x: &Decimal, y: &Decimal);
	fn __bid128_quiet_unordered(result: *mut i32, x: &Decimal, y: &Decimal);
	fn __bid128_quiet_greater(result: *mut i32, x: &Decimal, y: &Decimal);
}

impl Decimal {
	pub fn new() -> Self {
		0.into()
	}

	pub fn zero() -> Self {
		0.into()
	}

	pub fn from_str(string: &str) -> Self {
		let mut buf: Vec<u8> = string.as_bytes().to_vec();
		buf.push(0);
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_from_string(result.as_mut_ptr(), buf.as_ptr());
			result.assume_init()
		}
	}

	pub fn pi() -> Self {
		Decimal::from_str("3.141592653589793238462643383279503")
	}

	pub fn to_string(&self) -> String {
		let mut buf = [0; 64];
		unsafe {
			__bid128_to_string(&mut buf[0], &self);
		}
		let mut end = 64;
		for i in 0..64 {
			if buf[i] == 0 {
				end = i;
				break;
			}
		}
		String::from_utf8_lossy(&buf[0..end]).to_string()
	}

	pub fn sqrt(&self) -> Self {
		let one: Decimal = 1.into();
		let two: Decimal = 2.into();
		let one_half = one / two;
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			// Don't use sqrt here, we don't have a libm on dm42
			__bid128_pow(result.as_mut_ptr(), &self, &one_half);
			result.assume_init()
		}
	}

	pub fn exp(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_exp(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn ln(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_log(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn pow(&self, power: &Self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_pow(result.as_mut_ptr(), &self, power);
			result.assume_init()
		}
	}

	pub fn sin(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_sin(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn cos(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_cos(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn tan(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_tan(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn asin(&self) -> Self {
		Self::atan2(self, &(Decimal::from(1) - self * self).sqrt())
	}

	pub fn acos(&self) -> Self {
		Self::atan2(&(Decimal::from(1) - self * self).sqrt(), self)
	}

	pub fn atan(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_atan(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn sinh(&self) -> Self {
		(1.to_decimal() - (&(-2).to_decimal() * self).exp()) / (2.to_decimal() * (-self).exp())
	}

	pub fn cosh(&self) -> Self {
		(1.to_decimal() + (&(-2).to_decimal() * self).exp()) / (2.to_decimal() * (-self).exp())
	}

	pub fn tanh(&self) -> Self {
		let e_2x = (&2.to_decimal() * self).exp();
		(&e_2x - &1.to_decimal()) / (&e_2x + &1.to_decimal())
	}

	pub fn asinh(&self) -> Self {
		(self + &(self * self + 1.to_decimal()).sqrt()).ln()
	}

	pub fn acosh(&self) -> Self {
		(self + &(self * self - 1.to_decimal()).sqrt()).ln()
	}

	pub fn atanh(&self) -> Self {
		((&1.to_decimal() + self) / (&1.to_decimal() - self)).ln() / 2.to_decimal()
	}

	pub fn ln_1p(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_log1p(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn exp_m1(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_expm1(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn log10(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_log10(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn log2(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_log2(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn exp10(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_exp10(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn exp2(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_exp2(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn erf(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_erf(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn erfc(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_erfc(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn tgamma(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_tgamma(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn lgamma(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_lgamma(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn cbrt(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_cbrt(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn fract(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		let mut int = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_modf(result.as_mut_ptr(), &self, int.as_mut_ptr());
			result.assume_init()
		}
	}

	pub fn trunc(&self) -> Self {
		let mut fract = core::mem::MaybeUninit::<Decimal>::uninit();
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_modf(fract.as_mut_ptr(), &self, result.as_mut_ptr());
			result.assume_init()
		}
	}

	pub fn abs(&self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_abs(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}

	pub fn classify(&self) -> core::num::FpCategory {
		let class = unsafe {
			let mut class = core::mem::MaybeUninit::<Class>::uninit();
			__bid128_class(class.as_mut_ptr(), &self);
			class.assume_init()
		};
		match class {
			Class::SignalingNaN | Class::QuietNaN => core::num::FpCategory::Nan,
			Class::NegativeInfinity | Class::PositiveInfinity => core::num::FpCategory::Infinite,
			Class::NegativeZero | Class::PositiveZero => core::num::FpCategory::Zero,
			Class::NegativeSubnormal | Class::PositiveSubnormal => core::num::FpCategory::Subnormal,
			Class::NegativeNormal | Class::PositiveNormal => core::num::FpCategory::Normal,
		}
	}

	pub fn is_sign_negative(&self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_isSigned(result.as_mut_ptr(), &self);
			result.assume_init() != 0
		}
	}

	pub fn is_sign_positive(&self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_isSigned(result.as_mut_ptr(), &self);
			result.assume_init() == 0
		}
	}

	pub fn is_normal(&self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_isNormal(result.as_mut_ptr(), &self);
			result.assume_init() != 0
		}
	}

	pub fn is_finite(&self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_isFinite(result.as_mut_ptr(), &self);
			result.assume_init() != 0
		}
	}

	pub fn is_infinite(&self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_isInf(result.as_mut_ptr(), &self);
			result.assume_init() != 0
		}
	}

	pub fn is_nan(&self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_isNaN(result.as_mut_ptr(), &self);
			result.assume_init() != 0
		}
	}

	pub fn fma(x: &Self, y: &Self, z: &Self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_fma(result.as_mut_ptr(), x, y, z);
			result.assume_init()
		}
	}

	pub fn atan2(x: &Self, y: &Self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_atan2(result.as_mut_ptr(), x, y);
			result.assume_init()
		}
	}

	pub fn hypot(x: &Self, y: &Self) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_hypot(result.as_mut_ptr(), x, y);
			result.assume_init()
		}
	}

	pub fn from_raw(value: [u64; 2]) -> Self {
		Decimal { parts: value }
	}

	pub fn to_raw(&self) -> &[u64; 2] {
		&self.parts
	}
}

impl From<i32> for Decimal {
	fn from(value: i32) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_from_int32(result.as_mut_ptr(), &value);
			result.assume_init()
		}
	}
}

impl From<u32> for Decimal {
	fn from(value: u32) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_from_uint32(result.as_mut_ptr(), &value);
			result.assume_init()
		}
	}
}

impl From<i64> for Decimal {
	fn from(value: i64) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_from_int64(result.as_mut_ptr(), &value);
			result.assume_init()
		}
	}
}

impl From<u64> for Decimal {
	fn from(value: u64) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_from_uint64(result.as_mut_ptr(), &value);
			result.assume_init()
		}
	}
}

impl From<f32> for Decimal {
	fn from(value: f32) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__binary32_to_bid128(result.as_mut_ptr(), &value);
			result.assume_init()
		}
	}
}

impl From<f64> for Decimal {
	fn from(value: f64) -> Self {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__binary64_to_bid128(result.as_mut_ptr(), &value);
			result.assume_init()
		}
	}
}

impl ToDecimal for i32 {
	fn to_decimal(self) -> Decimal {
		self.into()
	}
}

impl ToDecimal for u32 {
	fn to_decimal(self) -> Decimal {
		self.into()
	}
}

impl ToDecimal for i64 {
	fn to_decimal(self) -> Decimal {
		self.into()
	}
}

impl ToDecimal for u64 {
	fn to_decimal(self) -> Decimal {
		self.into()
	}
}

impl ToDecimal for f32 {
	fn to_decimal(self) -> Decimal {
		self.into()
	}
}

impl ToDecimal for f64 {
	fn to_decimal(self) -> Decimal {
		self.into()
	}
}

impl core::ops::Add for Decimal {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_add(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::Add for &Decimal {
	type Output = Decimal;

	fn add(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_add(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::AddAssign for Decimal {
	fn add_assign(&mut self, rhs: Self) {
		unsafe {
			__bid128_add(self, &self.clone(), &rhs);
		}
	}
}

impl core::ops::Sub for Decimal {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_sub(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::Sub for &Decimal {
	type Output = Decimal;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_sub(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::SubAssign for Decimal {
	fn sub_assign(&mut self, rhs: Self) {
		unsafe {
			__bid128_sub(self, &self.clone(), &rhs);
		}
	}
}

impl core::ops::Mul for Decimal {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_mul(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::Mul for &Decimal {
	type Output = Decimal;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_mul(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::MulAssign for Decimal {
	fn mul_assign(&mut self, rhs: Self) {
		unsafe {
			__bid128_mul(self, &self.clone(), &rhs);
		}
	}
}

impl core::ops::Div for Decimal {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_div(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::Div for &Decimal {
	type Output = Decimal;

	fn div(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_div(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::DivAssign for Decimal {
	fn div_assign(&mut self, rhs: Self) {
		unsafe {
			__bid128_div(self, &self.clone(), &rhs);
		}
	}
}

impl core::ops::Rem for Decimal {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_fmod(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::Rem for &Decimal {
	type Output = Decimal;

	fn rem(self, rhs: Self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_fmod(result.as_mut_ptr(), &self, &rhs);
			result.assume_init()
		}
	}
}

impl core::ops::RemAssign for Decimal {
	fn rem_assign(&mut self, rhs: Self) {
		unsafe {
			__bid128_fmod(self, &self.clone(), &rhs);
		}
	}
}

impl core::ops::Neg for Decimal {
	type Output = Self;

	fn neg(self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_negate(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}
}

impl core::ops::Neg for &Decimal {
	type Output = Decimal;

	fn neg(self) -> Self::Output {
		let mut result = core::mem::MaybeUninit::<Decimal>::uninit();
		unsafe {
			__bid128_negate(result.as_mut_ptr(), &self);
			result.assume_init()
		}
	}
}

impl core::cmp::PartialEq for Decimal {
	fn eq(&self, other: &Self) -> bool {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		unsafe {
			__bid128_quiet_equal(result.as_mut_ptr(), &self, other);
			result.assume_init() != 0
		}
	}
}

impl core::cmp::PartialOrd for Decimal {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		let unordered = unsafe {
			__bid128_quiet_unordered(result.as_mut_ptr(), &self, other);
			result.assume_init() != 0
		};
		if unordered {
			return None;
		}

		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		let equal = unsafe {
			__bid128_quiet_equal(result.as_mut_ptr(), &self, other);
			result.assume_init() != 0
		};
		if equal {
			return Some(core::cmp::Ordering::Equal);
		}

		let mut result = core::mem::MaybeUninit::<i32>::uninit();
		let greater = unsafe {
			__bid128_quiet_greater(result.as_mut_ptr(), &self, other);
			result.assume_init() != 0
		};
		if greater {
			Some(core::cmp::Ordering::Greater)
		} else {
			Some(core::cmp::Ordering::Less)
		}
	}
}
