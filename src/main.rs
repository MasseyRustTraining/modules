mod hello;
use hello::hello as my_hello;

fn main() {
    my_hello();
}

#[cfg(test)]
mod test {
    const PARAM: usize = 17;

    #[test]
    fn test1() {
        assert!(PARAM < 18);
    }

    #[test]
    fn test2() {
        assert!(PARAM > 16);
    }
}
