#include <iostream>
#include <thread>
#include <chrono>

void task1(){
    for (int i = 0; i < 5; i++){
        std::cout << "Task 1 - Iteration: " << i << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(200));
    }
}

void task2(){
    for (int i = 0; i < 5; i++){
        std::cout << "Task 2 - Iteration: " << i << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(500));
    }
}

int main(){
    std::thread thread1(task1);
    std::thread thread2(task2);

    thread1.join();
    thread2.join();
}