enum ParseMode
{
	None,
	Numeric(usize),
	Alphanumeric(usize)
}

enum StackToken
{
	Number(f64),
	Operator(char),
	Keyword(String)
}

const OPERATIONS:[[char; 2]; 3] =
[
	['^', '\0'],
	['*', '/'],
	['+', '-']
];

fn run_fn(param: StackToken, func_name: &String) -> StackToken
{
	let num: f64 = match param
	{
		StackToken::Number(valid) => valid,
		_ => 0f64
	};
	return match func_name.as_str()
	{
		"sqrt" => StackToken::Number(num.sqrt()),
		"cbrt" => StackToken::Number(num.cbrt()),
		_ =>
		{
			println!("Error: Invalid function.");
			std::process::exit(0);
		}
	}
}

fn solve(mut to_solve: Vec<StackToken>) -> StackToken
{
	for operations in OPERATIONS
	{
		let mut x: usize = 0;
		while x < to_solve.len()
		{
			match to_solve[x]
			{
				StackToken::Operator(chr) =>
				{
					if operations.contains(&chr)
					{
						if x == 0
						{
							println!("Error: Operator at start of expression.");
							std::process::exit(0);
						}
						let val_0 = match to_solve.get(x - 1)
						{
							Some(StackToken::Number(valid)) => *valid,
							Some(_) =>
							{
								println!("Error: There is an operator that does not have a number to its left.");
								std::process::exit(0);
							},
							None => 0f64
						};
						let val_1 = match to_solve.get(x + 1)
						{
							Some(StackToken::Number(valid)) => *valid,
							Some(_) =>
							{
								println!("Error: There is an operator that does not have a number to its right.");
								std::process::exit(0);
							},
							None =>
							{
								println!("Error: Operator at end of expression.");
								std::process::exit(0);
							}
						};
						let out: f64 = match chr
						{
							'+' => val_0 + val_1,
							'-' => val_0 - val_1,
							'*' => val_0 * val_1,
							'/' => val_0 / val_1,
							'^' => val_0.powf(val_1),
							_ => 0f64
						};
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
	if to_solve.len() > 1
	{
		println!("Error: Syntax error.");
		std::process::exit(0);
	}
	else if to_solve.len() == 0
	{
		println!("Error: Blank expression.");
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
				if chr.is_numeric() || *chr == '.'
				{
					parse_mode = ParseMode::Numeric(x);
				}
				else if chr.is_alphabetic() || *chr == '_'
				{
					parse_mode = ParseMode::Alphanumeric(x);
				}
				else if !chr.is_whitespace()
				{
					if *chr == '('
					{
						stack.push(Vec::new());
					}
					else if *chr == ')'
					{
						let mut top: StackToken;
						match stack.pop()
						{
							Some(valid) =>
							{
								top = solve(valid);
							}
							None =>
							{
								println!("Error: More closing brackets than opening brackets.");
								return;
							}
						}
						match stack.last_mut()
						{
							Some(valid) =>
							{
								match valid.last()
								{
									Some(StackToken::Keyword(keyword)) =>
									{
										top = run_fn(top, &keyword);
										valid.pop();
									},
									Some(_) => {},
									None => {}
								}
								valid.push(top);
							}
							None =>
							{
								println!("Error: More closing brackets than opening brackets.");
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
				if !(next_chr.is_alphanumeric() || next_chr == '.')
				{
					let num_string: String = eq[start..=x].iter().collect();
					let num: f64 = match num_string.parse::<f64>()
					{
						Ok(valid) => valid,
						Err(_) =>
						{
							println!("Error: Invalid number.");
							return;
						}
					};
					stack.last_mut().unwrap().push(StackToken::Number(num));
					parse_mode = ParseMode::None;
				}
			}
			ParseMode::Alphanumeric(start) =>
			{
				if !(next_chr.is_alphanumeric() || next_chr == '_')
				{
					let string: String = eq[start..=x].iter().collect();
					stack.last_mut().unwrap().push(StackToken::Keyword(string));
					parse_mode = ParseMode::None;
				}
			}
		}
	}
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
			println!("Error: More closing brackets than opening brackets.");
			return;
		}
	}

	// Check stack length
	if stack.len() != 0
	{
		println!("Error: More opening brackets than closing brackets.");
		return;
	}

	// Print result
	match solve(top)
	{
		StackToken::Number(val) =>
		{
			println!("{}", val);
		},
		_ =>
		{
			println!("Error: Syntax error.");
			return;
		}
	}
}