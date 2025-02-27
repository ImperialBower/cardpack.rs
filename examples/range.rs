fn main() {
    let mut chart = vec![vec!["  ".to_string(); 13]; 13];
    let ranks = [
        "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
    ];

    for (i, &rank1) in ranks.iter().enumerate() {
        for (j, &rank2) in ranks.iter().enumerate() {
            if i < j {
                let iltg = format!("{}{}", rank1, rank2);
                chart[i][j] = iltg.clone();
            } else if i > j {
                let igtg = format!("{}{}", rank2, rank1);
                chart[i][j] = igtg.clone();
            } else {
                let ig = format!("{}{}", rank1, rank2);
                chart[i][j] = ig.clone();
            }
        }
    }

    for row in chart {
        for cell in row {
            print!("{:>3}", cell);
        }
        println!();
    }
}

// Original code
// fn main() {
//     let mut chart = vec![vec!["  "; 13]; 13];
//     let ranks = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];
//
//     for (i, &rank1) in ranks.iter().enumerate() {
//         for (j, &rank2) in ranks.iter().enumerate() {
//             if i < j {
//                 chart[i][j] = format!("{}{}", rank1, rank2).as_str();
//             } else if i > j {
//                 chart[i][j] = format!("{}{}", rank2, rank1).as_str();
//             } else {
//                 chart[i][j] = format!("{}{}", rank1, rank2).as_str();
//             }
//         }
//     }
//
//     for row in chart {
//         for cell in row {
//             print!("{:>3}", cell);
//         }
//         println!();
//     }
// }
