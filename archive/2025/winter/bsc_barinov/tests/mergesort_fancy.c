#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ARRAY_SIZE   10
#define RAND_SEED    42
#define RAND_MAX_VAL 10

/* Merge two sorted subarrays arr[l..m] and arr[m+1..r] */
void merge(int arr[], int l, int m, int r) {
    int len = r - l + 1;
    int *temp = (int*)malloc(len * sizeof(int));
    if (!temp) exit(EXIT_FAILURE);

    int i = l, j = m + 1, k = 0;

    while (i <= m && j <= r) {
        if (arr[i] <= arr[j]) {
            temp[k++] = arr[i++];
        } else {
            temp[k++] = arr[j++];
        }
    }

    while (i <= m) temp[k++] = arr[i++];
    while (j <= r)  temp[k++] = arr[j++];

    memcpy(&arr[l], temp, len * sizeof(int));
    free(temp);
}

/* Recursive merge sort */
void mergeSort(int arr[], int l, int r) {
    if (l < r) {
        int m = l + (r - l) / 2;
        mergeSort(arr, l, m);
        mergeSort(arr, m + 1, r);
        merge(arr, l, m, r);
    }
}

/* Checker: verify array is sorted in ascending order */
int isSorted(int arr[], int n) {
    for (int i = 1; i < n; i++) {
        if (arr[i - 1] > arr[i]) return 0;
    }
    return 1;
}

/* Checker: verify sum of elements is preserved after sort */
int sumEquals(int a[], int b[], int n) {
    int sa = 0, sb = 0;
    for (int i = 0; i < n; i++) {
        sa += a[i];
        sb += b[i];
    }
    return sa == sb;
}

/* Print a non-negative integer using putchar (non-variadic) */
void print_int(int n) {
    if (n < 0) {
        putchar('-');
        n = abs(n);
    }
    if (n >= 10) {
        print_int(n / 10);
    }
    putchar('0' + (n % 10));
}

/* Print an integer array */
void print_arr(int arr[], int n) {
    putchar('[');
    for (int i = 0; i < n; i++) {
        print_int(arr[i]);
        if (i < n - 1) {
            putchar(',');
            putchar(' ');
        }
    }
    putchar(']');
    putchar('\n');
}

int main(void) {
    int *arr  = (int*)malloc(ARRAY_SIZE * sizeof(int));
    int *orig = (int*)malloc(ARRAY_SIZE * sizeof(int));
    if (!arr || !orig) exit(EXIT_FAILURE);

    /* Generate random array using stdlib rand/srand */
    srand(RAND_SEED);
    for (int i = 0; i < ARRAY_SIZE; i++) {
        arr[i] = rand() % RAND_MAX_VAL;
    }

    /* Keep a copy of the original for sum-preservation check */
    memcpy(orig, arr, ARRAY_SIZE * sizeof(int));

    puts("Before: ");
    print_arr(arr, ARRAY_SIZE);

    mergeSort(arr, 0, ARRAY_SIZE - 1);

    puts("After:  ");
    print_arr(arr, ARRAY_SIZE);

    /* Run checkers */
    int sorted_ok = isSorted(arr, ARRAY_SIZE);
    int sum_ok    = sumEquals(arr, orig, ARRAY_SIZE);

    if (sorted_ok && sum_ok) {
        puts("Result: PASS\n");
    } else {
        puts("Result: FAIL");
        if (!sorted_ok) puts(" (not sorted)");
        if (!sum_ok)    puts(" (sum mismatch)");
        putchar('\n');
    }

    free(arr);
    free(orig);
    return 0;
}