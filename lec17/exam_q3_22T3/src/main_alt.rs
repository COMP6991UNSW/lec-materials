use exam_q3_lib::DBMap;

#[derive(Debug)]
enum FormulaOneTeams {
    RedBullRacing,
    McLaren,
    Alpine
}

fn main() {
    let names = DBMap {
        data: vec![
            (1, ("Max Verstappen", "NED")),
            (3, ("Daniel Riccardo", "AUS")),
            (4, ("Lando Norris", "GBR")),
            (5, ("Sebastian Vettel", "GER")),
            (6, ("Nicholas Latifi", "CAN")),
            (7, ("Kimi Räikkönen", "FIN")),
            (9, ("Nikita Mazepin", "RAF")),
            (11, ("Sergio Pérez", "MEX"))
        ]
    };

    let teams = DBMap {
        data: vec![
            (1, FormulaOneTeams::RedBullRacing),
            (4, FormulaOneTeams::McLaren),
            (81, FormulaOneTeams::McLaren),
            (10, FormulaOneTeams::Alpine),
            (11, FormulaOneTeams::RedBullRacing),
        ]
    };

    let merged = names.merge(teams);

    for (key, (val, other_val)) in merged.data {
        let other_val = other_val.map(|f| format!("{f:?}")).unwrap_or(String::from("None"));
        println!("#{key}: {val:?} ({other_val})");
    }

}
