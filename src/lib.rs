#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{U256, Address},
    msg,
};
use alloc::{collections::BTreeMap, vec::Vec, string::String};
use alloc::format;

// Define the TaskMarketplace struct manually
pub struct TaskMarketplace {
    tasks: BTreeMap<U256, (Address, Address, Vec<u8>, U256, bool)>,
    task_count: U256,
}

impl TaskMarketplace {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_count: U256::from(0),
        }
    }

    pub fn create_task(&mut self, description: Vec<u8>, reward: U256) -> Result<U256, Vec<u8>> {
        let task_id = self.task_count + U256::from(1);
        self.tasks.insert(task_id, (msg::sender(), Address::ZERO, description, reward, false));
        self.task_count = task_id;
        Ok(task_id)
    }

    pub fn accept_task(&mut self, task_id: U256) -> Result<(), Vec<u8>> {
        let (creator, assignee, description, reward, completed) = self.tasks.get(&task_id).ok_or("Task not found")?;
        if !assignee.is_zero() {
            return Err("Task already assigned".into());
        }
        self.tasks.insert(task_id, (*creator, msg::sender(), description.clone(), *reward, *completed));
        Ok(())
    }

    pub fn complete_task(&mut self, task_id: U256) -> Result<(), Vec<u8>> {
        let (creator, assignee, description, reward, completed) = self.tasks.get(&task_id).ok_or("Task not found")?;
        if *assignee != msg::sender() {
            return Err("Not assigned to this task".into());
        }
        if *completed {
            return Err("Task already completed".into());
        }
        self.tasks.insert(task_id, (*creator, *assignee, description.clone(), *reward, true));

        // Commented out msg::send_value for now, as it's not available
        // Transfer reward to assignee
        // msg::send_value(assignee, reward);

        Ok(())
    }

    pub fn get_task(&self, task_id: U256) -> Result<String, Vec<u8>> {
        let (creator, assignee, description, reward, completed) = self.tasks.get(&task_id).ok_or("Task not found")?;
        Ok(format!(
            "{{\"creator\":\"{}\",\"assignee\":\"{}\",\"description\":\"{}\",\"reward\":\"{}\",\"completed\":{}}}",
            creator,
            assignee,
            String::from_utf8_lossy(description).replace("\"", "\\\""),  // Updated for Vec<u8>
            reward,
            completed
        ))
    }

    pub fn get_all_tasks(&self) -> String {
        let mut all_tasks = String::from("[");
        let task_count = self.task_count;
        let mut first = true;
    
        // Fix for task_count iteration
        for i in 1..=task_count.as_limbs()[0] {
            if let Some((creator, assignee, description, reward, completed)) = self.tasks.get(&U256::from(i)) {
                if !first {
                    all_tasks.push_str(",");
                } else {
                    first = false;
                }
                all_tasks.push_str(&format!(
                    "{{\"id\":\"{}\",\"creator\":\"{}\",\"assignee\":\"{}\",\"description\":\"{}\",\"reward\":\"{}\",\"completed\":{}}}",
                    i,
                    creator,
                    assignee,
                    String::from_utf8_lossy(description).replace("\"", "\\\""),  // Updated for Vec<u8>
                    reward,
                    completed
                ));
            }
        }
        all_tasks.push_str("]");
        all_tasks
    }
}    