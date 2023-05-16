enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
    Area {residents: u64},
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> Self {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            // 👉 TODO Handle the other CitySize variants individually,
            //    in a similar way to how *town* is handled here
            CitySize::City => {
                let residents = 10_000;

                (
                    format!(
                        "a *city* of approximately {} residents",
                        residents
                    ),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;

                (
                    format!(
                        "a *metropolis* of approximately {} residents",
                        residents
                    ),
                    residents,
                )
            }
            CitySize::Area {residents} => {
                (
                    format!(
                        "an *area* of approximately {} residents",
                        residents
                    ),
                    residents
                )
            }
        };

        Self {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let rustville = City::new(CitySize::Area {residents: 100_000_000}, true);

    println!("This city is {}", rustville.description);
    println!("City is coastal: {}", rustville.is_coastal);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
