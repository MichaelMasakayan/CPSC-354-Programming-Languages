fn borrow() {
    // the point here is that an immutable reference is borrowed.
    fn sum_vec(v: &Vec<i32>) -> i32 {
    v.iter().fold(0, |a, &b| a + b)
    }
    // Borrow two vectors and sum them.
    // This kind of borrowing does not allow mutation through the borrowed reference.
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // we do stuff with ‘v1‘ and ‘v2‘.
    let s1 = sum_vec(v1);
    let s2 = sum_vec(v2);
    // this will return the answer
    s1 + s2
    }
    let v2 = vec![4, 5, 6];
    let v1 = vec![1, 2, 3];
    let answer = foo(&v1, &v2);
    println!("{}", answer);
    }
 
    fn owner() {
        let s = String::from("hello");  // s comes into scope
   
        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
   
        let x = 5;                      // x comes into scope
   
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
   
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
   
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.
   
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    fn hello(){
        println!("Hello World!")
    }
    //function runs all the programs talked about in the report
    fn main(){
        owner();
        //outputs 5
        hello();
        //outputs hello world
        borrow();
        //outputs 21
    }


