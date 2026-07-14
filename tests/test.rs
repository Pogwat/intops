#[test]
fn main() {
    use intops::IntOps;
    pub trait Nummy:IntOps {
        fn get_some_bit(&self, pos:usize) -> bool {
            (Self::ONE<<pos & *self)!=Self::ZERO
        }
    }
    impl <T:IntOps> Nummy for T {}
    assert_eq!(2_u8.get_some_bit(5),false);
    assert_eq!(2_u8.get_some_bit(1),true);
    assert_eq!(2_u8.get_some_bit(0),false);
}
