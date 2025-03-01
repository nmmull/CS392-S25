// PLEASE REMOVE THIS WHEN YOU'RE DONE
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

// Problem 1
//
// I'm currently sitting in on a reading group that's going over the
// Gödel's original proof of the incompleteness theorems.  In that
// proof, he defines a way of mapping logical formulas (sequences of
// symbols) to natural numbers.  The idea is simple: map each symbol
// to a number, and then map a sequence of numbers to THE EXPONENTS of
// the prime factorization of a number.  For example, given the
// mapping of words "a", "b", and "c":
//
//   a ↦ 1
//   b ↦ 2
//   c ↦ 3
//
// we can represent the string "abca" as
//
//   2¹ * 3² * 5³ * 7¹ = 15750
//
// In the context of modern computing, we can see that what Gödel came
// up with was an insane serializer.  But, just for the heck of it,
// we're going to build an encoder/decoder for Gödel numbers.

// I've included the collection of crates you'll need here. In order
// for this to be interesting, we need to use a representation of
// unsigned integers without overflow (the numbers get big fast).
// THIS MEANS YOU'LL HAVE TO READ THROUGH THE DOCUMENTATION OF
// BIGUINT.  Feel free to add any other `use` statements you need.
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{Formatter, Display};
use num::{BigUint, Integer, One, Zero};
use num::traits::Euclid;

// We start with a word encoder, which keeps track the list of words
// we want to encode (in the the example above, we had the words "a",
// "b", "c").
//
// Words in will be encoded by their position in a vector + 1.  We'll
// also create a hash map to from words to their encodings. Note that
// that you may have to add additional trait bounds to T.
struct WordEncoder<T : Hash + Eq + Display + Clone> {
    words: Vec<T>,
    encodes: HashMap<T, usize>,
}

#[derive(Debug)]
enum WordEncoderError { Overflow }

impl Display for WordEncoderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
	write!(f, "TOO MANY WORDS TO ENCODE")
    }
}
impl std::error::Error for WordEncoderError { }

// Take a look at the TryFrom trait in the standard library and
// implement it for WordEncoders.  This should fail (return an
// `Overflow` error) if there are more than 100 words in `v`.
impl<T : Hash + Eq + Display + Clone> TryFrom<Vec<T>> for WordEncoder<T> {
    type Error = WordEncoderError;
    fn try_from(v : Vec<T>) -> Result<WordEncoder<T>,WordEncoderError> {
	todo!()
    }
}

// Implement a structure which can be used as an interator for prime
// numbers.  That it, it will implement the Iterator trait with the
// next function that returns the next prime number. You can implement
// it in the most naive way, e.g., by trial division algorithm (feel
// free to look it up, it's basically: find the next number which is
// not divisible by any of the numbers you've seen so far).
struct Primes {
    // TODO
}

impl Primes {
    fn new() -> Primes {
	todo!()
    }
}

impl Iterator for Primes {
    type Item = BigUint;
    fn next(&mut self) -> Option<BigUint> {
	todo!()
    }
}

impl<T: Hash + Eq + Display + Clone> WordEncoder<T> {
    // Implement a function which encodes a vector of T's as a BigUint
    // according to the above procedure. That is, it should map v to a
    // number for which the exponent of the i-th prime number is the
    // number encoding v[i]
    fn encode(&self, v: &Vec<T>) -> BigUint {
	todo!()
    }

    // Implement a function which decodes a BigUint as a string (this
    // requires that T implements the Display trait). That is,
    // determine the sequence of exponents in the prime factorization
    // of `n`, decode those exponents into words, convert those to
    // strings using the `.to_string()` method, and then concatente
    // those strings together.
    fn decode(&self, mut n: BigUint) -> Option<String> {
	todo!()
    }
}

// Problem 2
//
// Suppose we wanted to create a structure which was a stack of
// reference to values stored in the structure itself.  This might
// look a bit like the following code.
//
// There are two problems with it as of now:
//
// 1. There are errors to do with lifetimes.  Uncomment the definition
//    below and ADD LIFETIME ANNOTATIONS so that it passes the borrow
//    checker.
//
// 2. Uncomment the example in `main` below. There are errors to do
//    with borrowing.  Describe in your own words what the problem is
//    (in particular, don't just repeat the error message).  Also
//    describe a possible solution to the problem.



// struct RefStack<T> {
//     values: Vec<T>,
//     stack: Vec<&T>,
// }

// impl<'a, T> RefStack<T> {
//     fn new() -> RefStack<T> {
// 	RefStack {
// 	    values: vec![],
// 	    stack: vec![],
// 	}
//     }

//     fn add_value(&mut self, t: T) -> &T {
// 	self.values.push(t);
// 	self.values.last().unwrap()
//     }

//     fn push(&mut self, t_ref: &T) {
// 	self.stack.push(t_ref)
//     }
// }



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PROBLEM 1
    // An example from Gödel's proof:
    // let words = WordEncoder::try_from(vec!["f","0","(", ")", "Π", "x", "∨", "~"])?;
    // let v = vec!["(", "x", "Π", "(", "x", "(", "f", "(", "0", ")",
    // 		 ")", ")", ")", "∨", "(", "~", "(", "x", "Π", "(",
    // 		 "x", "(", "f", "(", "0", ")", ")", ")", ")", ")"];
    // let gnum = words.encode(&v);
    // println!("encoding: {}", gnum);
    // println!("decoding: {}", words.decode(gnum).unwrap());

    // PROBLEM 2
    // let mut s : RefStack<'_, String> = RefStack::new();
    // let t_ref = s.add_value(String::from("foo"));
    // s.push(t_ref);
    Ok(())
}
