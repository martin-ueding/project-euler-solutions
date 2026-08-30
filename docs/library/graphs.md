# Graphs

::: definition Graph
A *directed graph* is a collection of *vertices* and *edges* that connect two vertices in a specific direction. Every edge has a non-negative weight associated with it, each vertex also has a non-negative weight associated with it.
:::

::: definition Cost
The cost to go from one vertex to another one along a path of vertices is the sum of all the edge weights and the target vertex weights.
:::

::: theorem Directed from Undirected Graph
If we have an undirected graph, we can map that onto directed graph by just duplicating all the edges, one for the forward and once for the backward direction.
:::

::: theorem Vertex Weights into Edges
A graph with vertex weights can be converted into a graph with just edge weights by adding the vertex weight to all incoming edges.
:::

::: theorem Dijksta's Algorithm
The optimal route between two vertices $v_1$ and $v_2$ in a directed graph with only edge weights can be found with the following algorithm:

1. Create a heap of (distance, vertex) pairs which contains only the starting node $(W(v_1), v_1)$.
2. Create a mapping of vertex → distance which records the shortest distances found to each intermediate vertex, initialize with $d(v_1) = W(v_1)$.
3. Take the leading element $(d, v)$ from the heap. Iterate through its edges $e = E(v)$. The edge will lead from vertex $v$ to vertex $v'$ For each edge, compute the distance of the target vertex $v'$ as $d(v') = d + W(e) + W(v')$. If that distance is smaller than the one already recorded, update it and add the pair $(d', v')$ to the heap.
4. Repeat until the distance to $v_2$ is available.
:::