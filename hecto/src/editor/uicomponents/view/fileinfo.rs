use super::super::super::FileType;
use std::{
    fmt::{self,Display},
    path::{Path,PathBuf},
};

#[derive(Default,Debug)]
pub struct FileInfo{
    path: Option<PathBuf>,
    file_type: FileType,
}

impl FileInfo{
    pub fn from(filename: &str) -> Self{
        let path = PathBuf::from(filename);
        let file_type = if let Some(ext) =  path.extension().and_then(|e| e.to_str()){
            match ext.to_ascii_lowercase().as_str() {
                "rs" => FileType::Rust,
                "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx"  => FileType::Cpp,
                "c" | "h" => FileType::C,
                _ => FileType::Text,
            }
        }else{
            FileType::Text
        };
        Self{
            path: Some(path),
            file_type
        }
    }

    pub fn get_path(&self) -> Option<&Path>{
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool{
        self.path.is_some()
    }

    pub const fn get_file_type(&self) -> FileType{
        self.file_type
    }

}

impl Display for FileInfo{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result{
        let name = self.get_path()
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("[No Name]");
        write!(formatter,"{name}")
        
    }
}
