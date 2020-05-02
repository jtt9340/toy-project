#![allow(dead_code)]

use std::{
	error,
	fmt,
	str,
	num::ParseFloatError,
};

/// A wrapper type used for conveniently converting between degrees and radians.
///
/// Many APIs involving geometry and trigonometry require users to read the documentation on whether angles
/// are expected in degrees or radians, and angles are passed in as untyped floating point numbers. What
/// makes things more confusing is that angles representing rotations are expected in degrees but angles used
/// in trigonometric calculations are expected in radians, which is something the user must commit to memory.
/// Although not difficult to memorize, this type takes the guesswork out of passing quantities representing angles
/// to functions, as the function can convert to whichever angle measure (degrees or radians) that it requires regardless
/// of whichever angle measure is passed in.
/*
    Currently use f64 as backing type but could change to num_traits::Float or num_traits::real::Real in the future.

    Also currently only has two variants: Degrees and Radians but could add Revolutions in the future
*/
#[derive(Clone, Copy, Debug)]
pub enum Angle {
	/// An angle in degrees, where one degree is defined as 1/360 of a circle.
	Degrees(f64),
	/// An angle in radians, where one radian is defined as 2pi of a revolution.
	Radians(f64),
}

impl Angle {
	/// Get the underlying number that this `Angle` wraps.
	pub fn unwrap(self) -> f64 {
		match self {
			Angle::Degrees(deg) => deg,
			Angle::Radians(rad) => rad,
		}
	}

	/// Determines if the given `Angle` is in `Degrees`.
	pub fn is_degrees(&self) -> bool {
		if let Angle::Degrees(_) = *self {
			true
		} else {
			false
		}
	}

	/// Determines if the given `Angle` is in `Radians`.
	pub fn is_radians(&self) -> bool {
		if let Angle::Radians(_) = *self {
			true
		} else {
			false
		}
	}

	/// Consume the given `Angle` and return a new one, with the new `Angle` in `Degrees`.
	///
	/// If the given `Angle` is already in degrees, then this function just returns the given angle.
	/// Otherwise, this function performs the conversion.
	pub fn to_degrees(self) -> Self {
		match self {
			Angle::Degrees(_) => self,
			Angle::Radians(rad) => Angle::Degrees(rad.to_degrees()),
		}
	}

	/// Consume the given `Angle` and return a new one, with the new `Angle` in `Radians`.
	///
	/// If the given `Angle` is already in radians, then this function just returns the given angle.
	/// Otherwise, this function performs the conversion.
	pub fn to_radians(self) -> Self {
		match self {
			Angle::Degrees(deg) => Angle::Radians(deg.to_radians()),
			Angle::Radians(_) => self,
		}
	}

	/// Convert the given `Angle` to degrees, minutes, and seconds.
	///
	/// Returns a tuple of three integers: the first represents the number of degrees in the given `Angle`, the second represents
	/// the number of minutes in the given `Angle`, and the third represents the number of seconds in the given `Angle`. While a
	/// whole angle can be negative, the number of minutes and seconds in an angle cannot, so the first integer in the tuple is
	/// signed while the the other two are not.
	///
	/// A minute is 1/60 of a degree and a second is 1/60 of a minute (1/3600 of a degree).
	pub fn to_dms(self) -> (i32, u32, u32) {
		let dd = self.to_degrees().unwrap();

		let d = dd.trunc();
		let m = ((dd - d) * 60.0).trunc();
		let s = (dd - d - m/60.0) * 3600.0;

		(d as i32, m as u32, s as u32)
	}

	/// Create a new `Angle` from a tuple of degrees, minutes, and seconds.
	///
	/// This method is the inverse of `to_dms`, i.e. passing the `Angle` returned by this function to `to_dms` will return the same tuple
	/// used to invoke this function.
	pub fn from_dms(theta: (i32, u32, u32)) -> Self {
		let d = theta.0 as f64;
		let m = theta.1 as f64;
		let s = theta.2 as f64;

		let dd = d + m/60.0 + s/3600.0;

		Angle::Degrees(dd)
	}
}

impl fmt::Display for Angle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Angle::Degrees(deg) => write!(f, "{}°", deg),
			Angle::Radians(rad) => write!(f, "{} rad.", rad),
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseAngleError {
	UnrecognizedUnit,
	ParseFloatError(ParseFloatError),
}

impl fmt::Display for ParseAngleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParseAngleError::UnrecognizedUnit => f.write_str("Could not determine if the angle is in degrees or radians"),
			ParseAngleError::ParseFloatError(e) => write!(f, "{}", e),
		}
	}
}

impl error::Error for ParseAngleError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			ParseAngleError::UnrecognizedUnit => None,
			ParseAngleError::ParseFloatError(e) => Some(e),
		}
	}
}

impl str::FromStr for Angle {
	type Err = ParseAngleError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.ends_with('º') {
			let deg_str = s.trim_end_matches('º').trim_end();
			let deg = match deg_str.parse::<f64>() {
				Ok(d) => d,
				Err(e) => return Err(ParseAngleError::ParseFloatError(e)),
			};
			Ok(Angle::Degrees(deg))
		} else if s.ends_with("rad.") {
			let rad_str = s.trim_end_matches("rad.").trim_end();
			let rad = match rad_str.parse::<f64>() {
				Ok(r) => r,
				Err(e) => return Err(ParseAngleError::ParseFloatError(e)),
			};
			Ok(Angle::Radians(rad))
		} else {
			Err(ParseAngleError::UnrecognizedUnit)
		}
	}
}
