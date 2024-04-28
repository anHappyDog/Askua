#include <types.h>

void *memcpy(void *dst, const void *src, size_t n) {
  void *dstaddr = dst;
  void *max = dst + n;

  if (((long)src & 3) != ((long)dst & 3)) {
    while (dst < max) {
      *(uint8_t *)dst++ = *(uint8_t *)src++;
    }
    return dstaddr;
  }

  while (((long)dst & 3) && dst < max) {
    *(char *)dst++ = *(char *)src++;
  }

  // copy machine words while possible
  while (dst + 4 <= max) {
    *(uint32_t *)dst = *(uint32_t *)src;
    dst += 4;
    src += 4;
  }

  // finish the remaining 0-3 bytes
  while (dst < max) {
    *(uint8_t *)dst++ = *(uint8_t *)src++;
  }
  return dstaddr;
}

void *memset(void *dst, int c, size_t n) {
  void *dstaddr = dst;
  void *max = dst + n;
  uint8_t byte = c & 0xff;
  uint32_t word = byte | byte << 8 | byte << 16 | byte << 24;

  while (((long)dst & 3) && dst < max) {
    *(uint8_t *)dst++ = byte;
  }

  // fill machine words while possible
  while (dst + 4 <= max) {
    *(uint32_t *)dst = word;
    dst += 4;
  }

  // finish the remaining 0-3 bytes
  while (dst < max) {
    *(uint8_t *)dst++ = byte;
  }
  return dstaddr;
}

size_t strlen(const char *s) {
  int n;

  for (n = 0; *s; s++) {
    n++;
  }

  return n;
}

char *strcpy(char *dst, const char *src) {
  char *ret = dst;

  while ((*dst++ = *src++) != 0) {
  }

  return ret;
}

const char *strchr(const char *s, int c) {
  for (; *s; s++) {
    if (*s == c) {
      return s;
    }
  }
  return 0;
}

int strcmp(const char *p, const char *q) {
  while (*p && *p == *q) {
    p++, q++;
  }

  if (*p < *q) {
    return -1;
  } else if (*p > *q) {
    return 1;
  }
  return 0;
}
