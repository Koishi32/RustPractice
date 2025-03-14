fn main() {
    //println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();
        // every iteration consumes the current item 
        //of the iterator when using next()
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test] // sum is a consuming adapter than consumes the iterator
    // to add the values to a result
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    // Example of Iterator adapters
    //produces a new iterator out of the 
    //original
    #[test]
    fn iterator_adapter(){
        let v1: Vec<i32> = vec![1, 2, 3];
        // map produces a new iterator
        // it applies the closure to each item to create a new iterator
        // with modified values
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        // collect consumes the new iterator 
        //and collects the resulting values into a collection data type
        //in this case a vector
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

}