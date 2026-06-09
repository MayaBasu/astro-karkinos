
use crate::datagrids::{DATAGRID1D,DATAGRID2D};

#[derive(Clone,Debug)]
pub enum DATAGRID{
    DATAGRID1D(DATAGRID1D),
    DATAGRID2D(DATAGRID2D),
}

pub enum FILETYPE{
    dat,
    fits,
}
#[derive(Clone, Debug)]
pub struct DataFile {
    pub name: &'static str,
    pub path: &'static str,
}

impl DataFile{
    
}