use std::arch::x86_64::{__m128i, _mm_setzero_si128};
use miniaes_sys::{encrypt, decrypt, expand_key};

#[repr(align(16))]
struct aes {
    key: [u8; 16],
    keys: [__m128i; 11],
}

impl aes {
    pub fn new() -> aes {
        let key =  [0u8; 16];
        //let mut keys =  [0; 11*16];
        let mut keys =  [unsafe { _mm_setzero_si128() } ; 11];
        unsafe {
            miniaes_sys::expand_key(keys.as_mut_ptr() as *mut __m128i, key.as_ptr());
        };
        aes { 
            key,
            keys,
        }
    } 

    pub fn encrypt(&self, data: [u8; 16]) -> [u8; 16]{
        let mut ret = data.clone();
        let key_ptr = self.keys.as_ptr() as *const __m128i;
        let data_ptr = ret.as_mut_ptr() as *mut __m128i;
        unsafe {
            miniaes_sys::encrypt(key_ptr, data_ptr)
        };

        ret
    }

    pub fn decrypt(&self, data: [u8; 16]) -> [u8; 16]{
        let mut ret = data.clone();
        let key_ptr = self.keys.as_ptr() as *const __m128i;
        let data_ptr = ret.as_mut_ptr() as *mut __m128i;
        unsafe {
            miniaes_sys::decrypt(key_ptr, data_ptr)
        };

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = aes::new();
        let data = [0u8; 16];
        a.encrypt(data);
        a.decrypt(data);
        assert_eq!(data[0], 0);
    }
}
