use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use astroimsim_geometry::grid1d::GRID1D;
use astroimsim_geometry::grid2d::GRID2D;
use plotpy::{Curve, Plot};

pub struct DATAGRID1D{
    pub grid1d: GRID1D,
    pub data: Vec<(usize,Vec<f64>)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each grid point
    pub label: &'static str,
}

pub struct DATAGRID2D{
    pub grid1d: GRID2D,
    pub data: Vec<(usize,Vec<f64>)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each point
    pub label: &'static str,
}



impl DATAGRID1D{

    pub fn load_dat_file(&mut self, path:&str, plot:bool){

        assert_ne!(0,self.data.len(),"Loading data {:?} into would overwrite current data",self.label);
        println!("Loading {:?} into {:?}", path, self.label);
        let start = Instant::now();
        let file = File::open(path.clone()).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut data = Vec::new();
        for (i,line_result) in reader.lines().enumerate() {
            let line = line_result.expect("failed to read line");
            let line = line.trim();
            let line:Vec<&str> = line.split("   ").collect();
            let mut valid_line = true;
            let mut parsed_line = Vec::new();
            for (index, element) in line.iter().enumerate(){
                match element.trim().parse::<f64>() {
                    Ok(val) => {
                        if index ==0{
                            parsed_line.push(val*self.grid1d.scale as f64)
                        }else{
                            parsed_line.push(val)
                        }}
                    Err(_) => {valid_line = false}
                };
            }
            if !valid_line{
                println!("line {:?} invalid: {:?}",i,parsed_line)
            }else{
                /*
                if parsed_line.len() == floats_per_record{

                 */
                data.push(parsed_line)
                /*
                }else{
                    println!("{:?}",parsed_line);
                    panic!("Parsed a different number of floats in a line than expected")
                }

                 */
            }
        }
        /*
        let mut result = [[0.0;floats_per_record]; records];
        for record in 0..records {
            let mut record_as_array = [0.0;floats_per_record];
            for float in 0..floats_per_record {
                record_as_array[float] = data[record][float]
            }
            result[record] = record_as_array;
        }

         */


        println!("Parsed {:?} records in {:?} ms",data.len(), start.elapsed().as_millis());
        assert_eq!(data.len(),self.grid1d.num(),"Retrieved a different number of records than expected");
        let mut same_num_data_points_per_record = true;
        for i in 0..data.len()-1{
            if data[i].len() - data[i+1].len() != 0{
                same_num_data_points_per_record = false;
            }
        }
        if !same_num_data_points_per_record{
            println!("Warning! There are different numbers of records for each line. This may mess up plotting or indicate a loading error. Will be plotting with {:?}", data[0].len()-1)
        }//TODO run this function witha  "verbose" to list out the differences

        if plot{
            let mut plot = Plot::new();
            for i in 1..data[0].len() {
                let mut curve = Curve::new();
                curve.set_line_width(2.0);
                curve.points_begin();
                for point in data.clone() {
                    // println!("{:?}", point);

                    curve.points_add(point[0], point[i]);
                }
                curve.points_end();
                plot.add(&curve).grid_and_labels("x", "y");
            }
            plot.show("ksenf").expect("hHHHHHH");
        }
        println!("The first record is {:?} and the last is {:?}, snapping to grid: {:?}",data[0],data[data.len()-1],self.grid1d);
        let mut snapped_data = Vec::new();
        for datum in data{
            let location = datum[0];
            let index = self.grid1d.snap(location as f64);

            snapped_data.push((index,datum)) //TODO this must change to plot multiple
        }
        self.data = snapped_data;
    }


}