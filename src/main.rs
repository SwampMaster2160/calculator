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
						x -= 1;
					}
				}
				_ => {}
			}
			x += 1;
		}
	}
	if to_solve.len() != 1
	{
		println!("Error: More opening brackets than close ones.");
		std::process::exit(0);
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
		// Parse char
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
						stack.push(Vec::new());
					}
					else if *chr == ')'
					{
						let top: Vec<StackToken>;
						match stack.pop()
						{
							Some(valid) =>
							{
								top = valid;
							}
							None =>
							{
								println!("Error: More closing brackets than open ones.");
								return;
							}
						}
						match stack.last_mut()
						{
							Some(valid) =>
							{
								valid.push(solve(top));
							}
							None =>
							{
								println!("Error: More closing brackets than open ones.");
								return;
							}
						}
					}
					else
					{
						stack.last_mut().unwrap().push(StackToken::Operator(*chr));
					}
				}
			},
			_ => {}
		}

		// Get next char
		let next_chr = *eq.get(x + 1).unwrap_or(&'\0');

		// Parse again depending on this and the next char
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
					parse_mode = ParseMode::None;
				}
			}
		}
	}

	// Check for bracket mismatch
	/*if stack.len() != 1
	{
		println!("Error: More opening brackets than close ones.");
		return;
	}*/

	// Last solve
	let top: Vec<StackToken>;
	match stack.pop()
	{
		Some(valid) =>
		{
			top = valid;
		}
		None =>
		{
			println!("Error: More closing brackets than open ones.");
			return;
		}
	}
	match solve(top)
	{
		StackToken::Number(val) =>
		{
			println!("{}", val);
		},
		_ => {}
	}
}