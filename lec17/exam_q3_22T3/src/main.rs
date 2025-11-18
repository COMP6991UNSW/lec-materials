use exam_q3_lib::DBMap;

fn main() {
    let names = DBMap {
        data: vec![
            (1, "Max Verstappen"),
            (3, "Daniel Riccardo"),
            (4, "Lando Norris"),
            (5, "Sebastian Vettel"),
            (6, "Nicholas Latifi"),
            (7, "Kimi Räikkönen"),
            (9, "Nikita Mazepin"),
            (11, "Sergio Pérez")
        ]
    };

    let teams = DBMap {
        data: vec![
            (1, "Red Bull Racing"),
            (4, "McLaren"),
            (81, "McLaren"),
            (10, "Alpine"),
            (11, "Red Bull Racing"),
        ]
    };

    let merged = names.merge(teams);

    for (key, (val, other_val)) in merged.data {
        println!("#{key}: {val} ({})", other_val.unwrap_or("None"));
    }

}
