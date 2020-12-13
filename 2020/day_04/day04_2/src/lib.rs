struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

impl Passport {
    pub fn new() -> PassportBuilder {
        PassportBuilder {
            passport: Self {
                byr: false,
                iyr: false,
                eyr: false,
                hgt: false,
                hcl: false,
                ecl: false,
                pid: false,
            },
        }
    }
    pub fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

struct PassportBuilder {
    passport: Passport,
}

impl PassportBuilder {
    pub fn byr(mut self, input: &str) -> Self {
        if let Ok(number) = input.parse::<u32>() {
            self.passport.byr = (1920..=2002).contains(&number);
        }
        self
    }

    pub fn iyr(mut self, input: &str) -> Self {
        if let Ok(number) = input.parse::<u32>() {
            self.passport.iyr = (2010..=2020).contains(&number);
        }
        self
    }

    pub fn eyr(mut self, input: &str) -> Self {
        if let Ok(number) = input.parse::<u32>() {
            self.passport.eyr = (2020..=2030).contains(&number);
        }
        self
    }

    pub fn hgt(mut self, input: &str) -> Self {
        if input.ends_with("in") {
            let rest = input.replace("in", "");
            if let Ok(number) = rest.parse::<u32>() {
                self.passport.hgt = (59..=76).contains(&number);
            }
        } else if input.ends_with("cm") {
            let rest = input.replace("cm", "");
            if let Ok(number) = rest.parse::<u32>() {
                self.passport.hgt = (150..=193).contains(&number);
            }
        }
        self
    }

    pub fn hcl(mut self, input: &str) -> Self {
        if input.len() != 7 {
            return self;
        }

        let chars = input.chars().enumerate();
        self.passport.hcl = chars.fold(true, |acc, (i, c)| {
            acc && (i == 0 && c == '#' || i > 0 && c.is_digit(16))
        });

        self
    }

    pub fn ecl(mut self, input: &str) -> Self {
        self.passport.ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input);
        self
    }

    pub fn pid(mut self, input: &str) -> Self {
        self.passport.pid =
            input.len() == 9 && input.chars().fold(true, |acc, c| acc && c.is_digit(10));
        self
    }

    pub fn build(self) -> Passport {
        self.passport
    }
}

pub fn solve(input: &str) -> usize {
    let passport_strings: Vec<&str> = input.trim_end().split("\n\n").collect();
    let mut valids = 0;
    for passport in passport_strings.iter() {
        let decomposed: Vec<&str> = passport.trim_end().split_whitespace().collect();

        let mut pass_builder = Passport::new();
        for field in decomposed.iter() {
            let split: Vec<&str> = field.split(':').collect();
            let key = split[0];
            let data = split[1];
            match key {
                "byr" => pass_builder = pass_builder.byr(data),
                "iyr" => pass_builder = pass_builder.iyr(data),
                "eyr" => pass_builder = pass_builder.eyr(data),
                "hgt" => pass_builder = pass_builder.hgt(data),
                "hcl" => pass_builder = pass_builder.hcl(data),
                "ecl" => pass_builder = pass_builder.ecl(data),
                "pid" => pass_builder = pass_builder.pid(data),
                _ => (),
            }
        }

        valids += if pass_builder.build().valid() { 1 } else { 0 }
    }

    valids
}

#[test]
fn aoc_invalid_example() {
    let input = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
    let expected = 0;
    let result = solve(input);
    assert_eq!(expected, result);
}

#[test]
fn aoc_valid_example() {
    let input = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    let expected = 4;
    let result = solve(input);
    assert_eq!(expected, result);
}
