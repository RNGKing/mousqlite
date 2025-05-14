use anyhow::Result;
use mousqlite_types::SqlRequest;

pub struct Process{
    pub PID : String,
    pub running_child : tokio::process::Child,
    pub active : bool,
}

pub struct ProcessManager{
    process_map: std::collections::HashMap<String, Process>
}

impl ProcessManager{
    pub fn new() -> ProcessManager{
        ProcessManager{
            process_map: std::collections::HashMap::new(),
        }
    }

    pub fn try_query(mut self, db_hash : String, req : SqlRequest ) -> Result<()>{
        // check to see if the process exists
        // if not, start it and enqueue the query
        // 
        Ok(())
    }
}

impl Default for ProcessManager{
    fn default() -> Self {
        ProcessManager::new()
    }
}

