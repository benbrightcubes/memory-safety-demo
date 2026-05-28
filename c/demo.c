// demo.c — use-after-free in C
//
// Klassieke memory-safety bug:
//   1. malloc een buffer en gebruik hem
//   2. free de buffer
//   3. lees nog steeds via de oude pointer
//
// De geheugen-slot wordt vaak meteen hergebruikt door een volgende
// malloc — wat je leest is dan gecorrumpeerde data van iemand anders.
// Soms crasht het, soms "werkt" het maar leest geheugen van een
// andere allocatie, soms levert het een security-exploit op.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* get_username(int user_id) {
    char* buffer = malloc(64);
    snprintf(buffer, 64, "user_%d", user_id);
    return buffer;  // caller is verantwoordelijk voor free()
}

int main(void) {
    char* name = get_username(42);
    printf("Eerste read:  %s\n", name);

    free(name);  // bewust vrijgegeven

    // Hergebruik van de slot — vaak krijgt malloc dezelfde
    // geheugen-adres terug die we net hebben vrijgegeven
    char* other = malloc(64);
    snprintf(other, 64, "iets_anders_%d", 999);

    // BUG: name is freed, deze read is undefined behavior
    printf("Tweede read:  %s\n", name);

    free(other);
    return 0;
}
