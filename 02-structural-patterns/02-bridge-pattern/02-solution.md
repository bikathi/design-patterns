# Solution
- The problem occurs because we're trying to extend the shape class in two independent dimensions: by form and by color.
- This is a super common issue related to class inheritance.
- Using bridge, we attempt to solve this problem by switching from inheritance to **object composition**.
- Ideally, we split one of the dimensions into a separate class hierarchy, so that the original classes will reference an
  object of the new hierarchy, instead of having all of it's state and behaviours within one class.
- We will have a base class *Shape* from which all shapes like *Circle* and *Square* extend from, and *Color* from which
  classes like *Red* and *Blue* extend from.
- Then in the *Shape* class, we can have a field referencing the color.
- From now on, adding new colors does not force us to change the shape hierarchy and vice versa.