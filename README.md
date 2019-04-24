# rust-exercism
# Perfect Number
## The Problem
In **PERFECT NUMBER**, we ordered to determine if a number is perfect, abundant, or deficient based on Nicomachus' (60 - 120 CE) classification scheme for natural numbers. The Greek mathematician [Nicomachus](https://en.wikipedia.org/wiki/Nicomachus) devised a classification scheme for natural numbers, identifying each as belonging uniquely to the categories of **perfect**, **abundant**, or **deficient** based on their [aliquot sum](https://en.wikipedia.org/wiki/Aliquot_sum). The aliquot sum is defined as the sum of the factors of a number not including the number itself. For example, the aliquot sum of 15 is (1 + 3 + 5) = 9.

## My Solution
I get the initial algorithm to solve this problem, which is first by finding the factor of the input value then adding up and storing it in a variable named 'sum'. then the value of 'sum' will be compared with the value of the input. But I forget the fact that the output turned out to be an option, poor me. So first I will filter the input value, is the input value zero or not? If it is zero, the result will be None. And if it is not zero, it will go to the next stage, which is finding the factor of the input value.

Here is my code:
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let mut sum: u64 = 0; // Declare veriable sum
    if num == 0 { // Filter the input value
        None // If it is zero, the result will be None
    } else {
        for i in 1..num { // Finding the factor of the input value
            if num%i == 0 { // Look for modulus from the result of division num with i
                sum = sum+i // Add i to sum
            }
        } // Start to classify
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
  ```
