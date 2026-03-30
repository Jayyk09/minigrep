pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // it is better to use iterate and reduce mutable states when possible
    //    let mut res = Vec::new();
    //    for line in file_content.lines() {
    //        if line.contains(query){
    //            res.push(line);
    //        }
    //    }
    //    res
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let file_content =  "\
Rust:
safe, fast, productive.
Pick three.";
    
        assert_eq!(vec!["safe, fast, productive."], search(query, file_content))
    }
}
