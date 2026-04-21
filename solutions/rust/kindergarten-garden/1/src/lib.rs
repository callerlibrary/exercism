pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    const STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    const PLANT_MAP: [&str; 256] = {
        let mut map = [""; 256];
        map[b'G' as usize] = "grass";
        map[b'C' as usize] = "clover";
        map[b'R' as usize] = "radishes";
        map[b'V' as usize] = "violets";
        map
    };

    let student_index = STUDENTS.iter().position(|&s| s == student).unwrap();
    let start = student_index * 2;

    diagram
        .lines()
        .flat_map(|row| row[start..start + 2].bytes().map(|b| PLANT_MAP[b as usize]))
        .collect()
}