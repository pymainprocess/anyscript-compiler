#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "decode.h"

char* decode(const char* string) {
    size_t len = strlen(string);
    char* result = (char*)malloc(len * 4 + 1); // Puffer fr das Ergebnis
    if (!result) return NULL;

    char* p = result;
    for (size_t i = 0; i < len; ++i) {
        char c = string[i];
        if (c >= 'a' && c <= 'z') {
            p += sprintf(p, "%d", c - 'a' + 1);
        } else if (c >= 'A' && c <= 'Z') {
            p += sprintf(p, "%d", c - 'A' + 1);
        } else if (c >= '0' && c <= '9') {
            p += sprintf(p, "00%c", c);
        } else {
            *p++ = c;
        }
        *p++ = ' ';
    }
    *(p - 1) = '\0'; // Letztes Leerzeichen durch Nullterminator ersetzen

    return result;
}