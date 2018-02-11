#include <stdbool.h>
#include <stdio.h>
#include "com_nova_graphic_RandomGraphGenerator.h"
#include "graphi.h"

JNIEXPORT void JNICALL Java_com_nova_graphic_RandomGraphGenerator_randomPermutation
  (JNIEnv *env, jobject obj, jint n, jint seed) {
    jclass cls = (*env)->GetObjectClass(env, obj);
    jfieldID fid = (*env)->GetFieldID(env, cls, "result", "[I");
    
    if (fid == NULL) {
        return;
    }
    
    jintArray jarr = (jintArray) (*env)->GetObjectField(env, obj, fid);
    jint *body = (*env)->GetIntArrayElements(env, jarr, 0);
    int perm[n];
    randomPermutation(n, seed, perm);
    
    for (int i = 0; i < n; i++) {
        body[i] = perm[i];
    }
        
    (*env)->ReleaseIntArrayElements(env, jarr, body, 0);
}

JNIEXPORT jint JNICALL Java_com_nova_graphic_RandomGraphGenerator_randomGraph
  (JNIEnv *env, jobject obj, jint nodes, jint edges, jint seed, 
  jboolean simple, jboolean directed, jboolean acyclic, 
  jboolean weighted, jint minWeight, jint maxWeight) {
    
    jclass cls = (*env)->GetObjectClass(env, obj);
    jfieldID fidNodeI = (*env)->GetFieldID(env, cls, "nodeI", "[I");
    jfieldID fidNodeJ = (*env)->GetFieldID(env, cls, "nodeJ", "[I");
    jfieldID fidWeight = (*env)->GetFieldID(env, cls, "weight", "[I");
    
    if (fidNodeI == NULL || fidNodeJ == NULL || fidWeight == NULL) {
        return (jint) 2;
    }
    
    jintArray jarrNodeI = (jintArray) (*env)->GetObjectField(env, obj, fidNodeI);
    jint *bodyNodeI = (*env)->GetIntArrayElements(env, jarrNodeI, 0);
    jintArray jarrNodeJ = (jintArray) (*env)->GetObjectField(env, obj, fidNodeJ);
    jint *bodyNodeJ = (*env)->GetIntArrayElements(env, jarrNodeI, 0);
    jintArray jarrWeight = (jintArray) (*env)->GetObjectField(env, obj, fidWeight);
    jint *bodyWeight = (*env)->GetIntArrayElements(env, jarrWeight, 0);
    
    int nodeI[edges];
    int nodeJ[edges];
    int weight[edges];
    
    int error = randomGraph(nodes, edges, seed, simple, directed, acyclic,
                weighted, minWeight, maxWeight, nodeI, nodeJ, weight);
    
    for (int i = 0; i < edges; i++) {
        bodyNodeI[i] = nodeI[i];
        bodyNodeJ[i] = nodeJ[i];
        bodyWeight[i] = weight[i];
    }
        
    (*env)->ReleaseIntArrayElements(env, jarrNodeI, bodyNodeI, 0);
    (*env)->ReleaseIntArrayElements(env, jarrNodeJ, bodyNodeJ, 0);
    (*env)->ReleaseIntArrayElements(env, jarrWeight, bodyWeight, 0);
    
    return (jint) error;
}