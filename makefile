hello : test.o helloworld2.o
	g++ test.o helloworld2.o -o hello

test.o: test.cpp helloworld2.h
	g++ -c test.cpp	

helloworld2.o: helloworld2.cpp
	g++ -c helloworld2.cpp

clean:
	rm *.o hello