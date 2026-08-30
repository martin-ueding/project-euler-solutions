# Maximum Path Sum I (18)

In [Problem 18](https://projecteuler.net/problem=18), we're asked to find the maximum path sum down a number triangle.

There are three ways to solve this problem. We will go through them from simple to complex.

## Brute force

As this small triangle has 15 rows, we have to make 14 left/right decisions on the way down. This means that we have $2^{14} = 16\,384$ possible paths. That is not a lot and we can just iterate through all of them, for instance with recursion.

## Bottom-up summing

If one looks at one node deep within the triangle, the problem appears symmetric: there are two incoming links and two outgoing links. The sides of the triangles are different though, the triangle gets wider towards the base.

What we're really interested in is the maximum path sum, not which path it was. So we can reduce the problem, line by line.

If the triangle would consist only of two lines, it would have three elements. There would be once choice, going left or right. The maximum path sum from the root node is the value of the root node plus the value of the bigger node from the level below. We can *replace* the root node with path sum and remove the nodes below that.

We do the same approach with all the nodes on the second-lowest line. If we have reached that node, there is only one more decision to do. We can resolve these and then delete the lowest line.

We can then apply the same trick again, removing the next-lowest line. Eventually only the root node will remain, holding the maximum path sum.

This approach is very elegant because it doesn't need additional memory for intermediate results, we can just overwrite the triangle itself.

## Dijkstra's algorithm

The number triangle can also be interpreted as a graph with edges going downward. We need a virtual terminal node with weight zero that can be reached from all the bottom-row nodes such that we have a clear target node that we want to reach.

Then we just need to apply Dijkstra's algorithm from the [graph library](../library/graphs.md) to find the maximum path from root node to the virtual terminal node. Dijkstra's algorithm usually finds the minimum path length from the edges, but we just count the node values towards the incoming edges and flip the signs to get the maximum path sum.

The algorithm will start at the root node, but it will keep track of the maximum path sum toward every visited node. This way we don't have to re-sum every path but can do this level by level.

This needs more memory than the bottom-up summing approach, the runtime likely is also worse. However, this is a more generic approach that can be used for other problems as well.