// This code relies on Rust's standard timer module.
use std::time::SystemTime;

// get_typically_true_random_number()
// Returns a 16-bit random number based on the std::time timer.  In case the 
// timer hasn't changed significantly since this function was last called, 
// returns a pseudorandom number based on a simple Park-Miller algorithm 
// seeded from the timer.
//
const U_PM88_CONST: u128 = 16807;     // full-period multiplier (7 to the 5th)
const U_BITNESS_CONST: u128 = 0xffff; // mask for returning 16 random bits
static mut U_SEED: u128 = 0;          // stores last computed random number

fn get_typically_true_random_number() -> u64
{

    // Get a random integer.
    let mut i_random = SystemTime::now().duration_since(
                                  SystemTime::UNIX_EPOCH).unwrap().as_nanos();

    unsafe  // Not really: threads may randomly overwrite a random value.
    {
        // Has the timer turned over, w/rt its least significant bits, since 
        // we were last here?
        if i_random - U_SEED < U_BITNESS_CONST
        {
            // Get a pseudorandom number based on the last seed value.
            i_random = (U_SEED * U_PM88_CONST) % U_BITNESS_CONST;
		    U_SEED = i_random;
        }
        else
        {
            // Save this seed value for next time.
            U_SEED = i_random;
        }
    }
 
    return (i_random & U_BITNESS_CONST) as u64;
}


fn main()
{
    println!("0x{:04x}", get_typically_true_random_number());
}
