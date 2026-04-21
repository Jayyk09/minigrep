// This file is to test the parse func for ReGex pattern. This would implement and test the rangeQuantifier. It can be defined as
// a{n} - match a exactly n times 
// a{n,} - match a atleast n times
// a{n,m} - in the range of n and m.


// The BNF grammer -> 

// The first approach is to create an type rangeQuantifier that would represent the states: exact, minimum and between

enum RangeQuantifier {
    Exact(u32),
    Minimum(u32),
    Between(u32, u32)
}

pub fn parse_range_quantifier(s: &str) -> Option<(RangeQuantifier, &str)> {
// the naive solution could be to go through the string and check manually if each type matche

    let s = s.strip_prefix('{')?; // the alternative is let else with explicit checks

// works but not idiomatic Rust
//    let mut digits = String::new();
//    for char in s.chars() {
//        if char.is_ascii_digit() {
//            digits.push(char);
//        } else {
//            break;
//        }
//    }
//    let lower_bound: u32 = digits.parse().ok()?;

    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (digit, s) = s.split_at(end);
    let lower_bound: u32 = digit.parse().ok()?;

    let (quantifer, s) = match s.strip_prefix(',') {
        None => (RangeQuantifier::Exact(lower_bound), s ),
        Some(s) => {
            let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
            let (digit, s) = s.split_at(end);
            match digit.is_empty() {
                true => (RangeQuantifier::Minimum(lower_bound), s),
                false => (RangeQuantifier::Between(lower_bound, digit.parse().ok()?), s)
            }
}
};

    let s = s.strip_prefix('}')?;

    Some((quantifer, s))

}

// we can abstract the parse string and numbers away, but as we can see that all of the different
// functions and the main one have the same signature

// We create a struct that takes a generic type A and calls the function Parser. 
// The parser fn takes in a string and returns type A with the rest of the string or None. 
// Since a closure in Rust has an anonymous type, we can not put in the stack so instead we make it
// heap allocated dynamic. 
// Since a parser should not be tied to a specific string and should be tied to whatver string
// calls it I implemented 

type ParseFn<A> = Box<dyn for<'a> Fn(&'a str) -> Option<(A, &'a str)>>;

struct Parser<A> {
    parse: ParseFn<A>
}

// implement a method over Parser - it constructs a generic constructor which returns a Parser
// 
// the static makes sure there is no short lived borrow. 
// It makes sure the user does not have to worry about Boxing.

impl<A> Parser<A> {
    fn new<F>(f: F) -> Self
        where F: for<'a> Fn(&'a str) -> Option<(A, &'a str)> + 'static,
    {
        Parser {parse: Box::new(f)}
    }
}

