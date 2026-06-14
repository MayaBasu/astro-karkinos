#[derive(Clone,Debug)]
pub enum FILETYPE{
    DAT(String), //type is delinator
    FITS,
}
#[derive(Clone, Debug)]
pub struct DataFile {
    pub name: &'static str,
    pub path: &'static str,
    pub file_type: FILETYPE
}

impl DataFile{
    pub fn new(name:&'static str, path:&'static str, file_type: FILETYPE)-> DataFile{
        DataFile{
            name,
            path,
            file_type
        }
    }

}