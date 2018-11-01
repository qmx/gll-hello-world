extern crate gll;
extern crate gll_macros;

mod hello {
    ::gll_macros::proc_macro_parser! {
        MyRule = Hello:"hello";
    }

    #[test]
    fn test_hello_world() {
        let input  =  "hello".parse().unwrap();
        MyRule::parse_with(input, |parser, result| {
        
            let v = result.unwrap();
        });

    }
}

fn main() {
    println!("Hello, world!");
}
