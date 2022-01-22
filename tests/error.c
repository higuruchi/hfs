#include <stdio.h>
#include <stdarg.h>
#include <string.h>
#include <errno.h>
#include <time.h>
#include <sys/types.h>
#include <unistd.h>

#include "hfs_test.h"

void
err_msg(const char *file, const char *function, int line, const char *type, const char *fmt, ...)
{
	time_t timer;
	time(&timer);
	char *t = ctime(&timer);
	t[strlen(t)-1] = '\0';
	fprintf(stderr, "%s ", t);

	fprintf(stderr, "%d ", getpid());
	fprintf(stderr, "%s %s %d %s ", file, function, line, type);

	va_list ap;
	va_start(ap, fmt);
	vfprintf(stderr, fmt, ap);
	va_end(ap);

	fprintf(stderr, " %s(%d)", strerror(errno), errno);
	fputc('\n', stderr);
}
