/*
possible <- Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
possible <- Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
impossible <-Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
impossible <-Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
possible <- Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*/

pub fn part1(input: String) {
    let cubes: [(&str, u16); 3] = [("red", 12), ("green", 13), ("blue", 14)];
    let total: u16 = input
        .lines()
        // this part can be reworked
        .map(|line| line.split(';').map(|round| round).collect::<Vec<&str>>())
        .enumerate()
        .map(|(i, round)| {
            let valid = round.iter().all(|game| {
                cubes.iter().all(|(color, max)| {
                    if let Some(x) = game.find(color) {
                        let mut chars = game.chars();
                        let n = match chars.nth(x - 3).unwrap() {
                            '1' => 1,
                            '2' => 2,
                            _ => 0,
                        } * 10
                            + chars.nth(0).unwrap() as u16
                            - 0x30;
                        return n <= (*max);
                    }
                    true
                })
            });
            return (i as u16 + 1) * (valid as u16);
        })
        .sum();
    println!("{total}");
}
