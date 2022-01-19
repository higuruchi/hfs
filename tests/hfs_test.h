#define SUCCESS     1
#define FAILURE     2

// error.c--------

void err_msg(const char *file, const char *function, int line, const char *type, const char *fmt, ...);

#define ERROR(fmt, ...) err_msg(__FILE__, __FUNCTION__, __LINE__, "error", fmt, ##__VA_ARGS__)
#define WARNNING(fmt, ...) err_msg(__FILE__, __FUNCTION__, __LINE__, "warnning", fmt, ##__VA_ARGS__)
// ---------------


// write.c--------
#define MODE_APPEND     1
#define MODE_OVERWRITE  2
#define MAX_BUFFER      256
int write_test(char *file_path, char *data,int mode);
// ---------------