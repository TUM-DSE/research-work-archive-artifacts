/* String in .rodata */
static const char s[] = "Hello, world!!!!!!!!!\n";
/* Pointer in .data, string in .rodata */
static const unsigned long long a = 1337;
static const unsigned long long b = 0x7272;
static const unsigned long long c = 0xCAFEBABECAFEBABE;
static const char* s2 = "Hello, world!!";

int main() {

  return 42;
}

