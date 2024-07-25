
# Error Handling in Rust with Actix Web: Best Practices and Strategies

Code used in the blog post https://medium.com/@naveenkumarkarthigayan/error-handling-in-rust-with-actix-web-best-practices-and-strategies-551602086fd3

I wanted to learn Rust and I find trying to make codes work more engaging. One example I learned from this that Rust allows multiple impl blocks.




## Fixes

Added this code to make it work

    impl Error {
        pub fn get_status_code(&self) -> StatusCode {
            self.get_codes().0
        }

        ....
    }

Created an apporximate folder structure based on errors were imported in main.rs


## Acknowledgements

 - [Error Handling in Rust with Actix Web: Best Practices and Strategies Blog Post by Naveenkumar Karthigayan](https://medium.com/@naveenkumarkarthigayan/error-handling-in-rust-with-actix-web-best-practices-and-strategies-551602086fd3)
 - [ChatGPT for understanding compile time error](https://chat.openai.com/chat)
 - [Readme file generator](https://readme.so/editor)