use std::cmp::Ordering;

pub struct TotalOrd<T>(pub T);

impl<T> PartialEq for TotalOrd<T>
where
    TotalOrd<T>: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for TotalOrd<T> where TotalOrd<T>: Ord {}

impl<T> PartialOrd for TotalOrd<T>
where
    TotalOrd<T>: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TotalOrd<f32> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
