# Rust and ownership

Pgms have a stack and heap. Stack is preprovisioned spaced based on data types determined at compile-time. If data type is dynamic and allocated at run-time it pgs use a heap to store/locate data. Data usage from the heap is complex, slow and garbage collectors responsible for cleanup have to deal with resolving data references/duplicate data cleanup/multiple frees to the same data.

C++ has a pattern called RAII (resource acquisition is initialization) that `drop`s data at the end of an items lifetime. That is the same idea in Rust,aka, when an item is out of scopr it is dropped.

Ownership Rules in Rust
* Each value has a variable thats called its owner
* There can only be one owner
* Once owner is out of scope, the value is dropped

- Normal Types

{ // no s in scope
  let s = "hello"; // s in scope
 // do stuff with s
}  // scope is now over, s is not valid

- String Type

let s1 = String::from("hello");
let s2 = s1;

s1 		index | value		s2
ptr         =>   0	h	 <=	ptr
len 5		 1      e		len 5
capacity 5 	 2	l		capacity 5
		 3 	l
		 4	o


- Function Returns

main() {
    let x = 5;
    let y = makes_copy(x); 
}

makes_copy(x) {
   x // x is returned and moves out to the calling function
}

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}



## References



- References
