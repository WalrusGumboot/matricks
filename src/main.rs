use std::default::Default;
use std::fmt;

/// A generic matrix struct which defines addition, multiplication and other essential operations.
///
/// Due to the fact that this matrix is generic, most operations will not be defined properly.
/// However, for all numeric types this works fine. This has the added benefit that you can define
/// your own custom type and its corresponding operations, and it'll work out of the box.
/// 
/// The default type used in all of these constructors is f64. However, when supplying elements by
/// hand, the type will be inferred automatically.



struct Matrix<T: Default> {
    rows: usize,
    columns: usize,
    contents: Vec<T>
}

impl<T: Default + fmt::Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO: 1) get the length of the longest string representation
        //      2) right pad all other string reps
        //      3) insert a line break every time [columns] items have passed
        //      4) add unicode box drawing characters for the edges
        let mut string_reps: Vec::<String> = Vec::with_capacity(self.contents.len());
        write!(f, "under construction lmao")
    }
}

impl<T: Default> Matrix<T> {
    fn zeroes(rows: usize, columns: usize) -> Matrix::<f64> {
        Matrix::<f64> {
            rows: rows,
            columns: columns, 
            contents: vec![0f64; rows * columns]
        }
    }

    fn ones(rows: usize, columns: usize) -> Matrix::<f64> {
        Matrix::<f64> {
            rows: rows,
            columns: columns,
            contents: vec![1f64; rows * columns]
        }
    }

    fn new(rows: usize, columns: usize, mut elements: Vec<T>) -> Matrix::<T> {
        if elements.len() > rows * columns {
            panic!("{elements} elements were given, but a {rows} by {columns} matrix can only hold {max}.",
                   elements = elements.len(),
                   rows = rows,
                   columns = columns,
                   max = rows * columns)
        }
        
        //this line is to make sure that the length is always correct
        elements.resize_with(rows * columns, Default::default); 

        Matrix::<T> {
            rows: rows,
            columns: columns,
            contents: elements
        }
    }
}


fn main() {
    
}
