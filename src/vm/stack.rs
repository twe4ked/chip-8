const SIZE: usize = 12;

pub struct Stack {
    sp: usize,
    stack: [u16; 12],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            sp: 0,
            stack: [0; 12],
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), &'static str> {
        if self.sp == SIZE {
            Err("stack overflow")
        } else {
            self.stack[self.sp] = value;
            self.sp += 1;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<u16, &'static str> {
        if self.sp == 0 {
            Err("stack empty")
        } else {
            self.sp -= 1;
            Ok(self.stack[self.sp])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), Err("stack empty"));

        // Push 12 items onto the stack
        assert!(stack.push(1).is_ok());
        for x in 2..=12 {
            assert!(stack.push(x).is_ok());
        }

        // Pushing a 13th item fails
        assert_eq!(stack.push(13), Err("stack overflow"));

        // Pop 12 items off the stack
        assert_eq!(stack.pop().unwrap(), 12);
        for x in (1..=11).rev() {
            assert_eq!(stack.pop().unwrap(), x);
        }

        // Popping a 13th item fails
        assert_eq!(stack.pop(), Err("stack empty"));
    }
}
