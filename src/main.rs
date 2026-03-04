use std::{io::{self, Write}};
use system_pause::{pause, pause_for_time};
use rand::prelude::*;

struct Player {
    name: String,
    symbol: char,
}

enum MenuOption{
    NewGame,
    Exit
}

enum PlayerTurn{
    Player1,
    Player2
}

enum FlowAction{
    Return,
    Continue
}

enum GameResult{
    Winner(char),
    Draw,
    Ongoing,
}

fn main() {
    let mut player1: Player = Player { name: String::new(), symbol: ' ' };
    let mut player2: Player = Player { name: String::new(), symbol: ' ' };

    loop {
        clearscreen::clear().unwrap();

        match main_panel() {
            Ok(MenuOption::NewGame) => {
                let mut grid:[[char;3];3] = [[' ';3];3];

                new_player(&mut player1, 1);
                new_player(&mut player2, 2);
                let mut is_player_turn = sort_player(&mut player1, &mut player2);

                pause_for_time!(3, "────────── GAME STARTING IN {} ──────────");
                
                loop {
                    let mut winner_symbol:GameResult = GameResult::Ongoing;

                    clearscreen::clear().unwrap();
                    print_grid(&grid);

                    match is_player_turn{
                        //player 1 round
                        PlayerTurn::Player1 => {
                            let flow_action = player_round(&mut grid, &player1, &player2, &mut winner_symbol);
                            is_player_turn = PlayerTurn::Player2;
                            match flow_action {
                                FlowAction::Continue => break,
                                FlowAction::Return => {

                                }
                            }
                        }

                        //player 2 round
                        PlayerTurn::Player2 => {
                        let flow_action = player_round(&mut grid, &player2, &player1, &mut winner_symbol);
                        is_player_turn = PlayerTurn::Player1;
                        match flow_action {
                                FlowAction::Continue => break,
                                FlowAction::Return => {
                                    
                                }
                        }
                        }
                    }
                }
            }
            Ok(MenuOption::Exit) => {
                break;
            }
            Err(_) => {
                print_restart_message();
                continue;
            }
        } 
    }
}

fn main_panel() -> Result<MenuOption, ()> {
    let mut input: String = String::new();
    print!("┌────── TicTacToe    Game ──────┐\n");
    println!("│                 │             │");
    print!("│ 1 - New Game    │   2 - Quit  │\n");
    println!("│                 │             │");
    print!("└───────────────────────────────┘");
    print!("\nWhat do you wanna do?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim(){
        "1" => Ok(MenuOption::NewGame),
        "2" => Ok(MenuOption::Exit),
        _ => Err(())
    }
}


fn new_player(player: &mut Player, player_number:i8) {
    let mut name: String = String::new();
    clearscreen::clear().unwrap();
    println!("─────────── P L A Y E R  {} ────────────", player_number);
    print!("\nYour name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    player.name = name.trim().to_string();
}

fn sort_player(player1:&mut Player, player2:&mut Player) -> PlayerTurn{
    let mut rng = rand::rng();
    let sorted = rng.random_range(0..=1);
    print!("\n");
    pause_for_time!(1, "SORTING THE FIRST PLAYER...{}");
    if sorted == 0{
        println!("\n{} you're first!\n", player1.name.to_uppercase());
        pause!("[ENTER TO CONTINUE]");
        player1.symbol = 'X';
        player2.symbol = 'O';
        return PlayerTurn::Player1;
    } else {
        println!("\n{} you're first!\n", player2.name.to_uppercase());
        pause!("[ENTER TO CONTINUE]");
        player1.symbol = 'O';
        player2.symbol = 'X';
        return PlayerTurn::Player2;
    }
}

fn position_input(player: &Player) -> Result<(usize, usize), ()> {
    let mut x: String = String::new();
    let mut y: String = String::new();
    println!("\n\n→ {}, YOU'RE TURN! ←\n",player.name.to_uppercase());
    print!("Type the ROW value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut x).unwrap();
    print!("Type the COL value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut y).unwrap();
    let x: usize = match x.trim().parse() {Ok(num) => num, Err(_) => 3};
    let y: usize = match y.trim().parse() {Ok(num) => num, Err(_) => 3};
    
    if x < 3 && y < 3 {
        Ok((x, y))
    } else {
        Err(())
    }
}

fn print_grid(grid: &[[char;3];3]) {
    println!("┌──── THE GAME PANEL ────┐");
    print!("│                        │\n");
    println!("│        0   1   2       │");
    print!("│                        │\n");
    for x in 0..grid.len() {
        print!("│  {}   │", x);
        for y in 0..grid[x].len() {
            print!(" {} │", grid[x][y]);
        }
        
        print!("     │");

        if x <= 1 {
            print!("\n│      ────+───+────     │\n");
        }
    }
    print!("\n│                        │\n│                        │\n└──── THE GAME PANEL ────┘");
}

fn insert_grid(x: usize, y: usize, grid: &mut [[char;3];3], player_symbol: char) {
    grid[x][y] = player_symbol;
}

fn print_restart_message() {
    println!("\nIT'S NOT A VALID VALUE!\n");
    pause!("[ENTER TO CONTINUE]");
}

fn player_round_input(grid: &mut [[char;3];3], player: &Player) -> FlowAction {
    
    match position_input(&player) {
        Ok((x,y)) => {
            if grid[x][y] != ' ' {
                 pause!("\nPosition already taken!\n\n[ENTER TO CONTINUE]");
                 clearscreen::clear().unwrap();
                 print_grid(&grid);
                 return FlowAction::Return;
            } else {
                insert_grid(x, y, grid, player.symbol);
                return FlowAction::Continue;
            }
        }
        Err(_) => {
            print_restart_message();
            clearscreen::clear().unwrap();
            print_grid(&grid);
            return FlowAction::Return;
        }
    }
}

fn player_round(grid:&mut [[char;3];3], player1:&Player, player2:&Player, game_result:&mut GameResult) -> FlowAction{
    loop {
        let flow_action:FlowAction = player_round_input(grid, &player1);
        *game_result = verify_result(&grid, &player1, &player2);
        match flow_action {
            FlowAction::Continue => break,
            FlowAction::Return => continue
        }
    }

    clearscreen::clear().unwrap();
    print_grid(&grid);

    let flow_action = print_winner(game_result, &player1, &player2);
    match flow_action{
        FlowAction::Continue => {
             pause!("[ENTER TO CONTINUE]");
            return FlowAction::Continue;
        } 
        FlowAction::Return => {
            return FlowAction::Return;
        }
    }
}

fn verify_result(grid:&[[char;3];3], player1:&Player, player2:&Player) -> GameResult{
    let len = grid.len();
    let mut qtd_filled = 0;

    //first diagonal
    if (0..len).all(|i| grid[i][i] == player1.symbol) { return GameResult::Winner(player1.symbol); }
    if (0..len).all(|i| grid[i][i] == player2.symbol) { return GameResult::Winner(player2.symbol); }
    //second diagonal
    if (0..len).all(|i| grid[i][len-1-i] == player1.symbol) { return GameResult::Winner(player1.symbol); }
    if (0..len).all(|i| grid[i][len-1-i] == player2.symbol) { return GameResult::Winner(player2.symbol); }
   //row
   for i in 0..len{
    if (0..len).all(|j| grid[i][j] == player1.symbol) { return GameResult::Winner(player1.symbol); }
    if (0..len).all(|j| grid[i][j] == player2.symbol) { return GameResult::Winner(player2.symbol);  }
   }
   //col
   for i in 0..len{
    if (0..len).all(|j| grid[j][i] == player1.symbol) { return GameResult::Winner(player1.symbol); }
    if (0..len).all(|j| grid[j][i] == player2.symbol) { return GameResult::Winner(player2.symbol);  }
   }
   //full grid
   for i in 0..len{
    for j in 0..len {
    if grid[i][j] != ' '{ 
        qtd_filled += 1;
     }
    }
   }
    if qtd_filled == len*len {
        return GameResult::Draw;
    }

   return GameResult::Ongoing;
}

fn print_winner(game_result:&GameResult, player1:&Player, player2:&Player) -> FlowAction{
    match game_result {
        GameResult::Winner(symbol) if *symbol == player1.symbol => {
            println!("\n\n⋆˚｡⋆ {} WINS! ⋆˚｡⋆\n", player1.name.to_uppercase());
            return FlowAction::Continue;
        }
        GameResult::Winner(symbol) if *symbol == player2.symbol => {
            println!("\n\n⋆˚｡⋆ {} WINS! ⋆˚｡⋆\n", player2.name.to_uppercase());
            return FlowAction::Continue;
        } 
        GameResult::Draw => {
            println!("\n\n NO WINS! :(  \n");
            return FlowAction::Continue;
        }
        _ => {
            return FlowAction::Return;
        }
    }
}
