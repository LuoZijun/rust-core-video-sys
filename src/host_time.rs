use crate::libc::{ c_double, uint32_t, uint64_t, };

extern "C" {
    /// @function   CVGetCurrentHostTime
    /// @abstract   Retrieve the current value of the host time base.
    /// @discussion On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
    ///             may be used interchangeably.
    /// @result     The current host time.
    pub fn CVGetCurrentHostTime() -> uint64_t;
    /// @function   CVGetHostClockFrequency
    /// @abstract   Retrieve the frequency of the host time base.
    /// @discussion On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
    ///             may be used interchangeably.
    /// @result     The current host frequency.
    pub fn CVGetHostClockFrequency() -> c_double;
    /// @function   CVGetHostClockMinimumTimeDelta
    /// @abstract   Retrieve the smallest possible increment in the host time base.
    /// @result     The smallest valid increment in the host time base.
    pub fn CVGetHostClockMinimumTimeDelta() -> uint32_t;
}


#[test]
fn test_get_curr_time() {
    unsafe {
        assert_eq!(CVGetCurrentHostTime() > 0, true);
    }
}