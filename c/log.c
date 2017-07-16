#include <string.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <cubeb.h>

typedef enum { false, true } bool;

void (*string_callback)(char *);

void variadic_to_string(const char *fmt, ...) {
    if (string_callback == NULL) {
        return;
    }

    // Allocate a buffer on the stack that's big enough for us almost
    // all the time.  Be prepared to allocate dynamically if it doesn't fit.
    va_list ap;
    va_start(ap, fmt);

    size_t characters = 64;
    char stackbuf[characters];
    char* stackbufptr = &stackbuf[0];
    char* heapbufptr;
    char* selectedbufptr = stackbufptr;
    int finished = false;

    while (!finished) {
        // Try to vsnprintf into our buffer.
        va_list ap_copy;
        va_copy(ap_copy, ap);

        int needed = vsnprintf(selectedbufptr, characters, fmt, ap_copy);

        va_end(ap_copy);

        // NB. C99 (which modern Linux and OS X follow) says vsnprintf
        // failure returns the length it would have needed.  But older
        // glibc and current Windows return -1 for failure, i.e., not
        // telling us how much was needed.
        if (needed <= (int) characters && needed >= 0) {
            finished = true;
        } else {
            if (heapbufptr == selectedbufptr) {
                free(heapbufptr);
            }

            // vsnprintf reported that it wanted to write more characters
            // than we allotted.  So try again using a dynamic buffer.  This
            // doesn't happen very often if we chose our initial size well.
            characters = (needed > 0) ? (needed + 1) : (characters * 2);
            heapbufptr = malloc(characters * sizeof(char));
            selectedbufptr = heapbufptr;
        }
    }

    if (heapbufptr == selectedbufptr) {
        free(heapbufptr);
    }

    va_end(ap);

    (*string_callback)(selectedbufptr);
}

int set_log_callback_c(cubeb_log_level log_level, void (*new_string_callback)(char *)) {
    string_callback = new_string_callback;
    return cubeb_set_log_callback(log_level, &variadic_to_string);
}
