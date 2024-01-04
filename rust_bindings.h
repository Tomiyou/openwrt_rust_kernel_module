#include <linux/version.h>
#include <linux/printk.h>
#include <linux/fs.h>
#include <linux/mutex.h>
#include <linux/netdevice.h>
#include <linux/module.h>

/* Spinlock */
void rust_helper_spin_lock_init(spinlock_t *lock);
void rust_helper_spin_lock(spinlock_t *lock);
void rust_helper_spin_unlock(spinlock_t *lock);

/* Mutex */
void rust_helper_mutex_init(struct mutex *lock);
void rust_helper_mutex_lock(struct mutex *lock);
void rust_helper_mutex_unlock(struct mutex *lock);
