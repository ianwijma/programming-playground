
fn main() {
	// mut allows name to be mutated
	mut name := 'Ian Wijma'

	// Here name turns into 'world' 
	name = 'world'

	// $<variable> allows us to use variables in a string
	println('Hello $name!')
}