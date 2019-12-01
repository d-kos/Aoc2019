pub mod data_1_12_2019;

fn calc_fuel_req(modules: Vec<usize>) -> usize {
    modules.iter()
        .map(|m| (m / 3) - 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calc_fuel_req() {
        assert_eq!(2, calc_fuel_req(vec![12]));
        assert_eq!(2, calc_fuel_req(vec![14]));

        let res = calc_fuel_req(data_1_12_2019::parse_input());
        assert!(res > 0);
    }
}

fn main() {
    println!("Answer to part one is = {}", calc_fuel_req(data_1_12_2019::parse_input()));
}