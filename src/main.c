#include<stdio.h>
#include <stdlib.h>
#include"searchc.h"

void main() {
    size_t len; 
    printf("donner la taille de tableau \n");
    scanf("%d",&len);
    int* arr = (int*)malloc(len * sizeof(int)) ;
    for (size_t i = 0; i < len; i++)
    {
        printf("donner l element n %d \n",i+1);
        scanf("%d",&arr[i]);
    }
    int a;
    printf("donner l element a chercher dans le tableau \n");
    scanf("%d",&a);
    size_t r = searchc(a,arr, len);
    if (r==0){
        printf("non-existant");
    }else{
        printf("existe position %d",r);
    }
    
    free(arr);
}