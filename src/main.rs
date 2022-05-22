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

const OPERATIONS:[[char; 2]; 2] =
[
	['*', '/'],
	['+', '-']
];

fn solve(mut to_solve: Vec<StackToken>) -> StackToken
{
	//for x in
	for operations in OPERATIONS
	{
		let mut x: usize = 0;
		while x < to_solve.len()
		{
			match to_solve.get(x).unwrap()
			{
				StackToken::Operator(chr) =>
				{
					if operations.contains(chr)
					{
						let val_0;
						if let StackToken::Number(maybe_val_0) = to_solve.get(x - 1).unwrap()
						{
							val_0 = *maybe_val_0;
						}
						else
						{
							val_0 = 0f64;
						}
						let val_1;
						if let StackToken::Number(maybe_val_1) = to_solve.get(x + 1).unwrap()
						{
							val_1 = *maybe_val_1
						}
						else
						{
							val_1 = 0f64;
						}
						let out: f64;
						match chr
						{
							'+' =>
							{
								out = val_0 + val_1;
							}
							'-' =>
							{
								out = val_0 - val_1;
							}
							'*' =>
							{
								out = val_0 * val_1;
							}
							'/' =>
							{
								out = val_0 / val_1;
							}
							_ =>
							{
								out = 0f64;
							}
						}
						to_solve.remove(x);
						to_solve.remove(x);
						to_solve[x - 1] = StackToken::Number(out);
					}
				}
				_ => {}
			}
			x += 1;
		}
	}

	return to_solve.pop().unwrap();
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
	let mut stack: Vec<Vec<StackToken>> = vec![Vec::new()];
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
					if *chr == '('
					{
						//stack.push(StackToken::BracketOpen);
						//enclose_start_stack
						stack.push(Vec::new());
					}
					else if *chr == ')'
					{
						let mut top = stack.pop().unwrap();
						stack.last_mut().unwrap().push(solve(top));
					}
					else
					{
						stack.last_mut().unwrap().push(StackToken::Operator(*chr));
					}
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
					stack.last_mut().unwrap().push(StackToken::Number(num));
					//println!("Num: {:?}", num);
					parse_mode = ParseMode::None;
				}
			}
		}
		//println!("{}, {}", chr, next_chr);
	}
	match solve(stack.pop().unwrap())
	{
		StackToken::Number(val) =>
		{
			println!("{}", val);
		},
		_ => {}
	}
}