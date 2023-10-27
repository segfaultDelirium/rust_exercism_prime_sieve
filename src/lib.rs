pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // todo!("Construct a vector of all primes up to {upper_bound}");
    sieve(upper_bound)
}

fn sieve_rec<'a>(list: Vec<u64>, sieve_nums: Vec<u64>) -> Vec<u64> {
    if sieve_nums.is_empty() {
        return list;
    }
    if list.is_empty() {
        return list;
    }
    let sieve_by = sieve_nums.get(0).unwrap().clone();
    let new_list: Vec<u64> = list
        .into_iter()
        .filter(|x| *x == sieve_by || *x % sieve_by != 0)
        .collect();
    let new_sieve_nums: Vec<u64> = sieve_nums
        .into_iter()
        .filter(|x| *x != sieve_by && *x % sieve_by != 0)
        .collect();
    sieve_rec(new_list, new_sieve_nums)
}

fn sieve(n: u64) -> Vec<u64> {
    let until = f64::sqrt(n as f64) as u64;
    let list: Vec<u64> = (2..=n).collect();
    let sieve_nums: Vec<u64> = (2..=until).collect();
    // let mut res = sieve_rec(list, sieve_nums);

    let res = sieve_rec(list, sieve_nums);
    // res.reverse();

    res
}
