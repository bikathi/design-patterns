- We use the composite pattern.
- This suggests that you work with *Products* and *Boxes* through a common interface that declares a method
  for calculating the total price.
- For a simple product, it would simply reutrn the product's price.
- For a box, it'd go over each item the box contains, asks its price and return a total for the box.
- If one of those items were a box, that box would start going over its contents and so on, until we have a cumm price.
- A box could even add some extra cost to the final price, such as the packaging cost.