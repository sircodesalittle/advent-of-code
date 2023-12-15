/* Puzzle Input
Time:        45     97     72     95
Distance:   305   1062   1110   1695
*/

struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn get_ways_to_win(&self) -> u64 {
        let mut ways_to_win = Vec::new();
        for charge_time in 0..self.time {
            let time_remaining = self.time - charge_time;
            let speed = charge_time;
            let distance_traveled = time_remaining * speed;
            if distance_traveled > self.record_distance {
                ways_to_win.push(charge_time);
            }
        }
        return ways_to_win.len().try_into().unwrap();
    }
}

fn main() {
    let easy = Race {
        time: 7,
        record_distance: 9,
    };
    let easy2 = Race {
        time: 15,
        record_distance: 40,
    };
    let easy3 = Race {
        time: 30,
        record_distance: 200,
    };
    let result = easy.get_ways_to_win() * easy2.get_ways_to_win() * easy3.get_ways_to_win();
    dbg!(result);
    let race_1 = Race {
        time: 45,
        record_distance: 305,
    };
    let race_2 = Race {
        time: 97,
        record_distance: 1062,
    };
    let race_3 = Race {
        time: 72,
        record_distance: 1110,
    };
    let race_4 = Race {
        time: 95,
        record_distance: 1695,
    };
    let result2 = race_1.get_ways_to_win()
        * race_2.get_ways_to_win()
        * race_3.get_ways_to_win()
        * race_4.get_ways_to_win();
    dbg!(result2);

    let final_race = Race {
        time: 45977295,
        record_distance: 305106211101695,
    };
    let result = final_race.get_ways_to_win();
    dbg!(result);
}
