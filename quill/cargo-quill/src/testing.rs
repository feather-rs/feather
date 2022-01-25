/*
    This file contains code for handeling the command 'cargo-quill test'.
    This command does different things depending on the current directory
    and, depending on the arguments you pass to it.     
*/

use std::{convert::{TryFrom, TryInto}, path::{Path, PathBuf}};

use anyhow::bail;
use argh::{FromArgs};


#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "test")]
/// Build a Quill plugin.
pub struct Testing {
    #[argh(positional)]
    args: Vec<String>
}


#[derive(Debug)]
pub enum Directory {
    ServerBinary(PathBuf),
    ServerFolder{
        cargo_toml: PathBuf,
        target_dir: PathBuf,
    },
    PluginBinary(PathBuf),
    PluginFolder {
        cargo_toml: PathBuf,
        target_dir: PathBuf,
        exec_name: String,
    }
}

impl TryFrom<&Path> for Directory {
    type Error = anyhow::Error;
    
    fn try_from(it: &Path) -> Result<Self, Self::Error> {

        if !it.exists() {
            bail!("The path {:?} does not exist", it); 
        }

        match it.is_file() {
            true if it.ends_with("Cargo.toml") => {
                Self::try_load_from_cargo_toml(it)
            },
            true if it.extension().map(|s| s.to_str()) == Some(Some(".plugin")) => Ok(Self::PluginBinary(it.to_owned())),
            true => 
                    // We assume it is a server binary.
                    Ok(Self::ServerBinary(it.to_owned())),
            false if it.is_dir() => {
                // Either a plugin dir or feather server directory 
                let cargo_toml = {
                    let mut path = it.to_owned();
                    path.push("Cargo.toml");

                    if ! path.is_file() {
                        bail!("The folder {:?} does not contain a Cargo.toml file.",it);
                    }
                    path
                };
                Self::try_load_from_cargo_toml(cargo_toml.as_path())
            }
            _ => {
                bail!("The path {:?} does not exist",it);
            }
        }
    }

}


impl Directory {
    fn try_load_from_cargo_toml(cargo_toml: &Path) -> anyhow::Result<Self> {
        
        let cargo_metadata = {
            let mut cmd = cargo_metadata::MetadataCommand::new();
            let cmd = cmd.manifest_path(&cargo_toml);
            cmd.exec()?
        };

        let target_dir = {
            cargo_metadata.target_directory.as_path()
        };


        /*
            If one of the package dependecies is quill
        
        */

    
        /*
            If the name is Some("feather-server") or its a workspace with feather server in it, 
            then it is a server folder. Else we assume plugin.
        */

        let name = {
            let root_package = cargo_metadata.root_package();
            root_package.map(|x| x.name.clone())
        };

        let is_server_dir = {
            
            if name == Some("feather-server".to_owned()) {
                true
            } else {
                cargo_metadata.workspace_members.iter().any(|x| {
                    let x  = &cargo_metadata[x];
                    x.name == "feather-server"
                })
            }
        };

        if is_server_dir {
            Ok(Self::ServerFolder{
                cargo_toml: cargo_toml.to_path_buf(),
                target_dir: target_dir.to_path_buf(),
            })
        } else {
            Ok(Self::PluginFolder{
                cargo_toml: cargo_toml.to_path_buf(),
                target_dir: target_dir.to_path_buf(),
                exec_name: name.expect("The path: {} is assumed to be a plugin directory, however we are not able to determine the plugins name")
            })
        }
    }
}


// This function is invoked when using the cargo-quill test, command.
pub fn test_command(args: Testing) -> anyhow::Result<()> {
    
    let args = args.args.iter()
        .map(|x| Path::new(x))
        .map(|x| x.try_into().unwrap())
        .collect::<Vec<Directory>>();

    if args.iter().all(|x| match x {
        Directory::ServerBinary(_) => false,
        Directory::ServerFolder { cargo_toml: _, target_dir: _ } => false,
        Directory::PluginBinary(_) => true,
        Directory::PluginFolder { cargo_toml: _, target_dir: _ , exec_name: _} => true,
    }) {
        bail!("At least one of the arguments after test, must be a path to a server binary or directory");
    }

    if args.iter().filter(|x| match x {
        Directory::ServerBinary(_) => true,
        Directory::ServerFolder { cargo_toml: _, target_dir: _ } => true,
        Directory::PluginBinary(_) => false,
        Directory::PluginFolder { cargo_toml: _, target_dir: _ , exec_name: _} => false,
    }).count() > 1 {
        bail!("Currently you can't run this command on multiple servers. This feature might be added in the future.");
    }


    /*
        We no compile every each on 

    */

    for arg in &args {
        println!("{:?}",arg);
    }



    Ok(())
}