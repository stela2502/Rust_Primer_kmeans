use rand::prelude::*;
use std::error::Error;
use csv::ReaderBuilder;

//https://docs.rs/csv/latest/csv/

pub fn read_tsv( fname:&str, ncol: usize, nrow: usize ) ->  Result< Vec<Vec<f32>>, &str>  {
    // Build the CSV reader and iterate over each record.

    let rdr = ReaderBuilder::new()
    	.delimiter(b'\t')
    	.from_path( fname );

    let mut i = 0;

    let mut res: Vec<Vec<f32>> = vec!( vec!(0.0; nrow); ncol );

    if let mut records = rdr.expect("REASON").records(){
    	while let Some(result) = records.next() {
	        match result{
	        	Ok(res) => {
	        		println!("I got one entry: {:?}", res.pop());
	        		
	        		res
	        	},
	        	Err(err) => {
	        		eprintln!("I got an error: {:?}", err);
	        		return Err::<Vec<Vec<f32>>, &str>("record unusable")
	        	}
	        };
	        //eprintln!("What have I got here? {}", record.len() );
	        i +=1;
	    }
    }else {
    	eprintln!("I expected at least one data line!" );
    }
     

    println!("I have read {} lines",i);
    return Ok(res);
}


fn e_dist( a:&Vec<f32>, b:&Vec<f32> ) -> f32{
	let mut res:f32 = 0.0;
	for i in 0..a.len()-1 {
		res += (a[i] -b[i]).powf(2.0);
	}
	res = res.sqrt();
	return res;
}

/*
pub fn kmeans( data:&Vec<Vec<f32>>, k:usize, it:usize ) -> Vec<usize> {
	// implement the thing here
	// 1. where do we store the data?
	let mut res:Vec<usize> = vec!(0; data.len() );
	// how do we get the initial state?
	let mut nums: Vec<i32> = (0..data.len()-1).collect();
	nums.shuffle(&mut rng);
	// and use the first k ones...

	'main for i in 0..it{
		for a in 0..data.len()-1{
		
			let tmp:Vec<f32> = vec!(0.0; k);
			let mut min = 1.1e+9;
				for ka in 0..k-1 {
				tmp[ka] = e_dist( data[a], data[ka] );
				if min > tmp[ka]{
					min = tmp[ka]
				}
			}

		}
	}
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_e_dist() {
    	let a:Vec<f32>;
    	let b:Vec<f32>;
    	a = vec!(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0);
    	b = vec!(1.0,2.0,3.0,4.0,2.0,6.0,7.0,8.0,9.0);
    	let result = e_dist( &a ,&b );
        assert_eq!(result, 3.0 as f32);
    }

    fn t_read_tsv(){
    	let fname = "TestData/Spellman_Yeast_Cell_Cycle.tsv";
    	read_tsv( fname );
    }

}