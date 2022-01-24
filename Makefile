TEST_PATH := ./tests
CC := /usr/bin/$(CC)
MOUNT_POINT := ./mountpoint

test: $(TEST_PATH)/test
	mkdir $(MOUNT_POINT)

	echo "Prepare config YAML files"
	cp ./development/config-template/attr.yaml ./tests/config/attr.yaml
	cp ./development/config-template/data.yaml ./tests/config/data.yaml
	cp ./development/config-template/entry.yaml ./tests/config/entry.yaml
	cp ./development/config-template/image.yaml ./tests/config/image.yaml

	RUST_LOG=debug ./target/debug/hfs --config-path ./tests/config/image.yaml --mountpoint $(MOUNT_POINT) &
	$(TEST_PATH)/test $(MOUNT_POINT)/file1
	touch $(MOUNT_POINT)/file2
	ls $(MOUNT_POINT)
	sudo umount $(MOUNT_POINT)
	rmdir $(MOUNT_POINT)

all: $(TEST_PATH)/main.o $(TEST_PATH)/error.o $(TEST_PATH)/write.o
	$(CC) -o $(TEST_PATH)/test $(TEST_PATH)/main.o $(TEST_PATH)/error.o $(TEST_PATH)/write.o
	cargo build

main.o: $(TEST_PATH)/main.c
	$(CC) -c -o $(TEST_PATH)/main.o $(TEST_PATH)/main.c

error.o: $(TEST_PATH)/error.c
	$(CC) -c -o $(TEST_PATH)/error.o $(TEST_PATH)/error.c

write.o: $(TEST_PATH)/write.c
	$(CC) -c -o $(TEST_PATH)/write.o $(TEST_PATH)/write.c

clean: $(TEST_PATH)/*.o
	rm $(TEST_PATH)/*.o

clean-all: $(TEST_PATH)/test $(TEST_PATH)/*.o
	rm $(TEST_PATH)/*.o $(TEST_PATH)/test