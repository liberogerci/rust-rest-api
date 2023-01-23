# Rust CRUD rest API

  At this project I developed a rest API using Rust, Rocket web framework (for async call functions) and Serde (for serialize && deserialize data).
  
  The software project developed can create, read, update and delete (CRUD) movie(s) in json format from a database.
  
  I'm use postaman to test the API functionalities, as can see at the images below:

**- Server conection and READ movies list:** ![Screenshot from 2023-01-22 13-48-18](https://user-images.githubusercontent.com/94999729/213931604-1f4c3d05-33c0-4815-89a4-f6b43a58d732.png)
                              
-----------------------------------------------------------------------------------------------------------------------------------------------------------

**- POST method (adding Harry Potter movie at the movies list):** ![Screenshot from 2023-01-22 13-50-31](https://user-images.githubusercontent.com/94999729/213931809-89b791a4-ffc8-4003-834a-d7ff9be3915b.png)

-----------------------------------------------------------------------------------------------------------------------------------------------------------

**- GET method (searching Pulp Fiction movie at the movies list the):** ![Screenshot from 2023-01-22 13-51-32](https://user-images.githubusercontent.com/94999729/213931815-55d5054f-ac1e-493e-a2c6-d720055d0077.png)
 
 ----------------------------------------------------------------------------------------------------------------------------------------------------------

**- DELETE method (delete Incridible movie from the movies list):** ![Screenshot from 2023-01-22 13-51-59](https://user-images.githubusercontent.com/94999729/213931818-73cbd940-acb8-4c1b-8918-6948caea8c5d.png)
                              
-----------------------------------------------------------------------------------------------------------------------------------------------------------

## Running project instructions 

For running this project is necessary install rustup tool, already include rustc and cargo.

After complete the instalation we going access the folder on terminal and run "rusctc" or "cargo run". 

Rustc command will compile and creating a executable file, ready for open and read the CSV document.

On other hand, cargo run command will compile and execute the file in one line.

The server run at the port 8000, how can see at the screenshots presented above.
