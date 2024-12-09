# Day 01 - Summary

This first entry was pretty straightfoward if you knew how to unzip, sort, and zip iterators.

## Lessons Learned:

- I was surprised to learn that Vec.sort sorts in place instead of returning a new vector. I expected something more functional.

# Day 02 - Summary

## Lessons Learned:

- Time constraints: Monday's are busy for me with meeting someone for a workout, my workday, and an in-person gaming meetup in the evening. I set a time
  box in which to work on this problem which had me take short-cuts which ended up being a bad decision. "If you want to go fast you have to go well." Thanks, Uncle Bob.
- Time constraints: I glossed over the instructions and missed a business rule to implement regarding how far apart two numbers could be. This lead to my solution being too large as I was not excluding enough information.
- A tricky part for me in day two was iterating over a pair / window in the iterator. After a search, I found a solution that did not require an external library. Zip an iterator off the same collection twice. On the second iterator, skip one value. This will give tuples of the 'current' and 'next' item in the list.

# Day 03 - Summary

- I struggled with part2. All my logic seemed correct, but I had been working under the assumption that the state resets for each line. That is not correct.

# Day 04 - Summary

## Lessons Learned

- The scanning window implementation seemed to work well. However, the window needed to be 7x7 so that the centerpoint could radiate out to check all the matches. Checkng every match in the window was a failure.
- I'm debating on whether padding the input vertically and horizontally would be useful versus handling the edges.
- Created the scan_radius.rs utility

## Day 05 - Summary

- Parsing remains challenging. This one required chunking the input into two segments. Consuming an iterator didn't make that easy. I could use a function that chunks an iterator into vectors based on a chunk separator

## Day 06 - Summary

- Always test your functions even if they look easy. You messed them up, I gurantee it.

## Day 07 - Summary

- The challenge was getting a dynamic number of operators. It was easy with two operators, but with 3 it was challege to create a combination of all the different operations.
- One of the things that was needed was to use function pointer.
- Other solutions worked backwards from the test_value. Checking if the next combination of operator & value was possible before proceeding. This limits the search space
