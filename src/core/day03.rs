use regex::Regex;

/// Receives input and prints output
pub fn day03_part1(lines: &mut dyn Iterator<Item = String>) {
    let multiplications: u32 = day03_part1_handler(lines);
    println!("Sum {}", multiplications);
}

pub fn day03_part2(lines: &mut dyn Iterator<Item = String>) {
    let multiplications: u32 = day03_part2_handler(lines);
    println!("Sum {}", multiplications);
}

fn day03_part1_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    lines.map(line_value).sum()
}

fn day03_part2_handler(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let joined = lines
        .map(|x| x.trim().to_string())
        .reduce(|a, b| format!("{}{}", a, b));
    line_value2(joined.unwrap())
}

fn line_value(line: String) -> u32 {
    let mut commands = vec![];
    for command in MultiplyCommand::regex()
        .find_iter(&line)
        .map(|x| x.as_str())
    {
        commands.push(MultiplyCommand::parse_mul_command(command));
    }

    commands.iter().flatten().map(|cmd| cmd.value()).sum()
}

fn line_value2(line: String) -> u32 {
    let mut commands: Vec<CommandType> = vec![];

    MultiplyCommand::regex().captures_iter(&line).for_each(|c| {
        let rng = c.get(0).unwrap().range();
        let (_fullstr, [lhs, rhs]) = c.extract();
        let l = lhs.parse::<u32>();
        let r = rhs.parse::<u32>();
        if let (Ok(lhs), Ok(rhs)) = (l, r) {
            let cmd = MultiplyCommand { lhs, rhs };
            commands.push(CommandType::Multiply {
                idx: rng.start,
                cmd,
            });
        }
    });

    do_not_regex().captures_iter(&line).for_each(|c| {
        let rng = c.get(0).unwrap().range();
        commands.push(CommandType::DoNot { idx: rng.start })
    });
    do_regex().captures_iter(&line).for_each(|c| {
        let rng = c.get(0).unwrap().range();
        commands.push(CommandType::Do { idx: rng.start })
    });

    commands.sort_by(|a, b| CommandType::get_idx(&a).cmp(CommandType::get_idx(&b)));

    let mut enabled = true;
    let mut total: u32 = 0;
    for cmd in commands {
        match cmd {
            CommandType::DoNot { idx: _ } => enabled = false,
            CommandType::Do { idx: _ } => enabled = true,
            CommandType::Multiply { idx: _, cmd } => {
                if enabled {
                    total += cmd.value();
                }
            }
        }
    }
    total
}

enum CommandType {
    DoNot { idx: usize },
    Do { idx: usize },
    Multiply { idx: usize, cmd: MultiplyCommand },
}

impl CommandType {
    fn get_idx(cmd: &CommandType) -> &usize {
        match cmd {
            CommandType::DoNot { idx } => idx,
            CommandType::Do { idx } => idx,
            CommandType::Multiply { idx, cmd: _ } => idx,
        }
    }
}

fn do_not_regex() -> Regex {
    Regex::new(r"don't\(\)").unwrap()
}

fn do_regex() -> Regex {
    Regex::new(r"do\(\)").unwrap()
}

#[derive(Eq, PartialEq, Debug)]
struct MultiplyCommand {
    lhs: u32,
    rhs: u32,
}

impl MultiplyCommand {
    fn value(&self) -> u32 {
        self.lhs * self.rhs
    }

    fn regex() -> Regex {
        Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
    }

    fn parse_mul_command(input: &str) -> Option<MultiplyCommand> {
        let reg = MultiplyCommand::regex();
        let mut itr = reg.captures_iter(input);
        match itr.next() {
            Some(c) => {
                let (_, [lhs, rhs]) = c.extract();
                let l = lhs.parse::<u32>();
                let r = rhs.parse::<u32>();
                if let (Ok(lhs), Ok(rhs)) = (l, r) {
                    Some(MultiplyCommand { lhs, rhs })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        let lines: Vec<String> =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                .split('\n')
                .map(|x| x.to_string())
                .collect();
        lines
    }

    fn sample_data2() -> Vec<String> {
        let lines: Vec<String> =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .split('\n')
                .map(|x| x.to_string())
                .collect();
        lines
    }
    fn sample_data3() -> Vec<String> {
        // 4 don't()
        // 5 do()
        //let lines: Vec<String> = "[#from())when()/}+%mul(982,733)mul(700,428)}},
        //don't(){:,$+mul(395,45)[; what()!mul(328,373)?!, <-)mul(139,254)^>#)*,[,&when()mul(719,688)-@,from()-@mul(616,640)from()!~{[' mul(666,309)(mul(889,261){select();why())<(/mul(364,750)>:+(:/mul(911,969)mul(360,631)~select() )$>mul(692,527)^mul(823,512)!]{where():&;,how()
        //don't()'?,*where()<[>!mul(345,427))~
        //do()((how()@mul(394,662)[mul(903,277)why()~from()mul(249,126)>?/-what()'when()from()[who()mul(823,356)$mul(257,129),%*what()+]when()(mul(571,713)+why()]#([mul(230,759)'</>%^}$]mul(978,804)%>~!{-&
        //don't() &'how() why()why(379,123)mul(816,166)}from(576,921) ?/when()?when()mul(946,526)/*mul(321,352)> where()'^ select(925,337)>[mul(233,433)<!}%;%:^>^mul(247,132)select(){[ ]!~ mul(104,931)why()&-+)~:<from()mul(587,547)<(where()where()where()how()mul(267,595)who()mul(834,996)where()select()@[mul(172,45)#+why()how()how()!#]~mul(3,396)why()~/mul(406,706){mul(209,557)?where() ?mul(716,930)what();&mul(92,580)'from()*[#!)where()when()mul(538,570)where()<when()/+-what()]mul(846,458)where()$why(){mul(18,615)when()#};:select()!*^mul(666,191):#from()when()from(),!^mul(837,567) #from()/when(612,5){/:mul(456,298)mul(570,670)mul(8,433)select()~}why()[^mul(358,86)who()/#mul(330,764)(how(905,838)from()#mul(683,182)]-!%:<[(why(257,862)mul(16,909);&who()*@where()when()mul(53,161)'where()?] [!mul(526,714)['}'@when()mul(101,388)?:how()select(762,661)where()%-<mul(862,638)what()select(),~^(mul(106,417){what()select()+from()}mul(946,829)^@mul(924,557),from()!when()mul(216,185)%mul(639,534) )!#<:]from()mul(710,22))>}#,}-$>&mul(412,598)*who()where();@why()
        //do()mul(814,674)%where(){;~mul(654,224)/(what()][^how(190,339)#mul(690,95)&}what()*[mul(821,149) :-who()from()~mul(279,247);)mul(66,274)who()$)why()@where(395,477)~<mul(550,43)]^(why()>:/~mul(442,748)*when()from()'who()}]^~%mul(130,259)}why()<})how(){^mul(768,298)>{-+when(366,794):-mul(489,845)*mul(442,721)-$[ {)mul(283,227)@how()[where()>mul(862,708)'who()?*!where(){mul(182,377)#^mul-[;select()^-!mul(472,672)'select()mul(117,275))$where()$('select(),$(mul(409,378)where()where()?'mul(448,267)]!,:&;what()%>what(265,341)mul(916,448)~when()/!/ ;how()mul(849,877)what()<mul(444,734))>*?
        //do()mul(851,406)~/%~mul(480,848)from(607,219)who()@~mul(803,80)
        //don't(){*how()%{!where()select()mul(482,344),]/mul(702,892)how()@)']mul(552,653))&<^,^+#mul(13,654) /?:mul&from()what()mul(247,264)<$'{*^how();;mul(453,796)&+select(338,67)mul(201,343)>select()) /who()mul(360,411)what()why()@mul(657,810):/}&$:+-<mul(712,186)$'<~)when()!mul(241,637)where(),,'!<%mul(829,983)mul(388,384)@mul(47,882)>^what()
        //do()what()<mul(796,344);'(what()+mul(460,350)how():,!how()mul(38,82)###}]&&from()select()mul(402,61)^]]-]mul(159,67))@who()! (] mul(618,693)~from()why()-*mul(5,516);how()&##@[^ +mul(632,327):where()when()($mul~,-mul(363,380)'!{how()'~when(),'how()mul(696,848)where()>'select()where()mul(129,10)$#where()!]select(),}how()~mul(545,776)@%*how()who(){mul(25,979)mul(448,493)what()'^>
        //do()why()# & mul(56,384)"
        let lines: Vec<String> = "[#from())when()/}+%mul(982,733)mul(700,428)}}don't(){:,$+mul(395,45)[; what()!mul(328,373)?!, <-)mul(139,254)^>#)*,[,&when()mul(719,688)-@,from()-@mul(616,640)from()!~{[' mul(666,309)(mul(889,261){select();why())<(/mul(364,750)>:+(:/mul(911,969)mul(360,631)~select() )$>mul(692,527)^mul(823,512)!]{where():&;,how()don't()'?,*where()<[>!mul(345,427))~ do()((how()@mul(394,662)[mul(903,277)why()~from()mul(249,126)>?/-what()'when()from()[who()mul(823,356)$mul(257,129),%*what()+]when()(mul(571,713)+why()]#([mul(230,759)'</>%^}$]mul(978,804)%>~!{-&don't() &'how() why()why(379,123)mul(816,166)}from(576,921) ?/when()?when()mul(946,526)/*mul(321,352)> where()'^ select(925,337)>[mul(233,433)<!}%;%:^>^mul(247,132)select(){[ ]!~ mul(104,931)why()&-+)~:<from()mul(587,547)<(where()where()where()how()mul(267,595)who()mul(834,996)where()select()@[mul(172,45)#+why()how()how()!#]~mul(3,396)why()~/mul(406,706){mul(209,557)?where() ?mul(716,930)what();&mul(92,580)'from()*[#!)where()when()mul(538,570)where()<when()/+-what()]mul(846,458)where()$why(){mul(18,615)when()#};:select()!*^mul(666,191):#from()when()from(),!^mul(837,567) #from()/when(612,5){/:mul(456,298)mul(570,670)mul(8,433)select()~}why()[^mul(358,86)who()/#mul(330,764)(how(905,838)from()#mul(683,182)]-!%:<[(why(257,862)mul(16,909);&who()*@where()when()mul(53,161)'where()?] [!mul(526,714)['}'@when()mul(101,388)?:how()select(762,661)where()%-<mul(862,638)what()select(),~^(mul(106,417){what()select()+from()}mul(946,829)^@mul(924,557),from()!when()mul(216,185)%mul(639,534) )!#<:]from()mul(710,22))>}#,}-$>&mul(412,598)*who()where();@why()do()mul(814,674)%where(){;~mul(654,224)/(what()][^how(190,339)#mul(690,95)&}what()*[mul(821,149) :-who()from()~mul(279,247);)mul(66,274)who()$)why()@where(395,477)~<mul(550,43)]^(why()>:/~mul(442,748)*when()from()'who()}]^~%mul(130,259)}why()<})how(){^mul(768,298)>{-+when(366,794):-mul(489,845)*mul(442,721)-$[ {)mul(283,227)@how()[where()>mul(862,708)'who()?*!where(){mul(182,377)#^mul-[;select()^-!mul(472,672)'select()mul(117,275))$where()$('select(),$(mul(409,378)where()where()?'mul(448,267)]!,:&;what()%>what(265,341)mul(916,448)~when()/!/ ;how()mul(849,877)what()<mul(444,734))>*?do()mul(851,406)~/%~mul(480,848)from(607,219)who()@~mul(803,80)don't(){*how()%{!where()select()mul(482,344),]/mul(702,892)how()@)']mul(552,653))&<^,^+#mul(13,654) /?:mul&from()what()mul(247,264)<$'{*^how();;mul(453,796)&+select(338,67)mul(201,343)>select()) /who()mul(360,411)what()why()@mul(657,810):/}&$:+-<mul(712,186)$'<~)when()!mul(241,637)where(),,'!<%mul(829,983)mul(388,384)@mul(47,882)>^what()do()what()<mul(796,344);'(what()+mul(460,350)how():,!how()mul(38,82)###}]&&from()select()mul(402,61)^]]-]mul(159,67))@who()! (] mul(618,693)~from()why()-*mul(5,516);how()&##@[^ +mul(632,327):where()when()($mul~,-mul(363,380)'!{how()'~when(),'how()mul(696,848)where()>'select()where()mul(129,10)$#where()!]select(),}how()~mul(545,776)@%*how()who(){mul(25,979)mul(448,493)what()'^>do()why()# & mul(56,384)"
                .split('\n')
                .map(|x| x.to_string())
                .collect();
        lines
    }

    #[test]
    fn test_parse_multiply_command() {
        assert_eq!(None, MultiplyCommand::parse_mul_command("mul[a,b]"));
        assert_eq!(
            Some(MultiplyCommand { lhs: 5, rhs: 10 }),
            MultiplyCommand::parse_mul_command("mul(5,10)")
        );
    }

    #[test]
    fn test_day03_part1_handler() {
        let lines = sample_data();
        let calculated = day03_part1_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(161, calculated);
    }

    #[test]
    fn test_day03_part2_handler() {
        let lines = sample_data2();
        let calculated = day03_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(48, calculated);
    }

    #[test]
    fn test_day03_part2_debug_handler() {
        let lines = sample_data3();
        let calculated = day03_part2_handler(&mut lines.iter().map(|x| x.to_string()));
        assert_eq!(48, calculated);
    }
}
