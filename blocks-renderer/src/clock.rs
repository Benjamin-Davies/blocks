use std::fmt;

pub trait Clock {
    type Instant: fmt::Debug + Copy;

    fn now(&self) -> Self::Instant;
    fn seconds_elapsed(&self, start: Self::Instant, end: Self::Instant) -> f32;
}
