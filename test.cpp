#include "dlfcn.h"
#include "iostream"
typedef int (*func)(int &&, int &&);
int main()
{
    void *handle = dlopen("./librt.so", RTLD_LAZY);
    if (!handle)
    {
        std::cout << "error" << std::endl;
        return 1;
    }
    func myfunc = (func)dlsym(handle, "add");
    std::cout << myfunc(1, 2) << std::endl;
    return 0;
}