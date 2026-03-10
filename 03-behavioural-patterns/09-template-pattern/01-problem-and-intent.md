Template method is a behavioural design pattern that defines the skeleton of an algorithm in the superclass, but
lets subclasses override specific steps of the algorithm without changing its structure.

# Problem
- You decide to create a data mining application that needs users to feed in documents in various formats - PDF, DOC, CSV
- Initially, the app only supported DOC files. Later, you added in support for CSV, then later PDF files.
- At some point, you notice that all three classes have a lot of similar code, differing mostly in how you handle
  different data formats.
- Wouldn't it be amazing to get rid if the code duplication but leave the algorithm structure intact?
- Another problem related to the client code is that it had lots of conditionals that picked a porper course of action
  depending on the class of the processing object.
- If all three processing classes had a common interface or base class, you'd be able to eliminate the conditionals
  in the client code and use polymorphism when calling methods on a processing object.