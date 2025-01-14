pub fn check_bitmask(bitmask: i32) {
    const EPOLLONESHOT: i32 = 0x40000000;
    const EPOLLIN: i32 = 0x1;
    const EPOLLET: i32 = 1 << 31;

    let read = bitmask & EPOLLIN != 0;
    let et   = bitmask & EPOLLET != 0;
    let oneshot = bitmask & EPOLLONESHOT != 0;

    println!("read_event? {read}, edge_triggered: {et}, oneshot?: {oneshot}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bitmask() {
        // TEST 1 from https://tools.ietf.org/html/rfc8032#section-7.1
//        let mut privkey = [0u8; 32];
//        privkey.copy_from_slice(
//            &hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
//                .unwrap(),
//        );
//        let mut expected_pubkey = [0u8; 32];
//        expected_pubkey.copy_from_slice(
//            &hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
//                .unwrap(),
//        );
//        assert_eq!(ed25519_privkey_to_pubkey(&privkey), expected_pubkey);
//

        let bitflag_a: i32 = 1 << 31;
        let bitflag_f: i32 = 1 << 30;
        let bitflag_b: i32 = 0x1;
        let bitmask: i32 = bitflag_a | bitflag_b;

        println!("{bitflag_a:032b}");
        println!("{bitflag_b:032b}");
        println!("{bitflag_f:032b}");
        println!("{bitmask:032b}");
        println!("{:?}", check_bitmask(bitmask));

    }
}
