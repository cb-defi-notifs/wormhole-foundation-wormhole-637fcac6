module Wormhole::Deserialize {
    use 0x1::vector::{Self};
    use Wormhole::cursor::{Self, Cursor};
    use Wormhole::Uints::{U16, U32, U256, into_u16, into_u32, into_u256};

    public fun deserialize_u8(cur: &mut Cursor<u8>): u8 {
        cursor::poke(cur)
    }

     public fun deserialize_u16(cur: &mut Cursor<u8>): U16 {
        let res = vector::empty();
        let i = 0;
        while (i < 2) {
            let b = cursor::poke(cur);
            vector::push_back(&mut res, b);
            i = i + 1;
        };
        into_u16(res)
    }

    public fun deserialize_u32(cur: &mut Cursor<u8>): U32 {
        let res = vector::empty();
        let i = 0;
        while (i < 4) {
            let b = cursor::poke(cur);
            vector::push_back(&mut res, b);
            i = i + 1;
        };
        into_u32(res)
    }

    public fun deserialize_u64(cur: &mut Cursor<u8>): u64 {
        let res: u64 = 0;
        let i = 0;
        while (i < 8) {
            let b = cursor::poke(cur);
            res = (res << 8) | (b as u64);
            i = i + 1;
        };
        res
    }

    public fun deserialize_u128(cur: &mut Cursor<u8>): u128 {
        let res: u128 = 0;
        let i = 0;
        while (i < 16) {
            let b = cursor::poke(cur);
            res = (res << 8) | (b as u128);
            i = i + 1;
        };
        res
    }

     public fun deserialize_u256(cur: &mut Cursor<u8>): U256 {
        let res = vector::empty();
        let i = 0;
        while (i < 32) {
            let b = cursor::poke(cur);
            vector::push_back(&mut res, b);
            i = i + 1;
        };
        into_u256(res)
    }

    public fun deserialize_vector(cur: &mut Cursor<u8>, len: u64): vector<u8> {
        let result = vector::empty();
        while ({
            spec {
                invariant len >= 0;
                invariant len <  vector::length(bytes);
            };
            len > 0
        }) {
            vector::push_back(&mut result, cursor::poke(cur));
            len = len - 1;
        };
        result
    }
}

#[test_only]
module Wormhole::TestDeserialize{
    use Wormhole::Deserialize::{deserialize_u8, deserialize_u64, deserialize_u128, deserialize_vector};
    use Wormhole::cursor::{Self};

    #[test]
    fun test_deserialize_u8() {
        let cur = cursor::init(x"99");
        let byte = deserialize_u8(&mut cur);
        assert!(byte==0x99, 0);
        cursor::destroy_empty(cur);
    }

    #[test]
    fun test_deserialize_u64() {
        let cur = cursor::init(x"1300000025000001");
        let u = deserialize_u64(&mut cur);
        assert!(u==0x1300000025000001, 0);
        cursor::destroy_empty(cur);
    }

    #[test]
    fun test_deserialize_u128() {
        let cur = cursor::init(x"130209AB2500FA0113CD00AE25000001");
        let u = deserialize_u128(&mut cur);
        assert!(u==0x130209AB2500FA0113CD00AE25000001, 0);
        cursor::destroy_empty(cur);
    }

    #[test]
    fun test_deserialize_vector() {
        let cur = cursor::init(b"hello world");
        let hello = deserialize_vector(&mut cur, 5);
        deserialize_u8(&mut cur);
        let world = deserialize_vector(&mut cur, 5);
        assert!(hello == b"hello", 0);
        assert!(world == b"world", 0);
        cursor::destroy_empty(cur);
    }
}
