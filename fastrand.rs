#[derive(Debug)]
pub struct FastRand(u64);

impl std::default::Default for FastRand {
    fn default() -> Self {
        Self(0x123456789abcdef0)
    }
}

pub trait Randable where Self: Sized {
    fn randomized(state: &mut FastRand) -> Self {
        let val = state.random_u64();
        Self::from_state(val)
    }
    fn rand_ranged(state: &mut FastRand, low: Self, high: Self) -> Self {
        let low = low.to_state();
        let high = high.to_state();
        assert!(high > low, "reversed bounds");
        let val = state.random_u64() % (high - low);
        Self::from_state(low + val)
    }
    fn from_state(state: u64) -> Self;
    fn to_state(&self) -> u64;
}

impl Randable for u32 {
    fn from_state(state: u64) -> Self {
        (state & 0xffffffff) as u32
    }

    fn to_state(&self) -> u64 {
        (*self) as u64
    }
}

impl Randable for usize {
    fn from_state(state: u64) -> Self {
        state as usize
    }

    fn to_state(&self) -> u64 {
        (*self) as u64
    }
}

impl Randable for char {
    fn from_state(state: u64) -> Self {
        let largest = char::MAX as u32;
        let surrogate_start = 0xd800u32;
        let surrogate_len = 0x800u32;
        let mut val = u32::from_state(state) % (largest - surrogate_len);
        if surrogate_start <= val {
            val += surrogate_len;
        }
        val.try_into().unwrap()
    }

    fn to_state(&self) -> u64 {
        ((*self) as u32).to_state()
    }
}

impl FastRand {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Produce a pseudo-random integer. Will likely be slow in
    /// the presence of contention.
    fn random_u64(&mut self) -> u64 {
        let new = self.0
            .wrapping_mul(2862933555777941757u64)
            .wrapping_add(3037000493u64);
        (*self).0 = new;
        new
    }

    pub fn random<R>(&mut self) -> R
        where R: Randable
    {
        R::randomized(self)
    }

    pub fn rand_range<R>(&mut self, low: R, high: R) -> R
        where R: Randable
    {
        R::rand_ranged(self, low, high)
    }
}
