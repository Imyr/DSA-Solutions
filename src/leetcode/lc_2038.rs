pub fn winner_of_game(colors: String) -> bool {
    let (mut alice_turns, mut alice_string) = (0, 0);
    let (mut bob_turns, mut bob_string) = (0, 0);
    for c in colors.chars() {
        match c {
            'A' => {
                bob_string = 0;
                alice_string += 1;
                if alice_string == 3 {
                    alice_turns += 1;
                    alice_string = 2;
                }
            },
            'B' => {
                alice_string = 0;
                bob_string += 1;
                if bob_string == 3 {
                    bob_turns += 1;
                    bob_string = 2;
                }
            },
            _ => unreachable!(),
        }
    }     
    alice_turns > bob_turns
}