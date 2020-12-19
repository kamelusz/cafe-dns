pub struct BitVector64 {
    data: u64
}

impl BitVector64 {
    pub fn new() -> Self {
        Self { 
            data: 0 
        }
    }

    pub fn set(&mut self, i: u8, value: bool) {
        if value {
            self.data |=  1 << i
        } else {
            self.data &= !(1 << i)
        }
    }

    pub fn get(&self, i: u8) -> bool {
        let result = (self.data >> i) & 1;
        result != 0
    }

    pub fn set_part(&mut self, offset: u8, length: u8, value: u64) {
        let mask = BitVector64::calculate_mask(length);
        let value = (value & mask) << offset;
        let mask = mask << offset;
        self.data &= !mask;
        self.data |= value;
    }

    pub fn get_part(&self, offset: u8, length: u8) -> u64 {
        let mask = BitVector64::calculate_mask(length);
        let value = self.data() >> offset;

        return value & mask;
    }

    pub fn data(&self) -> u64 {
        self.data
    }

    fn calculate_mask(length: u8) -> u64 {
        if length == 64 {
            0xffff_ffff_ffff_ffff
        } else {
            (1 << length) - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get() {
        let mut bvector = BitVector64::new();
        for x in 0 .. 63 {
            assert_eq!(bvector.get(x), false);
        }

        bvector.set(0, true);
        assert_eq!(bvector.get(0), true);
        for x in 1 .. 63 {
            assert_eq!(bvector.get(x), false);
        }

        bvector.set(12, true);
        assert_eq!(bvector.get(0), true);
        assert_eq!(bvector.get(12), true);
        assert_eq!(bvector.get(11), false);
        assert_eq!(bvector.get(13), false);

        bvector.set(0, false);
        bvector.set(31, true);
        assert_eq!(bvector.get(0), false);
        assert_eq!(bvector.get(11), false);
        assert_eq!(bvector.get(12), true);
        assert_eq!(bvector.get(13), false);
        assert_eq!(bvector.get(31), true);
        for x in 32 .. 63 {
            assert_eq!(bvector.get(x), false);
        }
    }

    #[test]
    fn set_get_part() {
        let mut bvector = BitVector64::new();

        // 0b0011 0000
        bvector.set_part(4, 2, 3);
        assert_eq!(bvector.data(), 48);
        assert_eq!(bvector.get_part(4, 2), 3);

        // 0b0011 0010
        bvector.set_part(1, 1, 1);
        assert_eq!(bvector.data(), 50);
        assert_eq!(bvector.get_part(0, 2), 2);

        // 0b0011 0001
        bvector.set_part(0, 3, 1);
        assert_eq!(bvector.data(), 49);
        assert_eq!(bvector.get_part(1, 4), 8);

        // 0b0000 0011
        bvector.set_part(1, 7, 1);
        assert_eq!(bvector.data(), 3);
        assert_eq!(bvector.get_part(1, 1), 1);
    }
}
