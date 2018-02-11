package com.nova.graphic;

public class HelloGraph {
			
	public static void main(String[] args) {
		RandomGraphGenerator rgg = new RandomGraphGenerator();
		int nodes = 5, edges = 8, minWeight = 90, maxWeight = 99;
		boolean simple = true, directed = false, 
				acyclic = false, weighted = true;
	    int error = rgg.randomGraph(nodes, edges, simple, directed, acyclic, weighted, minWeight, maxWeight);
	    
	    if (error != 0) {
	    	System.out.print("Error code: " + error);
	    	return;
	    } 
	    
	    System.out.println("List of Edges:\nfrom to weight");
        
	    for (int i = 0; i < 8; i++) {
	    	System.out.println(rgg.getNodeI()[i] + "    " + rgg.getNodeJ()[i] + "  " + rgg.getWeight()[i]);
	    }
	}
		
	static {
		System.loadLibrary("graphiC_lib");
	}

}