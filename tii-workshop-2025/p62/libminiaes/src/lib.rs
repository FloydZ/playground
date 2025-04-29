use std::{arch::x86_64::*};


macro_rules! expand_round {
    ($keys:expr, $pos:expr, $round:expr) => {
        let mut t1 = $keys[$pos - 1];
        let mut t2;
        let mut t3;

        t2 = _mm_aeskeygenassist_si128(t1, $round);
        t2 = _mm_shuffle_epi32(t2, 0xff);
        t3 = _mm_slli_si128(t1, 0x4);
        t1 = _mm_xor_si128(t1, t3);
        t3 = _mm_slli_si128(t3, 0x4);
        t1 = _mm_xor_si128(t1, t3);
        t3 = _mm_slli_si128(t3, 0x4);
        t1 = _mm_xor_si128(t1, t3);
        t1 = _mm_xor_si128(t1, t2);

        $keys[$pos] = t1;
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn expand_key(keys: &mut [__m128i; 11], key: &[u8; 16]) {
    let ptr = key.as_ptr() as *const __m128i;
    keys[0] = _mm_loadu_si128(ptr);
    expand_round!(keys, 1,  0x01);
    expand_round!(keys, 2,  0x02);
    expand_round!(keys, 3,  0x04);
    expand_round!(keys, 4,  0x08);
    expand_round!(keys, 5,  0x10);
    expand_round!(keys, 6,  0x20);
    expand_round!(keys, 7,  0x40);
    expand_round!(keys, 8,  0x80);
    expand_round!(keys, 9,  0x1B);
    expand_round!(keys, 10, 0x36);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encrypt(keys: &[__m128i; 11], block: &mut [__m128i; 16]){
    let mut b = _mm_loadu_si128(block as *const __m128i);
    b = _mm_xor_si128(b, keys[0]);
    b = _mm_aesenc_si128(b, keys[1]);
    b = _mm_aesenc_si128(b, keys[2]);
    b = _mm_aesenc_si128(b, keys[3]);
    b = _mm_aesenc_si128(b, keys[4]);
    b = _mm_aesenc_si128(b, keys[5]);
    b = _mm_aesenc_si128(b, keys[6]);
    b = _mm_aesenc_si128(b, keys[7]);
    b = _mm_aesenc_si128(b, keys[8]);
    b = _mm_aesenc_si128(b, keys[9]);
    b = _mm_aesenclast_si128(b, keys[10]);
    _mm_storeu_si128(block as *mut __m128i, b);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decrypt(keys: &[__m128i; 11], block: &mut [__m128i; 16]){
    let mut b = _mm_loadu_si128(block as *const __m128i);
    b = _mm_xor_si128(b, keys[10]);
    b = _mm_aesdec_si128(b, keys[9]);
    b = _mm_aesdec_si128(b, keys[8]);
    b = _mm_aesdec_si128(b, keys[7]);
    b = _mm_aesdec_si128(b, keys[6]);
    b = _mm_aesdec_si128(b, keys[5]);
    b = _mm_aesdec_si128(b, keys[4]);
    b = _mm_aesdec_si128(b, keys[3]);
    b = _mm_aesdec_si128(b, keys[2]);
    b = _mm_aesdec_si128(b, keys[1]);
    b = _mm_aesdeclast_si128(b, keys[0]);
    _mm_storeu_si128(block as *mut __m128i, b);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        //unsafe {
        //    let keys = [[0i64,2]]
        //}
    }
}
