pub trait BitCount: bitflags::Flags {
    fn count_bits(&self) -> Self::Bits;
}

impl<T: bitflags::Flags<Bits = u8>> BitCount for T {
    fn count_bits(&self) -> u8 {
        self.bits().count_ones() as u8
    }
}
