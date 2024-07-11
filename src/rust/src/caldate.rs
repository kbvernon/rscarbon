use extendr_api::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
pub struct CalDate {
    pub(crate) start: i32,
    pub(crate) end: i32,
    pub(crate) density: Vec<f64>
}

#[derive(Debug)]
pub struct CalGrid {
    pub(crate) start: i32,
    pub(crate) end: i32,
    pub(crate) grid: Vec<CalDate>
}

// have extendr convert to external pointer
#[extendr]
impl CalDate {}

impl CalDate {
    pub(crate) fn new(start: i32, end: i32, density: Vec<f64>) -> Self {
        CalDate {start, end, density}
    }

    pub(crate) fn mode(&self) -> i32 {
        let i = self.density.iter()
            .enumerate()
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .map(|(index, _)| index)
            .unwrap();

        let age: Vec<i32> = (self.end..=self.start).rev().collect();

        age[i]
    }

    pub(crate) fn normalize(&mut self) {
        let total: f64 = self.density.iter().sum();
        self.density.iter_mut().for_each(|y| *y /= total);
    }

    pub(crate) fn expand_ages(&self) -> Vec<i32> {
        (self.end..=self.start).rev().collect()
    }
}

impl CalGrid {
    pub(crate) fn new(start: i32, end: i32, grid: Vec<CalDate>) -> Self {
        CalGrid {start, end, grid}
    }

    pub(crate) fn into_list(self, cal_name: &str, sum_to_one: bool) -> List {
        let w: Vec<i32> = vec![self.start, self.end];
        let current_window = Integers::from_values(w);
        
        let mut list = List::from_values(self.grid);

        list.set_class(["CalGrid", "vctrs_vctr", "list"])
            .unwrap()
            .set_attrib("cal_name", cal_name)
            .unwrap()
            .set_attrib("normalize", sum_to_one)
            .unwrap()
            .set_attrib("window", current_window)
            .unwrap();

        list
    }

    pub(crate) fn sum(self, sum_to_one: bool) -> Robj {
        let year_bp: Vec<i32> = (self.end..=self.start).rev().collect();
        let nrow = year_bp.len() as i32;    

        let mut spd = vec![0.0; nrow as usize];
    
        for u in self.grid.into_iter() {
            let g_ages: Vec<i32> = u.expand_ages();
            let g_density: Vec<f64> = u.density;  

            let iterator = g_ages.iter()
                .zip(g_density.iter());
    
            for (&age, &density) in iterator {    
                let i: usize = (self.start - age).try_into().unwrap();    
                spd[i] += density;    
            }    
        }
    
        if sum_to_one {    
            let total: f64 = spd.iter().sum();    
            spd.iter_mut().for_each(|y| *y /= total);    
        }
    
        let mut table = list!(
            age=year_bp,
            prob_dens=spd
        );    
       
        table.set_class(&["tbl_df", "tbl", "data.frame"]).unwrap();
    
        let row_names: Vec<i32> = (1..=nrow).collect();
    
        table.set_attrib("row.names", row_names).unwrap();
    
        table.into_robj()
    }

    pub(crate) fn mode(&self) -> Vec<i32> {
        self.grid.into_par_iter().map(|v| v.mode()).collect()
    }
}

impl From<Vec<CalDate>> for CalGrid {
    fn from(grid: Vec<CalDate>) -> Self {
        let mut v_start = vec![0; grid.len()];
        let mut v_end = vec![0; grid.len()];

        for (i, y) in grid.iter().enumerate() {
            v_start[i] = y.start;
            v_end[i] = y.end;
        }

        let start: i32 = *v_start.iter().max().unwrap();
        let end: i32 = *v_end.iter().min().unwrap();

        CalGrid::new(start, end, grid)
    }
}

impl From<List> for CalGrid {
    fn from(x: List) -> Self {
        let w: Vec<i32> = x.get_attrib("window")
            .unwrap()
            .try_into()
            .unwrap();
        let grid: Vec<CalDate> = x.values()
            .map(|v|{
                let date: ExternalPtr<CalDate> = v.try_from().unwrap();
                *date
            })
            .collect();

        CalGrid::new(w[0], w[1], grid)
    }
}