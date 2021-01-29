#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/prctl.h>
#include <sys/signal.h>
#include <sys/time.h>
#include <sys/wait.h>
#include <unistd.h>

#define NONRET __attribute__((noreturn))
#define UNUSED __attribute__((unused))

#define TIMER_INTERVAL_US 500000

static void signal_alarm(int unused) { puts("SIG"); }

int main() {
  prctl(PR_SET_PDEATHSIG, SIGKILL);
  int pid = fork();
  if (pid) {
    struct sigaction sa;
    bzero(&sa, sizeof(sa));
    sa.sa_handler = signal_alarm;
    signal(SIGALRM, signal_alarm);
    struct itimerval timer = {
        .it_interval = {.tv_usec = TIMER_INTERVAL_US},
        .it_value = {.tv_usec = TIMER_INTERVAL_US},
    };
    setitimer(ITIMER_REAL, &timer, NULL);
    printf("PID: %d\n", pid);
    int status;
    while (1) {
      printf("Waiting\n");
      wait4(pid, &status, 0, NULL);
      if (errno != EINTR) {
        printf("Stopped with %d\n", status);
        break;
      }
    }
  } else {
    sleep(3);
  }
}
