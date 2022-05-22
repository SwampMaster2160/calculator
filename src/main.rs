enum ParseMode
{
	None,
	Numeric(usize)
}

enum StackToken
{
	Number(f64),
	Operator(char)
}

fn main()
{
	// Make sure we get 2 arguments
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2
	{
		println!("Error: Invalid argument count, needs 2 arguments.");
		return;
	}
	// For each char in second argument
	let eq: Vec<char> = args[1].chars().collect();
	let mut parse_mode = ParseMode::None;
	let mut stack: Vec<StackToken> = Vec::new();
	for (x, chr) in eq.iter().enumerate()
	{
		match parse_mode
		{
			ParseMode::None =>
			{
				if chr.is_numeric()
				{
					parse_mode = ParseMode::Numeric(x);
				}
				else if !chr.is_whitespace()
				{
					stack.push(StackToken::Operator(*chr));
					//println!("Char: {}", chr);
				}
			},
			_ => {}
		}

		// Get next char
		let next_chr : char;
		if x < eq.len() - 1
		{
			next_chr = eq[x + 1];
		}
		else
		{
			next_chr = '\0';
		}

		match parse_mode
		{
			ParseMode::None => {},
			ParseMode::Numeric(start) =>
			{
				if !(next_chr.is_numeric() || next_chr == '.')
				{
					let num_string: String = eq[start..=x].iter().collect();
					let num = num_string.parse::<f64>().unwrap();
					stack.push(StackToken::Number(num));
					//println!("Num: {:?}", num);
					parse_mode = ParseMode::None;
				}
			}
		}
		//println!("{}, {}", chr, next_chr);
	}
}