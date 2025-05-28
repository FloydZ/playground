/*
Code that contains kernels to run on accelerator in parallel. A kernel 
represents the basic unit of executable code. Each kernel will be 
executed on one work item ("pixel") of your parallel task:

1 work item = 1 "pixel" in your image 

A practical application may generate thousands or even millions of 
work-items, but for the simple task of adding 64 numbers, 
eight work-items will suffice. The program file add_numbers.cl 
contains a function called add_numbers that performs this operation. 
Like all kernel functions, it returns void and its name is preceded by 
the __kernel identifier.

The kernel has 64 values to add together and eight work-items with 
which to add them. After each work-item computes its sum of eight 
values, these partial results will be added together to form a sum for 
the entire group. In the end, the kernel will return two sums — one for 
each work-group executing the kernel.

data <===== input_buffer
group_result <===== sum_buffer
*/

#pragma OPENCL EXTENSION cl_intel_printf : enable   

typedef unsigned char u8;
typedef unsigned int  u32;
typedef unsigned long u64;
typedef unsigned long long u128;

typedef u64 T;
typedef u128 T2;

__constant u8 lookup[16] = {
    0x0, 0x8, 0x4, 0xc, 0x2, 0xa, 0x6, 0xe,
    0x1, 0x9, 0x5, 0xd, 0x3, 0xb, 0x7, 0xf,
};

/// \param n[in]: 
/// \param m[in]: max number of bits set in n
/// \return bit-reverse(n)
u8 reverse(const u8 n, const u32 m) {
   return ((lookup[n&0b1111] << 4) | lookup[n>>4]) >> m;
}

/// \param a[in] number 1
/// \param b[in] number 2
/// \return a*b in 128bits
u128 u64mul(const u64 a, const u64 b) {
    u128 ret = a*b;
    ret ^= ((u128)mul_hi(a, b)) << 64u;
    return ret;
}

/// \param a[in] number 1
/// \param b[in] number 2
/// \return a*b in 64bits
u64 u32mul(const u32 a, const u32 b) {
    u64 ret = a*b;
    ret ^= ((u64)mul_hi(a, b)) << 32u;
    return ret;
}

/// \param a[in] number 1
/// \param b[in] number 2
/// \return a*b in 128bits
T2 umul(const T a, const T b) {
#ifdef USE32 // TODO
    return u32mul(a, b);
#endif
    return u64mul(a, b);
}

/// NOTE: only for a special prime
/// \param a[in]:
/// \param b[in]:
/// \return c = a * b mod (2**64 - 2**32 + 1)
u64 goldilock_reduction_multiplication(const u64 a,
                                       const u64 b) {
    const u64 m = ((u32)-1u);
    const u64 hi = mul_hi(a, b);
    const u64 lo = a * b;
    const u64 X3 = hi >> 32;
    const u64 X2 = hi & m;
    const u64 X1 = lo;
    
    u64 c = X1 + (X2 * m) - X3;
    if (c > 0xffffffff00000001) { c -=0xffffffff00000001; }
    return c;
}

/// \param a[in]: 
/// \param b[in]:
/// \param q[in]: prime
/// \return c = a * b mod q
u64 barret_reduction_u128_u64(const u64 a, 
                              const u64 b,
                              const u64 prime) {
    // TODO precompute.
    const u64 k = 64 - clz(prime);
    const u64 mu = (((u128)1ull) << (2*k + 1)) / ((u128)prime);
   

    const u128 C = u64mul(a, b);
    u128 r = C >> (k - 2);
    r = r * mu;
    r = r >> (k+3);
    r = r*prime;
    u64 C2 = C - r;
    if (C2 >= prime) {
        C2 = C2 - prime;
    }

    return C2;
}

// scaled_op2 = (op2 << 64) / prime
u64 mul_and_reduce_shoup_u128_u64(const u64 op1,
                                  const u64 op2,
                                  const u64 scaled_op2,
                                  const u64 prime) {
    // auto Shoup(const word64 in, const word64 prime) {
    //   word128 temp = static_cast<word128>(in) << 64;
    //   return static_cast<word64>(temp / prime);
    // }
    const u64 hi = mul_hi(op1, scaled_op2);
    const u64 lo = op1 * op2;
    return lo - (hi * prime);
}

/// \param
void butterfly_ntt(T *a, 
                   T *b,
                   const T *w,
                   const T *scaled_w, 
                   const T p) {
    const T two_p = 2*p;
    const T U = mul_and_reduce_shoup_u128_u64(*b, *w, *scaled_w, p);
    if (*a >= two_p) { *a -= two_p; }
    *b = *a + (two_p - U);
    *a += U;
}

/// Alg2 https://eprint.iacr.org/2023/1410.pdf
/// Cooley Tukey Butterfly
/// \param u[in/out]: 
/// \param u[in/out]: 
/// \param w[in]: root of unity
/// \param q[in]: prime 
T CT(__global T *u, __global T *v, const T w, const T q) {
    const T u_ = *u;
    const T v_ = *v;
    //const T t = barret_reduction_u128_u64(v_, w, q);
    //*u = (u_ + t) % q;
    //*v = (u_ - t + q) % q;
    const T t = u64mul(v_, w) % q;
    *u = (u_ + t) % q;
    *v = (u_ - t + q) % q;
}

/// from python
/// \param coeffs[in/out]
/// \param roots[in] roots of unity
/// \param scaled_roots[in] scaled roots of unity = (w << 64) / prime
/// \param prime[in]: 
/// \param n[in] number of elements in `coeffs`
__kernel void ntt2(__global T *coeffs,
                  __global const T *roots,
                  __global const T *scaled_roots,
                  const T prime, 
                  const T n) {
    const u32 log_number_coeffs = sizeof(n)*8 - clz(n);
    const u32 tid = get_local_id(0);
    const u32 reverse_tid = reverse(tid, log_number_coeffs);

    coeffs[tid] = mul_and_reduce_shoup_u128_u64(coeffs[tid], roots[tid], scaled_roots[tid], prime);
    const T tmp = coeffs[tid];
    barrier(CLK_LOCAL_MEM_FENCE);
    coeffs[reverse_tid] = tmp;
}

/// from https://eprint.iacr.org/2023/1410.pdf (Merge-NTT)
/// \param coeffs[in/out]
/// \param roots[in] roots of unity
/// \param prime[in]: 
/// \param n[in] number of elements in `coeffs`
__kernel void ntt(__global T *coeffs,
                  __global T *roots,
                  const T prime, 
                  const T n) {
    const u32 log_number_coeffs = sizeof(n)*8 - clz(n);
    const u32 tid = get_local_id(0);
    u32 t = n;
    for (u32 m = 1u; m < n; m=m*2) {
        t = t >> 1u;
        for (u32 i = 0; i < m; i++) {
            const u32 j1 = 2 * i * t;
            const u32 j2 = j1 + t;
            
            if (tid >= j1 && tid < j2) {
                CT(coeffs + tid, coeffs + tid + t, roots[m + i], prime);
            }

            barrier(CLK_LOCAL_MEM_FENCE);
        }
    }
}

__kernel void add_numbers(__global float4* data, 
                          __local float* local_result, 
                          __global float* group_result) {
    //u128 t = u64mul(1ull<<63, 1ull<<63);
    //printf("%llu\n", t>>64);

    const u64 q = 576460752303423649;
    const u64 c = barret_reduction_u128_u64(q-1,q-2,q);
    printf("%llu\n", c);

    float sum;
    float4 input1, input2, sum_vector; // array of 4 floats which support vectorization
    uint global_addr, local_addr;
    
    global_addr = get_global_id(0) * 2;
    input1 = data[global_addr];
    input2 = data[global_addr+1];
    sum_vector = input1 + input2; // perform four floating-point additions simultaneously
    
    local_addr = get_local_id(0);
    local_result[local_addr] = sum_vector.s0 + sum_vector.s1 + 
                               sum_vector.s2 + sum_vector.s3; 
    barrier(CLK_LOCAL_MEM_FENCE);
    
    if(get_local_id(0) == 0) {
        sum = 0.0f;
        for(unsigned int i=0; i<get_local_size(0); i++) {
            sum += local_result[i];
        }

        group_result[get_group_id(0)] = sum;// + t.b;
    }
}
