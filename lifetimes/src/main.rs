fn main() {
    lifetimes();
}

fn lifetimes() {
    let s1 = String::from("foo is longer");

    {
        let s2 = String::from("bar");
        let longer = borrower(s1.as_str(), s2.as_str());

        println!("{}", longer);
    }
}

fn borrower<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    return y;
}
