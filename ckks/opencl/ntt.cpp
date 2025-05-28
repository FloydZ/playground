#define PROGRAM_FILE "test.cl"
#define KERNEL_FUNC "ntt"
#define N 128

#include "init.h"

typedef u64 T;

int main() {
    cl_device_id device;
    cl_context context;
    cl_program program;
    cl_kernel kernel;
    cl_command_queue queue;
    cl_int err; 
    size_t local_size= N, global_size = N;
    //cl_int num_groups = global_size/local_size;

    T a[N]; T w[N];
    T prime = 17; T n = N;
    cl_mem d_a, d_w;

    /* Initialize data */
    for(uint32_t i=0; i<N; i++) {
       a[i] = i;
       w[i] = i*i;
    }
    
    device = create_device();
    context = clCreateContext(NULL, 1, &device, NULL, NULL, &err);
    if(err < 0) {
       perror("Couldn't create a context");
       exit(1);   
    }
    
    /* Build program */
    program = build_program(context, device, PROGRAM_FILE);
    
    d_a = clCreateBuffer(context, CL_MEM_READ_WRITE | CL_MEM_COPY_HOST_PTR, N * sizeof(T), a, &err);
    d_w = clCreateBuffer(context, CL_MEM_READ_ONLY  | CL_MEM_COPY_HOST_PTR, N * sizeof(T), w, &err);
    if(err < 0) {
       perror("Couldn't create a buffer");
       exit(1);   
    };
    
    queue = clCreateCommandQueue(context, device, 0, &err);
    if(err < 0) {
       perror("Couldn't create a command queue");
       exit(1);   
    };
    
    /* Create a kernel */
    kernel = clCreateKernel(program, KERNEL_FUNC, &err);
    if(err < 0) {
       perror("Couldn't create a kernel");
       exit(1);
    };
    
    /* Create kernel arguments */
    err  = clSetKernelArg(kernel, 0, sizeof(cl_mem), &d_a);
    err |= clSetKernelArg(kernel, 1, sizeof(cl_mem), &d_w);
    err |= clSetKernelArg(kernel, 2, sizeof(u64), &prime);
    err |= clSetKernelArg(kernel, 3, sizeof(u64), &n);
    if(err < 0) {
       perror("Couldn't create a kernel argument");
       exit(1);
    }
    
    err = clEnqueueNDRangeKernel(queue, kernel, 1, NULL, &global_size, 
          &local_size, 0, NULL, NULL); 
    if(err < 0) {
       perror("Couldn't enqueue the kernel");
       exit(1);
    }
    
    /* Read the kernel's output    */
    memset(a, 0, sizeof(T)*N);
    err = clEnqueueReadBuffer(queue, d_a, CL_TRUE, 0, sizeof(T)*N, a, 0, NULL, NULL);
    if(err < 0) {
       perror("Couldn't read the buffer");
       exit(1);
    }
    
    /* Check result */
    for (size_t i = 0; i < n; i++) {
        printf("%ld ", a[i]);
    }
    printf("\n");


    
    /* Deallocate resources */
    clReleaseKernel(kernel);
    clReleaseMemObject(d_a);
    clReleaseMemObject(d_w);
    clReleaseCommandQueue(queue);
    clReleaseProgram(program);
    clReleaseContext(context);
    return 0;
}
