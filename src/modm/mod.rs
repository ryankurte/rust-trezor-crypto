use cty::c_int;




pub type Bignum25519 = [u32; 10];

pub type Bignum256Modm = [u32; 9];

#[no_mangle]
pub unsafe extern "C" fn check256_modm(v: *const Bignum256Modm) -> c_int {
    //ok &= iszero256_modm(x) ^ 1;
	//barrett_reduce256_modm(t, z, x);
	//ok &= eq256_modm(t, x);
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn iszero256_modm(v: *const Bignum256Modm) -> c_int {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn eq256_modm(a: *const Bignum256Modm, b: *const Bignum256Modm) -> c_int {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn get256_modm(v: *mut u64, r: *const Bignum256Modm) -> c_int{
    todo!()
}


#[no_mangle]
pub unsafe extern "C" fn set256_modm(r: *mut Bignum256Modm, v: u64) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn add256_modm(r: *mut Bignum256Modm, x: *const Bignum256Modm, y: *const Bignum256Modm) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn sub256_modm(r: *mut Bignum256Modm, x: *const Bignum256Modm, y: *const Bignum256Modm) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mul256_modm(r: *mut Bignum256Modm, x: *const Bignum256Modm, y: *const Bignum256Modm) {
    todo!()
}


#[no_mangle]
pub unsafe extern "C" fn copy256_modm(r: *mut Bignum256Modm, x: *const Bignum256Modm) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn mulsub256_modm(r: *mut Bignum256Modm, a: *const Bignum256Modm, b: *const Bignum256Modm, c: *const Bignum256Modm) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn muladd256_modm(r: *mut Bignum256Modm, a: *const Bignum256Modm, b: *const Bignum256Modm, c: *const Bignum256Modm) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn expand_raw256_modm(o: *mut Bignum256Modm, i: *const u8, len: usize) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn expand256_modm(o: *mut Bignum256Modm, i: *const [u8; 32usize],
) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn contract256_modm(o: *mut [u8; 32], i: *const Bignum256Modm) {
    todo!()
}