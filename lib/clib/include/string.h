#ifndef __STRING_H_
#define __STRING_H_

#include <types.h>

void *memmove(void *dest, const void *src, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
void *memcpy(void *dst, const void *src, size_t n);
void *memset(void *dst, int c, size_t n);
size_t strlen(const char *s);
char *strcpy(char *dst, const char *src);
char *strncpy(char *dst, const char *src, size_t n);
char *strchr(char *s, int c);
int strcmp(const char *p, const char *q);
int strncmp(const char *p, const char *q, size_t n);
void strcat(char *dst, const char *src);

#endif
