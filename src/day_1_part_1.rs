pub fn calc_fuel_req(modules: Vec<usize>) -> usize {
    modules.iter()
        .map(|m| (m / 3) - 2)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_1_data;

    use super::*;

    #[test]
    fn should_calc_fuel_req() {
        assert_eq!(2, calc_fuel_req(vec![12]));
        assert_eq!(2, calc_fuel_req(vec![14]));

        let res = calc_fuel_req(day_1_data::parse_input());
        assert!(res > 0);
    }
}