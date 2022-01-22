#include <stdio.h>
#include "hfs_test.h"
#include <unistd.h>

int main(int argc, char *argv[])
{
    if (argc != 2) {
        ERROR("MOUNT POINT PATH NOT FOUND");
        return 1;
    }

    if (write_test(argv[1], "test1", MODE_APPEND) == SUCCESS) {
        puts("success");
    } else {
        puts("fail1");
    }

    sleep(5);

    if (write_test(argv[1], "test1", MODE_OVERWRITE) == SUCCESS) {
        puts("success");
    } else {
        puts("fail2");
    }
    return 0;
}