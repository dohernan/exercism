//use crate::Error::EmptyBuffer;

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    buffer: Vec<Option<T>>,
    write_position: usize,
    read_position: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let buffer : Vec<Option<T>> = (0..capacity).map(|_| None::<T>).collect();
        CircularBuffer{buffer , write_position: 0, read_position:0, capacity}
        // todo!(
        //     "Construct a new CircularBuffer with the capacity to hold {}.",
        //     match capacity {
        //         1 => "1 element".to_string(),
        //         _ => format!("{capacity} elements"),
        //     }
        // );
    }

    pub fn increment_read_position(&mut self)
    {
        self.read_position = (self.read_position + 1)%self.capacity;
    }

     pub fn increment_write_position(&mut self)
    {
        let next_position = (self.write_position + 1)%self.capacity;
        // if self.read_position == next_position{
        //     self.increment_read_position();
        // }
        self.write_position = next_position;
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
      //"Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full."
        if self.buffer[self.write_position].is_some()
        {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.write_position] = Some(_element);
        self.increment_write_position();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        // "Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty."

        let result = self.buffer[self.read_position].take().ok_or(Error::EmptyBuffer)?;
        self.increment_read_position();
        Ok(result)
    }

    pub fn clear(&mut self) {
        let mut result = self.read();
        while result.is_ok()
        {
            result = self.read();
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        self.buffer[self.write_position] = Some(_element);
        if self.write_position == self.read_position
        {
            self.increment_read_position();
        }
        self.increment_write_position();
    }
}
