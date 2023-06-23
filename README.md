> 前言：这是我用来熟悉项目做的一个小调研，先放在README里面

# 背景

CUDA 作为目前最为流行的 GPU 通用计算框架已经被应用于众多的开源应用中，涵盖从机器学习、科学计算到图像处理的各个领域。

然而在发行版打包 CUDA 应用的过程中也遇到了诸多的阻碍，除了其闭源协议导致的难以重分发之外，CUDA 应用的测试也往往因为需要对真实硬件的访问而被忽视，或是只能由维护者手动测试。

在开发多机并行 CUDA 应用的过程中也往往需要依赖云服务商提供的硬件以测试其正确性。

本项目预期实现一个用户态 CUDA 模拟器，在 CPU 上运行 CUDA kernel，并模拟其他常用的 CUDA 运行时 API，以期实现在无需真实硬件的情况下对 CUDA 应用进行测试与开发。

> 可参考：
>
> https://github.com/vosen/ZLUDA/tree/master/
>
> https://github.com/gtcasl/gpuocelot

# 类似GEM5，那是什么？

$GEM5 = DES + ISA/ABI VM$

## DES：离散事件模拟/系统 Discrete Event Simulation/System

会有一个事件队列，还有一个调度器（典型应用：GUI）

- 事件的发生在时间上是离散的
- 系统在事件发生之间没有状态变化
- 不同事件可能以任意的顺序出现

## ISA/ABI层次的虚拟机

![avatar](https://cdnjson.com/images/2023/06/23/image.png)

- ISA层：VMware，VirtualBox
- ABI层：Rosetta转译层
- 高级：JVM(Java)，V8(JavaScript)，etc.

# CUDA编程

CUDA是一种编程模型和平台，允许开发人员使用C/C++和其他语言来编写并行程序，以在GPU上运行。而本项目旨在实现一个用户态CUDA模拟器，使得CUDA应用程序可以在CPU上运行，而无需真实的GPU硬件，以便于测试和开发。

> 可参考：
>
> https://face2ai.com/program-blog/#GPU%E7%BC%96%E7%A8%8B%EF%BC%88CUDA%EF%BC%89
>
> 上文源码：https://github.com/Tony-Tan/CUDA_Freshman
>
> https://zhuanlan.zhihu.com/p/34587739
>
> https://zhuanlan.zhihu.com/p/53773183
>
> https://zhuanlan.zhihu.com/p/97044592
>
> https://blog.csdn.net/sunmc1204953974/article/details/51000970

# GPGPU-Sim

与GPGPU-Sim相关的是，GPGPU-Sim是一个高性能的GPU模拟器，可以用于模拟各种GPU硬件和软件配置。但是，GPGPU-Sim是基于硬件的模拟器，而本项目是一个基于软件的模拟器，它在CPU上模拟CUDA运行时和kernel的执行。

GPGPU-Sim是一个基于硬件的GPU模拟器，需要GPU硬件来运行。它可以模拟各种不同的GPU架构和配置，以便开发人员可以在不同的硬件上测试和优化他们的GPU应用程序。但是，GPGPU-Sim并不是一个通用的软件模拟器，它需要特定的硬件和软件环境才能运行。

> 可参考：
>
> http://gpgpu-sim.org/manual/index.php/Main_Page#Introduction
>
> https://zhuanlan.zhihu.com/p/369673760

# NVIDIA Nsight Systems CUDA

与NVIDIA Nsight Systems CUDA跟踪相关的是，Nsight是一个功能强大的CUDA调试和性能分析工具，它可以帮助开发人员识别CUDA应用程序中的性能瓶颈和错误。但是，本项目是一个模拟器，旨在在无需真实GPU硬件的情况下测试和开发CUDA应用程序，而不是调试和性能分析。

NVIDIA Nsight Systems CUDA跟踪是一个CUDA应用程序的性能分析工具，可以帮助开发人员识别性能瓶颈和优化机会。它提供了非常详细的性能分析数据，例如每个CUDA核函数的运行时间、内存带宽和内存使用情况等等。因此，Nsight Systems可以被理解为一个性能分析工具，而不是一个测试工具。

Nsight Systems可以分析自己开发的模拟器的性能，但这需要在模拟器中嵌入Nsight Systems的性能分析代码。此外，由于模拟器是在CPU上运行的，它的性能特征与GPU上运行的CUDA应用程序有很大的不同，因此需要谨慎地分析和解释性能数据。可能还需要开发自己的基准测试套件来评估模拟器的性能，并将其与真实GPU硬件进行比较。

> 可参考：https://www.cnblogs.com/wujianming-110117/p/13943442.html

# CUDA项目源码

> https://github.com/bytedance/lightseq
>
> https://github.com/NVIDIA/DeepLearningExamples/tree/master/FasterTransformer
>
> https://github.com/Tencent/TurboTransformers
>
> https://github.com/microsoft/DeepSpeed

---

未完待续...