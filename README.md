# IntOps
methods and consts shared by all integers under a trait (for generic impls accros all ints)

## Examples

refer to [IntTraits](https://docs.rs/intops/latest/intops/trait.IntTraits.html) for all shared traits

refer to [IntOps](https://docs.rs/intops/latest/intops/trait.IntOps.html) for all shared methods

refer to [IntLayout](https://docs.rs/intops/latest/intops/trait.IntLayout.html) for all shared consts and types

Note: IntOps has these traits as trait bounds

Shl, And, Not
```rust
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
```
