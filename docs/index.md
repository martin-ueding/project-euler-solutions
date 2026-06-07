# Martin's Project Euler Solutions

There is [Project Euler](https://projecteuler.net/), a growing collection of math problems that are solved by programming. These have been around for more than twenty years and seem quite challenging.

This repository contains my solutions and some explanations for them. You can find the code in the [GitHub repository](https://github.com/martin-ueding/project-euler-solutions), the explanations right here. There are [more explanations on my blog](https://martin-ueding.de/pages/project-euler/).

This place is mostly for me to think through hard math problems. Writing down the core insights is helpful for me. Reading other people's notes on their solutions sometimes helps me to get unstuck. So I provide my solutions to help people that are stuck. There is the real danger of getting the fun of discovery spoiled.

## History

I have worked on these problems on and off over many years. A long time ago I implemented implemented some of these in one programming language. Then I started to use these problems to explore implementations in other programming languages. This way, I might have gotten lost in breadth instead of continuing with the more advanced problems. Eventually I discarded the code, I cannot find it any more.

In mid-2023, I came back to these problems and started to implement them anew in Python. I chose that language over C++ because it wouldn't get in my way and has the great `itertools` library that I could use for many combinatorial problems.

Computational performance isn't a concern here. The problems are conceived such that they take less than a minute on a computer from 2001. Given that it is 25 later, the factor 100 that Python is slower is easily alleviated with a modern CPU.

I would work on the problems in a mostly linear fashion and document my progress on my blog. The blog posts were a mix of my understanding of the problem, the abstract ideas and approaches that I've taken, the concrete Python implementation. Some of the posts had some detailed math in them, others were mostly showing my uninspired brute-force approach.

When problems built on previous ones, I would just include the functions from one problem in the next one. This made for an interesting looking dependency graph, though it didn't provide me with a good library. Also I had to search the old problems for the implementations, not a very clever way to structure a software project. At the time, I didn't think of it like that.

## Reboot in Mid 2026 with Rust

In mid-2026, I somehow returned to these problems. My programming work has shifted away from the full stack of C++, CUDA, and Python towards only Python. I wanted to do some more hands-on native language programming, so I thought about continuing here with C++. The mere act of setting header and source files, a CMake project and wondering how to include Google Benchmark into this project reminded me why I hated C++ project setup.

Coworkers and many others have said nice things about Rust. And it had been on my list of things that I wanted to learn eventually. So, I thought, why not continue here with Rust?

With Python, I don't have to think about the language, I can completely focus on the math. With Rust, I am learning on two fronts, perhaps that isn't too clever. At university I have seen people trying to write a data analysis framework in an unknown domain, with an unknown programming language for a novel application. That didn't end too well.

The other problem is that I have some 90 solutions in Python so far. There are some functions that I have reused, like prime finding or digit manipulation. When I continue in a different language, I don't have these available to me and have to reimplement them. This would slow down the progress that I could make with sticking to Python.

And the fragmentation is something that doesn't sit well with my love for structure and closure. Yet doing more of the same gets boring. So I've just started in Rust and started to like Rust. And now here we are, with some problems solved in Python, others in Rust. Some documented on my blog, some here.

Using AI, I could easily port the Python code to Rust, but I wouldn't learn anything. Porting the blog posts here would be an idea, but I am not sure whether they would fit into the concept. This option remains, until then I try to keep it fragmented to represent the creation process.