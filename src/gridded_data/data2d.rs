
use crate::geometry::*;
use uvex_fitrs::{Fits, Hdu};
use crate::telescope::SpatialEffect;

pub enum EffectType{
    Multiplicative,
    Additive
}

pub trait DATA2D<Unit:super::units::Unit>{
    fn grid(&self)-> &GRID2D;
    fn data(&self)-> &Vec<Vec<f64>>;
    fn mutable_data(&mut self)->&mut Vec<Vec<f64>>;
    fn unit(&self)-> &Unit;
    fn is_data_regular(data:&Vec<Vec<f64>>)->bool{
        match data.iter().flatten().find(|&x|{
            !Regular::check(*x)
        }){
            Some(_) => {false}
            _ => {true}
        }
    }
    fn value_at_index(&self, index:usize) -> f64{
        assert!(index<=self.grid().num_points(),
                "{}", format!("Data grid only has {:?} points, attempted to access point {:?}",
                        self.grid().num_points(),
                        index));
        let [x,y] = self.grid().xy_indices(index);
        self.data()[y][x]
    }
    fn value(&self,point:&Point)->f64{
        let interpolation_data = self.grid().projected_interpolation_coefficients(point);

        let corners = match interpolation_data.corners{
            Corners::Four(one, two, three, four) => {
                vec![one,two,three,four]}
            Corners::Two(one,two) => {vec![one,two]}
            Corners::One(one) => {vec![one]}
        };
        let sum:f64 = corners
            .into_iter().
            zip(interpolation_data.coefficients)
            .map(|(point,coefficient)|{
               // println!("{:?}",self.value_at_index(point));
                (self.value_at_index(point)*coefficient).value()
            }).sum();
        (Regular::try_from(sum)/interpolation_data.normalization).value()
    }
    fn re_grid_values(&self, new_grid: GRID2D) -> Vec<Vec<f64>>{
        let points = new_grid.locate_grid_points()
            .iter()
            .flatten()
            .map(|point|self.value(point))
            .collect::<Vec<f64>>();

        points.chunks(new_grid.x_num()).map(|v|v.to_vec())
            .collect()

    }
    fn combine(&mut self, effect: &SpatialEffect, effect_type: EffectType) {



        let upper_left_corner = effect.grid()
            .locate(effect.grid().grid_number(0,0));
        let lower_left_corner = effect.grid()
            .locate(effect.grid().grid_number(0,effect.grid().y_num()-1));
        let lower_right_corner = effect.grid()
            .locate(effect.grid().grid_number(effect.grid().x_num()-1,effect.grid().y_num()-1));

        let (min_x,min_y,_,_) = self.grid().project_fit(&upper_left_corner,false);
        let (_,max_y,_,_) = self.grid().project_fit(&lower_left_corner,false);
        let (max_x,_,_,_) = self.grid().project_fit(&lower_right_corner,false);
        //println!("lkjdf{:?} {:?} {:?} {:?}",min_x,min_y,max_x,max_y);
        //TODO:check bounds!
        (min_y..max_y).for_each(|i|{
            (min_x..max_x).for_each(|j| {
                let pixel = self.grid().grid_number(i, j);
                let detector_pixel_position = self.grid().locate(pixel);
               // println!("{:?}",detector_pixel_position);
                let effect_value = effect.value(&detector_pixel_position);
                match effect_type {
                    EffectType::Multiplicative => {
                        self.mutable_data()[j][i] = self.mutable_data()[j][i] * effect_value;
                    }
                    EffectType::Additive => {
                        self.mutable_data()[j][i] =self.mutable_data()[j][i] + effect_value;
                    }
                }
            })

        });

    }
    fn add(&mut self, effect: &SpatialEffect){
        self.combine(effect, EffectType::Additive)
    }
    fn multiply(&mut self, effect: &SpatialEffect){
        self.combine(effect, EffectType::Multiplicative)
    }
    fn write_to_fits(&self, path:&str){
        let shape = self.grid().xy_num();
        let primary_hdu = Hdu::new(&shape, self.data().into_iter().flatten().map(|x|*x).collect());
        //TODO: add header
        Fits::create(path, primary_hdu).expect(
            &format!("Failed to write data to FITS at {path}"));

    }
}


   
  



    









