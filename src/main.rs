use oops::make_line;

fn main()
{
	make_line! {
		for _ in 0..(8 << 10)
		{
			make_line! {
				for _ in 0..(8 << 10)
				{
					for _ in 0..(8 << 10)
					{
						make_line! { make_line! { println!("test"); } make_line! { println!("test"); } }
					}
				}
			}
		}
	}
}
