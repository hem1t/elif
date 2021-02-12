# elif!

#### Motive: Just for Fun (Nothing)

Well, I just don't like to use `else if`, I liked `elif` much better, so, tried to create a macro which would let me use `elif` instead of `else if`, but even instead of that I came with something else, you can see in below example.

### Example

```rust
    let num = // be a random value.
    elif! {
        (num == 1) "One", // <-- End with `,`
        // ^------------ This one will go to if block
        // After first one every one else will be else if block
        (num == 2) {      // <--- You can use {}
            
        },
        () { "None" }
    //   ^ --- Empty condition will create else block
    }
```

##### Thank You!
