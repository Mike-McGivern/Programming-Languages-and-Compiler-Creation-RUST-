use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::env;
use std::fs;
use std::process;

//Programmed by Michael McGivern

#[derive(Debug, Clone)]
pub struct Rule {
    lhs: char,
    rhs: String,
}

impl Rule {
    //creates a rule from non-terminal lhs and slices the rhs
    pub fn new(lhs: char, rhs: &str) -> Self {
        Rule {
            lhs,
            rhs: rhs.to_string()
        }
    }
    // checks if lhs is an uppercase non-terminal
    pub fn is_valid(&self) -> bool {
            self.lhs.is_ascii_uppercase()
        }

    //checks if A -> xB (right-regular grammar rule)
    // Strict: RHS is either 'a' (single terminal) or 'aA' (terminal + nonterminal)
    // Extended: RHS is either 's' (string of terminals) or 'sA' (terminals + nonterminal)
    // Does NOT allow empty string
    pub fn is_right_regular(&self) -> bool {
        let chars: Vec<char> = self.rhs.chars().collect();
        
        if chars.is_empty() {
            return false; // No empty/epsilon rules allowed
        }
        
        // Check if all are terminals (extended: s)
        if chars.iter().all(|c| !c.is_ascii_uppercase()) {
            return true;
        }
        
        // Check if it's terminals followed by exactly one non-terminal at the end (extended: sA)
        if chars.len() >= 2 {
            let last = chars[chars.len() - 1];
            let all_but_last = &chars[..chars.len() - 1];
            
            if last.is_ascii_uppercase() && all_but_last.iter().all(|c| !c.is_ascii_uppercase()) {
                return true;
            }
        }
        
        false
    }
    //checks if A -> Bx (left-regular grammar rule)
    // Strict: RHS is either 'a' (single terminal) or 'Ba' (nonterminal + terminal)
    // Extended: RHS is either 's' (string of terminals) or 'As' (nonterminal + terminals)
    // Does NOT allow empty string
    pub fn is_left_regular(&self) -> bool {
        let chars: Vec<char> = self.rhs.chars().collect();
        
        if chars.is_empty() {
            return false; // No empty/epsilon rules allowed
        }
        
        // Check if all are terminals (extended: s)
        if chars.iter().all(|c| !c.is_ascii_uppercase()) {
            return true;
        }
        
        // Check if it's exactly one non-terminal at the start followed by terminals (extended: As)
        if chars.len() >= 2 {
            let first = chars[0];
            let all_but_first = &chars[1..];
            
            if first.is_ascii_uppercase() && all_but_first.iter().all(|c| !c.is_ascii_uppercase()) {
                return true;
            }
        }
        
        false
    }
}

#[derive(Debug)]
pub struct Grammar {
    _nonterminals: Vec<char>,
    _terminals: Vec<char>,
    rules: Vec<Rule>,
    start: char,
}

impl Grammar {
    // builds context free grammar from rules, the first rule's LHS is the starting symbol
    pub fn from_rules(rules: &[Rule]) -> Self {
        let start = rules[0].lhs;
        let mut nts = Vec::new();
        let mut ts = Vec::new();

        for r in rules {
            if !nts.contains(&r.lhs) {
                nts.push(r.lhs);
            }
            for c in r.rhs.chars() {
                if c.is_ascii_uppercase() {
                    if !nts.contains(&c) {
                        nts.push(c);
                    }
                } else {
                    if !ts.contains(&c) {
                        ts.push(c)
                    }
                }
            }
        }

        Grammar {
            _nonterminals: nts,
            _terminals: ts,
            rules: rules.to_vec(),
            start,
        }
    }
    //checks that the rules are valid for the context-free grammar
    //does this by iterating through each rule within the grammar and using the rule's implemtation of is valid

    pub fn is_valid(&self) -> bool {
        self.rules.iter().all(Rule::is_valid)
    }
    //checks that the grammar is regular (all rules are right- or left-regular, but not mixing strict forms)
    pub fn is_regular(&self) -> bool {
        let all_right = self.rules.iter().all(Rule::is_right_regular);
        let all_left = self.rules.iter().all(Rule::is_left_regular);

        // Detect presence of strict forms (xB vs Bx). Terminal-only, ("") rules are neutral.
        let any_right_strict = self.rules.iter().any(|r| {
            let cs: Vec<char> = r.rhs.chars().collect();
            matches!(cs.as_slice(), [x, b] if !x.is_ascii_uppercase() && b.is_ascii_uppercase())
        });
        let any_left_strict = self.rules.iter().any(|r| {
            let cs: Vec<char> = r.rhs.chars().collect();
            matches!(cs.as_slice(), [b, x] if b.is_ascii_uppercase() && !x.is_ascii_uppercase())
        });

        (all_right && !any_left_strict) || (all_left && !any_right_strict)
    }

    pub fn rule_idx_from_nt(&self, nt: char) -> Vec<usize> {
        self.rules
            .iter()
            .enumerate()
            .filter_map(|(i, r)| if r.lhs == nt {
                Some(i)
            } else {
                None
            })
            .collect()
    }
}


#[derive(Debug, Clone)]
pub struct Sentential {
    form: String,
    first_nt_idx: isize,
}

#[derive(Debug)]
pub enum SententialError {
    NoNonTerminal,
    RuleMismatch,
}


impl Sentential {
//start with the starting symbol
pub fn new_init(grammar: &Grammar) -> Self {
    let s = grammar.start.to_string();
    Sentential {
        form: s,
        first_nt_idx: 0,
    }
}

//replace leftmost nonterminal by rhs rule "ridx"
pub fn new_next(
    &self,
    grammar: &Grammar,
    ridx: usize,

) -> Result<Self, SententialError> {
    if self.first_nt_idx < 0 {
        panic!("No non-terminals left to replace");
        //return Err(SententialError::NoNonTerminal);
    }

    let pos = self.first_nt_idx as usize;
    let nt = self.form.chars().nth(pos).unwrap();
    let rule = &grammar.rules[ridx];

    if rule.lhs != nt {
        panic!("Rule mismatch, left hand side is a terminal");
        //return Err(SententialError::RuleMismatch);
    }

    //build new form
    let mut next = String::new();
    next.push_str(&self.form[..pos]);
    next.push_str(&rule.rhs);
    next.push_str(&self.form[pos + 1..]);

    //find next nonterminal
    let next_nt:isize = next.chars()
        .position(|c| c.is_ascii_uppercase())
        .map(|i| i as isize)
        .unwrap_or(-1);

    Ok(Sentential {
        form: next,
        first_nt_idx: next_nt,
    })
}

    //function returns true if no more non-terminals are left to replace
    pub fn is_complete(&self) -> bool {
        self.first_nt_idx < 0
    }
}

#[derive(Debug)]
pub enum DerivationError {
    SententialErr(SententialError),
    StepLimitExceeded,
}

#[derive(Debug)]
pub struct Derivation {
    steps: Vec<(isize, Sentential)>,
}

impl Derivation {
    //start a new derivation with inital sentence form
    pub fn new(grammar: &Grammar) -> Self {
        let init = Sentential::new_init(grammar);
        Derivation {
            steps: vec![(-1, init)],
        }
    }

    //apply leftmost derivation step using ridx rule
    pub fn derive_leftmost(
        &mut self,
        grammar: &Grammar,
        ridx: usize,
    )  -> Result<(), DerivationError>  {
        let (_, last) = self.steps.last().unwrap().clone();
        let next = last
            .new_next(grammar, ridx)
            .map_err(DerivationError::SententialErr)?;
        self.steps.push((ridx as isize, next));
        Ok(())
    }

    //Check if final sentential is all terminals.
    pub fn is_complete(&self) -> bool {
        self.steps.last().unwrap().1.is_complete()
    }

    //return the current derivation word if done
    pub fn word(&self) -> Option<String> {
        if self.is_complete() {
            Some(self.steps.last().unwrap().1.form.clone())
        } else {
            None
        }
    }

    //returns the leftmost non-terminal of the most recent step
    pub fn leftmost_nonterminal(&self) -> Option<char> {
        let s = &self.steps.last().unwrap().1.form;
        s.chars().find(|c| c.is_ascii_uppercase())
    }

    //generate random word by repeatedly picking a rule uniformly
    // for all rules applicable to the leftmost nonterminal, up to step_limit
    pub fn print_random(
        grammar: &Grammar,
        step_limit: Option<usize>,
    ) -> Option<String> {
        let mut drv = Derivation::new(grammar);
        let mut rng = thread_rng();

        for step in 0.. {
            if drv.is_complete() {
                return drv.word();
            }
            if let Some(limit) = step_limit {
                if step >= limit {
                    return None;
                }
            }

            //pick the leftmost Non-terminal
            let nt = drv.leftmost_nonterminal().unwrap();
            let choices = grammar.rule_idx_from_nt(nt);
            let die = Uniform::from(0..choices.len());
            let ridx = choices[die.sample(&mut rng)];
            drv.derive_leftmost(grammar, ridx)/*.unwrap() */;
        }
        unreachable!()
    }
}

#[allow(dead_code)]
fn example_manual() {
    let rules = vec![
        Rule::new('E', "!E"), // 0: prefix !
        Rule::new('E', "E*E"),
        Rule::new('E', "E+E"),
        Rule::new('E', "(E)"),
        Rule::new('E', "n"),
    ];

    let grammar = Grammar::from_rules(&rules);
    println!("grammar valid = {}", grammar.is_valid());
    println!("grammar regular = {}", grammar.is_regular());


    let mut drv = Derivation::new(&grammar);
    drv.derive_leftmost(&grammar, 0).unwrap(); // !E
    drv.derive_leftmost(&grammar, 1).unwrap(); // !E*E
    drv.derive_leftmost(&grammar, 0).unwrap(); // !!E*E
    drv.derive_leftmost(&grammar, 4).unwrap(); // !n*E
    drv.derive_leftmost(&grammar, 4).unwrap(); // !n*n
    println!("derivation complete = {}", drv.is_complete());
    println!(
        "derivation word = {:?}",
        drv.word().unwrap()
    )
}

/*
fn main() {

    example_manual();

    //radom word example up to 10 steps
    let pe = Derivation::print_random(
        &Grammar::from_rules(&vec![
            Rule::new('E', "!E"),
            Rule::new('E', "E*E"),
            Rule::new('E', "E+E"),
            Rule::new('E', "(E)"),
            Rule::new('E', "n"),
        ]),
        Some(10),
    );

    println!("Random word = {:?}", pe);
} */

fn main() {

    let rules:Vec<Rule> = vec![
        Rule::new('E', "!E"),
        Rule::new('E', "E*E"), // i should make a rule that is E -> E * n
        Rule::new('E', "E+E"), // i should make a rule that is E -> E + n
        Rule::new('E', "(E)"),
        Rule::new('E', "n"),
        // Added rules:
        Rule::new('E', "E*n"),
        Rule::new('E', "E+n"),
        Rule::new('E', "E+B"),
        Rule::new('B', "-B"),
        Rule::new('B', "n/n"),
    ]; // add more nonterminals and rules for them
        // E -> E + B
        // B -> -B
        //B -> n/n
    let args: Vec<String> = env::args().collect();
    //creates a vector of strings from the arguments given at the command line

    if args.len() < 2 { //checks if arguments are given
        print_general_help();
        process::exit(0);
    }

    let command = &args[1];

    match command.as_str() {
        "help" => {
            if args.len() > 2 { //asking for help for certain command
                print_command_help(&args[2]);
            }
            else { //wants general help
                print_general_help();

            }
        }
        "print" => {
            handle_print_command(&args[2..])
            //prints arguments given from index 2 upto length of arguments
        }
        "list" => {
            list_commands();
        }

        "list_rules" => {
            for(i, rule) in rules.iter().enumerate() {
                println!("Rule {}: {} -> {}", i, rule.lhs, rule.rhs);
            }
        }
        "derive" => {
            if args.len() < 3 {
                println!("Not enough arguments");
                println!("Usage: cargo run -- derive <random | int-list>");
                process::exit(0);
            }
            else {
                derive(&rules, &args[2..]);
            }
        }
        _ => { //default case of switch statement
            println!("Unknown command: {}", command);
            println!("Try 'help' for a list of commands.");
            process::exit(0);
        }
    }
}


fn derive(rules: &[Rule], args: &[String]) {
    if args.is_empty() {
        println!("Error: Missing derivation mode.");
        println!("Usage: derive <random | int-list>");
        std::process::exit(1);
    }

    let grammar = Grammar::from_rules(rules);
    let mut drv = Derivation::new(&grammar);

    match args[0].as_str() {
        "random" => {
            use rand::Rng;
            use rand::distributions::WeightedIndex;
            let mut rng = rand::thread_rng();
            
            let step_limit = 15;

            // Global probabilities by rule index (sum to 1.0)
            // 0: E->!E   1: E->E*E   2: E->E+E   3: E->(E)   4: E->n
            // 5: E->E*n  6: E->E+n   7: E->E+B   8: B->-B    9: B->n/n
            let probs: Vec<f64> = vec![
                0.08,  // E -> !E
                0.01,  // E -> E*E
                0.01,  // E -> E+E
                0.08,  // E -> (E)
                0.35,  // E -> n
                0.10,  // E -> E*n
                0.10,  // E -> E+n
                0.10,  // E -> E+B
                0.02,  // B -> -B
                0.15,  // B -> n/n
            ];

            for step in 0..step_limit {
               if drv.is_complete() {
                    break;
               }

                let nt = match drv.leftmost_nonterminal() {
                    Some(nt) => nt,
                    None => {
                        println!("Error: No non-terminals found at step {}.", step);
                        std::process::exit(1);
                    }
                };

                
                let choices = grammar.rule_idx_from_nt(nt); // weighted selection over this subset

                if choices.is_empty() {
                    println!("Error: No rules applicable to non-terminal {}.", nt);
                    std::process::exit(1);
                }

                // Conditional probabilities: restrict to applicable rules and renormalize
                let mut choice_probs: Vec<f64> = choices
                    .iter()
                    .map(|&i| *probs.get(i).unwrap_or(&0.0))
                    .collect();

                let sum: f64 = choice_probs.iter().sum();

                // If the conditional mass is zero, fall back to uniform
                let ridx = if sum > 0.0 && sum.is_finite() {
                    for p in &mut choice_probs {
                        *p /= sum;
                    }
                    match WeightedIndex::new(&choice_probs) { // applies the weights of the probabilities to the grammar choices
                        Ok(dist) => choices[dist.sample(&mut rng)],
                        Err(_) => choices[rng.gen_range(0..choices.len())],
                    }
                } else {
                    choices[rng.gen_range(0..choices.len())]
                };

                if let Err(e) = drv.derive_leftmost(&grammar, ridx) {
                    println!("Error: Failed to apply rule {}: {:?}", ridx, e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            // All arguments should be rule indices
            let indices: Result<Vec<usize>, _> = args.iter().map(|s| s.parse::<usize>()).collect();
            match indices {
                Ok(seq) => {
                    println!("Applying rules: {:?}", seq);
                    println!("Initial form: {}", drv.steps.last().unwrap().1.form);
                    
                    for (step, &i) in seq.iter().enumerate() {
                        if drv.is_complete() {
                            println!("Derivation already complete before step {}.", step);
                            break;
                        }

                        let nt = match drv.leftmost_nonterminal() {
                            Some(nt) => nt,
                            None => {
                                println!("Error: No nonterminal found at step {}.", step);
                                std::process::exit(1);
                            }
                        };

                        let valid_indices = grammar.rule_idx_from_nt(nt);
                        println!("Step {}: nonterminal '{}', valid rules: {:?}, applying rule {}", 
                                 step, nt, valid_indices, i);
                        
                        if !valid_indices.contains(&i) {
                            println!("Error: Rule index {} is not valid for nonterminal '{}' at step {}", i, nt, step);
                            std::process::exit(1);
                        }

                        if let Err(e) = drv.derive_leftmost(&grammar, i) {
                            println!("Error: Failed to apply rule {}: {:?}", i, e);
                            std::process::exit(1);
                        }
                        
                        println!("  Result: {}", drv.steps.last().unwrap().1.form);
                    }
                }
                Err(_) => {
                    println!("Error: Invalid rule index in input. All values must be integers.");
                    std::process::exit(1);
                }
            }
        }
    }

    println!("Derivation complete = {}", drv.is_complete());
    println!("Derivation word = {:?}", drv.word().unwrap_or_else(|| "<invalid>".to_string()));
}


fn print_general_help() {
    println!("A command line utility for Rust");
    println!("Usage: cargo run -- <command> [arguments]");
    println!();
    println!("Commands:");
    println!("    help        Print this help message");
    println!("    help [command]       shows help information for a command");
    println!("    print <file> [numbered]       Print arguments given");
    println!("    list        List all commands");
    println!("    list_rules        List all grammar rules");
    println!("    derive <random | int-list>       Derive a word from a sentence");
}

fn print_command_help(command:&str) {
    match command {
        "help" => {
            println!("help - Show help information");
            println!();
            println!("Usage: ");
            println!("cargo run -- help [command]");
            println!();
            println!("Description: ");
            println!("    Prints help information for a command");
            println!("Arguments: ");
            println!("    [command] - The command to get help for (OPTIONAL)");
        }

        "print" => {
            println!("print file contents");
            println!();
            println!("Usage: ");
            println!("cargo run -- print <file> [numbered]");
            println!();
            println!("Description: ");
            println!("prints the contents of the specified file");
            println!("Arguments: ");
            println!("    <file> - The path of the file to print (REQUIRED)");
            println!("    [numbered] - Whether to number the lines (OPTIONAL)");
        }

        "list" => {
            println!("list - List all commands");
            println!();
            println!("Usage: ");
            println!("cargo run -- list");
            println!();
            println!("Description: ");
            println!("lists all commands");
        }

        "list-rules" => {
            println!("list-rules - lists all grammar rule");
            println!();
            println!("Usage: ");
            println!("cargo run -- list-rules");
            println!();
            println!("Description: ");
            println!("lists all grammar rule currently defined");
        }

        "derive" => {
            println!("derive - Derive a word from a sentence");
            println!();
            println!("Usage: ");
            println!("cargo run -- derive <random | int-list>");
            println!();
            println!("Description: ");
            println!("Applies grammar rules to derive a word");
            println!("Arguments: ");
            println!("    random -Applies 5 random rules");
            println!("    int-list - Applies rules by index (e.x. 0 1 4) applies rule 1, 2 and 5");
        }

        _ => { //default case of switch statement
            println!("Unknown command: {}", command);
            println!("Try 'help' to learn how to use this tool or list for a list of commands.");
            process::exit(1);
        }
    }
}

fn handle_print_command(args: &[String]) {
    if args.is_empty() {
        println!("No file specified");
        println!("USEAGE: cargo run -- print <file> [--numbered]");
        println!("Try 'help print' for more information");
        process::exit(0);
    }

    let file_path:&String = &args[0];
    let numbered:bool = args.len() > 1 && args[1] == "--numbered";

    match fs::read_to_string(file_path) {
        Ok(contents)  => {
            if numbered {
                for(line_number, line) in contents.lines().enumerate() {
                    println!("{:5} {}", line_number + 1, line);
                }
            } else {
                println!("{}", contents);
            }
        }
        Err(error) => {
            println!("Error reading file {}: {}", file_path, error);
            process::exit(0);
        }
    }
}

fn list_commands() {
    println!("Available commands:");
    println!("list - List all commands");
    println!("help - Show help information");
    println!("print - Print arguments given");
    println!("list_rules - lists all grammar rule");
    println!("derive - Derive a word from a sentence using a grammar");
}
