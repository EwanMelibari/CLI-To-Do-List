use std::io;

fn main() {
    
    // loop to keep the program running until the user chooses to exit
    loop {

        println!("Welcome to the To-Do List app");

        println!("Please select an option:");
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Update task");
        println!("4. Delete task");
        println!("5. Exit");

        let mut choice = String ::new();

        match choice.trim() {

            "1" => {

                // code to add a task

            },

            "2" => {

                // code to view tasks

            },

            "3" => {

                // code to update a task

            },

            "4" => {

                // code to delete a task

            },

            "5" => {

                // code to exit the program

                println!("Exiting the program...");

                break;
            },

            // handle invalid input
            _ => {
                println!("Invalid choice, please try again.");
            }

        }


    } 

}



