// pub use std::{ io::{Write, BufReader}, path::Path, fs::File };
// pub use chrono::{ DateTime, Local };
// pub use serde::{ Serialize, Deserialize };
// pub use colored::*;

// #[derive(Serialize, Deserialize)]
// pub enum Priority 
// {
//   Low,
//   Medium,
//   High,
// }

// impl Priority 
// {
// 	pub fn to_string(&self) -> ColoredString 
// 	{
// 		match self 
// 		{
// 			Self::Low => "Low".to_string().green(),
// 			Self::Medium => "Medium".to_string().yellow(),
// 			Self::High => "High".to_string().red(),
// 		}
// 	}
// }

// #[derive(Serialize, Deserialize)]
// pub struct Task 
// {
//   pub name: String,
//   pub description: String,
//   pub priority: Priority,
//   pub date: DateTime<Local>,
// }

// impl Task 
// {
// 	pub fn new(name: String, description: String, priority: Priority) -> Self 
// 	{
// 		Self 
// 		{ 
// 			name, 
// 			description, 
// 			priority, 
// 			date: Local::now() 
// 		}
// 	}

// 	pub fn new_console_task() -> Self 
// 	{
// 		let name = ConsoleManager::input("Enter new task name: ").unwrap();
// 		let description = ConsoleManager::input("Enter new task description: ").unwrap();
// 		let priority = match ConsoleManager::input("Enter new task priority: ").unwrap().to_lowercase().as_str() 
// 		{
// 			"low" => Priority::Low,
// 			"medium" => Priority::Medium,
// 			"high" => Priority::High,
// 			_ => 
// 			{
// 				println!("Invalid priority, setting to low");
// 				Priority::Low
// 			}
// 		};

// 		Self::new(name, description, priority)
// 	}

// 	pub fn print_task(&self) 
// 	{
// 		println!("Date: {}\nName: {}\nDescription: {}\nPriority: {}", 
// 		self.date.format("%d-%m-%Y %H:%M").to_string().italic(), 
// 		self.name.italic(), 
// 		self.description.italic(), 
// 		self.priority.to_string());
// 	}
// }

// pub struct TaskManager 
// {
// 	pub tasks: Vec<Task>,
// }

// impl TaskManager 
// {
	
// 	pub fn new() -> Self 
// 	{
// 		Self 
// 		{ 
// 			tasks: vec![] 
// 		}
// 	}


// 	pub fn print_tasks(&self) 
// 	{
// 		for task in &self.tasks
// 		{
// 			println!("{:-^22}", "TASK".magenta());
// 			task.print_task();
// 		}
// 	}


// 	pub fn add_task(&mut self, task: Task)
// 	{
// 		self.tasks.push(task);
// 	}


// 	pub fn find_task(&self, name: &str) -> Option<usize> 
// 	{
// 		self.tasks.iter().position(|task| task.name == name)
// 	}


// 	pub fn remove_task(&mut self, name: &str) -> Result<String, String>
// 	{
// 		if let Some(index) = self.find_task(name)  
// 		{
// 			self.tasks.remove(index);
// 			Ok(format!("Task \"{}\" removed successfully", name))
// 		}
// 		else 
// 		{
// 			Err(format!("Task with name \"{}\" doesn't exist", name))
// 		}
// 	}


// 	pub fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String>
// 	{
// 		if let Some(index) = self.find_task(name)  
// 		{
// 			match self.tasks.get_mut(index) 
// 			{
// 				Some(task) => 
// 				{
// 					task.name = updated_task.name;
// 					task.description = updated_task.description;
// 					task.priority = updated_task.priority;
// 					Ok(format!("Task \"{}\" updated successfully", name))
// 				},
// 				None => Err("Error borrowing task".to_string()),
// 			}
// 		}
// 		else 
// 		{
// 			Err(format!("Task with name \"{}\" doesn't exist", name))
// 		}
// 	}

// 	pub fn write_in_file(&self, filename: &str) -> Result<String, String>
// 	{
// 		if !Path::new(filename).exists()
// 		{
// 			let file = match File::create(filename) 
// 			{
// 				Ok(file) => file,
// 				Err(e) => { return Err(format!("Error creating file: {e}")); }
// 			};

// 			match serde_json::to_writer(&file, &self.tasks)
// 			{
// 				Ok(_) => Ok("Data stored successfully".to_string()),
// 				Err(e) => Err(format!("Error saving data: {e}"))
// 			}
// 		}
// 		else 
// 		{
// 			Err(format!("File \"{filename}\" already exists"))
// 		}
// 	}

// 	pub fn read_from_file(&mut self, filename: &str) -> Result<String, String>
// 	{
// 		if Path::new(filename).exists()
// 		{
// 			let file = match File::open(filename) 
// 			{
// 				Ok(file) => file,
// 				Err(e) => { return Err(format!("Error creating file: {e}")); }
// 			};

// 			let reader = BufReader::new(file);
// 			self.tasks = match serde_json::from_reader(reader) 
// 			{
// 				Ok(data) => data,
// 				Err(_) => { return Err("Error reading data".to_string()); }
// 			};
// 			Ok("Data read successfully".to_string())
// 		}
// 		else 
// 		{
// 			Err(format!("File \"{filename}\" doesn't exists"))
// 		}
// 	}
	
// }

// pub struct ConsoleManager
// {
// 	pub task_manager: TaskManager,
// 	pub menu_options: Vec<String>,
// }

// impl ConsoleManager 
// {
	
// 	pub fn new() -> Self 
// 	{
// 		Self 
// 		{ 
// 			task_manager: TaskManager::new(), 
// 			menu_options: vec!
// 			[
// 				"Add task".to_string(),
// 				"Find task".to_string(),
// 				"Edit task".to_string(),
// 				"Remove task".to_string(),
// 				"Print tasks".to_string(),
// 				"Save tasks in file".to_string(),
// 				"Read tasks from file".to_string(),
// 				"Clean console".to_string(),
// 				"Quit".to_string(),
// 			] 
// 		}
// 	}


// 	pub fn print_menu(&self) 
// 	{
// 		for (index, menu_option) in self.menu_options.iter().enumerate()
// 		{
// 			println!("{}. {}", index + 1, menu_option);
// 		}
// 	}


// 	pub fn input(query: &str) -> std::io::Result<String> 
// 	{
// 		print!("{query}");
// 		std::io::stdout().flush()?;

// 		let mut buffer = String::new();
// 		std::io::stdin().read_line(&mut buffer)?;
// 		Ok(buffer.trim().to_string())
// 	}

// 	pub fn process_command(&mut self)
// 	{
// 		loop 
// 		{
// 			match Self::input("\nEnter command index: ") 
// 			{
// 				Ok(command) => 
// 				{
// 					match command.as_str() 
// 					{

// 						"1" => 
// 						{
// 							self.task_manager.add_task(Task::new_console_task());
// 						}

// 						"2" => 
// 						{
// 							let name = match Self::input("Enter task name to find: ")
// 							{
// 								Ok(name) => name,
// 								Err(e) => 
// 								{
// 									println!("Error getting user input: {}", e);
// 									return;
// 								}
// 							};

// 							match self.task_manager.find_task(name.as_str())
// 							{
// 								Some(index) => 
// 								{
// 									println!("{:-^22}", "TASK".magenta());
// 									self.task_manager.tasks.get(index).unwrap().print_task()
// 								}
// 								None => println!("Task with name \"{}\" doesn't exist", name)
// 							}
// 						}

// 						"3" => 
// 						{
// 							let name = match Self::input("Enter task name to edit: ")
// 							{
// 								Ok(name) => name,
// 								Err(e) => 
// 									{
// 										println!("Error getting user input: {}", e);
// 										return;
// 									}
// 							};

// 							match self.task_manager.edit_task(name.as_str(), Task::new_console_task())
// 							{
// 								Ok(msg) => {println!("{}", msg)}
// 								Err(msg) => {println!("{}", msg)}
// 							}
// 						}

// 						"4" => 
// 						{
// 							let name = match Self::input("Enter task name to remove: ")
// 							{
// 								Ok(name) => name,
// 								Err(e) => 
// 									{
// 										println!("Error getting user input: {}", e);
// 										return;
// 									}
// 							};

// 							match self.task_manager.remove_task(name.as_str())
// 							{
// 								Ok(msg) => {println!("{}", msg)}
// 								Err(msg) => {println!("{}", msg)}
// 							}
// 						}

// 						"5" => 
// 						{
// 							self.task_manager.print_tasks();
// 						}

// 						"6" => 
// 						{
// 							let filename = match Self::input("Enter file name to write data: ")
// 							{
// 								Ok(filename) => filename,
// 								Err(e) => 
// 									{
// 										println!("Error getting user input: {}", e);
// 										return;
// 									}
// 							};

// 							match self.task_manager.write_in_file(filename.as_str())
// 							{
// 								Ok(msg) => {println!("{}", msg)}
// 								Err(msg) => {println!("{}", msg)}
// 							}
// 						}

// 						"7" => 
// 						{
// 							let filename = match Self::input("Enter file name to read data: ")
// 							{
// 								Ok(filename) => filename,
// 								Err(e) => 
// 									{
// 										println!("Error getting user input: {}", e);
// 										return;
// 									}
// 							};

// 							match self.task_manager.read_from_file(filename.as_str())
// 							{
// 								Ok(msg) => {println!("{}", msg)}
// 								Err(msg) => {println!("{}", msg)}
// 							}
// 						}

// 						"8" => 
// 						{
// 							print!("{esc}c", esc=27 as char);
// 							self.print_menu();
// 						}

// 						"9" => 
// 						{
// 							break;
// 						}

// 						_ => println!("There is no such input command")
// 					}
// 				}
// 				Err(e) => println!("Error getting user input: {e}")
// 			}
// 		}
// 	}
// }