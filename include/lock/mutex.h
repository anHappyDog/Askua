#ifndef __MUTEX_H_
#define __MUTEX_H_

typedef struct {

} mutex_t;

static inline int mutex_trylock(mutex_t *lock) { return 0; }

static inline void mutex_lock(mutex_t *lock) {}

static inline int mutex_is_locked(mutex_t *lock) { return 0; }

static inline void mutex_unlock(mutex_t *lock) {}

#endif // __MUTEX_H_