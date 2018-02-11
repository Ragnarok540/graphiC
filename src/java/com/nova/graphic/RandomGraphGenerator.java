package com.nova.graphic;

import java.util.*;

public class RandomGraphGenerator {
	
	private Random rand;	
	private int result[];
	private int nodeI[];
	private int nodeJ[];
	private int weight[];

	public int[] getResult() {
		return result;
	}

	public int[] getNodeI() {
		return nodeI;
	}

	public int[] getNodeJ() {
		return nodeJ;
	}

	public int[] getWeight() {
		return weight;
	}

	public RandomGraphGenerator() {
		rand = new Random();
	}
	
	private native void randomPermutation(int n, int seed);
	
	public void randomPermutation(int n) {
		result = new int[n];
		randomPermutation(n, rand.nextInt());
	}
	
	private native int randomGraph(int nodes, int edges, int seed,
			boolean simple, boolean directed, boolean acyclic,
			boolean weighted, int minWeight, int maxWeight);
	
	public int randomGraph(int nodes, int edges,
			boolean simple, boolean directed, boolean acyclic,
			boolean weighted, int minWeight, int maxWeight) {
		nodeI = new int[edges];
		nodeJ = new int[edges];
		weight = new int[edges];
		int error = randomGraph(nodes, edges, rand.nextInt(), simple,
				directed, acyclic, weighted, minWeight, maxWeight);
		return error;
	}

}