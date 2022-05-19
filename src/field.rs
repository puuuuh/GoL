use kiam::when;
use rand::Rng;

pub struct Field<const W: usize, const H: usize>
where
    [(); W * H]:,
{
    pub data: [bool; W * H],
}

impl<const W: usize, const H: usize> Field<W, H>
where
    [(); W * H]:,
{
    pub fn new() -> Self {
        Self {
            data: [false; W * H],
        }
    }

    pub fn random() -> Self {
        let mut rand = rand::thread_rng();
        let mut data = [false; W * H];
        rand.fill(&mut data[..]); // Waiting for const generics to stabilize
        Self { data }
    }

    pub fn turn(&self, other: &mut Self) {
        for (i, v) in self.data.iter().enumerate() {
            other.data[i] = match (v, Self::neighbours(self, i % W, i / W)) {
                (true, 0..=1 | 4..) => false,
                (false, 3) => true,
                (v, _) => *v,
            }
        }
    }

    // TODO: corner-cases for H,W = 1,1
    fn neighbours(&self, x: usize, y: usize) -> u8 {
        let mut res = 0u8;
        let (l, r) = when! {
            x == 0 => ((W - 1), x + 1),
            x == W - 1 => (x - 1, 0),
            _ => (x - 1, x + 1),
        };
        let (t, b) = when! {
            y == 0 => ((H - 1), y + 1),
            y == H - 1 => (y - 1, 0),
            _ => (y - 1, y + 1),
        };

        res += self.data[t * W + l] as u8;
        res += self.data[t * W + x] as u8;
        res += self.data[t * W + r] as u8;
        res += self.data[b * W + l] as u8;
        res += self.data[b * W + x] as u8;
        res += self.data[b * W + r] as u8;

        res += self.data[y * W + l] as u8;
        res += self.data[y * W + r] as u8;
        res
    }
}

impl<const W: usize, const H: usize> Default for Field<W, H>
where
    [(); W * H]:,
{
    fn default() -> Self {
        Self::random()
    }
}
