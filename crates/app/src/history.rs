use circular_buffer::FixedCircularBuffer;

pub type History<T> = FixedCircularBuffer<T, 1024>;
