#![doc = r#"Module with basic types."#]

use serde::{Deserialize, Serialize};
use std::convert::{AsRef, AsMut};
use std::convert::{From, Into};
use std::ops::{Deref, DerefMut};

#[doc = r#"Specifies which AA Codes will be interrogated for the BDS Registers in the next field. If omitted, implies all targets.

# Restrictions
* Length: `6`"#]
pub type AaCodeType = String;

#[doc = r#"Indicates acceleration in meters per second per second (m/s^2)."#]
pub type AccelerationType = f64;

#[doc = r#"Provides the Mode S Aircraft ID. See MIL-STD-6016 for detailed definition.

# Restrictions
* Pattern: `[A-Z0-9&#x20;]{8}`
* Length: `8`"#]
pub type AircraftIdentifierType = String;

#[doc = r#"A string representing alphanumeric characters and the dash, space, and underscore characters of length up to 20.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type AlphanumericDashSpaceUnderscoreString20Type = String;

#[doc = r#"A string representing alphanumeric characters and the dash, space, and underscore characters of length of 12.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{12}`
* Length: `12`"#]
pub type AlphanumericDashSpaceUnderscoreStringLength12Type = String;

#[doc = r#"A string representing alphanumeric characters and the dash, space, and underscore characters of length 15.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{15}`
* Length: `15`"#]
pub type AlphanumericDashSpaceUnderscoreStringLength15Type = String;

#[doc = r#"A string representing alphanumeric characters and the dash, space, and underscore characters of length of 20.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{20}`
* Length: `20`"#]
pub type AlphanumericDashSpaceUnderscoreStringLength20Type = String;

#[doc = r#"A string representing alphanumeric characters and the space character of length 15.

# Restrictions
* Pattern: `[A-Za-z0-9&#x20;]{15}`
* Length: `15`"#]
pub type AlphanumericSpaceStringLength15Type = String;

#[doc = r#"A string representing up to 20 alphanumeric characters.

# Restrictions
* Pattern: `[a-zA-Z0-9]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type AlphanumericString20Type = String;

#[doc = r#"A string representing up to 4 alphanumeric characters.

# Restrictions
* Pattern: `[a-zA-Z0-9]{1,4}`
* Minimum length: `1`
* Maximum length: `4`"#]
pub type AlphanumericString4Type = String;

#[doc = r#"A string representing alphanumeric characters up to length 54.

# Restrictions
* Pattern: `[a-zA-Z0-9]{1,54}`
* Minimum length: `1`
* Maximum length: `54`"#]
pub type AlphanumericString54Type = String;

#[doc = r#"A string representing up to 6 alphanumeric characters.

# Restrictions
* Pattern: `[a-zA-Z0-9]{1,6}`
* Minimum length: `1`
* Maximum length: `6`"#]
pub type AlphanumericString6Type = String;

#[doc = r#"A string representing exactly 4 alphanumeric characters.

# Restrictions
* Pattern: `[a-zA-Z0-9]{4}`
* Length: `4`"#]
pub type AlphanumericStringLength4Type = String;

#[doc = r#"A string representing alphanumeric characters of length 7.

# Restrictions
* Pattern: `[a-zA-Z0-9]{7}`
* Length: `7`"#]
pub type AlphanumericStringLength7Type = String;

#[doc = r#"Indicates height above Mean Sea Level (MSL) as measured by local barometric pressure in meters (m). The minimum value represents the maximum distance to the center of the EGM96 Geoid from MSL.

# Restrictions
* Minimum value: `-6378237` (Inclusive)"#]
pub type AltitudeBarometricType = f64;

#[doc = r#"Indicates height above reference in meters (m). Where such reference is unavailable, defaults to Height above WGS-84 ellipsoid in meters. The minimum value represents the maximum distance to the center of the EGM96 Geoid from MSL, and also includes the distance to the center of the WGS84 ellipsoid from the equator [Ref DMA TR 8350].

# Restrictions
* Minimum value: `-6378237` (Inclusive)"#]
pub type AltitudeType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [0,Pi].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `3.141592653589793238462` (Inclusive)"#]
pub type AngleHalfPositiveType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [-Pi/2,Pi/2].

# Restrictions
* Minimum value: `-1.570796326794896619232` (Inclusive)
* Maximum value: `1.570796326794896619232` (Inclusive)"#]
pub type AngleHalfType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [0,2*Pi).

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `6.283185307179586476926` (Inclusive)"#]
pub type AnglePositiveType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [0,Pi/2].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `1.570796326794896619232` (Inclusive)"#]
pub type AngleQuarterType = f64;

#[doc = r#"Indicates an angle rate in radians/sec (rad/s)."#]
pub type AngleRateType = f64;

#[doc = r#"Indicates angles in radians with values with a range of [-Pi,Pi).

# Restrictions
* Minimum value: `-3.141592653589793238462` (Inclusive)
* Maximum value: `3.141592653589793238462` (Inclusive)"#]
pub type AngleType = f64;

#[doc = r#"A string representing a 4 digit Pulse Internal Modulation (PIM) Code.

# Restrictions
* Pattern: `[0-9]{4}`
* Length: `4`"#]
pub type AoPimCodeType = String;

#[doc = r#"A string representing the Joint Pub 3-09.1 definition of the Pulse Repetition Frequency (PRF) Code. From Joint Pub 3-09.1: "Depending on the laser equipment, either a three or four-digit code can be set. Three digit code equipment settings range from 111 to 788. Four-digit code equipment settings range from 1111 to 1788."

# Restrictions
* Pattern: `1?[1-7][1-8]{2}`
* Minimum length: `3`
* Maximum length: `4`"#]
pub type AoPrfCodeType = String;

#[doc = r#"Indicates an angular measurement in units of arc seconds (arcsec)."#]
pub type ArcSecondsType = f64;

#[doc = r#"Indicates a non-negative area in square meters (m^2)."#]
pub type AreaType = DoubleNonNegativeType;

#[doc = r#"Indicates the identifier of a DMPI in an ATO.  This type is based on "DesiredMeanPointOfImpactDmpiIdentifierType" in MIL-STD-6040's ATO message/schema.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,30}`
* Minimum length: `1`
* Maximum length: `30`"#]
pub type AtoDmpiIdentifierType = String;

#[doc = r#"The alphanumeric identifier that joins two or more elements of composite force package.  This type is based on "PackageIdentificationType" in MIL-STD-6040's ATO message/schema.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9]{1,5}`
* Minimum length: `1`
* Maximum length: `5`"#]
pub type AtoPackageIdentificationType = String;

#[doc = r#"The name of the target/facility.  This type is based on "TargetFacilityNameType" in MIL-STD-6040's ATO message schema.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,38}`
* Minimum length: `1`
* Maximum length: `38`"#]
pub type AtoTargetFacilityNameType = String;

#[doc = r#"A string representing the name of a resource or location.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,256}`
* Minimum length: `1`
* Maximum length: `256`"#]
pub type AttributedUriType = String;

#[doc = r#"Surveillance Identifier (modern address type): lockout interactions

# Restrictions
* Length: `2`"#]
pub type BdsAddressType = String;

#[doc = r#"Indicates 8-bit integer values greater than or equal to one with a range of [1,255].

# Restrictions
* Minimum value: `1` (Inclusive)"#]
pub type BytePositiveType = u8;

#[doc = r#"A string representing a cycle number format.

# Restrictions
* Pattern: `[A-Z0-9]{2}-[0-9]{4}[TE]?`
* Minimum length: `7`
* Maximum length: `8`"#]
pub type CounterSpaceCycleNumberType = String;

#[doc = r#"A string representing the System Engagement Number (SENO) for a CounterSpace force.

# Restrictions
* Pattern: `[A-Z][IRS][0-9]{3}`
* Length: `5`"#]
pub type CounterSpaceSenoType = String;

#[doc = r#"Indicates a data rate in bits per seconds (bps)."#]
pub type DataRateType = u32;

#[doc = r#"UCI uses the W3C (www.w3.org) definition of date and time exactly as given in the specification for xs:dateTime with a further restriction that only the "Zulu" time zone be used.  xs:dateTime is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal digits to arbitrary precision.  See the W3C specification of xs:dateTime for further details.

# Restrictions
* Pattern: `.+Z`"#]
pub type DateTimeType = chrono::DateTime<chrono::Utc>;

#[doc = r#"Indicates non-negative values in Decibels (dB).

# Restrictions
* Minimum value: `0` (Inclusive)"#]
pub type DecibelNonNegativeType = DecibelType;

#[doc = r#"Indicates values in Decibels (dB)."#]
pub type DecibelType = f64;

#[doc = r#"Indicates a distance offset from an origin in 1-dimension, in meters (m)."#]
pub type DistanceOffsetType = f64;

#[doc = r#"Indicates a (non-negative) distance in meters (m)."#]
pub type DistanceType = DoubleNonNegativeType;

#[doc = r#"Indicates 64-bit floating point values greater than or equal to zero [0,Inf].

# Restrictions
* Minimum value: `0.0` (Inclusive)"#]
pub type DoubleNonNegativeType = f64;

#[doc = r#"Indicates 64-bit floating point values greater than zero (0,Inf].

# Restrictions
* Minimum value: `0.0` (Exclusive)"#]
pub type DoublePositiveType = f64;

#[doc = r#"UCI uses the W3C (www.w3.org) definition of time duration exactly as given in the specification for xs:duration.  xs:duration is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal digits to arbitrary precision.  See the W3C specification of xs:duration for further details."#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DurationType(
	#[serde(with = "crate::v2_5::serde_utils::time_delta")]
	pub chrono::TimeDelta,
);
impl Deref for DurationType {
	type Target = chrono::TimeDelta;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for DurationType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl AsRef<chrono::TimeDelta> for DurationType {
	fn as_ref(&self) -> &chrono::TimeDelta {
		&self.0
	}
}
impl AsMut<chrono::TimeDelta> for DurationType {
	fn as_mut(&mut self) -> &mut chrono::TimeDelta {
		&mut self.0
	}
}
impl From<chrono::TimeDelta> for DurationType {
	fn from(p: chrono::TimeDelta) -> Self {
		Self(p)
	}
}
impl Into<chrono::TimeDelta> for DurationType {
	fn into(self) -> chrono::TimeDelta {
		self.0
	}
}


#[doc = r#"Indicates the terrain level relative to Mean Sea Level (MSL) in meters (m)."#]
pub type ElevationType = f64;

#[doc = r#"A string representing an empty or blank type used to indicate that no further content is expected.

# Restrictions
* Pattern: `[a-zA-Z]{0}`
* Length: `0`"#]
pub type EmptyType = String;

#[doc = r#"Indicates the rate at which energy is dissipated, measured in watts per kilogram (W/kg)."#]
pub type EnergyDissipationRateType = DoubleNonNegativeType;

#[doc = r#"A string representing an emitter's name in the Combined Emitter Database (CED). See the Combined Emitter Database (CED) for more information.

# Restrictions
* Pattern: `[a-zA-Z0-9_\-]{1,12}`
* Minimum length: `1`
* Maximum length: `12`"#]
pub type EobCedNameType = String;

#[doc = r#"A string representing an OB's associated weapon system based on a correlation with the Combined Emitter Database. Reference the Combined Emitter Database (CED) for more information.

# Restrictions
* Pattern: `[A-Za-z0-9_\-]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type EobCedWeaponSystemType = String;

#[doc = r#"A string representing the file name of a stored product.  The minimum naming convention of file names is {at least one alphanumeric, underscore, or dash characters} {dot} {at least one alphanumeric, underscore, dash, or dot characters}. The format supports multiple dot extensions, such as "filename.tar.bz2".  The max length value of 255 is set to be consistent with  NTFS, ext4, btrfs, and zfs restrictions.

# Restrictions
* Pattern: `[a-zA-Z0-9_\-]+\.[a-zA-Z0-9_\.\-]+`
* Minimum length: `1`
* Maximum length: `255`"#]
pub type FileNameType = String;

#[doc = r#"A string representing a FIPS PUB 10-4 country code digraph (or two spaces). This data type is provided for conformance to existing standards and should not be used for new data types unless conformance to FIPS PUB 10-4 is explicitly required.

# Restrictions
* Pattern: `[A-Z]{2}|[&#x20;]{2}`
* Length: `2`"#]
pub type FipsCountryCodeType = String;

#[doc = r#"Indicates a frequency offset in Hertz (Hz)."#]
pub type FrequencyOffsetType = f64;

#[doc = r#"Indicates a (positive) frequency in Hertz (Hz)."#]
pub type FrequencyType = DoublePositiveType;

#[doc = r#"Indicates GEO drift rate in radians per day (rad/day)."#]
pub type GeoDriftRateType = f64;

#[doc = r#"Indicates the daily average level for geomagnetic activity, represented as the Ap Index.

# Restrictions
* Maximum value: `400` (Inclusive)"#]
pub type GeomagneticApIndexType = DoubleNonNegativeType;

#[doc = r#"Indicates the planetary amplitude of gamma deflections (geomagnetic activity), represented as the Kp Index.

# Restrictions
* Maximum value: `9` (Inclusive)"#]
pub type GeomagneticKpIndexType = DoubleNonNegativeType;

#[doc = r#"Indicates an ICAO Aircraft Address with a range of [0,16777215].

# Restrictions
* Maximum value: `16777215` (Inclusive)"#]
pub type IcaoAircraftAddressType = u32;

#[doc = r#"A string representing the ICAO (International Civil Aviation Organization) identifier for an airfield (DFI 1868 DUI 001).

# Restrictions
* Pattern: `[A-Z]{4}`
* Length: `4`"#]
pub type IcaoAirfieldIdentifierType = String;

#[doc = r#"These are from the IFF specification and identify 7 possible Subtypes to the Types in ADS-B.

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `7` (Inclusive)"#]
pub type IffAdsBSubtypeType = u8;

#[doc = r#"These are from the IFF specification and identify 31 possible formats for Mode5

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `31` (Inclusive)"#]
pub type IffAdsBType = u8;

#[doc = r#"IFF Barometric Pressure.

# Restrictions
* Minimum value: `800` (Inclusive)
* Maximum value: `1210` (Inclusive)"#]
pub type IffBarometricPressureType = f32;

#[doc = r#"These are from the IFF specification and identify 31 possible formats for Mode5.

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `31` (Inclusive)"#]
pub type IffMode5FormatType = u8;

#[doc = r#"IFF Mode 5 National Origin Code.

# Restrictions
* Maximum value: `2047` (Inclusive)"#]
pub type IffMode5NationalOriginType = u16;

#[doc = r#"IFF Mode 5 Platform Identification Number (PIN). See DoD AIMS 17-1000 for details.

# Restrictions
* Maximum value: `16383` (Inclusive)"#]
pub type IffMode5PinType = u16;

#[doc = r#"Mode code confidence.  See DoD AIMS 17-1000R1.1 for details.

# Restrictions
* Maximum value: `4095` (Inclusive)"#]
pub type IffModeCodeConfidenceType = u16;

#[doc = r#"Mode code.  See DoD AIMS 17-1000R1.1 for details.

# Restrictions
* Maximum value: `4095` (Inclusive)"#]
pub type IffModeCodeType = u16;

#[doc = r#"IFF ModeS Downlink format.

# Restrictions
* Maximum value: `24` (Inclusive)"#]
pub type IffModeSDownlinkFormatType = u16;

#[doc = r#"A string representing an IJMS 15 bit Track Number.

# Restrictions
* Pattern: `[0-7]{5}`
* Length: `5`"#]
pub type IjmsTrackNumberType = String;

#[doc = r#"A string representing an International Maritime Organization (IMO) identifier number.

# Restrictions
* Pattern: `IMO[0-9]{7}`
* Length: `10`"#]
pub type ImoNumberType = String;

#[doc = r#"Indicates 32-bit integer values greater than zero with a range of [1,4294967295].

# Restrictions
* Minimum value: `1` (Inclusive)"#]
pub type IntPositiveType = u32;

#[doc = r#"Interrogator ID (legacy address type): used for transponder lockout interactions

# Restrictions
* Minimum value: `0` (Inclusive)
* Maximum value: `15` (Inclusive)"#]
pub type InterrogatorIdentifierType = i32;

#[doc = r#"A string representing an IPv4 address, with validation rules derived from RFC 6991.

# Restrictions
* Pattern: `(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])`
* Minimum length: `7`
* Maximum length: `15`"#]
pub type Ipv4AddressType = String;

#[doc = r#"A string representing an IPv6 address, with validation rules derived from RFC 6991.

# Restrictions
* Pattern: `((:|[0-9a-fA-F]{0,4}):)([0-9a-fA-F]{0,4}:){0,5}((([0-9a-fA-F]{0,4}:)?(:|[0-9a-fA-F]{0,4}))|(((25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])))`
* Minimum length: `2`
* Maximum length: `45`"#]
pub type Ipv6AddressType = String;

#[doc = r#"Indicates the measured irradiance of the object being observed in Watts per square meter (W/m^2)."#]
pub type IrradianceType = DoubleNonNegativeType;

#[doc = r#"A string representing the object's launch piece (A-Z) within a launch number.  This is part of the unique designation for an Earth orbiting object assigned by the World Data Center-A for Rockets and Satellites (WDC-A-RS).

# Restrictions
* Pattern: `[A-Z]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type LaunchPieceType = String;

#[doc = r#"Indicates a Link-16 Track Number with a range of [0,32767].

# Restrictions
* Maximum value: `32767` (Inclusive)"#]
pub type Link16AddressTrackNumberType = u16;

#[doc = r#"Indicates a Link-16 Control Channel with a range of [0,127].

# Restrictions
* Maximum value: `127` (Inclusive)"#]
pub type Link16ControlChannelType = u8;

#[doc = r#"Indicates a Link-16 Message Generation Rate with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16MessageGenerationRateType = u8;

#[doc = r#"Indicates a Link-16 J-Message Label with a range of [0,31].

# Restrictions
* Maximum value: `31` (Inclusive)"#]
pub type Link16MessageLabelType = u8;

#[doc = r#"Indicates a Link-16 Message Priority with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16MessagePriorityType = u8;

#[doc = r#"Indicates a number of messages that can be stored in a queue with a range of [0,280].

# Restrictions
* Maximum value: `280` (Inclusive)"#]
pub type Link16MessageStorageLimitType = u16;

#[doc = r#"Indicates a Link-16 J-Message Sub-Label with a range of [0,7].

# Restrictions
* Maximum value: `7` (Inclusive)"#]
pub type Link16MessageSubLabelType = u8;

#[doc = r#"Indicates a Link-16 Missile Channel with a range of [1,63]. See MIL-STD-6016 DFI 852 DUI 002.

# Restrictions
* Maximum value: `63` (Inclusive)"#]
pub type Link16MissileChannelType = BytePositiveType;

#[doc = r#"Indicates a Link-16 Net Number with a range of [0,127].

# Restrictions
* Maximum value: `127` (Inclusive)"#]
pub type Link16NetNumberType = u8;

#[doc = r#"Indicates a Link-16 Network Participation Group with a range of [0,512].

# Restrictions
* Maximum value: `512` (Inclusive)"#]
pub type Link16NpgType = u16;

#[doc = r#"Indicates a Link-16 Participation Group Index number with a range of [0,511].

# Restrictions
* Maximum value: `511` (Inclusive)"#]
pub type Link16ParticipationGroupIndexType = u16;

#[doc = r#"Indicates a Link-16 Platform Strength with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16PlatformStrengthType = u8;

#[doc = r#"Indicates a Link-16 Position Quality with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16PositionQualityType = u8;

#[doc = r#"Indicates a Link-16 Radar RF channel with a range of [1,63]. See MIL-STD-6016 DFI 852 DUI 001.

# Restrictions
* Maximum value: `63` (Inclusive)"#]
pub type Link16RadarChannelType = BytePositiveType;

#[doc = r#"Indicates a Link-16 Slot Number with a range of [1,10].

# Restrictions
* Maximum value: `10` (Inclusive)"#]
pub type Link16SlotNumberType = BytePositiveType;

#[doc = r#"Indicates a Link-16 Slot Selection with a range of [0,31].

# Restrictions
* Maximum value: `31` (Inclusive)"#]
pub type Link16SlotSelectionType = u8;

#[doc = r#"The model designation suffix of a specific type of vehicle. Indicates the alpha character suffix used to designate a certain version of an alpha/numeric aircraft designator.  Use of a single character indicates the MIL-STD-6016 aircraft version; see MIL-STD-6016 DFI/DUI 1661/001 for additional details.  Use of a longer string is program dependent.

# Restrictions
* Pattern: `[A-Za-z0-9]{1,4}`
* Maximum length: `4`"#]
pub type Link16SpecificTypeModelType = String;

#[doc = r#"Indicates a number of Link-16 time slots before a message is considered stale with a range of [1,16383].

# Restrictions
* Maximum value: `16383` (Inclusive)"#]
pub type Link16StalenessLimitType = ShortPositiveType;

#[doc = r#"Indicates a Link-16 track index for a specific entity member with a range of [0,63]. See MIL-STD-6016 DFI 768 DUI 002.

# Restrictions
* Maximum value: `63` (Inclusive)"#]
pub type Link16TrackIndexType = u8;

#[doc = r#"Indicates a Link 16 19 bit alphanumeric Track Number.

# Restrictions
* Pattern: `[A-HJ-NP-Z0-7]{2}[0-7]{3}`
* Length: `5`"#]
pub type Link16TrackNumberType = String;

#[doc = r#"Indicates a Link-16 Track Quality with a range of [0,15].

# Restrictions
* Maximum value: `15` (Inclusive)"#]
pub type Link16TrackQualityType = u8;

#[doc = r#"The MIL-STD-6016 (Link 16) Voice Call Sign used to identify this vehicle.

# Restrictions
* Pattern: `[A-Z0-9&#x20;]{4}`
* Length: `4`"#]
pub type Link16VoiceCallSignType = String;

#[doc = r#"Indicates a Link-16 Voice Channel with a range of [0,127].

# Restrictions
* Maximum value: `127` (Inclusive)"#]
pub type Link16VoiceChannelType = u8;

#[doc = r#"A string representing a NATO Link 1 15-bit Track Number.

# Restrictions
* Pattern: `[AEGHJKLM]{2}[0-7]{3}`
* Length: `5`"#]
pub type Link1TrackNumberType = String;

#[doc = r#"Indicates the Mach value, which is unitless and represents the ratio of the speed of a body to the speed of sound in the surrounding medium."#]
pub type MachType = DoubleNonNegativeType;

#[doc = r#"Indicates mass in kilograms (kg)."#]
pub type MassType = DoubleNonNegativeType;

#[doc = r#"A string representing the letter M.

# Restrictions
* Pattern: `[m]`
* Length: `1`"#]
pub type MeterUnitLetterType = String;

#[doc = r#"Indicates an MIDB Evaluation Code with a range of [1,10].

# Restrictions
* Maximum value: `10` (Inclusive)"#]
pub type MidbEvaluationCodeType = BytePositiveType;

#[doc = r#"A string representing a military grid.

# Restrictions
* Pattern: `([1-9]|[1-5][0-9]|60)[C-HJ-NP-X]([A-HJ-NP-Z][A-HJ-NP-V]([0-9]{2}){0,5})?|[ABYZ]([A-CF-HJ-LP-UX-Z][A-HJ-NP-Z]([0-9]{2}){0,5})?`
* Minimum length: `14`
* Maximum length: `15`"#]
pub type MilitaryGridType = String;

#[doc = r#"Indicates power referenced to one milliwatt in units dBm."#]
pub type MilliwattPowerRatioType = f64;

#[doc = r#"A string representing a MIME type describing the product's physical encoding.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,256}`
* Minimum length: `1`
* Maximum length: `256`"#]
pub type MimeType = String;

#[doc = r#"A string representing a MISP classification or marking system.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{1,40}`
* Minimum length: `1`
* Maximum length: `40`"#]
pub type MispClassificationType = String;

#[doc = r#"A string representing a MISP Item designator ID

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{1,16}`
* Maximum length: `16`"#]
pub type MispItemDesignatorType = String;

#[doc = r#"Indicates the purpose or category of the mission as specified by an operator.  Used to provide the ability to filter missions by their type."#]
pub type MissionCategoryType = VisibleString32Type;

#[doc = r#"A string representing a MMSI number.

# Restrictions
* Pattern: `[0-9]{9}`
* Length: `9`"#]
pub type MmsiNumberType = String;

#[doc = r#"Indicates the calculated magnitude of moment expressed as force multiplied by perpendicular distance, measured in Newton-meters (Nm)."#]
pub type MomentType = DoubleNonNegativeType;

#[doc = r#"A string representing the official name of something, or object that represents it. Special characters are restricted the dash (-).  These are needed to enable automated data exchange with other systems.

# Restrictions
* Pattern: `[a-zA-Z0-9\-]{15}`
* Length: `15`"#]
pub type NameSpecialCharacterRestrictionType = String;

#[doc = r#"North Atlantic Treaty Organization Special WordsTotal number of distinct values is 54.North Atlantic Treaty Organization Special Words

# Restrictions
* Pattern: `NATO:[a-zA-Z\-_]{1,256}`
* Minimum length: `6`
* Maximum length: `261`"#]
pub type NatoSpecialWordsType = String;

#[doc = r#"A string representing a desired/observed rating level of an image in the National Imagery Interpretability Rating Scale. Scale levels are expressed as digit, dot, digit, like "3.2" with a range 0.0 to 9.9. NIIRS defines "scales" (with levels and associated rating criteria) for images of different types and/or collected in different bands of the electromagnetic spectrum. These scales include Visible NIIRS (visible panchromatic, military focused), Civil NIIRS (visible panchromatic, civil focused), Radar NIIRS (synthetic aperture radar), IR NIIRS (infrared/thermal), MS IIRS (multispectral) and MT IIRS (moving target radar).

# Restrictions
* Pattern: `[0-9]\.[0-9]`
* Length: `3`"#]
pub type NiirsType = String;

#[doc = r#"A string representing the NITF ACFTB scene source.

# Restrictions
* Pattern: `[0-9&#x20;]`
* Length: `1`"#]
pub type NitfAcftbSceneSourceType = String;

#[doc = r#"A string representing the NITF ACFTB sensor identifier.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{6}`
* Length: `6`"#]
pub type NitfAcftbSensorIdentifierType = String;

#[doc = r#"A string representing the NITF AIMIDB flight numbers.

# Restrictions
* Pattern: `[A-Z0-9][0-9]`
* Length: `2`"#]
pub type NitfAimidbFlightNumberType = String;

#[doc = r#"A string representing the NITF AIMIDB mission number.

# Restrictions
* Pattern: `((((U0)|[A-Z]{2})[0-9]{2})|UNKN)`
* Length: `4`"#]
pub type NitfAimidbMissionNumberType = String;

#[doc = r#"A string representing the NITF classification authority method.

# Restrictions
* Pattern: `[ODM&#x20;]`
* Length: `1`"#]
pub type NitfClassificationAuthorityMethodType = String;

#[doc = r#"A string representing NITF Classification Authority.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{40}`
* Length: `40`"#]
pub type NitfClassificationAuthorityType = String;

#[doc = r#"A string representing the NITF classification reason.

# Restrictions
* Pattern: `[A-H&#x20;]`
* Length: `1`"#]
pub type NitfClassificationReasonType = String;

#[doc = r#"A string representing NITF Classification Text.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{43}`
* Length: `43`"#]
pub type NitfClassificationTextType = String;

#[doc = r#"A string representing NITF codewords.

# Restrictions
* Pattern: `([A-Z]{2}&#x20;){3}[A-Z]{2}`
* Length: `11`"#]
pub type NitfCodewordsType = String;

#[doc = r#"A string representing NITF control and handling.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{2}`
* Length: `2`"#]
pub type NitfControlAndHandlingType = String;

#[doc = r#"A string representing the NITF date and time format of CCYYMMDDHHMM. Default of twelve ECS spaces (0x20) indicates that the date does not imply.

# Restrictions
* Pattern: `&#x20;{12}`
* Length: `12`"#]
pub type NitfDateAndTimeType = String;

#[doc = r#"A string representing the NITF date format of CCYYMMDD. Default of eight ECS spaces (0x20) indicates that the date does not imply.

# Restrictions
* Pattern: `&#x20;{8}`
* Length: `8`"#]
pub type NitfDateType = String;

#[doc = r#"A string representing NITF declassification and exemptions.

# Restrictions
* Pattern: `(X[1-8](&#x20;){2})|25X[1-9]|[DNIO]|(&#x20;){4}`
* Length: `4`"#]
pub type NitfDeclassificationExemptionType = String;

#[doc = r#"A string representing NITF file and image declassification.

# Restrictions
* Pattern: `(DD|DE|GD|GE|O(&#x20;)|X(&#x20;)|(&#x20;){2})`
* Length: `2`"#]
pub type NitfDeclassificationType = String;

#[doc = r#"A string representing NITF downgrades.

# Restrictions
* Pattern: `[SCR&#x20;]`
* Length: `1`"#]
pub type NitfDowngradeType = String;

#[doc = r#"A string representing the NITF EXPLTB sequence within a coupled imagery set.

# Restrictions
* Pattern: `[1-6&#x20;]`
* Length: `1`"#]
pub type NitfExpltbSequenceNumberType = String;

#[doc = r#"A string representing NITF file security classification.

# Restrictions
* Pattern: `[TSCRU]`
* Length: `1`"#]
pub type NitfFileSecurityClassificationType = String;

#[doc = r#"A string representing the NITF image source.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{42}`
* Length: `42`"#]
pub type NitfImageSourceType = String;

#[doc = r#"A string representing an IPON IID2 Program Code.

# Restrictions
* Pattern: `([0-9][A-Z]|[A-Z][0-9])`
* Length: `2`"#]
pub type NitfIponIid2ProgramCodeType = String;

#[doc = r#"A string representing an IPON IID2 Project Code.

# Restrictions
* Pattern: `[A-Z]{2}`
* Length: `2`"#]
pub type NitfIponIid2ProjectCodeType = String;

#[doc = r#"A string representing an IPON IID2 Sortie number.

# Restrictions
* Pattern: `([A-Z0-9]{2})`
* Length: `2`"#]
pub type NitfIponIid2SortieNumberType = String;

#[doc = r#"A string representing the NITF MSTGTA target functional category code in accordance with DIAM 65-3-1.

# Restrictions
* Pattern: `[1-9][0-9]{4}|[&#x20;]{5}`
* Length: `5`"#]
pub type NitfMstgtaTargetCategoryType = String;

#[doc = r#"A string representing the NITF MSTGTA target location.

# Restrictions
* Pattern: `([\+\-]{1}[0-8]\d\.\d{6}[\+\-]{1}(0\d{2}|1[0-7]\d)\.\d{6})`
* Length: `21`"#]
pub type NitfMstgtaTargetLocationType = String;

#[doc = r#"A string representing NITF MSTGTA target priority.

# Restrictions
* Pattern: `[0-9][1-9][0-9]|[0-9]{2}[1-9]|[1-9][0-9]{2}`
* Length: `3`"#]
pub type NitfMstgtaTargetPriorityType = String;

#[doc = r#"A string representing the NITF file originator's name.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{24}`
* Length: `24`"#]
pub type NitfOriginatorNameType = String;

#[doc = r#"A string representing the NITF files originator's phone number.

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{18}`
* Length: `18`"#]
pub type NitfOriginatorPhoneType = String;

#[doc = r#"A string representing the floating point value of Gravity at a local point in ft/sec^2 per The Compendium of Controlled Extensions (CE) for the National Imagery Transmission Formats (NITFS) Volumn 1 Tagged Record Extensions, STDI-0002-1.

# Restrictions
* Pattern: `([3][1-3]\.[0-9]{4})|(&#x20;{7})`
* Length: `7`"#]
pub type NitfPatchbGravityType = String;

#[doc = r#"A string representing NITF releasing instructions.

# Restrictions
* Pattern: `([A-Z]{2}&#x20;){6}[A-Z]{2}`
* Length: `20`"#]
pub type NitfReleasingInstructionsType = String;

#[doc = r#"Indicates a NITF Target Functional Category Code with a range of [10000,99999]. See DIAM-65-3-1.

# Restrictions
* Minimum value: `10000` (Inclusive)
* Maximum value: `99999` (Inclusive)"#]
pub type NitfTargetCategoryCodeType = i32;

#[doc = r#"Indicates a NITF Target Number with a range of [1,99999].

# Restrictions
* Maximum value: `99999` (Inclusive)"#]
pub type NitfTargetNumberType = IntPositiveType;

#[doc = r#"Indicates a NITF Target Priority with a range of [1,999].

# Restrictions
* Maximum value: `999` (Inclusive)"#]
pub type NitfTargetPriorityType = ShortPositiveType;

#[doc = r#"A string representing time in UTC.

# Restrictions
* Pattern: `(&#x20;){7}`
* Length: `7`"#]
pub type NitfUtcTimeType = String;

#[doc = r#"A string representing the notation for ELNOT or CENOT. ELNOT is Electronic Intelligence (ELINT) Notation for non-communications electronic emissions. CENOT is Communications Emitter Notation for communications electronic emissions.

# Restrictions
* Pattern: `[A-Z0-9]{5}|UNKN|NONE`
* Minimum length: `4`
* Maximum length: `5`"#]
pub type NotationType = String;

#[doc = r#"A string representing exactly 1 character in length, restricted to a numeric character.

# Restrictions
* Pattern: `[0-9]`
* Length: `1`"#]
pub type NumericStringLength1Type = String;

#[doc = r#"A string representing exactly 5 characters in length, restricted to numeric characters.

# Restrictions
* Pattern: `[0-9]{5}`
* Length: `5`"#]
pub type NumericStringLength5Type = String;

#[doc = r#"A string representing exactly 6 characters in length, restricted to numeric characters.

# Restrictions
* Pattern: `[0-9]{6}`
* Length: `6`"#]
pub type NumericStringLength6Type = String;

#[doc = r#"A string representing an entity's function or mission that it may or may not be engaged in at the moment. Based on the Global Command and Control System Integrated Imagery and Intelligence (GCCS-I3) Application Program Interface Reference Manual (APIRM) for MIDB Data Access Layer (MDAL).

# Restrictions
* Pattern: `[A-Z0-9]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type ObActivityCodeType = String;

#[doc = r#"A string representing an Air Defense District (ADD) or Air Defense Area (ADA) in which the geographic coordinates reside. ([A-Z][A-Z] - Position 1-2, Two character alphabetic field.; 0[0-9][0-9] - Position 3-5,  Air Defense Area in which the geographic coordinates resides.).

# Restrictions
* Pattern: `[A-Z]{2}[0][0-9]{2}`
* Length: `5`"#]
pub type ObAirDefenseAreaType = String;

#[doc = r#"A string representing an OB Code Word found in the classified appendix of the MIDB. Based on the Global Command and Control System Integrated Imagery and Intelligence (GCCS-I3) Application Program Interface Reference Manual (APIRM) for MIDB Data Access Layer (MDAL).

# Restrictions
* Pattern: `[a-zA-Z0-9_\-]{1,5}`
* Minimum length: `1`
* Maximum length: `5`"#]
pub type ObCodeWordType = String;

#[doc = r#"A string representing a surrogate key, which uniquely identifies the emitter in the source MIDB. Based on the Global Command and Control System Integrated Imagery and Intelligence (GCCS-I3) Application Program Interface Reference Manual (APIRM) for MIDB Data Access Layer (MDAL).

# Restrictions
* Pattern: `[a-zA-Z0-9]{5}[0-9]{9}`
* Maximum length: `14`"#]
pub type ObEmitterSurrogateKeyType = String;

#[doc = r#"A string representing the name of the OB facility or populated area.  Typically used in conjunction with other data to uniquely identify an OB.

# Restrictions
* Pattern: `[a-zA-Z0-9_\-]{1,54}`
* Minimum length: `1`
* Maximum length: `54`"#]
pub type ObFacilityNameType = String;

#[doc = r#"A string representing the name of the source of the last collection of the subject data for the order of battle item.

# Restrictions
* Pattern: `[A-Z0-9]{2,3}`
* Minimum length: `2`
* Maximum length: `3`"#]
pub type ObLastCollectorType = String;

#[doc = r#"A string representing an OB site facility or demographic area.  Typically used in conjunction with other data, such as a Basic Encyclopedia Number, to uniquely identify an OB site.

# Restrictions
* Pattern: `[A-Z]{2}[0-9]{3}`
* Length: `5`"#]
pub type ObOSuffixType = String;

#[doc = r#"A string representing the name of the creator and/or owner of the master instance of the order of battle item.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-_]{1,3}`
* Minimum length: `1`
* Maximum length: `3`"#]
pub type ObRecordOwnerType = String;

#[doc = r#"Indicates a 4 digit octal code.

# Restrictions
* Pattern: `[0-7]{4}`
* Length: `4`"#]
pub type OctalStringLength4Type = String;

#[doc = r#"Type containing an octal number in the form of a string.

# Restrictions
* Pattern: `[0-7]+`
* Minimum length: `1`
* Maximum length: `16`"#]
pub type OctalValueType = String;

#[doc = r#"Indicates the presence of clouds in the sky in units of okta with a range of [0,8].

# Restrictions
* Maximum value: `8` (Inclusive)"#]
pub type OktaType = u8;

#[doc = r#"A string representing a Basic Encyclopedia agency code.

# Restrictions
* Pattern: `[0-9]{4,5}`
* Minimum length: `4`
* Maximum length: `5`"#]
pub type OneUpNumberType = String;

#[doc = r#"A string representing up to a 20-digit US phone number. The operator may list several phone numbers including extensions and the string can include dots,hyphens and/or spaces.  Example formats include: 555-555-5555x5555, 5555555555, 555 555 5555x555555.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;\-\.]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type OperatorPhoneNumberType = String;

#[doc = r#"Non-negative double value indicating particles per cubic centimeter."#]
pub type ParticleDensityType = DoubleNonNegativeType;

#[doc = r#"Indicates a percentage where a value of 100.0 = 100%. Values greater than 100 are allowed."#]
pub type PercentType = DoubleNonNegativeType;

#[doc = r#"This type stores a power in Watts."#]
pub type PowerType = f64;

#[doc = r#"Indicates precipitation amounts in millimeters (mm)."#]
pub type PrecipitationAmountType = DoubleNonNegativeType;

#[doc = r#"Indicates pressure in kiloPascal (kPa)."#]
pub type PressureType = f64;

#[doc = r#"Indicates the priority ranking approach where lower values proceed first over higher values."#]
pub type PriorityRankType = u16;

#[doc = r#"Indicates the priority ranking approach where higher values proceed first over lower values."#]
pub type PriorityWeightType = u16;

#[doc = r#"A string representing up to 4096 characters in length. All characters are allowed for maximum compatibility with other string restrictions.

# Restrictions
* Pattern: `[&#x20;-&#x7E;\n\r]{0,4096}`
* Minimum length: `0`
* Maximum length: `4096`"#]
pub type QueryString4096Type = String;

#[doc = r#"A string representing a Basic Encyclopedia record originator.

# Restrictions
* Pattern: `[A-Z][A-Z]|[E]|[\-]`
* Minimum length: `1`
* Maximum length: `2`"#]
pub type RecordOriginatorType = String;

#[doc = r#"Indicates resolution in meters per pixel."#]
pub type ResolutionMetersPerPixelType = DoubleNonNegativeType;

#[doc = r#"A string representing the identity of the ROME ID, which is defined by ACTDF, where ROME is the acronym for Reconnaissance Operations Management Enterprise.

# Restrictions
* Pattern: `[a-zA-Z0-9]+`
* Minimum length: `1`
* Maximum length: `10`"#]
pub type RomeIdentityType = String;

#[doc = r#"This type indicates a SHA-256 cryptographic hash as defined by the U.S. Federal Information Processing Standard (FIPS) Publication 180-2.  SHA-256 is one of a family of Secure Hash Algorithms (SHA) published by the National Institute of Standards and Technology (NIST).  The 256 bit hash is encoded here as a hexadecimal number.

# Restrictions
* Length: `32`"#]
pub type Sha2256HashType = String;

#[doc = r#"Indicates 16-bit integer values greater than zero with a range of [1,65535].

# Restrictions
* Minimum value: `1` (Inclusive)"#]
pub type ShortPositiveType = u16;

#[doc = r#"Indicates F10.7 solar flux index."#]
pub type SolarFluxF107IndexType = u16;

#[doc = r#"Indicates speed in meters per second (m/s)."#]
pub type SpeedType = f64;

#[doc = r#"Indicates a scale factor with a range of [1.0,Inf].

# Restrictions
* Minimum value: `1.0` (Inclusive)"#]
pub type SpoilFactorType = f32;

#[doc = r#"A string representing a STANAG packet security classification.

# Restrictions
* Pattern: `[1-5]`
* Length: `1`"#]
pub type Stanag4607PacketSecurityClassificationType = String;

#[doc = r#"Surveillance Identifier (modern address type): lockout interactions

# Restrictions
* Minimum value: `1` (Inclusive)
* Maximum value: `63` (Inclusive)"#]
pub type SurveillanceIdentifierType = i32;

#[doc = r#"A string representing the vehicle's tail number. Typically used for aircraft vehicles.  DoD aircraft are identified by a five-digit tail number. For most military aircraft, the first two digits of the tail number are the fiscal year that the aircraft was ordered, and the remaining digits are the last three digits of the airframe's serial number. US Civil aircraft are identified by a two to six character alphanumeric registration number assigned by the ICAO or FAA. International tail numbers (registrations) follow similar patterns, but may be up to 10 characters long, often containing a dash separating the country code from the specific alphanumeric registration.

# Restrictions
* Pattern: `[A-Z0-9\-]{2,10}`
* Minimum length: `2`
* Maximum length: `10`"#]
pub type TailNumberType = String;

#[doc = r#"Indicates temperature in degrees Celsius (C)."#]
pub type TemperatureType = f64;

#[doc = r#"UCI uses the W3C (www.w3.org) definition of time exactly as given in the specification for xs:time with a further restriction that only the "Zulu" time zone be used.  xs:time is based on Coordinated Universal Time (UTC) and allows seconds to be specified with decimal digits to arbitrary precision.  See the W3C specification of xs:time for further details.

# Restrictions
* Pattern: `.+Z`"#]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimeType(
	#[serde(with = "crate::v2_5::serde_utils::naive_time")]
	pub chrono::NaiveTime,
);
impl Deref for TimeType {
	type Target = chrono::NaiveTime;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for TimeType {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl AsRef<chrono::NaiveTime> for TimeType {
	fn as_ref(&self) -> &chrono::NaiveTime {
		&self.0
	}
}
impl AsMut<chrono::NaiveTime> for TimeType {
	fn as_mut(&mut self) -> &mut chrono::NaiveTime {
		&mut self.0
	}
}
impl From<chrono::NaiveTime> for TimeType {
	fn from(p: chrono::NaiveTime) -> Self {
		Self(p)
	}
}
impl Into<chrono::NaiveTime> for TimeType {
	fn into(self) -> chrono::NaiveTime {
		self.0
	}
}


#[doc = r#"A string representing the schema component name or enumeration value as it appears in the schema.  The UCI Style and Design Specification restricts these values to the alphanumeric, underscore, and dash characters.

# Restrictions
* Pattern: `[a-zA-Z0-9_\-]{1,128}`
* Minimum length: `1`
* Maximum length: `128`"#]
pub type UciSchemaComponentNameType = String;

#[doc = r#"String representing the UCI version.

# Restrictions
* Pattern: `[0-9]{3}\.[0-9]{1,2}(\.[0-9]{1,2})([a-z]{1,2})?(_[a-zA-Z0-9\-]{1,45})?`
* Minimum length: `7`
* Maximum length: `57`"#]
pub type UciSchemaVersionStringType = String;

#[doc = r#"Indicates 64-bit floating point values with a range of [-1,1].

# Restrictions
* Minimum value: `-1` (Inclusive)
* Maximum value: `1` (Inclusive)"#]
pub type UnitBallDoubleType = f64;

#[doc = r#"Indicates 32-bit floating point values with a range of [-1,1].

# Restrictions
* Minimum value: `-1.0` (Inclusive)
* Maximum value: `1.0` (Inclusive)"#]
pub type UnitBallFloatType = f32;

#[doc = r#"A string representing a unique identifier for each unit. ([A-Z][A-Z] Position 1-2, SYSTEM ASSIGNED RECORD ORIGINATOR.  Two character code associated with the organization originating the unique UNIT ID.  Assigned by system at element creation time.     [ABCDEGJMNSX] Position 3, OB_TYPE     A Air Force     B Joint Forces     C Civilian     D Defensive Missile Forces     E Net     G Army     J Space Order of Battle (SOB)     M Ministry, Other Than Ministry of Defense (MOD)     N Navy     S Strategic Missile Forces     X Air Defense Order of Battle (ADOB)     [A-Z][A-Z] Position 4-5, ALLEGIANCE.  This item is selected from the 2 character list of valid State Department Allegiance codes.     [00001-99999] Position 6-10, ACCESSION_NUMBER.).

# Restrictions
* Pattern: `[A-Z]{2}[ABCDEGJMNSX][A-Z]{2}[0-9]{5}`
* Length: `10`"#]
pub type UnitIdentifierType = String;

#[doc = r#"Indicates 64-bit floating point values with a range of [0,1].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `1.0` (Inclusive)"#]
pub type UnitIntervalDoubleType = f64;

#[doc = r#"Indicates 32-bit floating point values with a range of [0,1].

# Restrictions
* Minimum value: `0.0` (Inclusive)
* Maximum value: `1.0` (Inclusive)"#]
pub type UnitIntervalFloatType = f32;

#[doc = r#"A string representing a translated unit name or identification given the unit by appropriate authority or orders as used in official orders or communications within the national military or civilian establishment of the country of allegiance. A unit name must be established for every unit in the database.  For each Unit logical record, unit naming conventions established in production programs should be employed.  If official sources are not available, the unit name believed most correct is used.  A unit's primary designation usually includes service specialty and command echelon.

# Restrictions
* Pattern: `[a-zA-Z0-9&#x20;'\(\).,@;+\-]{1,256}`
* Minimum length: `1`
* Maximum length: `256`"#]
pub type UnitNameType = String;

#[doc = r#"A UUID is a 128-bit number (32 hexadecimal digits, 16 bytes) that is conformant to any version of variant 1 or nil UUID, as described in IETF RFC 4122.

# Restrictions
* Pattern: `(0{8}(-0{4}){3}-0{12})|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[1-5][a-fA-F0-9]{3}-[89abAB][a-fA-F0-9]{3}-[a-fA-F0-9]{12})`
* Length: `36`"#]
pub type UniversallyUniqueIdentifierType = String;

#[doc = r#"A string representing the ID type for UCI IDs that correspond to system users.

# Restrictions
* Pattern: `[a-zA-Z0-9]{1,25}`
* Minimum length: `1`
* Maximum length: `25`"#]
pub type UserIdentifierType = String;

#[doc = r#"The call sign which identifies one or more communication facilities, commands, authorities, activities, or units. This type is based on MIL-STD-6040.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,38}`
* Minimum length: `1`
* Maximum length: `38`"#]
pub type UsmtfAircraftCallSignType = String;

#[doc = r#"The unique code name or nickname assigned to a joint exercise or plan or to designate exercise message traffic. Derived from MIL-STD-6040

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,56}`
* Minimum length: `1`
* Maximum length: `56`"#]
pub type UsmtfExerciseNicknameType = String;

#[doc = r#"Indicates the serial number assigned to a specific message. The originating command may develop the message serial number by any method. Derived from MIL-STD-6040.

# Restrictions
* Pattern: `[\-A-Z0-9&#x20;\.,\(\)&amp;\?!@#$%\^\*=_\+\[\]\{\}\\&quot;';>&lt;~`\|]{1,7}`
* Minimum length: `1`
* Maximum length: `7`"#]
pub type UsmtfMessageSerialNumberType = String;

#[doc = r#"The identifying number assigned to a specific mission within an approved project, task, operation, or exercise. The mission number may be the numeric assigned to an air mission that will accomplish a particular task. This type is based on MIL-STD-6040.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,8}`
* Minimum length: `1`
* Maximum length: `8`"#]
pub type UsmtfMissionNumberType = String;

#[doc = r#"The unique operation name, nickname, or codeword that identifies a specific operation.  This type is based on MIL-STD-6040.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,32}`
* Minimum length: `1`
* Maximum length: `32`"#]
pub type UsmtfOperationCodewordType = String;

#[doc = r#"Indicates the originator of the USMTF message. Derived from MIL-STD-6040. Note that the pattern "[\-\.,\(\)_\?A-Z0-9&#x20;]{1,30}" in MIL-STD-6040 was modified here to remove the "\" delimiter preceding the "_" character in order to validate within the UCI schema.

# Restrictions
* Pattern: `[\-\.,\(\)_\?A-Z0-9&#x20;]{1,30}`
* Minimum length: `1`
* Maximum length: `30`"#]
pub type UsmtfOriginatorType = String;

#[doc = r#"A number assigned serially to identify the sequential version of a message qualifier for a basic message.  This type is based on MIL-STD-6040.

# Restrictions
* Pattern: `[0-9]{1,3}`
* Minimum value: `1` (Inclusive)
* Maximum value: `999` (Inclusive)"#]
pub type UsmtfSerialNumberOfQualifierType = i32;

#[doc = r#"The official name or identification given a military unit by appropriate authority or orders, as used in official communications within the national military establishment.  This type is based on MIL-STD-6040.

# Restrictions
* Pattern: `[\-\.,\(\)\?A-Z0-9&#x20;]{1,24}`
* Minimum length: `1`
* Maximum length: `24`"#]
pub type UsmtfUnitDesignatorType = String;

#[doc = r#"Version information consists of four 3 digit fields (plus optional engineering field)
                                    WWWe.XXXe.YYYe.ZZZe
	                          where,
	                                WWW - reflects the Direct Structural Impact change history to a message or type
	                                XXX - reflects the Indirect Structural Impact change history to a message or type
	                                YYY - reflects the Direct Optional Impact change history to a message or type
	                                ZZZ - reflects the Indirect Optional Impact change history to a message or type
	                                e   - reflects engineering level revision for each of the four levels (internal use only)

# Restrictions
* Pattern: `([0-9]{3}([a-z])?\.){3}[0-9]{3}([a-z])?`
* Minimum length: `15`
* Maximum length: `19`"#]
pub type VersionType = String;

#[doc = r#"A string representing up to 1024 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,1024}`
* Minimum length: `1`
* Maximum length: `1024`"#]
pub type VisibleString1024Type = String;

#[doc = r#"A string representing up to 128 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,128}`
* Minimum length: `1`
* Maximum length: `128`"#]
pub type VisibleString128Type = String;

#[doc = r#"A string representing up to 20 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,20}`
* Minimum length: `1`
* Maximum length: `20`"#]
pub type VisibleString20Type = String;

#[doc = r#"A string representing between 2 and 4 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{2,4}`
* Minimum length: `2`
* Maximum length: `4`"#]
pub type VisibleString24Type = String;

#[doc = r#"A string representing up to 256 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,256}`
* Minimum length: `1`
* Maximum length: `256`"#]
pub type VisibleString256Type = String;

#[doc = r#"A string representing up to 32 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,32}`
* Minimum length: `1`
* Maximum length: `32`"#]
pub type VisibleString32Type = String;

#[doc = r#"String type of up to 480 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,480}`
* Minimum length: `1`
* Maximum length: `480`"#]
pub type VisibleString480Type = String;

#[doc = r#"A string representing up to 512 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,512}`
* Minimum length: `1`
* Maximum length: `512`"#]
pub type VisibleString512Type = String;

#[doc = r#"A string representing up to 64 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,64}`
* Minimum length: `1`
* Maximum length: `64`"#]
pub type VisibleString64Type = String;

#[doc = r#"A string representing up to 81 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{1,81}`
* Minimum length: `1`
* Maximum length: `81`"#]
pub type VisibleString81Type = String;

#[doc = r#"A string representing 10 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{10}`
* Length: `10`"#]
pub type VisibleStringLength10Type = String;

#[doc = r#"A string representing 12 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{12}`
* Length: `12`"#]
pub type VisibleStringLength12Type = String;

#[doc = r#"A string representing 15 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{15}`
* Length: `15`"#]
pub type VisibleStringLength15Type = String;

#[doc = r#"A string representing 17 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{17}`
* Length: `17`"#]
pub type VisibleStringLength17Type = String;

#[doc = r#"A string representing 20 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{20}`
* Length: `20`"#]
pub type VisibleStringLength20Type = String;

#[doc = r#"A string representing 80 characters in length, restricted to visible characters (0x20-0x7E).

# Restrictions
* Pattern: `[&#x20;-&#x7E;]{80}`
* Length: `80`"#]
pub type VisibleStringLength80Type = String;

#[doc = r#"Indicates the brightness of an observed object."#]
pub type VisualMagnitudeType = f64;

#[doc = r#"String type of 1024 characters in length, restricted to visible characters (0x21-0x7E) and whitespace characters.

# Restrictions
* Pattern: `[&#x20;-&#x7E;\n\r]{0,1024}`
* Minimum length: `0`
* Maximum length: `1024`"#]
pub type WhitespaceVisibleString1024Type = String;

#[doc = r#"String type of 4096 characters in length, restricted to visible characters (0x21-0x7E) and whitespace characters.

# Restrictions
* Pattern: `[&#x20;-&#x7E;\n\r]{0,4096}`
* Minimum length: `0`
* Maximum length: `4096`"#]
pub type WhitespaceVisibleString4096Type = String;

