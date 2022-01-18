#include <stdio.h>
#include "hfs_test.h"

int main(int argc, char *argv[])
{
    if (write_test("/home/higuruchi/fuse-exp/mountpoint", "test", MODE_APPEND) == SUCCESS) {
        puts("success");
    } else {
        puts("fail");
    }
    return 0;
}