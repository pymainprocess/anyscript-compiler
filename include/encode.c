#include "encode.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

const char *encode(const char *string) {
    // Erstelle einen leeren String
    char* result = (char*)malloc(1024 * sizeof(char));
    result[0] = '\0'; // Initialisiere den leeren String

    // SPeicher fuer das aktuelle zeicher
    char encoded_char;
    char buffer[4];

    // Teile den codierten inhalt in teile
    const char* delimiter = " ";
    char* content_copy = strdup(string);
    char* token = strtok(content_copy, delimiter);

    while (token != NULL) {
        // Bestimme die intsprechenden Zeichen
        if (strcmp(token, "1") == 0) encoded_char = 'a';
        else if (strcmp(token, "2") == 0) encoded_char = 'b';
        else if (strcmp(token, "3") == 0) encoded_char = 'c';
        else if (strcmp(token, "4") == 0) encoded_char = 'd';
        else if (strcmp(token, "5") == 0) encoded_char = 'e';
        else if (strcmp(token, "6") == 0) encoded_char = 'f';
        else if (strcmp(token, "7") == 0) encoded_char = 'g';
        else if (strcmp(token, "8") == 0) encoded_char = 'h';
        else if (strcmp(token, "9") == 0) encoded_char = 'i';
        else if (strcmp(token, "10") == 0) encoded_char = 'j';
        else if (strcmp(token, "11") == 0) encoded_char = 'k';
        else if (strcmp(token, "12") == 0) encoded_char = 'l';
        else if (strcmp(token, "13") == 0) encoded_char = 'm';
        else if (strcmp(token, "14") == 0) encoded_char = 'n';
        else if (strcmp(token, "15") == 0) encoded_char = 'o';
        else if (strcmp(token, "16") == 0) encoded_char = 'p';
        else if (strcmp(token, "17") == 0) encoded_char = 'q';
        else if (strcmp(token, "18") == 0) encoded_char = 'r';
        else if (strcmp(token, "19") == 0) encoded_char = 's';
        else if (strcmp(token, "20") == 0) encoded_char = 't';
        else if (strcmp(token, "21") == 0) encoded_char = 'u';
        else if (strcmp(token, "22") == 0) encoded_char = 'v';
        else if (strcmp(token, "23") == 0) encoded_char = 'w';
        else if (strcmp(token, "24") == 0) encoded_char = 'x';
        else if (strcmp(token, "25") == 0) encoded_char = 'y';
        else if (strcmp(token, "26") == 0) encoded_char = 'z';
        else if (strcmp(token, "00") == 0) encoded_char = '0';
        else if (strcmp(token, "001") == 0) encoded_char = '1';
        else if (strcmp(token, "002") == 0) encoded_char = '2';
        else if (strcmp(token, "003") == 0) encoded_char = '3';
        else if (strcmp(token, "004") == 0) encoded_char = '4';
        else if (strcmp(token, "005") == 0) encoded_char = '5';
        else if (strcmp(token, "006") == 0) encoded_char = '6';
        else if (strcmp(token, "007") == 0) encoded_char = '7';
        else if (strcmp(token, "008") == 0) encoded_char = '8';
        else if (strcmp(token, "009") == 0) encoded_char = '9';
        else if (strcmp(token, "0") == 0) encoded_char = '\n';
        else encoded_char = "?";

        snprintf(buffer, sizeof(buffer), "%c", encoded_char);
        strcat(result, buffer);

        token = strtok(NULL, delimiter);
    }

    free(content_copy);
    return result;
}