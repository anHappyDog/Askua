#ifndef __WAIT_H__
#define __WAIT_H__
#include <proc/task.h>
#include <types.h>

struct wait_queue {
  struct task_struct *task;
  struct wait_queue *next;
};

#endif // __WAIT_H__