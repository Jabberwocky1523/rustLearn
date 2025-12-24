#include "dlfcn.h"
#include "iostream"
typedef int (*func)();
int main()
{
    void *handle = dlopen("./librt.so", RTLD_LAZY);
    if (!handle)
    {
        std::cout << "error" << std::endl;
        return 1;
    }
    func myfunc = (func)dlsym(handle, "open");
    std::cout << myfunc();
    return 0;
}