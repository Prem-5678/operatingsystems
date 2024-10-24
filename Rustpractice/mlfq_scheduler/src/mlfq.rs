// Process struct represents a process in the MLFQ scheduler
#[derive(Clone)] // Allow cloning of Process for use in priority boosting
pub struct Process {
    pub id: u32,             // Unique identifier for the process
    pub priority: usize,     // Current priority level (queue index) of the process
    pub remaining_time: u32, // Remaining time for the process to complete
    pub total_executed_time: u32, // Total time the process has been executed so far
}

// Multi-Level Feedback Queue (MLFQ) scheduler structure
pub struct MLFQ {
    queues: Vec<Vec<Process>>, // Vector of queues (each queue is a vector of processes)
    num_levels: usize,         // Number of priority levels (queues)
    time_quanta: Vec<u32>,     // Time quanta for each priority level (queue)
    current_time: u32,         // Total time elapsed in the system
}

impl MLFQ {
    // Constructor for the MLFQ struct
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels], // Initialize empty queues for each level
            num_levels,
            time_quanta,
            current_time: 0, // Start with zero elapsed time
        }
    }

    // Add a new process to the appropriate queue based on its priority
    pub fn add_process(&mut self, mut process: Process) {
        // If priority is out of range, adjust it to the lowest priority queue
        if process.priority >= self.num_levels {
            process.priority = self.num_levels - 1;
        }
        // Push the process into the corresponding priority queue
        self.queues[process.priority].push(process);
    }

    // Simulate the execution of a process from a given queue (specified by queue_index)
    pub fn execute_process(&mut self, queue_index: usize) {
        // Check if the queue_index is valid and if there is a process to execute
        if queue_index >= self.num_levels || self.queues[queue_index].is_empty() {
            println!("No process to execute in queue {}", queue_index);
            return;
        }

        // Remove the process from the front of the current queue
        let mut process = self.queues[queue_index].remove(0);
        // Determine the time quantum for this priority level
        let time_quantum = self.time_quanta[queue_index];
        // Calculate how much time the process will run (min of remaining time or time quantum)
        let actual_runtime = process.remaining_time.min(time_quantum);

        // Update the process's remaining time and its total executed time
        process.remaining_time -= actual_runtime;
        process.total_executed_time += actual_runtime;
        // Update the system's current time
        self.current_time += actual_runtime;

        // Store the process id and remaining time before moving it (due to Rust ownership rules)
        let process_id = process.id;
        let remaining_time = process.remaining_time;

        // Log the process execution
        println!(
            "Executed process {} for {} time units from queue {}",
            process_id, actual_runtime, queue_index
        );

        // If the process still has remaining time, it needs to be placed back in a queue
        if remaining_time > 0 {
            // Move the process to the next lower-priority queue if not at the lowest level
            if queue_index < self.num_levels - 1 {
                process.priority = queue_index + 1; // Increase its priority (move down in queues)
                self.queues[queue_index + 1].push(process);
                // Log that the process was moved to a lower-priority queue
                println!(
                    "Process {} moved to queue {} with {} remaining time units",
                    process_id, queue_index + 1, remaining_time
                );
            } else {
                // If the process is in the lowest queue, keep it there
                self.queues[queue_index].push(process);
                // Log that the process stayed in the same queue
                println!(
                    "Process {} stayed in queue {} with {} remaining time units",
                    process_id, queue_index, remaining_time
                );
            }
        } else {
            // If the process is complete, log its completion
            println!("Process {} completed execution", process_id);
        }
    }

    // Perform a priority boost: move all processes to the highest priority queue
    pub fn priority_boost(&mut self) {
        println!("Priority boost triggered, moving all processes to highest priority queue");

        let mut boosted_processes = Vec::new(); // Temporary list to hold boosted processes

        // Iterate through all queues except the highest priority (queue 0)
        for queue_index in 1..self.num_levels {
            // Move processes from lower priority queues to the highest priority queue
            while let Some(mut process) = self.queues[queue_index].pop() {
                // Store process ID for logging before moving it
                let process_id = process.id;
                // Set process priority to 0 (highest priority)
                process.priority = 0;
                boosted_processes.push(process); // Temporarily store the boosted process
                // Log the boost
                println!("Process {} moved to queue 0 during priority boost", process_id);
            }
        }

        // Add all boosted processes to the highest priority queue
        self.queues[0].extend(boosted_processes);
    }

    // Simulate time passing and trigger a priority boost if necessary
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time; // Update the current time in the system
        let boost_interval = 100; // Define the time interval after which a priority boost occurs
        if self.current_time % boost_interval == 0 {
            self.priority_boost(); // Trigger priority boost when boost interval is reached
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test to ensure processes are added to the correct queue based on priority
    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]); // Create an MLFQ with 3 levels and time quanta

        // Define three processes with different priorities
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1); // Add process1 to the MLFQ
        mlfq.add_process(process2); // Add process2 to the MLFQ
        mlfq.add_process(process3); // Add process3 (priority out of range, should be moved to lowest queue)

        // Check if the processes are added to the correct queues
        assert_eq!(mlfq.queues[0].len(), 1); // process1 should be in queue 0
        assert_eq!(mlfq.queues[1].len(), 1); // process2 should be in queue 1
        assert_eq!(mlfq.queues[2].len(), 1); // process3 should be in the lowest queue
    }

    // Test the execution of a process and verify it moves to the next queue
    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]); // Create an MLFQ with 3 levels and time quanta
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0); // Execute process from queue 0

        // Check that the process has been moved to the next queue (queue 1)
        assert_eq!(mlfq.queues[0].len(), 0); // Queue 0 should be empty now
        assert_eq!(mlfq.queues[1].len(), 1); // Process should be in queue 1
        assert_eq!(mlfq.queues[1][0].remaining_time, 3); // Remaining time should be 3
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2); // Total executed time should be 2
    }

    
    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]); // Create an MLFQ with 3 levels
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });
    
        mlfq.update_time(100); // Trigger a priority boost 

        assert_eq!(mlfq.queues[0].len(), 2); // Both processes should now be in queue 0
        assert_eq!(mlfq.queues[1].len(), 0); // Queue 1 should be empty
        assert_eq!(mlfq.queues[2].len(), 0); // Queue 2 should be empty
    }
    
    
    }