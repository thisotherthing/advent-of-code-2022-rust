fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 13140);
    assert_eq!(part_a(include_str!("input.txt")), 12560);
    assert_eq!(
        part_b(include_str!("example.txt")),
        r#"
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     
"#
        .trim_start()
    );
    assert_eq!(
        part_b(include_str!("input.txt")),
        r#"
###  #    ###   ##  #### ###   ##  #    
#  # #    #  # #  # #    #  # #  # #    
#  # #    #  # #  # ###  ###  #    #    
###  #    ###  #### #    #  # #    #    
#    #    #    #  # #    #  # #  # #    
#    #### #    #  # #    ###   ##  #### 
"#
        .trim_start()
    ); // PLPAFBCL
}

struct State {
    cycle: i64,
    register: i64,
}

fn part_a(input: &str) -> i64 {
    let mut state = State {
        cycle: 1,
        register: 1,
    };

    let mut singal_strenghts: i64 = 0;

    let mut tick = |state: &mut State| {
        state.cycle += 1;
        if state.cycle % 40 == 20 {
            singal_strenghts += state.cycle * state.register;
            // dbg!((&cycle, &reg));
        }
    };

    for line in input.lines() {
        let data = line.split_at(4);

        match data.0 {
            "noop" => {
                tick(&mut state);
            }
            "addx" => {
                let val = data.1.trim().parse::<i64>().unwrap();
                tick(&mut state);
                state.register += val;
                tick(&mut state);
            }
            _ => {}
        }
    }

    singal_strenghts
}

fn part_b(input: &str) -> String {
    let mut state = State {
        cycle: 1,
        register: 1,
    };

    let mut x_index = 0;

    let mut printer: Vec<&str> = vec![];

    let mut tick = |state: &mut State| {
        x_index = (state.cycle - 1) % 40;

        if x_index >= state.register - 1 && x_index <= state.register + 1 {
            print!("#");
            printer.push("#");
        } else {
            print!(" ");
            printer.push(" ");
        }
        // print!("({}, {}) ", x_index, state.register);
        // print!("{} ", &x_index);

        state.cycle += 1;

        if x_index == 39 {
            println!();
            printer.push("\n");
        }
    };

    for line in input.lines() {
        let data = line.split_at(4);

        match data.0 {
            "noop" => {
                tick(&mut state);
            }
            "addx" => {
                let val = data.1.trim().parse::<i64>().unwrap();
                tick(&mut state);
                tick(&mut state);
                state.register += val;
            }
            _ => {}
        }
    }

    printer.iter().cloned().collect()
}
