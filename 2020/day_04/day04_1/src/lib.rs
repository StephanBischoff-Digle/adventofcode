struct Passport {
    pub byr: bool,
    pub iyr: bool,
    pub eyr: bool,
    pub hgt: bool,
    pub hcl: bool,
    pub ecl: bool,
    pub pid: bool,
}

impl Passport {
    pub fn new() -> Self {
        Self {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
        }
    }
    pub fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

pub fn solve(input: &str) -> usize {
    let passport_strings: Vec<&str> = input.trim_end().split("\n\n").collect();
    let mut valids = 0;
    for passport in passport_strings.iter() {
        let decomposed: Vec<&str> = passport.trim_end().split_whitespace().collect();

        let mut pass = Passport::new();
        for field in decomposed.iter() {
            match &field[..3] {
                "byr" => pass.byr = true,
                "iyr" => pass.iyr = true,
                "eyr" => pass.eyr = true,
                "hgt" => pass.hgt = true,
                "hcl" => pass.hcl = true,
                "ecl" => pass.ecl = true,
                "pid" => pass.pid = true,
                _ => (),
            }
        }

        valids += if pass.valid() { 1 } else { 0 }
    }

    valids
}

#[test]
fn aoc_example() {
    let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    let expected = 2;
    let result = solve(input);
    assert_eq!(expected, result);
}
