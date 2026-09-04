#include <unistd.h>
#include <signal.h>
#include <stdlib.h>

static void handle_signal(int sig) {
    (void)sig;
    _exit(0);
}

int main(int argc, char *argv[]) {
    // Suppress unused parameter warnings
    (void)argc;
    (void)argv;

    // Gracefully handle termination signals
    struct sigaction sa;
    sa.sa_handler = handle_signal;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = 0;

    sigaction(SIGTERM, &sa, NULL);
    sigaction(SIGINT, &sa, NULL);
    sigaction(SIGHUP, &sa, NULL);

    // Sleep indefinitely with 0% CPU consumption
    while (1) {
        pause();
    }

    return 0;
}
