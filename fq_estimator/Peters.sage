
from math import inf, comb as binom
import time
import math
def log2(x):
    if x == 0:
        return 0
    return math.log2(x)

# |%%--%%| <qjj1ZFjbPW|kPmGTTxf9f>

def _mem_matrix(n: int, k: int, r: int):
    """Stolen from CryptographicEstimator
    """
    return n - k + 2 ** r

# |%%--%%| <kPmGTTxf9f|iL4rXhJKQa>

def stern(n:int, k:int, w:int, q:int, memory_bound=inf):
    """Stolen from CryptographicEstimator
    """
    time_min, memory_min, params = inf, inf, []
    solutions = 0
    k1 = k // 2
    for p in range(0, w//2):
        l_val = int(log2(binom(k1, p)) - log2(q-1)*p)
        l_search_radius = 10
        for l in range(max(0, l_val - l_search_radius), min(l_val+l_search_radius, n-k)):
            
            L1 = binom(k1, p) * (q-1)**p
            L2 = binom(k-k1, p) * (q-1)**p
            if (L1 > 0) and (log2(L1) > time_min):
                continue

            memory = log2((L1 + L2)*l + _mem_matrix(n, k, 0)) + log2(n)
            if memory > memory_bound:
                continue

            Tp = max(0,log2(binom(n,w)) - log2(binom(n-k-l, w-2*p)) - log2(binom(k1,p)**2) - solutions)

            Tg = (n-k)**2 * (n+k) // 2

            build = max(((k1 - p + 1) + (L2+L1)) * l, 1)
            cost_early_exit = max(1,int(max(q/(q-1) * (w - 2*p + 1)*2*p*(1 + (q - 2)/(q - 1)), 1)))
            L = L1*L2//q**l
            ops = build + L*cost_early_exit
            time = log2(Tg + ops) + Tp
            
            if time < time_min:
                time_min = round(time, 3)
                memory_min = round(memory, 3)
                params = [p, l, Tp]

    return time_min, memory_min, params

# |%%--%%| <iL4rXhJKQa|g01jOSIXeY>

print(stern(100, 50, 10, 3))
print(stern(961, 771, 48, 31))

# |%%--%%| <g01jOSIXeY|x8FMvevz9S>

def reps(k: int, l: int, p: int, q: int, delta: int):
    """taken from LEMMA 7.2.1: https://www.cits.ruhr-uni-bochum.de/imperia/md/content/diss.pdf"""
    p2 = p//2
    return log2(sum(binom(p - 2*i, p2 - i) * 
               binom(p, 2*i) * 
               (q-2)**(2*i) * 
               binom(k+l-p, delta - i) * 
               (q-1)**(delta-i) 
               for i in range(min(p2, delta))))
    
def mmt(n:int, k:int, w:int, q:int, memory_bound=inf, solutions=0):
    """not Stolen from CryptographicEstimator
    :param L1 = base list
    :param L2 = intermediate list
    :param L  = final output list
    """
    time_min, memory_min, params = inf, inf, []    
    for p in range(0, w//2):
        for l in range(0, n-k-(w-2*p)):
            k1 = (k + l) // 2
            for p1 in range((p+1)//2, w):
                L1 = binom(k1, p1) * (q-1)**p1
                if (L1 > 0) and (log2(L1) > time_min):
                    continue

                l1 = ceil(reps(k, l, p, q, p1 - p//2))
                if (l1 > l):
                    continue
                    
                L2 = max(1, L1**2 / q**l1)
                memory = log2((2*L1 + L2)*l + _mem_matrix(n, k, 0)) + log2(n)
                if memory > memory_bound:
                    continue

                Tp = max(0,log2(binom(n,w)) - log2(binom(n-k-l, w-2*p)) - log2(binom(k1,p)**2) - solutions)
                Tg = (n-k)**2 * (n+k) // 2
                L = L2**2/q**(l - l1)
    
                # need to construct l field elements of each vector of the base lists
                # you only need two baselists lists, the other two are copies
                build_base = max(((k1 - p1 + 1) + (2*L1)) * l, 1)
                # cost are per element
                build_intermediate = l*2 # addition of the intermediate targets + normal addition
                build_final = max(1, int(max(q/(q-1) * (w - 2*p1 + 1)*2*p1*(1 + (q - 2)/(q - 1)), 1)))
                ops = max(1, build_base + L*build_final + 2*L2*build_intermediate)
                time = log2(Tg + ops) + Tp

                if time < time_min:
                    time_min = round(time, 3)
                    memory_min = round(memory, 3)
                    params = [p, p1, l, l1, log2(L1), log2(L2), log2(L), Tp]

    return time_min, memory_min, params

# |%%--%%| <x8FMvevz9S|SHH0QzZmWr>

print(mmt(100, 50, 10, 3))
print(mmt(961, 771, 48, 31))

# |%%--%%| <SHH0QzZmWr|jPJEEazJog>

# Base Definitions
R1 = RealField(3000)
#R1 = RIF
RPrint = RealField(50)
# NOTE: l == 0 is only needed to sanity check to peters implementation
n,k,w,c,p,l=2918, 2918//2, 54, 10, 3, 20

# |%%--%%| <jPJEEazJog|r4Vv2c0g42>

def fq_iters(n: int, k: int, w: int, p: int, l:int):
    """ compute tha normally needed number of permutations. On logarithmic scale"""
    return log2(binom(n, w)/ (binom((k+l), 2*p) * binom(n-k-l, w-2*p)))

def print_matrix(m):
    """helper functions: prints the matrix in a smaller field, to save space"""
    for i in range(m.nrows()):
        for j in range(m.ncols()):
            print(RPrint(m[i, j]), end='')
            print(" ", end='')

        print()

# |%%--%%| <r4Vv2c0g42|qP375SCjo9>

def compute_beta(n: int, k: int, w: int, p: int, l: int, m=1):
    """
    computes the beta factor as described in https://eprint.iacr.org/2009/589.pdf
    This factor is the probability to go from state (2p)S to Success. This 
    additional transition is needed in the stern case, to describe the needed
    weight distribution of [p, p] and the needed \ell zeros in the error vector.
    """
    beta = R1(binom(k//2, p) * binom(k - k//2, p)) / R1(binom(k, 2*p))
    sign = R1(-1)
    tmp = R1(0)
    for i in range(1, m+1):
        sign *= R1(-1)
        tmp += R1(binom(m, i) * binom(n-k-w+2*p, i*l)) / R1(binom(n-k, i*l))
    beta *= tmp
    return beta

def transition_matrix(n: int, k: int, w: int, c: int, p: int, l: int):
    """
    Transitions Matrix by Andre/Floyd. (https://eprint.iacr.org/2022/1329)
    In comparision to Peters Transitions Matrix, this one aknowledges a 
    weight distribution on [n-k-l, k+l]. To be compatible to peters 
    matrix call this function with \ell=0
    """
    def transition(u: int, d: int, c: int, n: int, k: int, w: int, l: int):
        # from u to u+d by exchanging c columns
        return R1(sum(binom(w-u, i) * 
                      binom(n-k-l-w+u, c-i) *
                      binom(u, i-d) *
                      binom(k+l-u, c+d-i)
                      for i in range(max(d, 0), min(w-u+1, c+1, c+d+1)))) / R1(binom(n-k-l, c) * binom(k+l, c))
    A = matrix(R1, w+1, w+1)
    for i in range(w+1):
        for j in range(w+1):
            A[i,j] = transition(i, j-i, c, n, k, w, l)

    return A
    
def transition_matrix_peters(n: int, k: int, w: int, c: int, p: int, l: int):
    """
    Translations of Peters https://eprint.iacr.org/2009/589.pdf C code into python
    """
    def si_inc(u: int, d: int, i: int):
        return R1(binom(w-u, i) * 
                  binom(n-k-w+u, c-i) * 
                  binom(u, d+i) * 
                  binom(k-u, c-d-i)) / R1(binom(n-k, c) * binom(k, c))
    def si_dec(u: int, d: int, i: int):
        return R1(binom(w-u, d+i) * 
                  binom(n-k-w+u, c-d-i) * 
                  binom(u, i) * 
                  binom(k-u, c-i)) / R1(binom(n-k, c) * binom(k, c))

    P = matrix(R1, w+1, w+1)
    
    for u in range(w+1):
        for d in range(c, -1, -1):
            if (u-d >= 0) and (u-d < w+1):
                tmp = R1(0)
                for i in range(c-d+1):
                    if (w-u >= i) and (u >= d+i):
                        tmp += si_inc(u, d, i)
               
                P[u, u-d] = tmp

            if (u+d > 0) and (u+d < w+1):
                tmp = R1(0)
                for i in range(c-d+1):
                    if (w-u >= d+i) and (u >= i):
                        tmp += si_dec(u, d, i)
                
                P[u, u+d] = tmp
    return P

# |%%--%%| <qP375SCjo9|738l9SoKXX>

def peters_number_perms(n: int, k: int, w: int, c: int, p:int, l: int, m=1):
    """
    Implementation of Peters C code. Its binary compatible to the C code
    :param n: code length
    :param k: code dimension
    :param w: weight
    :param c: number of coordinates to exchange during each gaussian elimination
    :param p: weight of the good state
    :param l: window

    Additional parameters:
    :param x: k//2
    :param m: #Indyk Motwani iterations
    :param r: M4ri parameter
    """
    # one can use both
    P = transition_matrix(n,k,w,c,p,0)
    #P = transition_matrix_peters(n,k,w,c,p,l)

    beta = compute_beta(n,k,w,p,l,m)

    # success probabiity matrix R
    tmp = identity_matrix(R1, w+1, w+1)
    tmp[2*p, 2*p] = R1(1) - beta

    R = P * tmp
    R = (identity_matrix(R1, w+1, w+1) - R).inverse()
    #print_matrix(R)

    def pi(u: int):
        pi0 = R1(0)
        if u == 2*p:
            pi0 = R1(1) - beta
        else:
            pi0 = R1(1)

        return pi0 * R1(binom(w, u) * binom(n-w, k-u)) / R1(binom(n, k))

    # finally iteration count
    num = R1(0)
    for u in range(w+1):
        tmp0 = R1(0)
        for v in range(w+1):
            tmp0 += R[u, v]

        num += tmp0 * pi(u)

    if num < 1:
        return inf
        
    num = log(num, 2)
    return round(num, 10)


# |%%--%%| <738l9SoKXX|UJ7ge1cCmZ>

print(peters_number_perms(2918, 2918//2, 54, 10, 3, 20))
# 2918, 2918//2, 54, 10, 3, 20 = 33.0270244994

# |%%--%%| <UJ7ge1cCmZ|hTE2mhrmgf>

def marcovchain_number_perms(n: int, k: int, w: int, c: int, p: int, l: int):
    """
    code by Andre/Floyd
    NOT compatible with peters estimation

    :param n: code length
    :param k: code dimension
    :param w: weight
    :param c: number of coordinates to exchange during each gaussian elimination
    :param p: weight of the good state
    :param l: window
    Example:
        p = 3
        n = 2918
        k = n//2
        w = 54
        l = 0
        c = 95
        marcovchain_number_perms(n,k,w,c,p,l)
    :return: number of expected iteration in logarithmic notation
    """
    A = transition_matrix(n,k,w,c,p,l)

    # transition matrix excluding success-state
    Q = A[[i for i in range(w+1) if i!=p],[i for i in range(w+1) if i!=p]]
    
    # fundamental matrix of markov process
    R = (identity_matrix(R1, w, w) - Q).inverse()
    
    # initial state of markov chain
    state = [R1((binom(n-k,w-i)*binom(k,i))/binom(n,w)) for i in range(w+1) if i!=p]

    # number of permutations
    summ = sum(state[i]*sum(R[i,j] for j in range(w)) for i in range(w) if i!=p)
    return log(summ, 2) - log2(n-k)

# |%%--%%| <hTE2mhrmgf|pqjrqjCW5c>

print(marcovchain_number_perms(n,k,w,c,p,l))
#print(fq_iters(n,k,w,p,l))

# |%%--%%| <pqjrqjCW5c|di2GfeoEHP>

# McElice parameters
print(marcovchain_number_perms(3348,2720,64,10,4,0))
print(peters_number_perms(3348,2720,64,10,4,20))
# should be: 112.998742

# |%%--%%| <di2GfeoEHP|nnkwXb4Fdz>

def peters_stern(n: int, k: int, w: int, q: int, p_value=-1, l_value=-1, c_value=0, m_value=0, r_value=0, memory_bound=inf, to_bit_sec=True):
    """
    :param n: code length
    :param k: code dimension
    :param w: weight
    :param c: number of coordinates to exchange during each gaussian elimination
    :param p: weight enumerated in the base list
    :param l: window
    :param m: number of indyk motwani iterations, should be atleas 1
    :param r: number of columns to solve in parallel in the gaussian elimination: 1 = gaus
                is also known as the M4RI parameter
    :param memory_bound: max memory
    :param to_bit_sec: if true the output will be in 2**X bit operations, otherwise in 2**X Fq Operations
    """
    time_min, memory_min, params = inf, inf, []
    solutions = 0
    k1 = k // 2
    cost = log2(q) if to_bit_sec else 1
    hardcoded_upper_value_p = 10 # change this if needed. But I think everything above this is not usable
    hardcoded_upper_value_c = 30 # increase if really needed
    l_search_radius = 30

    # steps of two to increase optimization time
    for c in range(0, min(n, hardcoded_upper_value_c), 2):
        if c_value and c != c_value:
            continue

        # hardcoded to 1 as, this is relly not worth to optimize
        for r in range(1, 2):
            if r_value and r != r_value:
                continue

            if c == 0:
                Tg = (n-k)**2 * (n+k) // 2
            else:
                rq = max(1, 2**(r*log2(q)))
                Tg = (n-1) * ((k-1) * (1-1/rq) + (rq - r)) * ceil(c / r)

            for m in range(1, 3):
                if m_value and m != m_value:
                    continue

                for p in range(0, min(hardcoded_upper_value_p, w//2)):
                    if p_value != -1 and  p != p_value:
                        continue
                    
                    L1 = binom(k1, p) * (q-1)**p
                    L2 = binom(k-k1, p) * (q-1)**p
                    if (L1 > 0) and (log2(L1) > time_min):
                        continue
                    
                    l_exp_val = int(log2(binom(k1, p)) - log2(q-1)*p)
                    l_lower = l_value+0 if l_value!=-1 else max(l_exp_val - l_search_radius, 0)
                    l_highe = l_value+1 if l_value!=-1 else min(l_exp_val + l_search_radius, n-k)
                    for l in range(l_lower, l_highe):
                        if l_value != -1 and l != l_value:
                            continue
                        memory = log2((L1 + L2)*l + _mem_matrix(n, k, 0)) + log2(n)
                        if memory > memory_bound:
                            continue
                        if c == 0 or c == n:
                            Tp = max(0,log2(binom(n,w)) - log2(binom(n-k-l, w-2*p)) - log2(binom(k1,p)**2) - solutions)
                        else:
                            Tp = peters_number_perms(n,k,w,c,p,l,m)

                        build = max(((k1 - p + 1) + (L2+L1)) * l * m, 1)

                        cost_early_exit = max(1,int(max(q/(q-1) * (w - 2*p + 1)*(2*p*(1 + (q - 2)/(q - 1))), 1)))
                        L = L1*L2//q**l
                        ops = (build + L*cost_early_exit) * cost
                        time = log2(Tg + ops) + Tp
                        
                        if time < time_min:
                            time_min = round(time, 10)
                            memory_min = round(memory, 10)
                            params = [p, l, c, m, r, Tp, log2(L1), log2(L)]

    return time_min, memory_min, params

# |%%--%%| <nnkwXb4Fdz|bo0kN4dJ5o>

%time print(peters_stern(256,128,80,256,c_value=20))

# |%%--%%| <bo0kN4dJ5o|NfvEvVV22h>

%time print(peters_stern(256,128,80,256,p_value=0, m_value=1, r_value=1))

# |%%--%%| <NfvEvVV22h|RZ2ubEOlVr>

# taken from the paper: https://eprint.iacr.org/2009/589.pdf
print(peters_stern(961,771,48,31,p_value=4,l_value=20,c_value=10,m_value=1,r_value=1))
# q=31 n=961 k=771 w=48 p=4 l=20 m=1 c=10 r=1:  bit ops  138.419795, bit ops per it 57.013019, log2 #it 81.406776

# |%%--%%| <RZ2ubEOlVr|ERqCQUYIMr>

%time print(peters_stern(961,771,48,31,c_value=10,m_value=1))

# |%%--%%| <ERqCQUYIMr|CJhMsA2CUF>

%time print(peters_stern(961,771,48,31))

# |%%--%%| <CJhMsA2CUF|GtvvymGqk5>

print(peters_stern(3348,2720,64,2,p_value=4//2,l_value=20,c_value=10))
# q=2 n=3348 k=2720 w=64 p=4 l=20 m=1 c=10 r=1:  bit ops  176.924095, bit ops per it 63.925352, log2 #it 112.998742

# |%%--%%| <GtvvymGqk5|dXQaGq2cTx>

print(peters_stern(3348,2720,64,2,p_value=4,c_value=10))
print(peters_stern(3348,2720,64,2,c_value=10))
print(peters_stern(3348,2720,64,2))

# |%%--%%| <dXQaGq2cTx|qLqttdtcHh>
r"""°°°
# Fq Prange
°°°"""
# |%%--%%| <qLqttdtcHh|k69MXcT7tT>

import time

q = 2
m = 3
n = 10
k = 5
w = 3
BF = GF(q)
# F, f = BF.extension(m, repr='int', map=True)
# F, f = BF.extension(m, 'a', map=True)
F.<a> = GF(q^m)

# |%%--%%| <k69MXcT7tT|W4DTwDPxSi>

bases = [F(a**i) for i in range(m)]

def create_random_weight_vector(F, n, w):
    """samples a random weight w vector"""
    v = [0] * n
    for i in range(w):
        while v[i] == 0:
            v[i] = F.random_element()
    P = Permutations(n)
    p = P.random_element()

    for i in range(n):
        tmp = v[i]
        v[i] = v[p[i] - 1] # Permutation is not 0 based
        v[p[i] - 1] = tmp

    return vector(F, v)

def create_instance(F, n, k, w):
    """simply generates a random ISD instance"""
    H = random_matrix(F,n-k,n)
    e = create_random_weight_vector(F, n, w)
    s = H*e
    return H, Matrix(F, n-k, 1, s), e

def f_el(e: F):
    """map from extension field to base field (without repetition of the bases)"""
    ret = [0] * m
    for k, v in e.polynomial().dict().items():
        ret[k] = BF(v)
    return ret
    
def f(e: F):
    """map from extension field to base field"""
    ret = [[]] * m # these are the columns
    for i in range(m):
        el = f_el(bases[i] * e)
        ret[i] = el
    return Matrix(F, m, m, ret).transpose()

def matrix_f(H):
    """ given a parity check matrix H, returns the m*(n-k) \times m*n
    extended matrix in the base field
    """
    nrows = H.nrows()
    ncols = H.ncols()
    ret = zero_matrix(BF, m*nrows, m*ncols)
    for i in range(nrows):
        for j in range(ncols):
            val = f(H[i, j])
            for k1 in range(m):
                for k2 in range(m):
                    ret[i*m + k1, j*m + k2] = val[k1, k2]
    return ret

def bwt(A, col=0, m=1):
    """computes the hamming weight of a column of the input matrix A
    block wise.
    """
    ret = 0
    assert m > 0 and col >= 0
    for row in range(0, A.nrows(), m):
        for j in range(m):
            if A[row + j, col] > 0:
                ret += 1
                break  
    return ret

def wt(A, col=0):
    """computes the hamming weight of a column of the input matrix A
    """
    ret = 0
    assert m > 0 and col >= 0
    for row in range(0, A.nrows()):
        if A[row, col] > 0:
            ret += 1
    return ret

# |%%--%%| <W4DTwDPxSi|n3h4NA2xgg>

def attack(HH, s, n: int, k: int, w: int, q: int, m: int, epsilon=1):
    """
    simple prange:
    NOTE: n,k,w must not be scaled with m. This function is doing it
    NOTE: the parity check matrix must be over the extension field,
            the syndrome also
    """
    print("n:", n, "k:", k, "w:", w, "q:", q, "m:", m)
    print("weight limit:", int(w*m/2) - epsilon, int(w*m/2) + epsilon)
    P = Permutations(m*n)
    p = [i for i in range(m*n)]
    H = HH.augment(s)

    # compute the matrix over the base field
    bH = matrix_f(H)

    ctr = 0
    while true:
        ctr += 1
        np = P.random_element()
        for i in range(m*n):
            p[i], p[np[i]-1] = p[np[i]-1], p[i]
            bH.swap_columns(i, np[i]-1)

        bH.echelonize()
        if bH.rank() < m*(n-k):
            continue
        
        # this comptes the block wise hamming weight
        # for the syndrome column multiplied with the base element `1`
        cw = wt(bH, m*n)

        # check if the weight is blockwise below its limit
        if int(w*m/2) - epsilon <= cw <= int(w*m/2) + epsilon:
            # print some debug information
            #print(bH.column(m*n))
            #print(cw)
            
            # allocate 3 different errors
            e1 = [0] * (m*n) # needed to extract the syndrome
            e2 = [0] * (m*n) # needed to apply the inverse permutation
            e = [F(0)] * (n) # error in the extension field

            # first extract the syndrome and write it into the error
            for i in range((n-k) * m):
                e1[i] = bH[i, m*n]
            #print(e1, "e1")

            
            #  apply the inverse permutation
            for i in range(m*n):
                e2[i] = e1[p[i]]
            #print(e2, "e2")
            #print(p, "p")

                
            # apply the back isomorphishm: translate the error from F_q^{m*n}
            # to F_{q^m}^n. Note that we therefore need to mulitply with the 
            # base elemetns of F_{q^m}^n
            for i in range(n):
                for j in range(m):
                    e[i] += F(e2[i*m + j])* F(bases[j])
            #print(e, "e")
        
            e = Matrix(F, n, 1, e)
            if HH*e == s:
                return e, ctr

set_random_seed(time.time())
Hpriv, s, e = create_instance(F, n, k, w)
eprime, ctr = attack(copy(Hpriv), s, n, k, w, q, m)
print(eprime)
print(e)

print("iterations needed:", ctr, "log2:", math.log(ctr, 2))
print("syndrome (should)")
print(s)
print("syndrome (is)")
print(Hpriv*eprime)

# |%%--%%| <n3h4NA2xgg|lpRckVqivN>
r"""°°°
# FqStern
°°°"""
# |%%--%%| <lpRckVqivN|8ZyX6qpGpA>

class gray:
    """ enumerate the changelist of the gray coce """
    def __init__(self, n: int, p: int):
        self.p = p
        self.n = n
        self.cL = [0, 1]

        self.last_pos = 0
        self.jump = False
        self.ctr = 0
        
    def __iter__(self):
        return self

    def __next__(self):
        self.ctr += 1
        if self.p == 1:
            self.next1()
        elif self.p == 2:
            self.next2()
        if self.ctr == math.comb(self.n, self.p):
            raise StopIteration()
        return self.cL

    def next1(self):
        self.cL[0] += 1
        self.cL[1] += 1
        return

    def next2(self):
        if self.jump:
            self.cL[0] = self.n - 1
            self.cL[1] = self.last_pos + 1
            self.jump = False
            return

        if self.cL[1] == self.n-1:
            self.cL[0] = self.last_pos
            self.cL[1] = self.last_pos + 1
            self.last_pos += 1
            self.jump = True
            return
            
        self.cL[0] = self.cL[1]
        self.cL[1] += 1
        return

def example():
    g = gray()

# |%%--%%| <8ZyX6qpGpA|lP5mCKLVil>

def extract_l_bits(H, n: int, l: int, offset_row: int=0, offset_col: int=0):
    """ currently only valid for binary base field """
    ret = []
    for i in range(n):
        val = 0
        for j in range(l):
            val = val ^^ int(H[j+offset_row, i+offset_col]) << j

        ret.append(val)

    # extract the syndrome
    ws = 0 
    for j in range(l):
        ws = ws ^^ int(H[j + offset_row, n+offset_col]) << j
        
    return ret, ws

# |%%--%%| <lP5mCKLVil|wkHLhYiCYZ>

def fqsten_attack(HH, s, n: int, k: int, w: int, q: int, m: int, l: int=2, p: int=1, epsilon=1):
    """
    simple stern:
    NOTE: n,k,w must not be scaled with m. This function is doing it
    NOTE: the parity check matrix must be over the extension field,
            the syndrome also
    """
    if l > n-k:
        print('l to big')
        return -1
    print("n:", n, "k:", k, "w:", w, "q:", q, "m:", m, "p:", p, "l:", l)
    print("weight limit:", int(w*m/2) - epsilon, int(w*m/2) + epsilon)
    P = Permutations(m*n)
    perm = [i for i in range(m*n)]
    H = HH.augment(s)

    # compute the matrix over the base field
    bH = matrix_f(H)
    ctr = 0
    while true:
        ctr += 1
        np = P.random_element()
        for i in range(m*n):
            perm[i], perm[np[i]-1] = perm[np[i]-1], perm[i]
            bH.swap_columns(i, np[i]-1)

        bH.echelonize()
        if bH.rank() < m*(n-k):
            continue

        lHT, ws = extract_l_bits(bH, m*(k+l), l, m*(n-k-l), m*(n-k-l)) 
        print(lHT)

        # build up left side:
        HM = {}
        val = 0
        for i in range(p):
            val = val ^^ lHT[i]

        i = 0
        for diff in gray((m*k+l)//2, p):
            if val not in HM.keys():
                HM[val] = [i]
            else:
                HM[val].append(i)
                
            i += 1
            for j in range(2):
                val = val ^^ lHT[diff[j]]

        # iterate right side:
        val = ws
        for i in range(p):
            val = val ^^ lHT[i]
        i = 0
        for diff in gray((m*k+l)//2, p):
            for coll_index in HM[val]:
                print(coll_index)
                
            i += 1
            for j in range(2):
                val = val ^^ lHT[diff[j]]
        return 1
    
        # this comptes the block wise hamming weight
        # for the syndrome column multiplied with the base element `1`
        cw = wt(bH, m*n)

        # check if the weight is blockwise below its limit
        if int(w*m/2) - epsilon <= cw <= int(w*m/2) + epsilon:
            # allocate 3 different errors
            e1 = [0] * (m*n) # needed to extract the syndrome
            e2 = [0] * (m*n) # needed to apply the inverse permutation
            e = [F(0)] * (n) # error in the extension field

            # first extract the syndrome and write it into the error
            for i in range((n-k) * m):
                e1[i] = bH[i, m*n]
            
            #  apply the inverse permutation
            for i in range(m*n):
                e2[i] = e1[p[i]]
                
            # apply the back isomorphishm: translate the error from F_q^{m*n}
            # to F_{q^m}^n. Note that we therefore need to mulitply with the 
            # base elemetns of F_{q^m}^n
            for i in range(n):
                for j in range(m):
                    e[i] += F(e2[i*m + j])* F(bases[j])        
            e = Matrix(F, n, 1, e)
            if HH*e == s:
                return e, ctr

set_random_seed(time.time())
Hpriv, s, e = create_instance(F, n, k, w)
eprime, ctr = fqsten_attack(copy(Hpriv), s, n, k, w, q, m)
print(eprime)
print(e)

print("iterations needed:", ctr, "log2:", math.log(ctr, 2))
print("syndrome (should)")
print(s)
print("syndrome (is)")
print(Hpriv*eprime)

# |%%--%%| <wkHLhYiCYZ|1h3NafTvOX>
r"""°°°
### Adaptive Permutations:
°°°"""
# |%%--%%| <1h3NafTvOX|PKUk6sypJw>

n = 9
l = 3
P = Permutations(n)
perm = [i for i in range(n)]

def adaptive_permutation_step(perm: list[int], n: int, l: int):
    """
    apply random permutatations on 
    """
    assert n % l == 0
    nr = n // l
    subP = Permutations(l)
    for i in range(nr):
        np = subP.random_element()
        
        for j in range(l):
            perm[i*l + j], perm[i*l + np[j] - 1] = perm[i*l + np[j] - 1], perm[i*l + j]

    return perm

perm = adaptive_permutation_step(perm, n, l)
print(perm)
perm = adaptive_permutation_step(perm, n, l)
print(perm)

    

# |%%--%%| <PKUk6sypJw|l7gAobcbkg>



# |%%--%%| <l7gAobcbkg|npu70MOh0e>


