pub mod priority;
pub mod round_robin;

pub enum Scheduler {
    RoundRobin(round_robin::RoundRobinScheduler),
    Priority(priority::PriorityScheduler),
}
