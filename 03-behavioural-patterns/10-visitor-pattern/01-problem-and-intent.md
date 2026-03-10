Visitor is a behavioural pattern that lets you separate algorithms from the objects on which they operate.

# Problem
- You have an app that works with geographical information structured as one colossal graph.
- Every node of the graph may be a complex entity such as a city, but also more granular things like industries, parks e.t.c
- The nodes are connected to each other if there is a road between the real objects they represent.
- Under the hood, each node type is represented by its own class, while each specific node is an object.
- You are required to implement a feature that helps export the graph to XML format.
- However, you are not allowed to alter the existing node classes (e.g. add an `export` method to each node class), as this
  would introduce potential bugs.
- Furthermore, the primary purpose of the class is to work with geodata, adding XML export behaviour would be alien here.
- Moreover, what it you were to support exporting to different formats? The node classes would become a mess very quickly.