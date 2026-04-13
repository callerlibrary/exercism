pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let key = [
        "No", "One", "Two", "Three", "Four", "Five",
        "Six", "Seven", "Eight", "Nine", "Ten",
    ];
    let mut song = String::new();

    for i in 0..take_down {
        let current_num = start_bottles - i;
        let next_num = current_num - 1;

        let current_word = key[current_num as usize];
        let next_word = key[next_num as usize].to_lowercase();

        let bottle1 = if current_num == 1 { "bottle" } else { "bottles" };
        let bottle2 = if next_num == 1 { "bottle" } else { "bottles" };

        let verse = format!(
            "{0} green {1} hanging on the wall,\n\
             {0} green {1} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {2} green {3} hanging on the wall.\n",
            current_word, bottle1, next_word, bottle2
        );

        song.push_str(&verse);

        if i < take_down - 1 {
            song.push_str("\n");
        }
    }

    song
}