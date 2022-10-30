mod error;
mod project;

use error::CargoCleanerError;
use project::Project;
use toml::Value;
use std::path::PathBuf;
use std::fs;
use console::style;


fn main(){
    let mut projects: Vec<Project> = Vec::new();

    let mut all_size = 0usize;

    find_projects(PathBuf::from(".\\"), &mut projects);

    for project in projects{
        if let Some(project_name) = &project.project_name{
            match project.get_project_size(){
                Ok(file_size_kb_before) => {
                    match project.clean_project(){
                        Ok(file_size_kb_after) => {
                            all_size = all_size + file_size_kb_before - file_size_kb_after;
                            println!("{} {} (before: {}, after: {}, difference: {}) {}", style("Cleaned").green(), style(project_name).color256(7), style(file_size_kb_before).color256(31), style(file_size_kb_after).color256(31), style(file_size_kb_before - file_size_kb_after).color256(76), project.project_path.display());
                        }
                        Err(error) =>  println!("{} ({}) {}", style("Failed").red(), style(error).color256(33), style(project_name).color256(7)),
                    }
                }
                Err(error) => println!("{} ({}) {}", style("Failed").red(), style(error).color256(33), style(project_name).color256(7)),
            }
        }else{
            println!("{} ({}) {}", style("Failed").red(), style(CargoCleanerError::ReadCargoToml).color256(33), style(project.project_path.display()).color256(7))
        }
        
        

    }

    println!("You have saved a total of {}", style(bytes_to_string(all_size)).green());
   
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Crashed while reading line");

}



fn find_projects(path: PathBuf, projects: &mut Vec<Project>){
    if let Ok(dirs) = fs::read_dir(&path){
        for dir in dirs{
            if let Ok(dir) = dir{
                if let Ok(metada) = dir.metadata(){
                    if metada.is_dir(){
                        find_projects(dir.path(), projects);
                    }else{
                        if dir.file_name() == "Cargo.toml"{
                            if let Ok(file_text) =  fs::read_to_string(dir.path()){
                                if let Ok(toml) = file_text.parse::<Value>(){
                                    if let Some(a) = toml.get("package"){
                                        if let Some(b) = a.get("name"){
                                            projects.push(Project::new_project(Some(b.to_string()), path.clone()));   
                                            continue;
                                        }
                                    }
                                }
                            }
                            projects.push(Project::new_project(None, path.clone()));   
                        }
                       
                    }
                }
            }
        }
    }

}

const BYTEUNITS: [(usize, &str); 4] = [(1073741824, "gigabyte"), (1048576, "megabyte"), (1024, "kilobyte"), (1, "byte")];


//Could have been a better algorithm
fn bytes_to_string(mut bytes: usize) -> String{
    let mut converted_text = String::new();
    if bytes != 0{
        let mut converted_text_vec = Vec::new();
        for (unit_size, unit_text) in BYTEUNITS{
            let count = bytes / unit_size ;
            if count >= 1{
                bytes -= count * unit_size;
                let text = format!("{} {}{}", count, unit_text, (count != 1).then_some('s').unwrap_or('\0'));
                converted_text_vec.push(text);
            }
        }
        let converted_text_vec_len = converted_text_vec.len();
        if converted_text_vec_len > 1{
            for (i, text) in converted_text_vec.into_iter().enumerate(){
                if i != converted_text_vec_len-1{
                    converted_text.push_str(text.as_str());
                    if i != converted_text_vec_len-2{
                        converted_text.push_str(", ");
                    }else{
                        converted_text.push_str(" and ");
                    }
                    
                }else{
                    converted_text.push_str(text.as_str());
                }
            }
        }
        
        
    }else{
        converted_text.push_str("0 byte");
    }
    

    converted_text
}
