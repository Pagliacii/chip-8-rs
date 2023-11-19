/// # Timers & Sound
///
/// Chip-8 provides 2 timers, a delay timer and a sound timer.
///
/// The delay timer is active whenever the delay timer register (DT) is non-zero.
/// This timer does nothing more than subtract 1 from the value of DT at a rate
/// of 60Hz. When DT reaches 0, it deactivates.
///
/// The sound timer is active whenever the sound timer register (ST) is non-zero.
/// This timer also decrements at a rate of 60Hz, however, as long as ST's value
/// is greater than zero, the Chip-8 buzzer will sound. When ST reaches zero,
/// the sound timer deactivates.
///
/// The sound produced by the Chip-8 interpreter has only one tone. The frequency
/// of this tone is decided by the author of the interpreter.
