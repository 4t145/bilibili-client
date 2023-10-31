use rand::prelude::Distribution;

pub fn buvid3() -> String {
    use std::time::SystemTime;
    let time_stamp = (SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time goes back"))
    .as_secs()
        / 10000;

    format!(
        "{}-{}-{}-{}-{}{:05}infoc",
        random_hex(8),
        random_hex(4),
        random_hex(4),
        random_hex(4),
        random_hex(12),
        time_stamp
    )
}

#[test]
fn test_buvid3() {
    println!("{}", buvid3());
}

pub fn random_hex(size: usize) -> String {
    const ALPHABETA: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    rand::distributions::Uniform::from(0..16)
        .sample_iter(&mut rand::thread_rng())
        .take(size)
        .map(|n| ALPHABETA[n])
        .collect::<String>()
}
