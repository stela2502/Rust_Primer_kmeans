

pub fn e_dist( a:&Vec<f32>, b:&Vec<f32> ) -> f32{
	let mut res:f32 = 0.0;
	for i in 0..a.len()-1 {
		res += (a[i] -b[i]).powf(2.0);
	}
	res = res.sqrt();
	return res;
}




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
}