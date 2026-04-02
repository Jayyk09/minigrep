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
// the naive solution could be to go through the string and check manually if each type matches

    None

}
