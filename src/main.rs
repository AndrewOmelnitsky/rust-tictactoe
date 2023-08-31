use std::io;

fn print_map(game_map: &mut Vec<Vec<i32>>) {
    fn format_row(row: Vec<i32>) -> String {
        let string_row = row
            .iter()
            .map(|x| x.to_string())
            .map(|x| if x == "0" { " ".to_string() } else { x })
            .collect::<Vec<String>>();
        format!(" {} \n", string_row.join(" | "))
    }

    let formatted_map = game_map
        .iter()
        .map(|x| format_row(x.to_vec()))
        .collect::<Vec<String>>();
    let repeat_sep = formatted_map[0].len() - 1;
    let row_sep = format!("{}\n", "-".repeat(repeat_sep));

    println!("{}", formatted_map.join(row_sep.as_str()));
}

fn input_int(default: i32) -> i32 {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        // Ok(_) => print!("\n"),
        Ok(_) => {}
        Err(_e) => println!("{}", _e),
    }

    user_input.trim().parse().unwrap_or(default)
}

fn input_step(game_map: &mut Vec<Vec<i32>>, player: i32) {
    let mut x: i32;
    let mut y: i32;

    let game_map_size = game_map.len() as i32;

    loop {
        println!("Player {}, now it's your turn", player);
        println!("input x: ");
        x = input_int(-1);

        println!("input y: ");
        y = input_int(-1);

        x = x - 1;
        y = game_map_size - y;

        if !((x >= 0 && game_map_size > x) && (y >= 0 && game_map_size > y)) {
            println!(
                "Coordinates is invalid or out of bounds. Bounds is [1 {}]",
                game_map_size
            );
            continue;
        }

        if game_map[y as usize][x as usize] != 0 {
            println!(
                "This cell is already in use by the player: {}",
                game_map[y as usize][x as usize]
            );
            continue;
        }

        break;
    }

    game_map[y as usize][x as usize] = player;
}

fn dose_game_end(game_map: &mut Vec<Vec<i32>>, necessary_sequence: usize) -> i32 {
    let mut count_free = 0;
    let game_map_size = game_map.len();

    for i in 0..game_map_size {
        for j in 0..game_map_size {
            if game_map[i][j] == 0 {
                count_free += 1;
            }
        }
    }

    for i in 0..game_map_size {
        let mut count_sequence = 0;
        let mut current_symbol: i32 = -1;

        for j in 0..game_map_size {
            if current_symbol == game_map[i][j] && game_map[i][j] != 0 {
                count_sequence += 1;
            } else {
                current_symbol = game_map[i][j];
                count_sequence = 1;
            }

            if count_sequence == necessary_sequence {
                return current_symbol;
            }
        }
    }

    for i in 0..game_map_size {
        let mut count_sequence = 0;
        let mut current_symbol: i32 = -1;

        for j in 0..game_map_size {
            if current_symbol == game_map[j][i] && game_map[j][i] != 0 {
                count_sequence += 1;
            } else {
                current_symbol = game_map[j][i];
                count_sequence = 1;
            }

            if count_sequence == necessary_sequence {
                return current_symbol;
            }
        }
    }

    for mut i in 0..(game_map_size * 2 - 1) {
        let mut count_sequence = 0;
        let mut current_symbol: i32 = -1;

        let is_second_part = i >= game_map_size;
        i = if is_second_part {
            game_map_size * 2 - i - 2
        } else {
            i
        };

        if necessary_sequence - 1 > i {
            continue;
        };

        for j in 0..(i + 1) {
            if !is_second_part {
                if current_symbol == game_map[i - j][j] && game_map[i - j][j] != 0 {
                    count_sequence += 1;
                } else {
                    current_symbol = game_map[i - j][j];
                    count_sequence = 1;
                }
            } else {
                if current_symbol == game_map[game_map_size - 1 - i + j][game_map_size - 1 - j]
                    && game_map[game_map_size - 1 - i + j][game_map_size - 1 - j] != 0
                {
                    count_sequence += 1;
                } else {
                    current_symbol = game_map[game_map_size - 1 - i + j][game_map_size - 1 - j];
                    count_sequence = 1;
                }
            }

            if count_sequence == necessary_sequence {
                return current_symbol;
            }
        }
    }

    for mut i in 0..(game_map_size * 2 - 1) {
        let mut count_sequence = 0;
        let mut current_symbol: i32 = -1;

        let is_second_part = i >= game_map_size;
        i = if is_second_part {
            game_map_size * 2 - i - 2
        } else {
            i
        };

        if necessary_sequence - 1 > i {
            continue;
        };

        for j in 0..(i + 1) {
            if !is_second_part {
                if current_symbol == game_map[i - j][game_map_size - 1 - j]
                    && game_map[i - j][game_map_size - 1 - j] != 0
                {
                    count_sequence += 1;
                } else {
                    current_symbol = game_map[i - j][game_map_size - 1 - j];
                    count_sequence = 1;
                }
            } else {
                if current_symbol == game_map[game_map_size - 1 - i + j][j]
                    && game_map[game_map_size - 1 - i + j][j] != 0
                {
                    count_sequence += 1;
                } else {
                    current_symbol = game_map[game_map_size - 1 - i + j][j];
                    count_sequence = 1;
                }
            }

            if count_sequence == necessary_sequence {
                return current_symbol;
            }
        }
    }

    if count_free == 0 {
        return 0;
    }

    -1
}

fn main() {
    println!("Input game map size: ");
    let game_map_size: usize = input_int(3) as usize;

    println!("Input necessary sequence to wine: ");
    let necessary_sequence: usize = input_int(3) as usize;

    let mut game_map = vec![vec![0; game_map_size]; game_map_size];

    println!("Input number of players: ");
    let players = input_int(2);

    let mut current_player = 1;
    let mut winner: i32;

    loop {
        print_map(&mut game_map);
        input_step(&mut game_map, current_player);

        current_player = current_player % players + 1;
        winner = dose_game_end(&mut game_map, necessary_sequence);
        if winner != -1 {
            break;
        }
    }

    print_map(&mut game_map);

    if winner == 0 {
        println!("Draw!!!");
    } else {
        println!("Player {} is winner!!!", winner);
    }
}
