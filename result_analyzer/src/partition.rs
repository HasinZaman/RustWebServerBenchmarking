pub fn partition<'a, D, F>(data: &'a [D], filter_func: F) -> (Vec<&'a D>, Vec<&'a D>)
where
    F: Fn(&(usize, &D)) -> bool,
{
    let success: Vec<&'a D> = data
        .iter()
        .enumerate()
        .filter(|val| filter_func(val))
        .map(|val| val.1)
        .collect();

    let fail: Vec<&'a D> = data
        .iter()
        .enumerate()
        .filter(|val| !filter_func(val))
        .map(|val| val.1)
        .collect();

    (success, fail)
}

pub fn sort<'a, D, K>(data: &'a [D], mut key: K) -> Vec<&'a D>
where
    K: FnMut(&D) -> usize,
{
    let mut sorted_data: Vec<&D> = data.iter().collect();

    sorted_data.sort_by_cached_key(|val| key(*val));

    sorted_data
}

pub fn percentile<'a, D>(k: f32, sorted_data: &'a [D]) -> &'a D {
    let index: usize = (sorted_data.len() as f32 * k).floor() as usize;

    &sorted_data[index]
}

pub fn partition_outlier<'a, D, K, V>(sorted_data: &'a [D], key: K) -> Vec<&D>
where
    K: Fn(&D) -> f32,
{
    let percentile_25 = percentile(0.25, &sorted_data);
    let percentile_75 = percentile(0.75, &sorted_data);

    let quartile_range = key(percentile_75) - key(percentile_25);

    let upper = key(percentile_75) + 1.5 * quartile_range;
    let lower = key(percentile_25) - 1.5 * quartile_range;

    // todo!(find index of lower bound & upper bound and return slice)
    sorted_data
        .iter()
        .filter(|val| lower <= key(*val) || key(*val) <= upper)
        .collect()
}
