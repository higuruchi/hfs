exec_test: test
	./test

test: main.o error.o write.o
	gcc -o test main.o error.o write.o

main.o: main.c
	gcc -c main.c

error: error.c
	gcc -c error.c

write: write.c
	gcc -c write.c

clean:
	rm *.o
