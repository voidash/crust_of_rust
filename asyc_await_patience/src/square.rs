pub mod square {

    pub fn misc() {
        println!("test");
            if 2 < 3 {
                println!("test");
            }
    }


    #[cfg(test)]
    mod test{
        use super::*;
        #[test]
        fn some_awesome_test() {
            misc();
            println!("quick brown fox jumps over the lazy dog");
            assert_eq!(5,5);
        }
    }
}
