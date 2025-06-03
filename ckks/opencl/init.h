#pragma once
#define CL_TARGET_OPENCL_VERSION 200


#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef MAC
#include <OpenCL/cl.h>
#else
#include <CL/cl.h>
#endif

typedef unsigned char u8;
typedef unsigned int  u32;
typedef unsigned long u64;
typedef unsigned long long u128;

cl_device_id create_device();
cl_program build_program(cl_context ctx, cl_device_id dev, const char* filename);
