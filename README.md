# Rust_Primer_kmeans

Buld a simple kmeans algorithm in Rust.

The main issue in kmeans is to find the objects closest to the center of a cluster,
recenter the cluster to the mean of all entries of that cluster and then rinse and repeat.

Like 100 times.

This here will be implemented like the kmeans algorithm implemented in [our R course](https://github.com/shambam/R_programming_1/blob/main/13-Kmeans.Rmd).

In short we will learn here how to read data into Rust starting from a tab separated text file. Next there will be the kmeans rust library. And of cause a script calling that library.

And we will have some tests.

Hope we all have a lot of fun with this project.

# First steps

First we need to create a new Rust project:

```
cargo new MyKmeans --bin
```

This will create a folder names MyKmeans and put a Cargo.toml and a src folder into that.
The Cargo.toml is very important as all dependencies will go there.

The src/main.rs is where we will later code the command line tool, and we need to create a lob.rs file that will contain our library.