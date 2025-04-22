from numba import jit, vectorize
from numba import cuda
import numpy as np
import math

n = 1000000
rho1 = np.random.uniform(0.5, 1.5, size=n).astype(np.float32)
theta1 = np.random.uniform(-np.pi, np.pi, size=n).astype(np.float32)


@vectorize(['float32(float32,float32)'], target='cuda')
def hypot(x, y):
    return x * math.sqrt(1.+y*y)


@cuda.jit(device=True, inline=True)
def H(x: np.float32):
    if x < 0.0 or x > 1.0:
        return 0.0
    if x == 0.0:
        return 0
    if x == 1.0:
        return 0
    return (-x*math.log(x)-(1-x)*math.log(1-x))/math.log(2)


@cuda.jit(device=True, inline=True)
def binom(a: np.uint32, b: np.uint32):
    return a*b  # TODO not implemented math.comb(a, b)


@cuda.jit
def bbinom(c, a, b):
    idx = cuda.grid(1)
    c[idx] = binom(a[idx], b[idx])


@cuda.jit(device=True, inline=True)
def binomH(a: np.float32, b: np.float32):
    return H(b/a)


@cuda.jit
def bbinomH(c, a, b):
    idx = cuda.grid(1)
    c[idx] = binomH(a[idx], b[idx])


@cuda.jit(device=True, inline=True)
def log2(a: np.uint32):
    return math.log(a, 2)


@cuda.jit
def llog2(c, a):
    idx = cuda.grid(1)
    c[idx] = log2(a[idx])


@cuda.jit
def hypot1(z, x, y):
    idx = cuda.grid(1)
    z[idx] = x[idx] * math.sqrt(1.+y[idx]*y[idx])


def opt_exp_mmt(k: np.float32, w: np.float32, mem: np.float32, mode: bool):
    """
    mode = true
    """
    mini_t = 0
    idx = np.uint32(cuda.grid(1))
    stride = np.uint32(cuda.gridsize(1))
    stride = np.float32(stride) * 0.0001

    p = np.float32(0.0001)*idx
    l = np.float32(0.0001)*idx

    while p < 0.01:
        p += stride
        while l < np.float32(1-k):
            l += np.float32(0.0005)
            #    for l2 in float_range(0., 1.-k-w+p, 0.01):
            reps1 = binomH(p, p/2.)
            l1 = reps1
            l2 = l - l1

            if l1 < 0 or l2 < 0 or l > 1-k-w-p or (w-p) > (1.-k-l) or p >= k+l-w:
                continue

            tmp = binomH((k+l)/2, p/4.)
            L1 = tmp
            L2 = 2*L1 - (L1 if mode else l1)
            L3 = 2*L2 - l2 

            space = max(max(L1/2, L2), L3)
            #if space > mem:
            #    continue

            perms = binomH(1, w) - binomH(1.-k-l, w-p) - binomH(k+l, p)
            run = max(max(L1, L2), L3)
            run += perms 
            #+ (max(0, L1-l1) if mode else 0.)

            if run_ < mini_t:
                mini_t = run_
    return mini_t


data = hypot(rho1, theta1)
print(data)

threadsperblock = 32
blockspergrid = (n + (threadsperblock - 1)) // threadsperblock
d_rho = cuda.to_device(rho1)
d_theta = cuda.to_device(theta1)
d_z = cuda.device_array_like(rho1)
hypot1[blockspergrid, threadsperblock](d_z, d_rho, d_theta)
cuda.synchronize()
print(d_z.copy_to_host())


A = cuda.to_device(np.random.uniform(0.0001, 0.4999, size=n).astype(np.float32))
B = cuda.to_device(np.random.uniform(0.0001, 0.4999, size=n).astype(np.float32))
C = cuda.device_array_like(A)
bbinomH[blockspergrid, threadsperblock](C, A, B)
cuda.synchronize()
print(C.copy_to_host())


k = np.float32(0.8)
w = np.float32(0.2)
m = np.float32(1.0)
opt_exp_mmt[blockspergrid, threadsperblock](k, w, m)

