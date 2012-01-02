all: mm3

mm3: main.o MurmurHash3.o
	g++ main.o MurmurHash3.o -o mm3

main.o: main.cpp
	g++ -c main.cpp

MurmurHash3.o: MurmurHash3.cpp
	g++ -c MurmurHash3.cpp

clean:
	rm -rf *.o mm3
