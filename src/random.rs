///A very simple random number generator.
///Not especially good at generating truly random bits,
///but good enough for our needs in this package.
pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(s: u32) -> Self {
        let mut seed = s & 0x7fffffffu32;
        //avoid bad seeds.
        if seed == 0 || seed == 2147483647u32 {
            seed = 1;
        }
        Random { seed: seed }
    }

    pub fn next(&mut self) -> u32 {
        const M: u32 = 2147483647u32; // 2^31-1
        const A: u64 = 16807; // bits 14, 8, 7, 5, 2, 1, 0

        // We are computing
        //       seed_ = (seed_ * A) % M,    where M = 2^31-1
        //
        // seed_ must not be zero or M, or else all subsequent computed values
        // will be zero or M respectively.  For all other values, seed_ will end
        // up cycling through every number in [1,M-1]
        let product = self.seed as u64 * A;

        // Compute (product % M) using the fact that ((x << 31) % M) == x.
        self.seed = ((product >> 31) + (product & M as u64)) as u32;
        // The first reduction may overflow by 1 bit, so we may need to
        // repeat.  mod == M is not possible; using > allows the faster
        // sign-bit-based test.
        if self.seed > M {
            self.seed -= M;
        }
        self.seed
    }

    // Returns a uniformly distributed value in the range [0..n-1]
    pub fn uniform(&mut self, n: u32) -> u32 {
        self.next() % n
    }

    // Randomly returns true ~"1/n" of the time, and false otherwise.
    pub fn one_in(&mut self, n: u32) -> bool {
        self.next() % n == 0
    }

    // Skewed: pick "base" uniformly from range [0,max_log] and then
    // return "base" random bits.  The effect is to pick a number in the
    // range [0,2^max_log-1] with exponential bias towards smaller numbers.
    pub fn skewed(&mut self, max_log: u32) -> u32 {
        let offset = self.uniform(max_log + 1);
        self.uniform(1 << offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn random_test() {
        let mut rand = Random::new(16);
        for i in 1..100 {
            let random = rand.uniform(i);
            assert!(random < i);
        }
    }
}
