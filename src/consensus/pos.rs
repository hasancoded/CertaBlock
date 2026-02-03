use rand::Rng;

#[allow(dead_code)]
pub struct Staker {
    pub address: String,
    pub stake: u64,
}

#[allow(dead_code)]
pub fn select_staker(stakers: &Vec<Staker>) -> Option<String> {
    let total_stake: u64 = stakers.iter().map(|s| s.stake).sum();

    if total_stake == 0 {
        return None;
    }

    let mut rng = rand::rng();
    let mut selection_point = rng.random_range(0..total_stake);

    for staker in stakers {
        if selection_point < staker.stake {
            return Some(staker.address.clone());
        }
        selection_point -= staker.stake;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_staker() {
        let stakers = vec![
            Staker {
                address: "Alice".to_string(),
                stake: 100,
            },
            Staker {
                address: "Bob".to_string(),
                stake: 50,
            },
        ];

        let winner = select_staker(&stakers);
        assert!(winner.is_some());
    }
}
