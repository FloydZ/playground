#include <immintrin.h>

void expand_key(__m128i* keys, const unsigned char *key); 
void encrypt(const __m128i *key, __m128i *in); 
void decrypt(const __m128i *key, __m128i *in); 
