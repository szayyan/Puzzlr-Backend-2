use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha3::{Digest, Sha3_256};

pub struct Generator
{
    rng_base: StdRng
}

pub fn construct( src: &str, epoch: u32  ) -> Generator
{
    let mut hasher = Sha3_256::new();
    hasher.update( src.as_bytes() );
    hasher.update( epoch.to_be_bytes() );
    let mut finalized = hasher.finalize();
    let result: &[u8;32] = finalized.as_mut();

    return Generator { rng_base: StdRng::from_seed( *result )  };
}

impl Generator {
    
    pub fn int(&mut self, max: u32) -> u32
    {
        return self.rng_base.gen_range(0..max);
    }
}