// graphi.h

void randomPermutation(int n, int seed, int perm[]);

int randomGraph(int nodes, int edges, int seed,
                bool simple, bool directed, bool acyclic,
                bool weighted, int minWeight, int maxWeight,
                int nodeI[], int nodeJ[], int weight[]);