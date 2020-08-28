#include <semaphore.h>

typedef struct Rwlock {
    sem_t lock; // 普通锁
    sem_t writelock; // 写锁
    int readers;
} Rwlock;

void rwlock_init(Rwlock* rw) {
    sem_init(&rw->lock, 0, 1);
    sem_init(&rw->writelock, 0, 1);
    rw->readers = 0;
}

void rwlock_acquire_readlock(Rwlock* rw) {
    sem_wait(&rw->lock);
    rw->readers++;
    if (rw->readers == 1) {
        sem_wait(&rw->writelock);
    }
    sem_post(&rw->lock);
}

void rwlock_release_readlock(Rwlock* rw) {
    sem_wait(&rw->lock);
    rw->readers--;
    if (rw->readers == 0) {
        sem_post(&rw->writelock);
    }
    sem_post(&rw->lock);
}

void rwlock_require_writelock(Rwlock* rw) {
    sem_wait(&rw->writelock);
}

void rwlock_release_writelock(Rwlock* rw) {
    sem_post(&rw->writelock);
}

