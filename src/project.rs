use std::{path::PathBuf, process::Command, fs};

use crate::error::CargoCleanerError;





#[derive(Debug)]
pub struct Project{
    pub project_name: Option<String>,
    pub project_path: PathBuf,
}



impl Project{
    pub fn new_project(project_name:  Option<String>, project_path: PathBuf) -> Project{

        Project { project_name, project_path }
    }


    pub fn clean_project(&self) -> Result<usize, CargoCleanerError>{
        let mut cargo_clean = Command::new("cargo");
        cargo_clean.arg("clean");
        cargo_clean.current_dir(&self.project_path);
        if let Ok(mut cmd) = cargo_clean.spawn(){
            if let Ok(_) = cmd.wait(){
                self.get_project_size()
            }else {
                Err(CargoCleanerError::SpawnCargoCleanCommand)
            }
        }else {
            Err(CargoCleanerError::SpawnCargoCleanCommand)
        }
    }

    pub fn get_project_size(&self) ->  Result<usize, CargoCleanerError>{
        fn get_dir_size(path: &PathBuf) ->  Result<usize, CargoCleanerError>{
           match fs::read_dir(path){
                Ok(dir) => {
                    let mut size = 0usize;
                    for element in dir{
                        match element{
                            Ok(entry) => {
                                if let Ok(metada) = entry.metadata(){
                                    if metada.is_dir(){
                                        if let Ok(new_size) = get_dir_size(&entry.path()){
                                            size += new_size;
                                        }
                                    }else{
                                        size += metada.len() as usize;
                                    }
                                }
                            },
                            Err(_) => return Err(CargoCleanerError::CountDirSize),
                        }
                    }
                    Ok(size)
                },
                Err(_) => Err(CargoCleanerError::ReadDir),
            }
        }

        get_dir_size(&self.project_path)
    }
}



