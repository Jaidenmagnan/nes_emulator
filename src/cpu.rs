pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_y: u8,
    memory: [u8; 0xffff],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            register_y: 0,
            program_counter: 0,
            register_x: 0,
            memory: [0; 0xffff],
        }
    }

    // all of our memory operations
    pub fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
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


    // loads, runs, and resets
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
        self.register_y = 0;
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
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

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
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }

                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }

                0xAD => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }

                0xAA => {self.tax();}

                0xE8 => {self.inx();}

                0x00 => {return;}

                _ => {}

            }

        }

    }

    // each instruction has specific modes which tells the CPU how to interpret the next 1 or 2 bytes
    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {return self.program_counter;}

            AddressingMode::ZeroPage => {return self.mem_read(self.program_counter) as u16;}

            AddressingMode::Absolute => {return self.mem_read_u16(self.program_counter);}

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                return addr;
            }

            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                return addr;
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                return addr;
            }
            
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                return addr;
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                return (hi as u16) << 8 | (lo as u16);
            }

            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                return deref;
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }



        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}