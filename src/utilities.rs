//! Utility functions to help make more modular code.

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
const C: f32 = 1080000000.0;
const C_MPH: f32 = 671000000.0;

/// Calculated the speed detected by a detected by a doppler radar.
///
/// The calculated speed would be in kmph unit.
pub fn calculate_speed(detected: f32, transmitted: f32) -> f32 {
    C * detected / (2.0 * transmitted)
}

/// Calculated the speed detected by a detected by a doppler radar.
///
/// The calculated speed would be in mph unit.
pub fn calculate_speed_mph(detected: f32, transmitted: f32) -> f32 {
    C_MPH * detected / (2.0 * transmitted)
}

/// Converts a 2 digit decimal number into a BCD encoding.
///
/// The BCD number is encoded in an 8-bit unsigned integer in which
/// the first 4 bits of the number represents the 10s and the last 4
/// bits of number represents the 1s.
//
/// If the number given is greater than 99 None will be returned.
//
/// It uses the [Double dabble algorithm](https://en.wikipedia.org/wiki/Double_dabble) to convert a binary number
/// to BCD.
pub fn bin_to_bcd(mut number: u8) -> Option<u8> {
    let mut result = 0;

    // Returning 0 if number is 100 or above
    if number > 99 {
        return None;
    }

    while number > 0 {
        // Getting the MSB
        if number & 0x80 != 0 {
            result += 1;
        }
        // Shifting number
        number = number << 1;

        // Breaking out once it has shifted 8 times
        if number == 0 {
            break;
        }

        // Adding 3 if any of the numbers are 5 or greater
        if (result & 0x0f) > 0x05 {
            result += 0x03;
        }

        if (result & 0xf0) > 0x50 {
            result += 0x30;
        }

        // Shifting output
        result = result << 1;
    }

    Some(result)
}

pub fn use_global<T, F>(var: &'static Mutex<RefCell<Option<T>>>, mut f: F) -> ()
where
    F: FnMut(&mut T),
{
    cortex_m::interrupt::free(|cs| {
        // Moving out comp
        let mut output: T = var.borrow(cs).replace(None).unwrap();

        f(&mut output);

        // Moving comp back
        *var.borrow(cs).borrow_mut() = Some(output);
    });
}
