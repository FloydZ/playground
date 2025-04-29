
132 bit:
            rug    astro-float
add          84 ms         136 ms
sub          85 ms         135 ms
mul          64 ms         124 ms
div         172 ms         387 ms

256 bit
add          81 ms         139 ms
sub          82 ms         139 ms
mul         105 ms         138 ms
div         195 ms         447 ms

512 bit
add          88 ms         152 ms
sub          90 ms         154 ms
mul         172 ms         222 ms
div         303 ms         759 ms

C++ CLN:
256 bit, multiplication
real time:    0.508 s, run time:    0.507 s

operation   mul, precision :    32x   32 to    32 bits ...   24198431 ops
operation   mul, precision :    64x   64 to    64 bits ...   23653874 ops
operation   mul, precision :   128x  128 to   128 bits ...   16898898 ops 15.6
operation   mul, precision :   256x  256 to   256 bits ...   11018410 ops  9.8
operation   mul, precision :   512x  512 to   512 bits ...    6360893 ops  5.8
operation   mul, precision :  1024x 1024 to  1024 bits ...    3110149 ops
operation   mul, precision :  2048x 2048 to  2048 bits ...    1171639 ops
operation   mul, precision :  4096x 4096 to  4096 bits ...     368739 ops
operation   mul, precision :  8192x 8192 to  8192 bits ...     119980 ops
operation   mul, precision : 16384x16384 to 16384 bits ...      41469 ops
operation   mul, precision :    50x   50 to    50 bits ...   25866387 ops
operation   mul, precision :   100x  100 to   100 bits ...   23284593 ops
operation   mul, precision :   200x  200 to   200 bits ...   11063702 ops
operation   mul, precision :   350x  350 to   350 bits ...    8327411 ops
operation   mul, precision :   700x  700 to   700 bits ...    4216915 ops
operation   mul, precision :  1500x 1500 to  1500 bits ...    2014326 ops
operation   mul, precision :  3000x 3000 to  3000 bits ...     618873 ops
operation   mul, precision :  6000x 6000 to  6000 bits ...     207025 ops
operation   mul, precision : 10000x10000 to 10000 bits ...      88266 ops
operation   mul, precision :  1500x 3000 to  3000 bits ...     666151 ops
operation   mul, precision :  3000x 6000 to  6000 bits ...     239776 ops
operation   mul, precision :  5000x10000 to 10000 bits ...     105897 ops
operation   add, precision :    32x   32 to    32 bits ...   29188315 ops
operation   add, precision :    64x   64 to    64 bits ...   28225159 ops
operation   add, precision :   128x  128 to   128 bits ...   26354322 ops
operation   add, precision :   256x  256 to   256 bits ...   18762995 ops 
operation   add, precision :   512x  512 to   512 bits ...   15985342 ops 11.4
operation   add, precision :  1024x 1024 to  1024 bits ...   12010306 ops
operation   add, precision :  2048x 2048 to  2048 bits ...    7788981 ops
operation   add, precision :  4096x 4096 to  4096 bits ...    4848154 ops
operation   add, precision :  8192x 8192 to  8192 bits ...    2612761 ops
operation   add, precision : 16384x16384 to 16384 bits ...    1308920 ops
operation   add, precision :    50x   50 to    50 bits ...   28989828 ops
operation   add, precision :   100x  100 to   100 bits ...   25944896 ops
operation   add, precision :   200x  200 to   200 bits ...   18771373 ops
operation   add, precision :   350x  350 to   350 bits ...   17066504 ops
operation   add, precision :   700x  700 to   700 bits ...   14125585 ops
operation   add, precision :  1500x 1500 to  1500 bits ...    9634041 ops
operation   add, precision :  3000x 3000 to  3000 bits ...    6070853 ops
operation   add, precision :  6000x 6000 to  6000 bits ...    3502673 ops
operation   add, precision : 10000x10000 to 10000 bits ...    2129541 ops
operation   add, precision :  1500x 3000 to  3000 bits ...    6211026 ops
operation   add, precision :  3000x 6000 to  6000 bits ...    3784304 ops
operation   add, precision :  5000x10000 to 10000 bits ...    2320331 ops
operation   sub, precision :    32x   32 to    32 bits ...   27187790 ops
operation   sub, precision :    64x   64 to    64 bits ...   28138320 ops
operation   sub, precision :   128x  128 to   128 bits ...   25377451 ops
operation   sub, precision :   256x  256 to   256 bits ...   17432591 ops
operation   sub, precision :   512x  512 to   512 bits ...   15416483 ops
operation   sub, precision :  1024x 1024 to  1024 bits ...   12308252 ops
operation   sub, precision :  2048x 2048 to  2048 bits ...    8501154 ops
operation   sub, precision :  4096x 4096 to  4096 bits ...    5114464 ops
operation   sub, precision :  8192x 8192 to  8192 bits ...    2598884 ops
operation   sub, precision : 16384x16384 to 16384 bits ...    1255075 ops
operation   sub, precision :    50x   50 to    50 bits ...   26759908 ops
operation   sub, precision :   100x  100 to   100 bits ...   25549253 ops
operation   sub, precision :   200x  200 to   200 bits ...   17365706 ops
operation   sub, precision :   350x  350 to   350 bits ...   16067981 ops
operation   sub, precision :   700x  700 to   700 bits ...   13848732 ops
operation   sub, precision :  1500x 1500 to  1500 bits ...   10098745 ops
operation   sub, precision :  3000x 3000 to  3000 bits ...    6415885 ops
operation   sub, precision :  6000x 6000 to  6000 bits ...    3610364 ops
operation   sub, precision : 10000x10000 to 10000 bits ...    2089156 ops
operation   sub, precision :  1500x 3000 to  3000 bits ...    5926045 ops
operation   sub, precision :  3000x 6000 to  6000 bits ...    3654574 ops
operation   sub, precision :  5000x10000 to 10000 bits ...    2123325 ops
operation   div, precision :    32x   32 to    32 bits ...   25671529 ops
operation   div, precision :    64x   64 to    64 bits ...   19220736 ops
operation   div, precision :   128x  128 to   128 bits ...    8813949 ops
operation   div, precision :   256x  256 to   256 bits ...    5793870 ops 5.13
operation   div, precision :   512x  512 to   512 bits ...    3556652 ops
operation   div, precision :  1024x 1024 to  1024 bits ...    1790865 ops
operation   div, precision :  2048x 2048 to  2048 bits ...     663414 ops
operation   div, precision :  4096x 4096 to  4096 bits ...     221350 ops
operation   div, precision :  8192x 8192 to  8192 bits ...      70431 ops
operation   div, precision : 16384x16384 to 16384 bits ...      23443 ops
operation   div, precision :    50x   50 to    50 bits ...   25380974 ops
operation   div, precision :   100x  100 to   100 bits ...   15097610 ops
operation   div, precision :   200x  200 to   200 bits ...    6159495 ops
operation   div, precision :   350x  350 to   350 bits ...    4763469 ops
operation   div, precision :   700x  700 to   700 bits ...    2696303 ops
operation   div, precision :  1500x 1500 to  1500 bits ...    1026422 ops
operation   div, precision :  3000x 3000 to  3000 bits ...     367270 ops
operation   div, precision :  6000x 6000 to  6000 bits ...     116595 ops
operation   div, precision : 10000x10000 to 10000 bits ...      52301 ops
operation   div, precision :  1500x 3000 to  3000 bits ...     368720 ops
operation   div, precision :  3000x 6000 to  6000 bits ...     117106 ops
