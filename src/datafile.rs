
use astroimsim_geometry::grid1d::GRID1D;
use astroimsim_geometry::grid2d::GRID2D;

#[derive(Clone,Debug)]
pub enum GRID{
    GRID1D(GRID1D),
    GRID2D(GRID2D),
}

pub enum FILETYPE{
    dat,
    fits,
}
#[derive(Clone, Debug)]
pub struct DataFile {
    pub name: &'static str,
    pub path: &'static str,
    pub grid: GRID,
    pub 
}

impl DataFile{
    pub fn new(name:&str, path: &str, grid:GRID){
        DataFile{
            
        }
    }
}