Here are some popular ways to use the proxy pattern:
1. Lazy initialization (virtual proxy). This is when you have a heavyweight service object that wastes system resources
   by always being up all the time, even though you only need it from time to time.
   Instead of creating the object when the app launches, you can delay the object's initialization to a time when it is really needed.
2. Access control (protection proxy). This is when you want only specific clients to be able to use the service object. For
   instance, when your objects are crucal parts of an operating system and clients are various launched apps (including
   malicious ones).
   The proxy can pass the request to the service object only if the client's credentials match some criteria.
3. Local execution of a remote service (remote proxy). This is when the service object is located on a remote server.
   In this case, the proxy passes the client request over the network, handling all of the nasty details of working with the
   network.
4. Logging requests (logging proxy). This is when you want to keep a history of requests to the service object.
   The proxy can log each request before passing it to the service.
5. Caching request results (caching proxy). This is when you want to cache the results of client requests and manage the life
   cycle of this cache, especially if the results are quite large.
   The proxy can implement caching for recurring requests that always yield the same results. The proxy may use the parameters
   of requests as the cache keys.
6. Smart reference. This is when you need to be able to dismiss a heavyweight object once there are no clients that use it.
   The proxy can keep track of clients that obtained a reference to the service object or its results. 
   From time to time, the proxy may go over the clients, checking whether they are still active. If the client list gets
   empty, the proxy might dismiss the servic eobject and free the underlying system resources.
   The proxy can also track whether the client had modified the service object, then the unchanged objects may be re-used by
   other clients.