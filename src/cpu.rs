pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xffff],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            memory: [0; 0xffff],
        }
    }

    // all of our memory operations
    fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    // writes data in little endian instead of big endian
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
    
    // reads data in little endian instead of big endian
    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        return (hi << 8) | (lo as u16);
    }


    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    // loads the program from memory
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
        self.mem_write_u16(0xfffc, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xfffc)
    }

    // preforms bitwise operations on registers
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0x80 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    // load a byte of memory into the accumulator setting zero and negative flags
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    
    // copies current contents of the accumulator into the X register and sets flags
    fn tax(&mut self) {
       self.register_x = self.register_a;
       self.update_zero_and_negative_flags(self.register_x);
    }


    // Adds one to the X register
    fn inx(&mut self) {
      if self.register_x == 0b1111_1111 {
          self.register_x = 0b0000_0000;
      } else  {
          self.register_x = self.register_x + 1;
      }
      self.update_zero_and_negative_flags(self.register_x);
    }

    /// The CPU implements the following steps:
    /// 1. Fetch next instruction from memory (the vector)
    /// 2. Decode the instruction (setting opcode)
    /// 3. Executing the instruction (setting status)
    /// 4. Repeating the instructions
    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1; 

            match code { // this is where we decode the instruction
                0xA9 => {
                    let param = self.memory[self.program_counter as usize];
                    //let param = self.mem_read(self.program_counter as usize );
                    self.program_counter += 1;
                    self.lda(param);
                }

                0xAA => {self.tax();}

                0xE8 => {self.inx();}

                0x00 => {return;}

                _ => {}

            }

        }
    }
}