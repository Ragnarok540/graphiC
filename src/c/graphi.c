#include <stdlib.h>
#include <stdbool.h>

void randomPermutation(int n, int seed, int perm[]) {
    int i, j, k;
    srand(seed);

    for (i = 0; i < n; i++) {
        perm[i] = i + 1;
    }

    for (i = 0; i < n; i++) {
        j = rand() % n;
        k = perm[i];
        perm[i] = perm[j];
        perm[j] = k;
    }
}

int randomGraph(int nodes, int edges, int seed,
                bool simple, bool directed, bool acyclic,
                bool weighted, int minWeight, int maxWeight,
                int nodeI[], int nodeJ[], int weight[]) {
    int maxEdges, nodeA, nodeB, numEdges = 0, temp;
    int dagPermute[nodes];
    bool adj[nodes][nodes];
    srand(seed);
    
    for (nodeA = 0; nodeA < edges; nodeA++) {
            for (nodeB = 0; nodeB < edges; nodeB++) {
                adj[nodeA][nodeB] = false;
        }
    }
    
    if (simple) {
        maxEdges = edges * (edges - 1);
        if (!directed) maxEdges /= 2;
        if (edges > maxEdges)  return 1;
    }
    
    if (acyclic) {
        maxEdges = (edges * (edges - 1)) / 2;
        if (edges > maxEdges) return 1;
        randomPermutation(nodes, seed, dagPermute);
    }
    
    while (numEdges < edges) {
        nodeA = rand() % nodes + 1;
        nodeB = rand() % nodes + 1;
        
        if (simple || acyclic) {
            if (nodeA == nodeB) continue;
        }
        
        if ((simple && !directed) || acyclic) {
            if (nodeA > nodeB) {
                temp = nodeA;
                nodeA = nodeB;
                nodeB = temp;
            }
        }
        
        if (acyclic) {
            nodeA = dagPermute[nodeA];
            nodeB = dagPermute[nodeB];
        }
        
        if (!simple || (simple && !adj[nodeA][nodeB])) {
            nodeI[numEdges] = nodeA;
            nodeJ[numEdges] = nodeB;
            adj[nodeA][nodeB] = true;
            
            if (weighted) {
                weight[numEdges] = minWeight + (rand() % (maxWeight - minWeight));
            }
            
            numEdges++;
        }
    }

    return 0;
}