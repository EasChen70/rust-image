pub trait GetC<T:Eq> {
    ///this function needs to be called on a Vec<Vec<T>> and have a tuple of a coordinate
/// where you can want to look for an element 
/// It will return either Some(T) where T is the element at the coordinates 
/// or will return None if the coordinates are out of bounds of the Vec<Vec<T>>
    fn get_c(&self,index: (usize,usize)) -> Option<&T>;
}



///Array2 is a struct that stores the width,height,and a 1d vector 
/// that we will represent with a 2d vector 
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}


impl<T: Clone> Array2<T> {
    //Create slate Array2
    ///You call this function with arguemnets of the imgs height and width and an element 
    /// of the 1d vector 
    /// this will return a struc of Array2 with data being a vector of data that you can copy over 
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        let data = vec![default_value; width * height];
        Array2 { width, height, data }
    }
    //Allows you to push in data 
    ///You call this function with the imgs height, width, and a 1d vector that you want to represent 
    /// It will return an Array2 struc with width, height, and a 1d vector we will represent with a 
    /// 1d vector
    pub fn with_data(width: usize, height: usize, data: Vec<T>) -> Self {
        Array2 { width, height, data }
    }
    
    pub fn printwidth(self){
        println!("{0}",self.width);
    }
    
    //Makes the Array2
    ///you call this on a Array2 struct 
    /// this will return a Vec<Vec<T>>
    pub fn from_row_major(self) -> Vec<Vec<T>> {
        let mut array: Vec<Vec<T>> = Vec::with_capacity(self.height);
        let mut hold: Vec<T> = Vec::with_capacity(self.width);
        let mut x = 1;
        
        for i in self.data {
            if x <= self.width {
                hold.push(i);
                x += 1;
            } else {
                array.push(hold);
                hold = Vec::with_capacity(self.width); // Clear the vector without deallocating memory
                hold.push(i);
                x = 2;
            }
        }
        array.push(hold); // Push the last row
        return array;
    }
    
}



impl<T: Eq> GetC<T> for Vec<Vec<T>> {
    ///this function needs to be called on a Vec<Vec<T>> and have a tuple of a coordinate
    /// where you can want to look for an element 
    /// It will return either Some(T) where T is the element at the coordinates 
    /// or will return None if the coordinates are out of bounds of the Vec<Vec<T>>
    fn get_c(&self, index: (usize, usize) ) -> Option<&T> {
        if index.0 < self.len() {
            let row = &self[index.0];
            if index.1 < row.len() {
                return Some(&row[index.1]);
            }
        }
        None
    }
}


/// Provides an iterator over a 2D vector in row-major order.

/// Row-major order means iterating over each row from top to bottom,
/// and within each row from left to right.
pub trait IterRowMajor<'a, T: Clone> {
    /// Returns a boxed iterator that yields each element's row index, column index,
    /// and a reference to the element itself.
    fn iter_row_major(&'a self) -> Box<dyn Iterator<Item = (usize, usize, &'a T)> + 'a>;
}

/// Implementation of IterRowMajor for any 2D vector which is a vector of vectors.
impl<'a, T: Clone> IterRowMajor<'a, T> for Vec<Vec<T>> {
    /// Constructs and returns the row-major order iterator.
    fn iter_row_major(&'a self) -> Box<dyn Iterator<Item = (usize, usize, &'a T)> + 'a> {
        Box::new(
            self.iter().enumerate().flat_map(move |(row_idx, row)| {
                row.iter().enumerate().map(move |(col_idx, pixel)| (row_idx, col_idx, pixel))
            }),
        )
    }
}

/// Provides an iterator over a 2D vector in column-major order.

/// Column-major order means iterating over each column from left to right,
/// and within each column from top to bottom.
pub trait IterColumnMajor<'a, T: Clone> {
    /// Returns a boxed iterator that yields each element's row index, column index,
    /// and a reference to the element itself.
    fn iter_column_major(&'a self) -> Box<dyn Iterator<Item = (usize, usize, &'a T)> + 'a>;
}

/// Implementation of IterColumnMajor for any 2D vector, which is a vector of vectors.
impl<'a, T: Clone> IterColumnMajor<'a, T> for Vec<Vec<T>> {
    /// Constructs and returns the column-major order iterator.
    fn iter_column_major(&'a self) -> Box<dyn Iterator<Item = (usize, usize, &'a T)> + 'a> {
        let num_rows = self.len(); // Number of rows is equal to the length of the outer vector.
        let num_cols = self.get(0).map_or(0, |row| row.len()); // Number of columns is equal to the length of any inner vector.
        
        Box::new((0..num_cols).flat_map(move |col_idx| {
            (0..num_rows).map(move |row_idx| {
                (row_idx, col_idx, &self[row_idx][col_idx]) // Access elements in column-major order.
            })
        }))
    }
}
