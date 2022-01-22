#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/types.h>

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
    // ファイルの末尾に文字列を追記
    int fd = open(file_path, O_RDWR | O_APPEND);
    if (fd < 0) {
        ERROR("FILE NOT FOUND");
        return FAILURE;
    }

    write(fd, data, strlen(data));

    if (close(fd) < 0) {
        ERROR("FILE CLOSE ERROR");
        return FAILURE;
    }

    // 追記した文字列が正しく追記されているか検査
    FILE *fp = fopen(file_path, "r");
    char file_str[MAX_BUFFER];

    while (fgets(file_str, MAX_BUFFER, fp) != NULL);
    int last_lign_length = strlen(file_str);

    if (strcmp(data, (file_str + last_lign_length) - strlen(data)) != 0) {
        fclose(fp);
        return FAILURE;
    }

    fclose(fp);
    return SUCCESS;
}

int overwrite_test(char *file_path, char *data)
{
    // ファイルを上書き
    int fd = open(file_path, O_WRONLY | O_TRUNC);
    if (fd < 0) {
        ERROR("FILE NOT FOUND");
        return FAILURE;
    }

    ftruncate(fd, 0);
    write(fd, data, strlen(data));

    if (close(fd) < 0) {
        ERROR("FILE CLOSE ERROR");
        return FAILURE;
    }

    // 上書きした文字列が書き込まれているか検査
    FILE *fp = fopen(file_path, "r");
    char c;
    int i = 0;
    while ((c = fgetc(fp)) != EOF) {
        if (c != data[i]) {
            fclose(fp);
            return FAILURE;
        }
        i++;
    }
    return SUCCESS;
}