#include <iostream>
#include "helloworld2.h"
#include <stdio.h>
#include <stdlib.h>

#include <string.h>	
#include <sys/stat.h>
#include <dirent.h>
#include <sys/types.h>
using namespace std;
#include <functional>
// void listFiles(const std::string &path, std::function<void(const std::string &)> cb) {
//     if (auto dir = opendir(path.c_str())) {
//         while (auto f = readdir(dir)) {
//             if (!f->d_name || f->d_name[0] == '.') continue;
//             if (f->d_type == DT_DIR) 
//                 listFiles(path + f->d_name + "/", cb);

//             if (f->d_type == DT_REG)
//                 cb(path + f->d_name);
//         }
//         closedir(dir);
//     }
// }


void echo(const std::string& message){
    std::cout << message << std::endl;
}

class Random{
    public:
        union Payload{
            std::string config;
        };
};









#include <vector>
#include <map>
typedef std::map < string, string > config;
int main(){
    cout << "Hello World" <<endl;



    if (__cplusplus == 202101L) std::cout << "C++23";
    else if (__cplusplus == 202002L) std::cout << "C++20";
    else if (__cplusplus == 201703L) std::cout << "C++17";
    else if (__cplusplus == 201402L) std::cout << "C++14";
    else if (__cplusplus == 201103L) std::cout << "C++11";
    else if (__cplusplus == 199711L) std::cout << "C++98";
    else std::cout << "pre-standard C++." << __cplusplus;
    std::cout << "\n";

    std::vector<int> arraysss;
    // d[11] = 3;
    std::cout << arraysss[11] << std::endl;

    // helloworld();
    // int32_t a =24, b =3;
    // int32_t* c = (int32_t*)malloc(sizeof(a)); 
    // *c = a/b;
    // cout << *c << endl;
    // std::string s = "~/vendor/";
    // cout << s << endl;
    // free(c);

    // echo("helloooooooo");

    // DIR *dr;
    // struct dirent *en;
    // dr = opendir("../../Presentations"); //open all directory
    // if (dr) {
    //     while ((en = readdir(dr)) != NULL) {
    //         cout<<" \n"<<en->d_name; //print all directory name
    //     }
    //     closedir(dr); //close all directory
    // }

    // listFiles("../../Presentations", [](const std::string &path) {
    // std::cout << path << std::endl;
    // });
    
}