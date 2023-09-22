<<<<<<< HEAD
#include <iostream>
#include <cuda_runtime_api.h>

#define N 10000000

__global__ void vector_add(float *out, float *a, float *b, int n) {
    for(int i = 0; i < n; i++){
        out[i] = a[i] + b[i];
    }
}

int main(){
    float *a, *b, *out;

    // Allocate memory
    a   = (float*)malloc(sizeof(float) * N);
    b   = (float*)malloc(sizeof(float) * N);
    out = (float*)malloc(sizeof(float) * N);

    // Initialize array
    for(int i = 0; i < N; i++){
        a[i] = 1.0f; b[i] = 2.0f;
    }

    vector_add<<<1,1>>>(out, a, b, N);
    cudaError_t err = cudaGetLastError();
    std::cout << cudaGetErrorString(err) << std::endl;
}
=======
#include <iostream>
#include <cuda_runtime_api.h>

#define N 10000000

__global__ void vector_add(float *out, float *a, float *b, int n) {
    for(int i = 0; i < n; i++){
        out[i] = a[i] + b[i];
    }
}

int main(){
    float *a, *b, *out;

    // Allocate memory on the host
    a   = (float*)malloc(sizeof(float) * N);
    b   = (float*)malloc(sizeof(float) * N);
    out = (float*)malloc(sizeof(float) * N);

    // Initialize arrays
    for(int i = 0; i < N; i++){
        a[i] = 1.0f; b[i] = 2.0f;
    }

    vector_add<<<1,1>>>(out, a, b, N);
    cudaError_t err = cudaGetLastError();


    std::cout << "=========================================================" << std::endl;

    std::cout << ">>> Print Test >>> " << "HelloWorld!" << std::endl;
    std::cout << ">>> cudaGetLastError Test >>> " << cudaGetLastError() << std::endl;
    std::cout << ">>> cudaGetErrorString Test >>> " << cudaGetErrorString(err) << std::endl;

    std::cout << "=========================================================" << std::endl;
}
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
