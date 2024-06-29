#ifndef DECODE_H
#define DECODE_H
// Wir schreiben den Code um und codieren es in Zahlen um
// Das sieht so aus, dass jede Zahl die Stelle des Alphabets entspricht mit dem Splitter 0
// 0 steht fr die Trennung zwischen den verschiedenen Zeilen
// 1 steht fr a
// 2 steht fr b
// usw...
// Sonderzeichen bleiben wo sie sind, das Resultat ist ein Code-String mit dem wir arbeiten
// Es gibt 26, 1 bis 26 sind Buchstaben,
// vor und nach Zahlen kommen 00, 0 steht fr nächster Buchstabe, aber 00 steht nur dafr, dass es eine Zahl und kein Buchstabe ist
// Alle Zahlen nach 27 sind ungltig, heißt dann ist der String Invalid, der String dient dazu, den String zu codieren, also besser fr das Programm hier
// Jetzt mssen wir den Content so splitten und speichern, dass es in einem Vektor ist, wo immer exakt ein Zeichen drinnen ist.
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

char* decode(char* string);

#endif