# toy-cryptography

## Introduction
**Disclaimer:** don't actually use this to encrypt anything important. I just found implementing these toy examples in code to be helpful in increasing my understanding of cryptographic primitives. The parameter settings are not intended to be secure.

## One-Way Functions (OWF)
### Definition
One-way functions are the fundamental building block with which nearly all modern cryptography is built. Intuitively a function is "one-way" if it is easy to compute but hard to invert. 

More formally, a function $f: \{0,1\}^* \rightarrow \{0,1\}^*$ is **one-way** if the following two conditions hold.
1. **Easy to compute:** There exists a polynomial time algorithm $M_f$ such that on any input $x\in\{0,1\}^*$, $$M_f(x) = f(x)$$
2. **Hard to invert:** For every PPT algorithm $\mathcal{A}$, there exists a negligible function $\varepsilon(n)$ such that
$$\Pr[\mathcal{A}(1^n, f(x)) = x' \text{ st. } f(x') = f(x)] \leq \varepsilon(n)$$
The probability is taken over the uniform choice of $x \in \{0,1\}^n$ and the random coins of $\mathcal{A}$. A couple of unimportant but potnetially confusing points:
1. $1^n$ just means the string of $n$ $1$'s. The adversary $\mathcal{A}$ is allowed to run time polynomial in its input; however, if it only recieves $f(x)$ as input and if $f(x)$ is exponentially smaller than the input $x$, then the adversary will never be able to output a preimage because it can't even read a preimage. Generally, you should assume any adversary gets input $1^n$ where $n$ is the security parameter, but it isn't always explicitly written.
2. We only require $\mathcal{A}$ to find a preimage of $f(x)$ not $x$ itself. This is to exclude trivial but non useful functions such as the constant function $f(x)=0$. Since the output gives no information about the input, the adversary $\mathcal{A}$ can only guess the original input with exponentially small probability. However, it is easy to find a preimage to the output of the constant function

### Cryptographic Assumptions
Interestingly no function has ever been definitively **proven** to be one-way, so nearly all cryptographic schemes rely on the assumption that some function is one-way. A few things to consider when choosing your assumptions are
1. How long has this problem been studied? A problem which has been studied for a hundreds of years and yielded no polynomial time inversion algorithm is much more likely to be secure than if a problem was only introduced in the last few years.
2. Is your assumption reasonable independent of your encryption scheme. Often times practioners will first construct a scheme because it has nice properties like speed and then afterwards craft a convoluted assumption closely related to their scheme which "proves" the scheme is secure. Generally, this is frowned upon by the theorists, but sometimes these schemes are used in practice anyway if the properties are valuable enough. A more detailed analysis of this can be found [here](https://eprint.iacr.org/2015/907.pdf) 

### Discrete Log Problem
The one-way function I have implemented is group exponentiation where $g$ is a generator of a group.
$$f(x) = g^x$$ 
Inverting group exponentiation is called the *discrete log problem* and is a standard well studied problem. In practice, I have implemented this using a group of prime order $2147483647$.
$$f(x) = g^x \mod p$$
* $g = 1313897859$
* $p = 98784247762$

[TODO]: These parameters are way too small to be used in practice. I'm just using them for now so I can use Rust's builtin integer types. Also, I think there can be some choices of parameters that are "bad" besides just being too small, but it's not something I have studied much before.

## Hardcore Predicate (HCP)
Let $f(x)$ be a OWF. Although it is hard to invert $f(x)$, it does not neccesarily hide all information about $x$. For example, the function $g(x_1, x_2) = (f(x_1), x_2)$ is also technically a OWF, but it reveals half the bits of the input!

### Definition
[TODO intuition]
A function $b: \{0,1\}^* \rightarrow \{0,1\}$ is a **hardcore predicate** of a OWF $f$ if 
$$\Pr[\mathcal{A}(f(x)) = b(x)] \leq \frac{1}{2} + \epsilon(n)$$
where the probability is over the choice of $x \leftarrow \{0,1\}^n$ and the random coins of the adversary $\mathcal{A}$.

## Pseudo Random Generator (PRG)
## Pseduo Random Function (PRF)

## Sources
Many thanks to these people for being great teachers and inspiring my interest in cryptography.
1. MIT *Cryptography and Cryptanalysis (6.875)* class notes taught by Yael Kalai & Noah Stevens-Davidowitz (Fall 2019)
2. *Introduction to Modern Cryptography* by Jonathan Katz & Yehuda Lindell
