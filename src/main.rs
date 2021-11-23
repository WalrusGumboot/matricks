use std::default::Default;
use std::fmt;
use std::ops;




/// A generic matrix struct which defines addition, multiplication and other essential operations.
///
/// Due to the fact that this matrix is generic, most operations will not be defined properly.
/// However, for all numeric types this works fine. This has the added benefit that you can define
/// your own custom type and its corresponding operations, and it'll work out of the box.

struct Matrix<T: Default> {
    rows: usize,
    columns: usize,
    contents: Vec<T>
}

impl<T: 'static> fmt::Display for Matrix<T> where
    T:  Default + fmt::Display,
    &'static T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_reps: Vec::<String> = vec![String::from(""); self.contents.len()];
        let mut longest = 0;
        for e in &self.contents {
            let len = e.to_string().len();
            if len > longest {
                longest = len;
            }
        }

        // now that we have the length of the longest element; get string reps
        // and right-pad them with spaces to fit the length.

        for (pos, e) in self.contents.iter().enumerate() {
            string_reps[pos] = format!("{:>width$} ", *e, width = longest)
        }

        // we need to add two to accomodate for the top and bottom rows
        let mut row_reps: Vec::<String> = vec![String::from(""); self.rows + 2]; 

        for i in 0..self.rows + 2 {
            
            // top and bottom delimiters
            if i == 0             {row_reps[i] = format!("{}{}{}", "┌ ", " ".repeat((longest + 1) * self.columns), "┐"); continue}
            if i == self.rows + 1 {row_reps[i] = format!("{}{}{}", "└ ", " ".repeat((longest + 1) * self.columns), "┘"); continue}
            
            // remaining rows
            //let content = &string_reps[(i-1)*self.columns .. (i-1) * self.columns].join("");
            row_reps[i] = ["│ ", &string_reps[(i-1)*self.columns .. (i) * self.columns].join(""), "│"].join("");

            //println!("row: {}", row_reps[i]);
        }
        
        //println!("{:?}", row_reps);
        //println!("{:?}", string_reps);
        write!(f, "{}", row_reps.join("\n"))
    }
}

impl<T: Default> Matrix<T> {
    /// Returns an all-zero matrix of the given size.
    fn zeroes(rows: usize, columns: usize) -> Matrix::<f64> {
        Matrix::<f64> {
            rows: rows,
            columns: columns, 
            contents: vec![0f64; rows * columns]
        }
    }
    
    /// Returns an all-ones matrix of the given size.
    fn ones(rows: usize, columns: usize) -> Matrix::<f64> {
        Matrix::<f64> {
            rows: rows,
            columns: columns,
            contents: vec![1f64; rows * columns]
        }
    }

    /// Returns a matrix of the given size and populates it with the given data.
    /// 
    /// Note that if more elements are supplied than the matrix can hold,
    /// this will panic. If less are given, the remaining slots are filled with zeroes.
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

impl<T: Default + Clone> ops::Add<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, o: Matrix<T>) -> Matrix<T> {
        assert!(self.columns == o.columns && self.rows == o.rows, "Can only add matrices of the same dimension.");
        Matrix::<T> {
            rows: self.rows, 
            columns: self.columns, 
            contents: vec![T::default(); self.rows * self.columns] //TODO: make this do the proper addition
        }
    }
}


fn main() {
    let m1: Matrix<f64> = Matrix::new(3, 2, vec![1.0, 2.5, 3.14, 90000.22, 5.11111111]);
    println!("{}", m1);

    let m2: Matrix<f64> = Matrix::<f64>::ones(3, 2);
    println!("{}", m2);

    println!("{}", m1 + m2);
}
