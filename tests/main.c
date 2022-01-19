#include <stdio.h>
#include "hfs_test.h"
#include <unistd.h>

int main(int argc, char *argv[])
{
    if (write_test("/home/higuruchi/fuse-exp/data/hfs/tests/test.txt", "test1", MODE_APPEND) == SUCCESS) {
        puts("success");
    } else {
        puts("fail1");
    }

    sleep(5);

    if (write_test("/home/higuruchi/fuse-exp/data/hfs/tests/test.txt", "test2", MODE_OVERWRITE) == SUCCESS) {
        puts("success");
    } else {
        puts("fail2");
    }
    return 0;
}