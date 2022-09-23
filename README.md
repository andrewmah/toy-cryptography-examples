# toy-cryptography

## Introduction
**Disclaimer:** don't actually use this to encrypt anything important. I just found implementing these toy examples in code to be helpful in increasing my understanding of cryptographic primitives. The parameter settings are not intended to be secure, and these constructions would not be used in practice because they are not the most efficient.

The goal of this project is to show how to build a semantically secure symmetric encryption scheme from the hardness of *any* one-way function. The general roadmap is assume the existence of a one-way function and use that to construct subsequently more complex primitives
- one-way function (OWF) ->
- hardcore predicate (HCP) ->
- pseudorandom generator (PRG) -> 
- pseudorandom function (PRF) ->
- semantically secure encryption

## One-Way Functions (OWF)
One-way functions are the fundamental building block with which nearly all modern cryptography is built. Intuitively a function is "one-way" if it is easy to compute but hard to invert. 

### Definition
More formally, a function $f: \{0,1\}^* \rightarrow \{0,1\}^*$ is **one-way** if the following two conditions hold.
1. **Easy to compute:** There exists a polynomial time algorithm $M_f$ such that on any input $x\in\{0,1\}^*$, $$M_f(x) = f(x)$$
2. **Hard to invert:** For every PPT algorithm $\mathcal{A}$, there exists a negligible function $\varepsilon(n)$ such that
$$\Pr[\mathcal{A}(1^n, f(x)) = x' \text{ st. } f(x') = f(x)] \leq \varepsilon(n)$$
The probability is taken over the uniform choice of $x \in \{0,1\}^n$ and the random coins of $\mathcal{A}$. Additionally, a function is negligible if it is smaller than any polynomial function on large enough inputs. More formally, $\varepsilon(n)$ is **negligible** if $\forall c \in \mathbb{N}$,
$$\exist n_c \in \mathbb{N} \;\;\text{s.t.}\;\; \forall n > n_c, \quad |\varepsilon(n)| < \frac{1}{n^c}$$
 

A couple of other not super important but potentially confusing points:
1. $1^n$ just means the string of $n$ $1$'s. The adversary $\mathcal{A}$ is allowed to run time polynomial in its input; however, if it only receives $f(x)$ as input and if $f(x)$ is exponentially smaller than the input $x$, then the adversary will never be able to output a preimage because it can't even read a preimage. Generally, you should assume any adversary gets input $1^n$ where $n$ is the security parameter, but it isn't always explicitly written.
2. We only require $\mathcal{A}$ to find a preimage of $f(x)$ not $x$ itself. This is to exclude trivial but non useful functions such as the constant function $f(x)=0$. Since the output gives no information about the input, the adversary $\mathcal{A}$ can only guess the original input with exponentially small probability. However, it is easy to find a preimage to the output of the constant function.

### Cryptographic Assumptions
Interestingly no function has ever been definitively **proven** to be one-way, so nearly all cryptographic schemes rely on the assumption that some function is one-way. A few things to consider when choosing your assumptions are
1. How long has this problem been studied? A problem which has been studied for a hundreds of years and yielded no polynomial time inversion algorithm is much more likely to be secure than if a problem was only introduced in the last few years.
2. Is your assumption reasonable independent of your encryption scheme? Often times practioners will first construct a scheme because it has nice properties like efficiency and then afterwards craft a convoluted assumption closely related to their scheme which "proves" the scheme is secure. Generally, this is frowned upon by the theorists, but sometimes these schemes are used in practice anyway if the properties are valuable enough. A more detailed analysis of this can be found [here](https://eprint.iacr.org/2015/907.pdf). 

### Discrete Log Problem
The one-way function I have implemented is group exponentiation where $g$ is a generator of a group.
$$f(x) = g^x$$ 
Inverting group exponentiation is called the *discrete log problem* and is a standard well studied hard problem (although it can be solved in polynomial time by a quantum computer). 

### Parameter Settings
In practice, I have implemented this as
$$f(x) = 2^x \mod p$$
where $p = 2147483783$. $p$ is prime and $2$ generates a group of prime order $1073741891$. 

[TODO]: Explain how these parameters where generated.

[TODO]: Update these paremters. These parameters are way too small to be used in practice. I'm just using them for now so I can use Rust's builtin integer types. Also, I think there can be some choices of parameters that are "bad" besides just being too small, but it's not something I have studied much before.

## Hardcore Predicate (HCP)
Let $f(x)$ be a OWF. Although it is hard to invert $f(x)$, it does not neccesarily hide all information about $x$. For example, the function $g(x_1, x_2) = (f(x_1), x_2)$ is also technically a OWF, but it reveals half the bits of the input! A hardcore predicate is essentially all the hardness of a OWF distilled into a single bit which is hard to predict based on the output of a OWF.

### Definition
A function $b: \{0,1\}^* \rightarrow \{0,1\}$ is a **hardcore predicate** of a OWF $f$ if 
$$\Pr[\mathcal{A}(f(x)) = b(x)] \leq \frac{1}{2} + \epsilon(n)$$
where the probability is over the choice of $x \leftarrow \{0,1\}^n$ and the random coins of the adversary $\mathcal{A}$.

[TODO add easy to compute criteria]

### Goldreich Levin Theorem
While it is unknown if a hardcore predicate exists for every OWF, it is true that every OWF can be transformed into another OWF which has a specific hardcore predicate. [TOOD double check this]. Let $f$ be a OWF. Define 
$$g(x, r) = (f(x), r)$$
where to the length of $r$ equals the length of $x$. Then $g$ is also a OWF, and $b$ is a hardcore predicate of $g$.
$$b(x, r) = \langle x, r\rangle \mod 2= \bigoplus_j x_j \cdot r_j$$
This is known as the Goldreich Levin Theorem.

To give some intuition we can show how an adversary who can find the hardcore bit with probability 1 can invert the underlying one way function with probability 1. 

Let the length of $x$ and $r$ be $n$. Let $e_i$ denote the length $n$ vector which is zeroes in all positions except for $i$ where it contains a 1. $\mathcal{A}$ is the PPT adversary which finds the hardcore predicate of $g$ with probability 1. We will use $\mathcal{A}$ to construct another PPT adversary $\mathcal{A}'$ which can invert $y=f(x)$ with probability 1.

To find the $i$th bit of $x$, $\mathcal{A}'$ calls $\mathcal{A}$ on input $(y, e_i)$. The hardcore predicate equals $\langle x, e_i\rangle = x_i$, so $\mathcal{A}$ outputs the $i$th bit of $x$. $\mathcal{A}'$ can then repeat this same procedure for each bit of $x$ until it outputs the entire string. Therefore, if $f$ is a OWF, then a PPT adversary cannot find the hardcore predicate with probability 1.

The full proof is very long but one of the key ideas is that
$$b(x, r) \oplus b(x, r \oplus e_i) = x_i$$
and this allows the adversary to get repeated guess at $x_i$ even if it gets $b(x, e_i)$ wrong.

[TODO add full proof]

## Pseudo Random Generator (PRG)
### Definition

### Blum Micali Construction 

## Pseduo Random Function (PRF)
### Definition
$F$ is a **pseudorandom function** if for all PPT distinguishers $D$, there exists a negligible function $\epsilon(n)$ such that
$$\left| \Pr\left[D^{F_k(\cdot)}(1^n) = 1\right] - \Pr\left[D^{f_n(\cdot)}(1^n) = 1\right]\right| \leq \epsilon(n)$$
where $k\leftarrow \{0,1\}^n$ is chosen uniformly at random and $f_n$ is chosen uniformly at random from the set of function mapping $n$-bit strings to $n$-bit strings.
### Goldreich Goldwasser Micali construction

## Sources
Many thanks to these people for being great teachers and inspiring my interest in cryptography.
1. MIT *Cryptography and Cryptanalysis (6.875)* class notes taught by Yael Kalai & Noah Stevens-Davidowitz (Fall 2019)
2. *Introduction to Modern Cryptography* by Jonathan Katz & Yehuda Lindell
