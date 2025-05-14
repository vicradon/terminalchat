fn main() {
    let mut i = 3; 

    { 

        i += 1;
        let borrow2 = &i;
        let borrow1 = &i; 

        println!("{}", borrow1 == borrow2);

        println!("borrow1: {}", borrow1); 
    } 

    { 
        let borrow2 = &i; 

        println!("borrow2: {}", borrow2); 
    } 

}   
