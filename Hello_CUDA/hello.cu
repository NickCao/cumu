// add.cpp
#include <iostream>
#include <math.h>
#include <sys/time.h>

// function to add the elements of two arrays
__global__
void add(int n, float *x, float *y)
{
  for (int i = 0; i < n; i++)
      y[i] = x[i] + y[i];
}

int main(void)
{
  int N = 1<<25; // 30M elements

  float *x = new float[N];
  float *y = new float[N];
  cudaMallocManaged(&x, N*sizeof(float));
  cudaMallocManaged(&y, N*sizeof(float));
  // initialize x and y arrays on the host
  for (int i = 0; i < N; i++) {
    x[i] = 1.0f;
    y[i] = 2.0f;
  }

  struct timeval t1,t2;
  double timeuse;
  gettimeofday(&t1,NULL);
  // Run kernel on 30M elements on the CPU
  add<<<1, 1>>>(N, x, y);
  gettimeofday(&t2,NULL);
  timeuse = (t2.tv_sec - t1.tv_sec) + (double)(t2.tv_usec - t1.tv_usec)/1000.0;

  std::cout << "add(int, float*, float*) time: " << timeuse << "ms" << std::endl;
  // Check for errors (all values should be 3.0f)
  float maxError = 0.0f;
  for (int i = 0; i < N; i++)
    maxError = fmax(maxError, fabs(y[i]-3.0f));
  std::cout << "Max error: " << maxError << std::endl;

  // Free memory
  delete [] x;
  delete [] y;

  cudaFree(x);
  cudaFree(y);
  
  return 0;
}