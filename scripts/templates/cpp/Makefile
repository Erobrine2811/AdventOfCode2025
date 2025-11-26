CXX=g++
CXXFLAGS=-std=c++17 -Wall -O2

.PHONY: all clean part1 part2

all: part1exec part2exec

part1exec: part1.cpp
	$(CXX) $(CXXFLAGS) -o part1 part1.cpp

part2exec: part2.cpp
	$(CXX) $(CXXFLAGS) -o part2 part2.cpp

part1: part1exec
part2: part2exec

clean:
	rm -f part1 part2