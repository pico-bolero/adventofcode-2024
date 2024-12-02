# Day 01 - Summary

This first entry was pretty straightfoward if you knew how to unzip, sort, and zip iterators.

## Lessons Learned:

- I was surprised to learn that Vec.sort sorts in place instead of returning a new vector. I expected something more functional.

# Day 02 - Summary

## Lessons Learned:

- Time constraints: Monday's are busy for me with meeting someone for a workout, my workday, and an in-person gaming meetup in the evening. I set a time
  box in which to work on this problem which had me take short-cuts which ended up being a bad decision. "If you want to go fast you have to go well." Thanks, Uncle Bob.
- Time constraints: I glossed over the instructions and missed a business rule to implement regarding how far apart two numbers could be. This lead to my solution being too large as I was not excluding enough information.
- A tricky part for me in day two was iterating over a pair / window in the iterator. After a search, I found a solution that did not require an external library. Zip an iterator off the same collection twice. On the second iterator, skip one valuee. This will give tuples of the 'current' and 'next' item in the list.
