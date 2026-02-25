Use the flyweight pattern when:
1. Your program must support a huge number of objects which barely fit into available RAM. You can deduce such objects if:
  1. an application needs to spawn a huge number of similar objects
  2. your app is draining all the available RAM on a target device
  3. the objects being created contai duplicate states that can be extracted and shared between multiple objects.
