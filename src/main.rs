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

const OPERATIONS:[[char; 3]; 3] =
[
	['^', '\0', '\0'],
	['*', '/', '%'],
	['+', '-', '\0']
];

fn run_fn(params: Vec<StackToken>, func_name: &String) -> StackToken
{
	let mut nums: Vec<f64> = Vec::new();
	for param in params
	{
		nums.push(match param
		{
			StackToken::Number(valid) => valid,
			_ => 0f64
		});
	}
	let param_count: usize = match func_name.as_str()
	{
		"sqrt" => 1,
		"cbrt" => 1,
		"sin" => 1,
		"cos" => 1,
		"tan" => 1,
		"asin" => 1,
		"acos" => 1,
		"atan" => 1,
		"sinh" => 1,
		"cosh" => 1,
		"tanh" => 1,
		"asinh" => 1,
		"acosh" => 1,
		"atanh" => 1,
		"root" => 2,
		"neg" => 1,
		_ => 0
	};
	if param_count == 0
	{
		println!("Error: Invalid function.");
		std::process::exit(0);
	}
	if nums.len() != param_count
	{
		println!("Error: Invalid parameter count.");
		std::process::exit(0);
	}
	return match func_name.as_str()
	{
		"sqrt" => StackToken::Number(nums[0].sqrt()),
		"cbrt" => StackToken::Number(nums[0].cbrt()),
		"sin" => StackToken::Number(nums[0].sin()),
		"cos" => StackToken::Number(nums[0].cos()),
		"tan" => StackToken::Number(nums[0].tan()),
		"asin" => StackToken::Number(nums[0].asin()),
		"acos" => StackToken::Number(nums[0].acos()),
		"atan" => StackToken::Number(nums[0].atan()),
		"sinh" => StackToken::Number(nums[0].sinh()),
		"cosh" => StackToken::Number(nums[0].cosh()),
		"tanh" => StackToken::Number(nums[0].tanh()),
		"asinh" => StackToken::Number(nums[0].asinh()),
		"acosh" => StackToken::Number(nums[0].acosh()),
		"atanh" => StackToken::Number(nums[0].atanh()),
		"root" => StackToken::Number(nums[0].powf(1f64 / nums[1])),
		"neg" => StackToken::Number(-nums[0]),
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
						if x == 0
						{
							if chr == '-'
							{
								to_solve.remove(x + 1);
								to_solve[x] = StackToken::Number(-val_1);
								continue;
							}
							println!("Error: Operator at start of expression.");
							std::process::exit(0);
						}
						let val_0 = match to_solve.get(x - 1)
						{
							Some(StackToken::Number(valid)) => *valid,
							Some(_) =>
							{
								if chr == '-'
								{
									to_solve.remove(x);
									to_solve[x - 1] = StackToken::Number(-val_1);
									continue;
								}
								println!("Error: There is an operator that does not have a number to its left.");
								std::process::exit(0);
							},
							None => 0f64
						};
						let out: f64 = match chr
						{
							'+' => val_0 + val_1,
							'-' => val_0 - val_1,
							'*' => val_0 * val_1,
							'/' => val_0 / val_1,
							'%' => val_0 % val_1,
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
	let mut stack: Vec<Vec<Vec<StackToken>>> = vec![vec![Vec::new()]];
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
					if *chr == ','
					{
						stack.last_mut().unwrap().push(Vec::new());
					}
					else if *chr == '('
					{
						stack.push(vec![Vec::new()]);
					}
					else if *chr == ')'
					{
						let mut solved: Vec<StackToken> = Vec::new();
						match stack.pop()
						{
							Some(valid) =>
							{
								//top = solve(valid);
								//let mut solved: Vec<StackToken> = Vec::new();
								for sub_exp in valid
								{
									solved.push(solve(sub_exp));
								}
							}
							None =>
							{
								println!("Error: More closing brackets than opening brackets.");
								return;
							}
						}
						let top;
						match stack.last_mut()
						{
							Some(valid) =>
							{
								match valid.last().unwrap().last()
								{
									Some(StackToken::Keyword(keyword)) =>
									{
										top = run_fn(solved, &keyword);
										valid.last_mut().unwrap().pop();
									},
									Some(_) =>
									{
										if solved.len() != 1
										{
											println!("Error: Only allowed one result.");
											return;
										}
										top = solved.pop().unwrap();
									},
									None =>
									{
										if solved.len() != 1
										{
											println!("Error: Only allowed one result.");
											return;
										}
										top = solved.pop().unwrap();
									}
								}
								valid.last_mut().unwrap().push(top);
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
						stack.last_mut().unwrap().last_mut().unwrap().push(StackToken::Operator(*chr));
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
					stack.last_mut().unwrap().last_mut().unwrap().push(StackToken::Number(num));
					parse_mode = ParseMode::None;
				}
			}
			ParseMode::Alphanumeric(start) =>
			{
				if !(next_chr.is_alphanumeric() || next_chr == '_')
				{
					let string: String = eq[start..=x].iter().collect();
					stack.last_mut().unwrap().last_mut().unwrap().push(StackToken::Keyword(string));
					parse_mode = ParseMode::None;
				}
			}
		}
	}

	// Last solve and check
	if stack.len() != 1
	{
		println!("Error: More opening brackets than closing brackets.");
		return;
	}
	let mut top = stack.pop().unwrap();
	if top.len() != 1
	{
		println!("Error: Only allowed one result.");
		return;
	}
	let top = top.pop().unwrap();
	let last_solved = solve(top);

	// Print result
	match last_solved
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