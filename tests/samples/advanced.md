---
title: "The Impact of Stochastic Quantization on Deep Neural Network Inference Latency in Edge Computing Environments"
author:
  - name: "Dr. Elena V. Thorne"
    affiliation: "Institute for Advanced Computational Systems"
    email: "e.thorne@iacs.edu"
  - name: "Marcus J. Sterling"
    affiliation: "Department of Robotics, University of New Haven"
abstract: |
  This paper investigates the trade-offs between precision and performance in edge-based deep learning. As neural networks grow in complexity, deploying them on resource-constrained devices remains a significant challenge. We propose a Stochastic Quantization (SQ) framework that reduces model size by up to 75% with less than a 1% drop in Top-1 accuracy. This study details the mathematical foundations of SQ, provides a comparative analysis against traditional 8-bit integer quantization, and evaluates real-world latency across three heterogeneous hardware architectures.
link-citations: true
---

# 1. Introduction

The proliferation of Internet of Things (IoT) devices has led to an unprecedented demand for localized intelligence. However, modern **Large Language Models (LLMs)** and **Convolutional Neural Networks (CNNs)** are often too computationally expensive for edge deployment.

## 1.1 The Problem of Model Bloat
Standard deep learning models typically utilize 32-bit floating-point precision ($FP32$). While accurate, $FP32$ arithmetic is power-intensive. Previous works have suggested:
*   **Pruning**: Removing redundant neurons.
*   **Knowledge Distillation**: Training a smaller "student" model.
*   **Quantization**: Reducing the bit-depth of weights.

> "The bottleneck of edge AI is not the algorithm, but the movement of data between memory and processing units." — *Journal of Hardware Acceleration (2022)*

# 2. Methodology

We implemented a Stochastic Quantization (SQ) algorithm. Unlike deterministic rounding, SQ treats weights as probability distributions.

## 2.1 Mathematical Framework
Let $W$ be a high-precision weight and $Q(W)$ be its quantized counterpart. The probability of rounding to the upper bound $b_{u}$ is defined as:

$$
P(Q(W) = b_{u}) = \frac{W - b_{l}}{b_{u} - b_{l}}
$$

Where:
- $b_{l}$ is the lower quantization bound.
- $b_{u}$ is the upper quantization bound.
- $W \in [b_{l}, b_{u}]$.

The expected value of the quantized weight is shown in Equation :

$$
E[Q(W)] = b_{u} \cdot P(Q(W) = b_{u}) + b_{l} \cdot (1 - P(Q(W) = b_{u})) = W
$$

## 2.2 Implementation in Python
The core logic for our quantization layer was implemented using PyTorch. Below is a simplified snippet of the stochastic rounding kernel:

```python
import torch

def stochastic_quantize(tensor, bits=8):
    """
    Perform stochastic quantization on a given tensor.
    """
    q_min, q_max = 0, (2**bits) - 1
    scale = (tensor.max() - tensor.min()) / q_max
    
    # Normalize and add noise for stochasticity
    normalized = (tensor - tensor.min()) / scale
    noise = torch.rand_like(normalized)
    quantized = torch.floor(normalized + noise)
    
    return torch.clamp(quantized, q_min, q_max) * scale + tensor.min()
```

# 3. Experimental Results

We tested our framework on three distinct platforms. The results indicate that while SQ increases training time slightly, the inference speedup is substantial.

### Table 1: Performance Comparison Across Platforms
| Platform | Precision | Accuracy (%) | Latency (ms) | Energy (mJ) |
| :--- | :---: | :---: | :---: | :---: |
| NVIDIA Jetson Orin | $FP32$ | 94.2 | 12.4 | 450 |
| Raspberry Pi 4 | $INT8$ | 91.8 | 45.2 | 120 |
| **Our Framework (SQ)** | **4-bit** | **93.5** | **8.1** | **85** |

### 3.1 Latency Observations
1.  **Memory Bandwidth**: The reduction from 32-bit to 4-bit allowed the entire model to fit into L1 cache.
2.  **Vectorization**: Modern ARM processors utilize NEON instructions to process multiple 4-bit values in a single cycle[^1].


# 4. Discussion

Our findings suggest that SQ is particularly effective for non-linear activation functions like Swish or GELU, which typically suffer under deterministic $INT8$ quantization. 

### 4.1 Comparison with State-of-the-Art
As noted by Thorne, the primary risk of low-bit quantization is gradient instability. We mitigated this using a *Warm-up Quantization* schedule:
- [x] Phase 1: Full-precision training (10 epochs)
- [x] Phase 2: Weight-only SQ (20 epochs)
- [x] Phase 3: Activation & Weight SQ (50 epochs)
- [ ] Phase 4: Hardware-specific optimization (Future Work)

# 5. Conclusion

This research demonstrates that Stochastic Quantization provides a robust middle ground for deploying high-performance AI on the edge. By leveraging the probabilistic nature of weight rounding, we maintain high accuracy while drastically reducing the hardware footprint.

[^1]: NEON is the SIMD (Single Instruction Multiple Data) architecture extension for the ARM Cortex-A series.