#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Sleeping,
    Exited,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: usize,
    pub stack_pointer: usize,
    pub program_counter: usize,
    pub state: TaskState,
}

impl Task {
    pub const fn new(id: usize, stack_pointer: usize, program_counter: usize) -> Self {
        Self {
            id,
            stack_pointer,
            program_counter,
            state: TaskState::Ready,
        }
    }
}

pub struct Scheduler<const N: usize> {
    tasks: [Option<Task>; N],
    cursor: usize,
}

impl<const N: usize> Scheduler<N> {
    pub const fn new() -> Self {
        Self {
            tasks: [None; N],
            cursor: 0,
        }
    }

    pub fn add(&mut self, task: Task) -> Result<(), TaskError> {
        for slot in &mut self.tasks {
            if slot.is_none() {
                *slot = Some(task);
                return Ok(());
            }
        }
        Err(TaskError::Full)
    }

    pub fn next_runnable(&mut self) -> Option<Task> {
        for _ in 0..N {
            let index = self.cursor % N;
            self.cursor = (self.cursor + 1) % N;
            if let Some(mut task) = self.tasks[index] {
                if task.state == TaskState::Ready || task.state == TaskState::Running {
                    task.state = TaskState::Running;
                    self.tasks[index] = Some(task);
                    return Some(task);
                }
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskError {
    Full,
}

