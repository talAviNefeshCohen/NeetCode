// pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//     // presolution thinking,
//     // so i dont think i ever saw this question, the O(n**2) is very easy
//     // doing a for loop inside a for loop skipping the index.
//     // one way is using the "-30 <= nums[i] <= 30" constraint to create a counting 
//     // bucket sized 61, this takes O(n) time. then for each value in nums
//     // get all of the counting bucket value reduce the value of the index one by one
//     // power accordingly and multiply between them.
//     // since there is a constant max of 61 values this takes O(1)
//     // techincally it also takes O(1) memory, ill create this solution 
//     // and if ill figure a better way ill fix it.
//     let mut bucket: [i32; 61] = [0; 61];
//     nums.iter().for_each(|&num| bucket[(num+30) as usize ]+=1 );
//     let mut product_except_self_vec = Vec::with_capacity(nums.len());
//     for &num in nums.iter(){
//         let mut product:i32 = 1;
//         for (bucket_idx,&count) in bucket.iter().enumerate(){
//             let mut pow = count;
//             if bucket_idx == (num as usize +30){
//                 pow -= 1;
//             }
//             product *= (bucket_idx as i32 -30).pow(pow as u32);
//         }
//         product_except_self_vec.push(product);
//     }
//     return product_except_self_vec;
// }


//this is my original solution, after submitting i saw a better solution and this is it.

// pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//     // in this solution im going too to create two helper vectors of the mult, from right to left and left too right
//     // i thought about pointer methods while trying to come up woht my own solution but the front too back didnt pop.
//     if nums.len() <= 1{
//         return nums;
//     }
//     let mut asc_cumprod = vec![1;nums.len()];
//     let mut desc_cumprod = vec![1;nums.len()];
//     let mut asc_num = 1;
//     let mut desc_num = 1;
//     for (idx,&num) in nums.iter().enumerate(){
//         asc_num *= num;
//         asc_cumprod[idx] *= asc_num;
//         let mirror_index = (nums.len()-1-idx) as usize;
//         desc_num *= nums[mirror_index];
//         desc_cumprod[mirror_index] *= desc_num;
//     }
//     println!("{:?}",asc_cumprod);
//     println!("{:?}",desc_cumprod);

//     let mut product_except_self_vec = Vec::with_capacity(nums.len());
//     product_except_self_vec.push(desc_cumprod[1]);
//     for idx in 1..nums.len()-1{
//         product_except_self_vec.push(desc_cumprod[idx+1]*asc_cumprod[idx-1]);
//     }
//     product_except_self_vec.push(asc_cumprod[(nums.len()-1) -1 as usize]);
//     return product_except_self_vec;
// }

// this solution works but isnt very elegent and uses Theta(n) space

// now this is the optimal solution
// the trick is that no added vectors were needed if you plan in the right order

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    // makes sure no index problem will occur
    if n <= 1 {
        return nums;
    }
    // creates the result vector ahead
    let mut product_except_self_vec = vec![1; n];
    // initialized both sides products
    let mut left_product = 1;
    let mut right_product = 1;

    for i in 0..n {
        // note that both left product and right product are initialized with 1 so it 
        // solve the 1 to n-1 runing problem
        // we mutltply the current index by the last left product
        // then we doi the same on the mirror index with the last right product, this means 
        // that by the end of this iteration and the mirroed index iteration the value here will be 
        // correct.
        product_except_self_vec[i] *= left_product;
        product_except_self_vec[n - 1 - i] *= right_product;
        // then we just update the product accoridngly
        left_product *= nums[i];
        right_product *= nums[n - 1 - i];
    }

    product_except_self_vec
}

// what i learnt from this problem.
// bucket solution was creative but didnt take into considiration 
// nature of the problem of going left to right and right to left
// the simple solution that use two vectores was very cumbersome and i could have dealt with writing it better
// the optimal solution is intricate but possible, next time ill need to give myself time to try and improve
// and not go to the solution
// the rust part is getting way easier in this regard.
fn main() {
    let nums = vec![1,2,3,4];
    println!("{:?}",product_except_self(nums));
    println!("Hello, world!");
}
