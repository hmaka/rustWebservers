# rustWebservers
Different types of Async/Sync web server implementations in the Rust programing language.

Each Folder contains a different web server implementation.

1. The "single_threaded" folder contains a simple API built only from the Standard Library, it can only handle one request at a time.
2. The "Multi_threaded" folder contains a simple API built only from the Standard Library. It is multi-threaded, and can handle multiple requests at a time.
   The Multi-threading is done by keeping track of a max thread count which acts as a sort of thread pool and limiting request load by that thread count.
3. The "async_tokio" folder contains a more involved chat-server implementation that allows for any number of users to connect and chat with each other. The project uses
   the tokio async runtime.
4. The "async_warp" folder contains a simple API build using the tokio async runtime and the warp framework for simplicity. 
