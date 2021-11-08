use std::fs;
use std::error::Error;
use std::collections::*;
use std::io::Write;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
enum Inst
{
    Inc(i32), // increment memory cell
    Dec(i32), // decrement memory cell
    Ind(i32), // increment memory cell referred to by memory cell
    Ded(i32), // decrement memory cell referred to by memory cell
    
    Toz(i32), // copy memory cell into memory cell 0
    Frz(i32), // copy memory cell 0 into memory cell
    Tod(i32), // copy memory cell referred to by memory cell into memory cell 0
    Frd(i32), // copy memory cell 0 into memory cell referred to by memory cell
    
    Sez(i32), // copy immediate into memory cell 0
    
    Pri(i32), // print memory cell as unicode codepoint
    Cal(i32), // call function
    May(i32), // jump over next instruction if memory cell is 0
    Nmy(i32), // jump over next instruction if memory cell is NOT 0
}

#[derive(Clone, Debug)]
struct Func
{
    code : Rc<Vec<Inst>>,
    pc : u32
}
impl Default for Func
{
    fn default() -> Func
    {
        Func { code : Rc::new(Vec::new()), pc : 0 }
    }
}
impl Func
{
    fn push(&mut self, inst : Inst)
    {
        Rc::get_mut(&mut self.code).unwrap().push(inst);
    }
}
#[derive(Debug)]
struct Global
{
    mem : Box<Vec<i32>>,
    funcs : Vec<Func>,
    func_stack : Vec<u32>,
}

impl Default for Global
{
    fn default() -> Global
    {
        let mut mem = Box::new(Vec::with_capacity(65536));
        println!("initializing memory...");
        for _ in 0..65536
        {
            mem.push(0);
        }
        Global { mem, funcs : Vec::new(), func_stack : Vec::new() }
    }
}

impl Global
{
    fn step(&mut self)
    {
        //println!("running an iteration...");
        let current_func = *self.func_stack.last().unwrap() as usize;
        if self.funcs[current_func].code.len() == 0
        {
            return;
        }
        let pc = self.funcs[current_func].pc as usize;
        let inst = self.funcs[current_func].code[pc];
        self.funcs[current_func].pc += 1;
        match inst
        {
            Inst::Inc(x) =>
            {
                self.mem[x as usize] += 1;
            }
            Inst::Dec(x) =>
            {
                self.mem[x as usize] -= 1;
            }
            Inst::Ind(x) =>
            {
                let addr = self.mem[x as usize] as usize;
                self.mem[addr] += 1;
            }
            Inst::Ded(x) =>
            {
                let addr = self.mem[x as usize] as usize;
                self.mem[addr] -= 1;
            }
            Inst::Toz(x) =>
            {
                let data = self.mem[x as usize];
                self.mem[0] = data;
            }
            Inst::Frz(x) =>
            {
                let data = self.mem[0];
                self.mem[x as usize] = data;
            }
            Inst::Tod(x) =>
            {
                let addr = self.mem[x as usize] as usize;
                let data = self.mem[addr];
                self.mem[0] = data;
            }
            Inst::Frd(x) =>
            {
                let addr = self.mem[x as usize] as usize;
                let data = self.mem[0];
                self.mem[addr] = data;
            }
            Inst::Sez(x) =>
            {
                self.mem[0] = x;
            }
            Inst::Pri(x) =>
            {
                print!("{}", char::from_u32(self.mem[x as usize] as u32).unwrap());
                std::io::stdout().flush();
            }
            Inst::Cal(x) =>
            {
                self.func_stack.push(x as u32);
                self.step();
                self.func_stack.pop();
            }
            Inst::May(x) =>
            {
                if self.mem[x as usize] == 0
                {
                    self.funcs[current_func].pc += 1;
                }
            }
            Inst::Nmy(x) =>
            {
                if self.mem[x as usize] != 0
                {
                    self.funcs[current_func].pc += 1;
                }
            }
        }
        // functions loop at the bottom
        if self.funcs[current_func].pc >= self.funcs[current_func].code.len() as u32
        {
            if self.func_stack.len() > 1
            {
                self.funcs[current_func].pc -= self.funcs[current_func].code.len() as u32;
            }
            // except for the implicitly called main function
            else
            {
                //self.funcs[current_func].pc = self.funcs[current_func].code.len() as u32-1;
                self.func_stack.pop();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    let f = fs::read_to_string("main.fl")?;
    let lines = f.split("\n").collect::<Vec<_>>();
    let mut global = Global { ..Default::default() };
    
    
    let re = regex::Regex::new(r"[ \t]*(inc|dec|ind|ded|toz|frz|tod|frd|sez|pri|cal|may|nmy|def)[ \t]+(-?[0-9]+|[a-z_]+)").unwrap();
    
    let mut func_defs = HashMap::<String, Func>::new();
    let mut func_names = Vec::<String>::new();
    let mut current_func = String::new();
    
    println!("compiling program...");
    
    for line in lines
    {
        if let Some(caps) = re.captures(line)
        {
            let inst = caps.get(1).unwrap().as_str();
            let data = caps.get(2).unwrap().as_str().to_string();
            //println!("parsing {} {}", inst, data);
            match inst
            {
                "inc" => func_defs.get_mut(&current_func).unwrap().push(Inst::Inc(data.parse().unwrap())),
                "dec" => func_defs.get_mut(&current_func).unwrap().push(Inst::Dec(data.parse().unwrap())),
                "ind" => func_defs.get_mut(&current_func).unwrap().push(Inst::Ind(data.parse().unwrap())),
                "ded" => func_defs.get_mut(&current_func).unwrap().push(Inst::Ded(data.parse().unwrap())),
                "toz" => func_defs.get_mut(&current_func).unwrap().push(Inst::Toz(data.parse().unwrap())),
                "frz" => func_defs.get_mut(&current_func).unwrap().push(Inst::Frz(data.parse().unwrap())),
                "tod" => func_defs.get_mut(&current_func).unwrap().push(Inst::Tod(data.parse().unwrap())),
                "frd" => func_defs.get_mut(&current_func).unwrap().push(Inst::Frd(data.parse().unwrap())),
                "sez" => func_defs.get_mut(&current_func).unwrap().push(Inst::Sez(data.parse().unwrap())),
                "pri" => func_defs.get_mut(&current_func).unwrap().push(Inst::Pri(data.parse().unwrap())),
                "may" => func_defs.get_mut(&current_func).unwrap().push(Inst::May(data.parse().unwrap())),
                "nmy" => func_defs.get_mut(&current_func).unwrap().push(Inst::Nmy(data.parse().unwrap())),
                "cal" =>
                {
                    func_defs.get_mut(&current_func).unwrap().push(Inst::Cal(func_names.len() as i32));
                    func_names.push(data);
                }
                "def" =>
                {
                    current_func = data.clone();
                    if !func_defs.contains_key(&data)
                    {
                        func_defs.insert(data, Func::default());
                    }
                }
                _ => {panic!("unhandled instruction")}
            }
        }
    }
    
    global.funcs = func_names.iter().map(|x| func_defs[x].clone()).collect::<Vec<_>>();
    global.funcs.push(func_defs["main"].clone());
    global.func_stack.push((global.funcs.len()-1) as u32);
    
    println!("compiled");
    
    // brainfuck interpreter memory: 0~99
    // brainfuck code: 100+
    // brainfuck memory: 10000+
    
    let mut fizzbuzz_bf = "
++++++++++[>++++++++++<-]>>++++++++++>->>>>>>>>>>>>>>>>-->+++++++[->++
++++++++<]>[->+>+>+>+<<<<]+++>>+++>>>++++++++[-<++++<++++<++++>>>]++++
+[-<++++<++++>>]>>-->++++++[->+++++++++++<]>[->+>+>+>+<<<<]+++++>>+>++
++++>++++++>++++++++[-<++++<++++<++++>>>]++++++[-<+++<+++<+++>>>]>>-->
---+[-<+]-<[+[->+]-<<->>>+>[-]++[-->++]-->+++[---++[--<++]---->>-<+>[+
+++[----<++++]--[>]++[-->++]--<]>++[--+[-<+]->>[-]+++++[---->++++]-->[
->+<]>>[.>]++[-->++]]-->+++]---+[-<+]->>-[+>>>+[-<+]->>>++++++++++<<[-
>+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>>++++++++++<[->-[>+>>]>[+[-<+>]>
+>>]<<<<<]>[-]>>[>++++++[-<++++++++>]<.<<+>+>[-]]<[<[->-<]++++++[->+++
+++++<]>.[-]]<<++++++[-<++++++++>]<.[-]<<[-<+>]+[-<+]->>]+[-]<<<.>>>+[
-<+]-<<]";
    
    //fizzbuzz_bf = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++......";
    let mut i = 0;
    for c in fizzbuzz_bf.chars()
    {
        match c
        {
            '>' => global.mem[100+i] = 1,
            '<' => global.mem[100+i] = 2,
            '+' => global.mem[100+i] = 3,
            '-' => global.mem[100+i] = 4,
            '.' => global.mem[100+i] = 5,
            ',' => global.mem[100+i] = 6,
            '[' => global.mem[100+i] = 7,
            ']' => global.mem[100+i] = 8,
            _ => i -= 1
        }
        i += 1
    }
    
    let mut pc = 0;
    let mut i = 0;
    while global.func_stack.len() > 0
    {
        global.step();
        if false//global.mem[1] != pc
        {
            pc = global.mem[1];
            println!("pc {} ptr {} *ptr {} 0:{} 3:{} 4:{} 5:{} 6:{} 7:{} 8:{} 9:{} 15:{} 16:{}", pc-100, global.mem[2]-10000, global.mem[global.mem[2] as usize],
            global.mem[0],
            global.mem[3], global.mem[4], global.mem[5],
            global.mem[6], global.mem[7], global.mem[8],
            global.mem[9],
            global.mem[15], global.mem[16]);
            println!("{}", i);
        }
        if pc > 10000
        {
            break;
        }
        i += 1;
    }
    // 52 520 395
    
    Ok(())
}
