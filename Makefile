TEST_PATH := ./tests
CC := /usr/bin/$(CC)
MOUNT_POINT := ./mountpoint

test: $(TEST_PATH)/bin/test
	mkdir $(MOUNT_POINT)

	echo "Prepare config YAML files"
	cp ./development/config-template/attr.yaml ./tests/config/attr.yaml
	cp ./development/config-template/data.yaml ./tests/config/data.yaml
	cp ./development/config-template/entry.yaml ./tests/config/entry.yaml
	cp ./development/config-template/image.yaml ./tests/config/image.yaml

	# ./target/debug/hfs --config-path ./tests/config/image.yaml --mountpoint $(MOUNT_POINT) &
	RUST_LOG=debug ./target/debug/hfs --config-path ./tests/config/image.yaml --mountpoint $(MOUNT_POINT) &
	$(TEST_PATH)/bin/test $(MOUNT_POINT)/file1
	touch $(MOUNT_POINT)/file3
	ls $(MOUNT_POINT)
	sudo umount $(MOUNT_POINT)
	rmdir $(MOUNT_POINT)

all: $(TEST_PATH)/bin/main.o $(TEST_PATH)/bin/error.o $(TEST_PATH)/bin/write.o
	$(CC) -o $(TEST_PATH)/bin/test $(TEST_PATH)/bin/main.o $(TEST_PATH)/bin/error.o $(TEST_PATH)/bin/write.o
	cargo build

tests/bin/main.o: $(TEST_PATH)/main.c
	$(CC) -c -o $(TEST_PATH)/bin/main.o $(TEST_PATH)/main.c

tests/bin/error.o: $(TEST_PATH)/error.c
	$(CC) -c -o $(TEST_PATH)/bin/error.o $(TEST_PATH)/error.c

tests/bin/write.o: $(TEST_PATH)/write.c
	$(CC) -c -o $(TEST_PATH)/bin/write.o $(TEST_PATH)/write.c

clean: $(TEST_PATH)/*.o $(TEST_PATH)/bin/test
	rm $(TEST_PATH)/bin/*.o
	rm $(TEST_PATH)/bin/test