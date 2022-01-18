#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <string.h>

#include "hfs_test.h"

int append_test(char *file_path, char *data);
int overwrite_test(char *file_path, char *data);

int write_test(char *file_path, char *data, int mode)
{
    if (mode == MODE_APPEND) {
        return append_test(file_path, data);
    }

    if (mode == MODE_OVERWRITE) {
        return overwrite_test(file_path, data);
    }
}

int append_test(char *file_path, char *data)
{
    FILE *fp = fopen(file_path, "a");
    if (fp == NULL) {
        ERROR("FILE NOT FOUND");
        return FAILURE;
    }
    fprintf(fp, "%s", data);

    fp = fopen(file_path, "r");
    char file_str[MAX_BUFFER];

    while (fgets(file_str, MAX_BUFFER, fp) != NULL);

    if (strcmp(data, file_str) != 0) {
        fclose(fp);
        return FAILURE;
    }

    fclose(fp);
    return SUCCESS;
}

int overwrite_test(char *file_path, char *data)
{
    // TODO
    return SUCCESS;
    FILE *fd = fopen(file_path, "r+");
    if (fd == NULL) {
        ERROR("FILE NOT FOUND");
        return FAILURE;
    }
}