use crate::libc::{ c_double, int16_t, int32_t, int64_t, uint32_t, uint64_t, };


// https://developer.apple.com/documentation/corevideo/cvoptionflags?language=objc
pub type CVOptionFlags = uint64_t;
pub type CVSMPTETimeType = uint32_t;
pub type CVSMPTETimeFlags = uint32_t;
pub type CVTimeFlags = int32_t;
pub type CVTimeStampFlags = uint64_t;


pub const kCVSMPTETimeType24: CVSMPTETimeType        = 0;
pub const kCVSMPTETimeType25: CVSMPTETimeType        = 1;
pub const kCVSMPTETimeType30Drop: CVSMPTETimeType    = 2;
pub const kCVSMPTETimeType30: CVSMPTETimeType        = 3;
pub const kCVSMPTETimeType2997: CVSMPTETimeType      = 4;
pub const kCVSMPTETimeType2997Drop: CVSMPTETimeType  = 5;
pub const kCVSMPTETimeType60: CVSMPTETimeType        = 6;
pub const kCVSMPTETimeType5994: CVSMPTETimeType      = 7;

pub const kCVSMPTETimeValid: CVSMPTETimeFlags   = (1 << 0);
pub const kCVSMPTETimeRunning: CVSMPTETimeFlags = (1 << 1);

pub const kCVTimeIsIndefinite: CVTimeFlags = 1 << 0;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct CVSMPTETime {
    pub subframes: int16_t,
    pub subframeDivisor: int16_t,
    pub counter: uint32_t,
    pub type_: uint32_t,
    pub flags: uint32_t,
    pub hours: int16_t,
    pub minutes: int16_t,
    pub seconds: int16_t,
    pub frames: int16_t,
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct CVTime {
    pub timeValue: int64_t,
    pub timeScale: int32_t,
    pub flags: int32_t
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CVTimeStamp {
    pub version: uint32_t,
    pub videoTimeScale: int32_t,
    pub videoTime: int64_t,
    pub hostTime: uint64_t,
    pub rateScalar: c_double,
    pub videoRefreshPeriod: int64_t,
    pub smpteTime: CVSMPTETime,
    pub flags: uint64_t,
    pub reserved: uint64_t,
}


pub const kCVTimeStampVideoTimeValid: CVTimeStampFlags              = (1 << 0);
pub const kCVTimeStampHostTimeValid: CVTimeStampFlags               = (1 << 1);
pub const kCVTimeStampSMPTETimeValid: CVTimeStampFlags              = (1 << 2);
pub const kCVTimeStampVideoRefreshPeriodValid: CVTimeStampFlags     = (1 << 3);
pub const kCVTimeStampRateScalarValid: CVTimeStampFlags             = (1 << 4);

// There are flags for each field to make it easier to detect interlaced vs progressive output
pub const kCVTimeStampTopField: CVTimeStampFlags                    = (1 << 16);
pub const kCVTimeStampBottomField: CVTimeStampFlags                 = (1 << 17);

// Some commonly used combinations of timestamp flags
pub const kCVTimeStampVideoHostTimeValid: CVTimeStampFlags          = (kCVTimeStampVideoTimeValid | kCVTimeStampHostTimeValid);
pub const kCVTimeStampIsInterlaced: CVTimeStampFlags                = (kCVTimeStampTopField | kCVTimeStampBottomField);


extern "C" {
    pub static kCVZeroTime: CVTime;
    pub static kCVIndefiniteTime: CVTime;
}
