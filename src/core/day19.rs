//! Part1:
//! Thoughts:
//! * There are single characters except for white (w)
//! * To find the shortest path (not requested) need to match the longest strings
//! * Match the longest strings solves the problem faster
//! * Can remove items from the search space, once their length exceeds the remaining length
//!   of the string being searched
//! * Bucket the 'towels' that can be used based on their first character
//! * Quick part 1 filter: Any strings without a 'w' can be made. Looking at the data, it doesn't help.
//! * Smells like the towels could be a node in a graph with an edge weight related to the length
//!   of the string representation. However, all nodes would be interconnected so that is that same
//!   as 'not a graph'.
//! * maybe scan up to the 'w', then look left and right for e.g., 'gwu' check for 'gw' or 'wu' or 'gwu'. i
//!   If that fails, exapand 'rgwub' look for 'rgw','wub','rgwub'

//! Questions:
//! * What is the distribution of starting towels?
//! * Are there subtowels that makes up major towels? That way they could be removed from the search
//!   space? Yes, there are. For instance: rubr is made up of r, u, b, r. So, 'rubr' does not need to
//!   be in the search space in part one as it is possible based on the smaller items.
//! * Can other strings that contain 'w' be decomposed/factored into smaller items? Yes. ugbbww is made
//!   up of smaller towels ugb and bww. I'm sure this will bite me in part2 as they will want to know
//!   the minimum number of towels that can be used to create the pattern.
//! * Is there a way just to examine the white strings?
//! Approach:
//! * Bucket the strings based on the first character
//! * Sort the buckets by string len so that the longest strings are attempted to match first.
