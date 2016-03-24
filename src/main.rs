use std::io::prelude::*;
use std::fs::File;

use std::io;


const TRACE_SIZE:usize = 16;
const REG_OFFSET:u16 = 32768;

/*
If the operand is a register, return register value
Otherwise return operand value

*/
fn get_val(operand: u16, register:&Vec<u16>) -> u16 {
	match operand {
		32768 ... 32775 => get_reg(operand, register),
		_ => operand
	}
}

fn is_reg(operand: u16) -> bool {
	match operand {
		32768 ... 32775 => true,
		_ => false
	}
}

/*
return value in register
*/
fn get_reg(operand: u16, register:&Vec<u16>) -> u16 {
	register[(operand - REG_OFFSET) as usize]
	//(operand - REG_OFFSET) as u16
}

fn get_opcode_debug_string(adr:usize, memory: &Vec<u16>, register:&Vec<u16>) -> String {
	//println!("adr = {}, op = {}, adr+1 = {}, adr+2 = {}", adr, memory[adr], memory[adr+1], memory[adr+2]);
	match memory[adr] {
		1 => { // set reg a to the value of b
			format!("set w{} to {}", memory[adr+1], memory[adr+2])
		}
		2 => { // push a onto the stack
			format!("push {} onto stack", memory[adr+1])
		}
		3 => { // pop from stack, store in a.  empty stack = error
			format!("pop, store in {}", memory[adr+1])
		}
		4 => {
			format!("set {} to 1 if {} == {}, else 0", memory[adr+1], memory[adr+2], memory[adr+3])
		}
		5 => {
			format!("set {} to 1 if {} > {}, else 0", memory[adr+1], memory[adr+2], memory[adr+3])
		}
		6 => { // jmp to
			format!("jmp to {}", get_val(memory[adr+1], register))
		}
		7 => {  // jmp to b if a is != 0
			format!("jmp to {} if {} |= 0", memory[adr+2], memory[adr+1])
		}
		8 => {	// jmp to b if a == 0
			format!("jmp to {} if {} == 0", memory[adr+2], memory[adr+1])
		}
		9 => { // set reg a to the value of b + c
			format!("assign {} to {} + {}", memory[adr+1], memory[adr+2], memory[adr+3])
		}
		//10 => {
		//11 => {
		12 => { // stores into a the bitwise and of b and c
			format!("{} = {} AND {}", memory[adr+1], memory[adr+2], memory[adr+3])
		}
		13 => { // stores into a the bitwise or of b and c
			format!("{} = {} OR {}", memory[adr+1], memory[adr+2], memory[adr+3])
		}
		14 => { // stores 15-bit bitwise inverse of b in a
			format!("{} = NOT {}", memory[adr+1], memory[adr+2])
		}
		//15 => {
		//16 => {
		17 => { //write the address of the next instruction to the stack and jump to a
			format!("write address of {} to stack, jump to {}", adr+2, memory[adr+1])
		}
		//18 => {
		19 => {	// print to terminal
			format!("{}", (memory[adr+1] as u8) as char)
		}
		//20 => {
		21 => { // nop
			format!("nop")
		}
		_ => format!("no string")
	}
}


fn call_trace(v: &[usize], ptr: usize, memory: Vec<u16>, pc: usize, register:&Vec<u16>) {
	println!("traceptr = {}", ptr);


	println!("{:<7}{:<7}{:<7}", "pc", "op", "desc");
	println!("-----------------------------------------");

	for p in ptr..TRACE_SIZE {
		let trace_pc = v[p];
		let mut ts = format!("");
		/*if p == (ptr-1) {
			ts = format!(" <------ trace pointer");
		}*/
		//let mut s = format!("pc = {}\top = {}\t\t{}\t{}",trace_pc, memory[trace_pc], get_opcode_debug_string(trace_pc, &memory, register), ts);
		let mut s = format!("{:<7}{:<7}{:<7}{:<7}",trace_pc, memory[trace_pc], get_opcode_debug_string(trace_pc, &memory, register), ts);
		println!("{}", s);
	}

	for p in 0..ptr{
		let trace_pc = v[p];
		let mut ts = format!("");
		/*if p == (ptr-1) {
			ts = format!(" <------ trace pointer");
		}*/
		let mut s = format!("{:<7}{:<7}{:<7}{:<7}",trace_pc, memory[trace_pc], get_opcode_debug_string(trace_pc, &memory, register), ts);
		println!("{}", s);
	}
}

fn main() {
	let mut f = File::open("challenge.bin").unwrap();
	let mut v: Vec<u8> = Vec::new();
	let mut memory: Vec<u16> = Vec::with_capacity(32768);
	let mut pc = 0; // program counter
	let mut register: Vec<u16> = vec![0,0,0,0,0,0,0,0];
	let mut stack:Vec<u16> = Vec::with_capacity(1024);
	f.read_to_end(&mut v).unwrap();

	let mut i = 0;

	while i < v.len() {
		memory.push(v[i] as u16 + ((v[i+1] as u16)<<8));
		i += 2;
	}

	//let mut trace:Vec<usize> = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
	//let a = 32;
	let mut trace:[usize; TRACE_SIZE]  = [0; TRACE_SIZE];
	let mut traceptr = 0;
	let mut totalExecutions = 0;



	//try!(io::stdin().read_line(&mut input));

	//println!("You typed: {}", input.trim());
	loop {
		let mut input = String::new();
		let mut exs: i32 = 0;
		println!("How many Executions?");
		io::stdin().read_line(&mut input).ok().expect("could not read input");
		exs = input.trim().parse::<i32>().ok().expect("invalid input") + totalExecutions;
		loop {
			if totalExecutions >= exs {
				println!("");
				for r in 0..8 {
					println!("w{} = {}", r, register[r]);
				}
				println!("totalExecutions = {}", totalExecutions);
				call_trace(&trace, traceptr, memory.clone(), pc, &register);
				break;
			}
			traceptr &= (TRACE_SIZE-1);
			totalExecutions += 1;
			trace[traceptr] = pc;
			traceptr += 1;
			match memory[pc] {
				0 => { // halt
					//call_trace(&trace, traceptr, memory.clone(), pc, &register);
					println!("");
					//call_trace(&trace, traceptr, memory, pc, &register);

					println!("totalExecutions = {}", totalExecutions);

					return;
				}

				1 => {	// set reg a to the value of b
					let a = memory[pc+1];
					let b = memory[pc+2];
					//println!("pc = {}, op = {}, a = {}, b = {}", pc, memory[pc], a, b);
					if a < REG_OFFSET {
						println!("a = {}", a);
						register[a as usize] = get_val(b, &register);
						pc += 3;
					}
					else {
						let reg_num = (a-REG_OFFSET) as usize;
						//println!("reg num = {}", reg_num);
						match reg_num {
							0...8 => { register[reg_num] = get_val(b, &register); pc += 3; },
							_ => { pc = 1118; }
						}
					}
					continue;
				}

				2 => { // push a onto the stack
					let a = memory[pc+1];
					stack.push(get_val(a, &register));
					pc += 2;
					continue;
				},

				3 => { // pop from stack, store in a.  empty stack = error
					let a = memory[pc+1];
					let reg_num = (a-REG_OFFSET) as usize;
					match stack.pop() {
						Some(val) => register[reg_num] = val,
						None => panic!("error poping from stack, adr = {}\n", pc)
					}
					pc += 2;
					continue;
				}
				4 => { // set a to 1 if b is equal to c; set it to 0 otherwise
					let a = memory[pc+1];
					let b = memory[pc+2];
					let c = memory[pc+3];

					if a < REG_OFFSET {
						println!("a = {}", a);
						register[a as usize] = get_val(b, &register);
						pc += 3;
					}
					else {
						let reg_num = (a-REG_OFFSET) as usize;
						match reg_num {
							0...8 => {
								if get_val(b, &register) == get_val(c, &register) {
										register[reg_num] = 1;
								}
								else {
									register[reg_num] = 0;
								}
								pc += 4;
								continue;
							},
							_ => { panic!("invalid input to eq\n"); }
						}
					}
				}

				5 => { //set a to 1 if b is greater than c; set it to 0 otherwise
					let a = memory[pc+1];
					let b = memory[pc+2];
					let c = memory[pc+3];

					let reg_num = (a-REG_OFFSET) as usize;
					match reg_num {
						0...8 => {
							if get_val(b, &register) > get_val(c, &register) {
									register[reg_num] = 1;
							}
							else {
								register[reg_num] = 0;
							}
							pc += 4;
							continue;
						},
						_ => { panic!("invalid input to gt\n"); }
					}


				}

				6 => { // jmp to
					pc = memory[pc+1] as usize;
					continue;
				},
				7 => {  // jmp to b if a is != 0
					let a = memory[pc+1];
					let b = memory[pc+2];

					if get_val(a, &register) != 0 {
						pc = get_val(b, &register) as usize;
					}
					else {
						pc += 3;
					}

					continue;
				}

				8 => {	// jmp to b if a == 0
					let a = memory[pc+1];
					let b = memory[pc+2];

					if get_val(a, &register) == 0 {
						pc = get_val(b, &register) as usize;
					}
					else {
						pc += 3;
					}
					continue;
				}

				9 => { // assign in a the sum of b and c (modulo 32768)
					let a = memory[pc+1];
					let b = memory[pc+2];
					let c = memory[pc+3];

					if a < REG_OFFSET {
						println!("a = {}", a);
						register[a as usize] = get_val(b, &register);
						pc += 3;
					}
					else {
						let reg_num = (a-REG_OFFSET) as usize;
						match reg_num {
							0...8 => {
								register[reg_num] = (get_val(b, &register) + get_val(c, &register)) % REG_OFFSET;
								pc += 4;
							},
							_ => { panic!("invalid input to 9\n") }
						}
					}
					continue;
				}

				10 => { //store into a the product of b and c (modulo 32768)
					let a = memory[pc+1];
					let b = memory[pc+2];
					let c = memory[pc+3];


					let reg_num = (a-REG_OFFSET) as usize;
					match reg_num {
						0...8 => {
							register[reg_num] = (get_val(b, &register) * get_val(c, &register)) % REG_OFFSET;
							pc += 4;
						},
						_ => { panic!("invalid input to 9\n") }
					}
				continue;
				}

				12 => { // stores into a the bitwise and of b and c
					let a = memory[pc+1];
					let b = memory[pc+2];
					let c = memory[pc+3];
					let reg_num = (a-REG_OFFSET) as usize;
					match reg_num {
						0...8 => {
							register[reg_num] = (get_val(b, &register) & get_val(c, &register));
							pc += 4;
							continue;
						},
						_ => { panic!("invalid input to eq\n"); }
					}
				}

				13 => { //stores into a the bitwise or of b and c
					let a = memory[pc+1];
					let b = memory[pc+2];
					let c = memory[pc+3];
					let reg_num = (a-REG_OFFSET) as usize;
					match reg_num {
						0...8 => {
							register[reg_num] = (get_val(b, &register) | get_val(c, &register));
							pc += 4;
							continue;
						},
						_ => { panic!("invalid input to eq\n"); }
					}
				}

				14 => { // stores 15-bit bitwise inverse of b in a
					let a = memory[pc+1];
					let b = memory[pc+2];
					let reg_num = (a-REG_OFFSET) as usize;
					match reg_num {
						0...8 => {
							register[reg_num] = (!get_val(b, &register)) & (0x7FFF);
							pc += 3;
							continue;
						},
						_ => { panic!("invalid input to eq\n"); }
					}
				}
				17 => { //write the address of the next instruction to the stack and jump to a
					let a = memory[pc+1];
					stack.push((pc+2) as u16);
					pc = get_val(a, &register) as usize;
					continue;
				}

				19 => { // write char to terminal
					let c: u8 = (memory[pc+1] & 0xff) as u8;
					print!("{}", c as char);
					pc += 2;
					continue;
				}

				21 => { // nop

				}
				_ => {

				}
			}
			pc += 1;
		}
	}


}
