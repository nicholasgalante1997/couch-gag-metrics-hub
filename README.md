# Couch Gag Metrics Hub

> This is a minimal approach to a web server, that listens for metric emissions that are emitted from `couch-gag-server` and `couch-gag-website`. We (Couch Gag), want to be able to iterate on what's successful, and revise areas of repeated concern. We'll be able to learn what those areas are via metric emission.

---

## Couch Gag Abstract

Couch Gag is a collective of several microservices that compose a modern markup site. We, Couch Gag, believe in a domain driven architectural design. A service should be small. It should do one thing exceedingly well. If it begins to dilute its responsibilities, the complexity of maintenance of the service rises geometrically along with it. It becomes arduous to onboard onto for new developers. It becomes bad code.

Bad code is so difficult to maintain. Its not even funny. I am not laughing, do you see my face. Its the reason why entire teams of developers rewrite services after 2 years, while only marginally adding onto overall functionality. As a developer, I want the code I write to run a thousand years. It won't. But with that mentality it might run 5, well.

**Couch Gag is a collection of microservices** and the list of associated services can be found below.

- couch-gag-common-library
  - A Typescript package, isomorphic, that supplied common utils to `couch-gag-server` and `couch-gag-website`. 
  - These utils include loggers, theme primitives, and shared types.
  - It is a compile time dependency for `couch-gag-website` and `couch-gag-server`.
- couch-gag-metrics-hub
  - A Rust implemented web-server microservice.
  - `couch-gag-website` and `couch-gag-server` pump metrics via http requests to this server.
  - This server listens for incoming http requests, pulls metric values off the request, and maps it to a Metric type
- couch-gag-server
  - A Node/Typescript/Express http server implementation
  - Manages **STORY MARKUP** and serves **story collection data*, and *individual story data* to `couch-gag-website`
  - pumps metric data about stories to `couch-gag-metrics-hub` 
  - Stories are written in **MARKDOWN** and can be found in this package in `src/data/`
- couch-gag-website
  - A typescript/nextjs/react application that acts as the frontend to serve **STORY MARKUP** through.
  - Fetches **story data** from `couch-gag-server` to render to users in browser/mobile.
  - pumps metric data about user events to `couch-gag-metrics-hub`

## Developer Guide

## The other packages are typescript packages, why are we doing this in Rust?

1. Rust is remarkably fast. Miles faster than its express/http(s) node counterpart.
2. Rust makes concurrency easy. Concurrency would work well for us here to cut down on operation time.
   1. [Rust Docs](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
   2. [Applications of Concurrency](https://doc.rust-lang.org/book/ch16-01-threads.html#:~:text=For%20example%2C%20a%20web%20server%20could%20have%20multiple%20threads%20so%20that%20it%20could%20respond%20to%20more%20than%20one%20request%20at%20the%20same%20time.)
   3. [Oracle Docs on 1:1 Threads](https://docs.oracle.com/cd/E19620-01/805-4031/6j3qv1oej/index.html#:~:text=The%20one%2Dto%2Done%20model%20(one%20user%20thread%20to%20one%20kernel%20thread)%20is%20among%20the%20earliest%20implementations%20of%20true%20multithreading.%20In%20this%20implementation%2C%20each%20user%2Dlevel%20thread%20created%20by%20the%20application%20is%20known%20to%20the%20kernel%2C%20and%20all%20threads%20can%20access%20the%20kernel%20at%20the%20same%20time.)
