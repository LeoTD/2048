extern crate termios;

use rand::random;
use std::io::{self, Read};
use std::time::SystemTime;
use termios::{cfsetspeed, tcflush, tcsetattr, Termios, ECHO, ICANON};

struct GameObjects {
    board: [[u32; 4]; 4],
    score: u32,
    start_time: SystemTime,
    frequency: u32,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn print_board_debug(game: &GameObjects) {
    for row in 0..4 {
        for col in 0..4 {
            print!("{n:>3} ", n = (*game).board[row][col]);
        }
        println!();
    }
    println!();
}

fn print_board(game: &GameObjects) {
    print!("\x1B[2J\x1B[2;H --A Rusty 2048--\x1B[3;H");
    println!();
    for row in 0..4 {
        print!("\x1B[2K");
        for col in 0..4 {
            if (*game).board[row][col] == 0 {
                print!(" \x1B[0m{:>3} \x1B[0m", (*game).board[row][col]);
            } else {
                print!(
                    " \x1B[47m\x1B[30m\x1B[1m{:>3} \x1B[0m",
                    (*game).board[row][col]
                );
            }
        }
        println!();
        println!();
    }
    println!();
}

fn shift_dir(game: &mut GameObjects, dir: Direction) {
    let mut new_stack: [u32; 4];
    match dir {
        Direction::UP => {
            for i in 0..4 {
                new_stack = [
                    (*game).board[0][i],
                    (*game).board[1][i],
                    (*game).board[2][i],
                    (*game).board[3][i],
                ];
                new_stack = shift_arr(new_stack);
                new_stack = add_new_tiles(new_stack, (*game).frequency);

                (*game).board[0][i] = new_stack[0];
                (*game).board[1][i] = new_stack[1];
                (*game).board[2][i] = new_stack[2];
                (*game).board[3][i] = new_stack[3];
            }
        }
        Direction::DOWN => {
            for i in 0..4 {
                new_stack = [
                    (*game).board[3][i],
                    (*game).board[2][i],
                    (*game).board[1][i],
                    (*game).board[0][i],
                ];
                new_stack = shift_arr(new_stack);
                new_stack = add_new_tiles(new_stack, (*game).frequency);

                (*game).board[3][i] = new_stack[0];
                (*game).board[2][i] = new_stack[1];
                (*game).board[1][i] = new_stack[2];
                (*game).board[0][i] = new_stack[3];
            }
        }
        Direction::LEFT => {
            for i in 0..4 {
                new_stack = [
                    (*game).board[i][0],
                    (*game).board[i][1],
                    (*game).board[i][2],
                    (*game).board[i][3],
                ];
                new_stack = shift_arr(new_stack);
                new_stack = add_new_tiles(new_stack, (*game).frequency);

                (*game).board[i][0] = new_stack[0];
                (*game).board[i][1] = new_stack[1];
                (*game).board[i][2] = new_stack[2];
                (*game).board[i][3] = new_stack[3];
            }
        }
        Direction::RIGHT => {
            for i in 0..4 {
                new_stack = [
                    (*game).board[i][3],
                    (*game).board[i][2],
                    (*game).board[i][1],
                    (*game).board[i][0],
                ];
                new_stack = shift_arr(new_stack);
                new_stack = add_new_tiles(new_stack, (*game).frequency);

                (*game).board[i][3] = new_stack[0];
                (*game).board[i][2] = new_stack[1];
                (*game).board[i][1] = new_stack[2];
                (*game).board[i][0] = new_stack[3];
            }
        }
    }
}

fn shift_arr(arr: [u32; 4]) -> [u32; 4] {
    let mut new_arr: [u32; 4] = [0; 4];
    let mut new_index: usize = 0;

    for i in 0..4 {
        if arr[i] != 0 {
            if new_arr[new_index] == 0 {
                new_arr[new_index] = arr[i];
            } else if new_arr[new_index] == arr[i] {
                new_arr[new_index] += arr[i];
                new_index += 1;
            } else {
                new_index += 1;
                new_arr[new_index] = arr[i];
            }
        }
    }

    return new_arr;
}

fn add_new_tiles(mut arr: [u32; 4], frequency: u32) -> [u32; 4] {
    let mut rand: usize;

    // For each tile, if it's empty get a random number. on a 1/fq roll, convert tile to a 2.
    for i in 0..4 {
        if arr[i] == 0 {
            rand = random::<usize>() % (frequency as usize);
            if rand == 0 {
                arr[i] = 2;
            }
        }
    }
    return arr;
}

fn set_initial_tiles(game: &mut GameObjects) {
    (*game).board[0][0] = 2;
    (*game).board[0][1] = 2;
    (*game).board[0][2] = 2;
    (*game).board[0][3] = 2;
    (*game).board[1][2] = 2;
    (*game).board[2][2] = 2;
    (*game).board[2][3] = 2;
    (*game).board[3][0] = 2;
    (*game).board[3][1] = 2;
    (*game).board[3][2] = 2;
    (*game).board[3][3] = 2;
}

fn setup_fd(fd: i32) -> io::Result<()> {
    let mut termios = Termios::from_fd(fd)?;

    termios.c_lflag = !(ICANON | ECHO);

    cfsetspeed(&mut termios, termios::B9600)?;
    tcsetattr(fd, termios::TCSANOW, &termios)?;
    tcflush(fd, termios::TCIOFLUSH)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // let mut input: String = String::new();
    let mut game: GameObjects = GameObjects {
        board: [[0; 4], [0; 4], [0; 4], [0; 4]],
        score: 0,
        start_time: SystemTime::now(),
        frequency: 7,
    };
    set_initial_tiles(&mut game);
    game.score += 1;

    // Set up terminal.
    let fd = 0;
    let original_termios = Termios::from_fd(fd)?;
    setup_fd(fd);

    // print!("\x1B[2J");
    // print_board_debug(&game);
    // shift_dir(&mut game, Direction::DOWN);
    // print_board_debug(&game);
    // shift_dir(&mut game, Direction::RIGHT);
    // print_board_debug(&game);
    // shift_dir(&mut game, Direction::UP);
    // print_board_debug(&game);
    // shift_dir(&mut game, Direction::LEFT);
    // print_board_debug(&game);
    let mut input_iter = std::io::stdin().bytes();
    print_board(&game);
    loop {
        // io::stdin().read_line(&mut input)?;
        let input: Option<i32> = input_iter
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as i32);

        match input {
            Some(65) => {
                println!("Shifting UP");
                shift_dir(&mut game, Direction::UP);
                print_board(&game);
            }
            Some(66) => {
                println!("Shifting DOWN");
                shift_dir(&mut game, Direction::DOWN);
                print_board(&game);
            }
            Some(67) => {
                println!("Shifting RIGHT");
                shift_dir(&mut game, Direction::RIGHT);
                print_board(&game);
            }
            Some(68) => {
                println!("Shifting LEFT");
                shift_dir(&mut game, Direction::LEFT);
                print_board(&game);
            }
            Some(27) | Some(91) => (),
            Some(113) => break,
            _ => (),
        }
    }

    // Restore terminal to original settings.
    tcsetattr(fd, termios::TCSANOW, &original_termios)?;

    Ok(())
}
