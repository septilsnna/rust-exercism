#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let mut sum: u64 = 0;
    if num == 0 {
        None
    } else {
        for i in 1..num {
            if num%i == 0 {
                sum = sum+i
            }
        }
        if num < sum {
            let res = Classification::Abundant;
            Some(res)
        } else if num > sum {
            let res = Classification::Deficient;
            Some(res)
        } else {
            let res = Classification::Perfect;
            Some(res)
        }
    }
}