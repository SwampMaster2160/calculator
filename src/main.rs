fn main()
{
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2
	{
		println!("Error: Invalid argument count, needs 2 arguments.");
		return;
	}
	let eq = &args[1];
	println!("{}", eq);
}