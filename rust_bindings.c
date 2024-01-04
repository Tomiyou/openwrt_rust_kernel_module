#include "rust_bindings.h"

/* Global variables */
const gfp_t RUST_GFP_KERNEL = GFP_KERNEL;

/* Spinlock */
void rust_helper_spin_lock_init(spinlock_t *lock) {
    spin_lock_init(lock);
}
EXPORT_SYMBOL(rust_helper_spin_lock_init);

#if LINUX_VERSION_CODE < KERNEL_VERSION(6,4,0)
void rust_helper_spin_lock(spinlock_t *lock) {
    spin_lock(lock);
}
EXPORT_SYMBOL(rust_helper_spin_lock);

void rust_helper_spin_unlock(spinlock_t *lock) {
    spin_unlock(lock);
}
EXPORT_SYMBOL(rust_helper_spin_unlock);

/* Mutex */
void rust_helper_mutex_init(struct mutex *lock) {
    mutex_init(lock);
}
EXPORT_SYMBOL(rust_helper_mutex_init);

void rust_helper_mutex_lock(struct mutex *lock) {
    mutex_lock(lock);
}
EXPORT_SYMBOL(rust_helper_mutex_lock);
#endif


// /* Printk */
// #if (LINUX_VERSION_CODE < KERNEL_VERSION(5, 15, 0))
// /* After Linux 5.15, printk() becomes an inline function */
// asmlinkage __visible int _printk(const char *fmt, ...)
// {
// 	va_list args;
// 	int r;

// 	va_start(args, fmt);
// 	r = vprintk(fmt, args);
// 	va_end(args);

// 	return r;
// }
// EXPORT_SYMBOL(_printk);
// #endif

MODULE_LICENSE("GPL");
