static int temp[1000];

// Merge two sorted subarrays arr[l..m] and arr[m+1..r]
void merge(int arr[], int l, int m, int r) {
  int i = l;
  int j = m + 1; 
  int k = l;

  while (i <= m && j <= r) {
    if (arr[i] <= arr[j]) {
      temp[k++] = arr[i++];
    } else {
      temp[k++] = arr[j++];
    }
  }

  while (i <= m) {
    temp[k++] = arr[i++];
  }
  while (j <= r) {
    temp[k++] = arr[j++];
  }

  for (i = l; i <= r; i++) {
    arr[i] = temp[i];
  }
  return;
}

// Merge sort function
void mergeSort(int arr[], int l, int r) {
  if (l < r) {
    int m = l + (r - l) / 2;
    mergeSort(arr, l, m);
    mergeSort(arr, m + 1, r);
    merge(arr, l, m, r);
  }
}

// Verify if array is sorted in ascending order
int isSorted(int arr[], int n) {
  for (int i = 1; i < n; i++) {
    if (arr[i - 1] > arr[i]) {
      return 0; // Not sorted
    }
  }
  return 1; // Sorted
}

void my_write(volatile char val) {
    __asm__ volatile("mov x0, #1\n"  // fd = stdout
                     "mov x1, %0\n"  // buffer pointer
                     "mov x2, #1\n"  // length = 1
                     "mov w8, #64\n" // write syscall
                     "svc #0\n"
                     :
                     : "r"(&val) // pass pointer to the character
                     : "x0", "x1", "x2", "x8");
    return;
}

#define ARRAY_SIZE 7
int main() {
  int arr[ARRAY_SIZE];

  arr[0] = 4;
  arr[1] = 9;
  arr[2] = 2;
  arr[3] = 2;
  arr[4] = 3;
  arr[5] = 1;
  arr[6] = 8;

  for (int i = 0; i < ARRAY_SIZE; ++i) {
    my_write(arr[i] + '0');
  }

  my_write('\n');

  mergeSort(arr, 0, ARRAY_SIZE - 1);
  // isSorted(arr, 7);

  for (int i = 0; i < ARRAY_SIZE; ++i) {
    my_write(arr[i] + '0');
  }

  return 0;
}
