#[derive(Clone, Debug)]
pub struct DATAGRID2D{
    pub grid1d: GRID2D,
    pub data: Vec<(usize,Vec<f64>)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each point
    pub label: &'static str,
    pub source: DataSource,
}