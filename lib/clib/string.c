#include <string.h>
#include <types.h>

int memcmp(const void *s1, const void *s2, size_t n) {
  const unsigned char *p1 = (const unsigned char *)s1;
  const unsigned char *p2 = (const unsigned char *)s2;

  for (size_t i = 0; i < n; i++) {
    if (p1[i] != p2[i]) {
      return p1[i] - p2[i];
    }
  }

  return 0;
}

void *memmove(void *dest, const void *src, size_t n) {
  unsigned char *d = (unsigned char *)dest;
  const unsigned char *s = (const unsigned char *)src;

  if (d == s) {
    return dest;
  }

  if (d < s) {
    for (size_t i = 0; i < n; i++) {
      d[i] = s[i];
    }
  } else {
    for (size_t i = n; i != 0; i--) {
      d[i - 1] = s[i - 1];
    }
  }

  return dest;
}

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

char *strncpy(char *dst, const char *src, size_t n) {
  char *ret = dst;
  size_t i;
  for (i = 0; i < n && src[i] != '\0'; i++) {
    dst[i] = src[i];
  }
  dst[i] = '\0';
  return ret;
}

char *strchr(char *s, int c) {
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

int strncmp(const char *p, const char *q, size_t n) {
  while (n > 0 && *p && *p == *q) {
    n--, p++, q++;
  }
  if (n == 0) {
    return 0;
  }
  if (*p < *q) {
    return -1;
  }
  return 1;
}

void strcat(char *dst, const char *src) {
  while (*dst) {
    dst++;
  }
  while ((*dst++ = *src++) != 0)
    ;
}
