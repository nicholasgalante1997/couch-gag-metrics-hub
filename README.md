# Rust Based Web Server Microservice

> This is a minimal approach to a web server, that listens for metric emissions that are emitted from `couch-gag-server` and `couch-gag-website`. We (Couch Gag), want to be able to iterate on what's successful, and revise areas of repeated concern. We'll be able to learn what those areas are via metric emission.

---

## The other packages are typescript packages, why are we doing this in Rust?

1. Rust is remarkably fast. Loads faster than its express/http(s) node counterpart.
2. Rust makes concurrency easy. Concurrency would work well for us here to cut down on operation time.
   1. [Rust Docs](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
   2. [Applications of Concurrency](https://doc.rust-lang.org/book/ch16-01-threads.html#:~:text=For%20example%2C%20a%20web%20server%20could%20have%20multiple%20threads%20so%20that%20it%20could%20respond%20to%20more%20than%20one%20request%20at%20the%20same%20time.)
   3. [Oracle Docs on 1:1 Threads](https://docs.oracle.com/cd/E19620-01/805-4031/6j3qv1oej/index.html#:~:text=The%20one%2Dto%2Done%20model%20(one%20user%20thread%20to%20one%20kernel%20thread)%20is%20among%20the%20earliest%20implementations%20of%20true%20multithreading.%20In%20this%20implementation%2C%20each%20user%2Dlevel%20thread%20created%20by%20the%20application%20is%20known%20to%20the%20kernel%2C%20and%20all%20threads%20can%20access%20the%20kernel%20at%20the%20same%20time.)
