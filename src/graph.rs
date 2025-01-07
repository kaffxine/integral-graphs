use std::alloc;
use std::error;
use std::fmt;
use std::ptr;
use rand::thread_rng;
use rand::distributions::{Distribution, Bernoulli};

/// Adjacency matrix, representing a graph.
#[derive(Debug)]
pub struct AdjMatrix {
    last_node: u32,
    n_bits: u64,
    n_bytes: usize,
    data: *mut u8,
}

impl AdjMatrix {
    /// Initialize an empty graph.
    pub fn empty(n_nodes: u64) -> Result<Self, String> {
        let (last_node, n_bits, n_bytes) = Self::calculate_primitive_fields(n_nodes)?;
        let data = unsafe { Self::alloc(n_bytes)? };

        Ok(Self { last_node, n_bits, n_bytes, data })
    }
    
    /// Initialize a random graph, with an approximate density.
    pub fn random(n_nodes: u64, density: f64) -> Result<Self, String> {
        if density < 0.0 || 1.0 < density {
            return Err("density must be in [0, 1] range".to_string());
        }

        let bernoulli = Bernoulli::new(density)
            .map_err(|_| "failed to init Bernoulli")?;
        let mut rng = thread_rng();

        let (last_node, n_bits, n_bytes) = Self::calculate_primitive_fields(n_nodes)?;
        let data = unsafe { Self::alloc(n_bytes)? };

        for byte_i in 0..n_bytes {
            let mut byte = 0u8;
            for bit_i in 0..8 {
                if bernoulli.sample(&mut rng) {
                    byte |= 0x80 >> bit_i;
                }
            }
            unsafe { *data.add(byte_i) = byte; };
        }

        Ok(Self { last_node, n_bits, n_bytes, data })
    }

    /// Initialize a graph from adjacency lists.
    pub fn from_adj_lists(adj_lists: Vec<(u32, Vec<u32>)>) -> Result<Self, String> {
        let mut output = Self {
            last_node: 0,
            n_bits: 0,
            n_bytes: 0,
            data: ptr::null_mut(),
        };

        for adj_list in adj_lists.into_iter() {
            let (node, adj_list) = adj_list;
            if node > output.last_node {
                unsafe { output = output.realloc(node)?; };
            }
            for adj in adj_list.into_iter() {
                if adj > output.last_node {
                    unsafe { output = output.realloc(adj)?; };
                }
                let index = output.index_of(node, adj)?;
                output.set(node, adj, true)?;
            }
        }

        Ok(output)
    }

    /// Return adjacency lists of each node.
    pub fn adj_lists(&self) -> Vec<(u32, Vec<u32>)> {
        let output: Vec<(u32, Vec<u32>)> = (0..(self.last_node + 1))
            .map(|index| (index, self.adj_list(index)))
            .collect();
        output
    }

    /// Return adjacency list of a single node.
    pub fn adj_list(&self, node: u32) -> Vec<u32> {
        let mut output = Vec::new();
        for i in 0..node {
            if unsafe { self.unsafe_is_edge(i, node) } {
                output.push(i);
            }
        }
        for i in (node + 1)..(self.last_node + 1) {
            if unsafe { self.unsafe_is_edge(node, i) } {
                output.push(i);
            }
        }
        output
    }

    /// Encode the graph in base64.
    pub fn base64(&self) -> String {
        let n_chunks = self.n_bits / 6;
        let n_remaining_bits = (self.n_bits % 6) as u8;
        let mut output = String::with_capacity(n_chunks as usize + (n_remaining_bits != 0) as usize);

        // should work for 0 < n_bits <= 8
        let get_bits_at = |bit_i: u64, n_bits: u8| -> u8 {
            let byte_i = (bit_i / 8) as usize;
            let bit_offset = (bit_i % 8) as u8;
            
            let byte_cur = unsafe { *self.data.add(byte_i) };
            if bit_offset > 8 - n_bits {
                let byte_next = unsafe { *self.data.add(byte_i + 1) };
                byte_cur << bit_offset >> (8 - n_bits) | byte_next >> (16 - n_bits - bit_offset)
            } else {
                byte_cur >> (8 - n_bits - bit_offset) & (0xff >> (8 - n_bits))
            }
        };
        
        for chunk_i in 0..n_chunks {
            let chunk = get_bits_at(6 * chunk_i, 6);
            output.push((chunk + 0x3f) as char);
        }

        let chunk = get_bits_at(6 * n_chunks, n_remaining_bits) << (6 - n_remaining_bits);
        output.push((chunk + 0x3f) as char);

        output
    }

    /// Encode the graph in graph6.
    pub fn graph6(&self) -> Result<String, String> {
        if self.last_node > 64 {
            return Err("Cannot encode a graph with more than 64 vertices in graph6".to_string());
        }
        
        let mut output = String::with_capacity(((self.n_bits + 1) / 6 + 1) as usize);
        output.push((self.last_node as u8 + 64) as char);
        output.push_str(&self.base64());
        
        Ok(output)
    }
    
    /// Check if there is a link between two nodes.
    pub fn is_edge(&self, node_a: u32, node_b: u32) -> Result<bool, String> {
        let bit_index = self.index_of(node_a, node_b)?;
        let byte_index = (bit_index as usize) / 8;
        let bit_index = bit_index % 8;

        let byte = unsafe { *self.data.add(byte_index) };
        Ok(byte & 0x80 >> bit_index != 0)
    }

    /// Unsafe is_edge, for internal use only.
    unsafe fn unsafe_is_edge(&self, node_a: u32, node_b: u32) -> bool {
        let bit_index = self.unchecked_index_of(node_a, node_b);
        let byte_index = (bit_index as usize) / 8;
        let bit_index = bit_index % 8;

        let byte = unsafe { *self.data.add(byte_index) };
        byte & 0x80 >> bit_index != 0
    }

    /// Change the value of a matrix field
    /// corresponding to the indices of two nodes.
    fn set(&mut self, node_a: u32, node_b: u32, value: bool) -> Result<(), String> {
        let bit_index = self.index_of(node_a, node_b)?;
        let byte_index = (bit_index as usize) / 8;
        let bit_index = bit_index % 8;

        unsafe {
            if value {
                *self.data.add(byte_index) |= 0x80 >> bit_index;
            } else {
                *self.data.add(byte_index) &= !(0x80 >> bit_index);
            }
        }
        
        Ok(())
    }

    /// Same as set, but without bounds check. Use with care,
    /// as wrong node_a and node_b can cause a memory leak.
    /// Check unchecked_index_of for bound guidelines.
    unsafe fn unsafe_set(&mut self, node_a: u32, node_b: u32, value: bool) {
        let bit_index = self.unchecked_index_of(node_a, node_b);
        let byte_index = (bit_index as usize) / 8;
        let bit_index = bit_index % 8;

        unsafe {
            if value {
                *self.data.add(byte_index) |= 0x80 >> bit_index;
            } else {
                *self.data.add(byte_index) &= !(0x80 >> bit_index);
            }
        }
    }

    /// Calculate the bit index of a node pair.
    fn index_of(&self, node_a: u32, node_b: u32) -> Result<u64, String> {
        if node_a == node_b 
            || node_a > self.last_node
            || node_b > self.last_node
        { 
            return Err(
                format!("wrong indices: ({}, {}), while max is {}",
                    node_a, node_b, self.last_node));
        }

        let (node_a, node_b) = if node_a < node_b {
            (node_a as u64, node_b as u64)
        } else {
            (node_b as u64, node_a as u64)
        };
        
        Ok(node_b * (node_b - 1) / 2 + node_a)
    }

    /// Same as index_of but with no bounds check.
    /// For this function to return a valid index, node_b needs
    /// to be greater than node_a, and both need to be less or
    /// equal to self.last_node.
    fn unchecked_index_of(&self, node_a: u32, node_b: u32) -> u64 {
        node_b as u64 * (node_b as u64 - 1) / 2 + node_a as u64
    }

    fn calculate_primitive_fields(n_nodes: u64) -> Result<(u32, u64, usize), String> {
        if n_nodes - 1 > 0x100000000 {
            return Err("n_nodes must be an integer between 1 and 2^32 (inclusive)".to_string());
        }

        let last_node = (n_nodes - 1) as u32;
        let n_bits = n_nodes * (n_nodes - 1) / 2;
        let n_bytes = (n_bits as usize + 7) / 8;

        Ok((last_node, n_bits, n_bytes))
    }

    unsafe fn alloc(n_bytes: usize) -> Result<*mut u8, String> {
        let data = unsafe {
            alloc::alloc_zeroed(alloc::Layout::array::<u8>(n_bytes)
                .map_err(|_| "allocation layout error")?)
        };
        
        if data.is_null() {
            return Err("allocated a null pointer".to_string());
        }

        Ok(data)
    }



    unsafe fn realloc(mut self, last_node: u32) -> Result<Self, String> {
        let n_nodes = last_node as u64 + 1;
        let n_bits = n_nodes * (n_nodes - 1) / 2;
        let n_bytes = (n_bits as usize + 7) / 8;
        let data = unsafe {
            alloc::realloc(
                self.data,
                alloc::Layout::array::<u8>(self.n_bytes)
                    .map_err(|_| "reallocation layout error")?,
                n_bytes,
            )
        };
    
        if data.is_null() {
            return Err("reallocated a null pointer".to_string());
        }

        Ok(Self {
            last_node,
            n_bits,
            n_bytes,
            data,
        })
    }
}

impl fmt::Display for AdjMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut bit_index = 0;
        let mut adj_count = 1;
        let mut next_node_index = 1;
        
        // this code is so pretty i refuse to comment it
        loop {
            let byte_index = bit_index / 8;
            let new_node_in_this_byte = next_node_index / 8 == byte_index;

            let bit_seq_start = bit_index % 8;
            let bit_seq_end = if new_node_in_this_byte {
                next_node_index % 8
            } else {
                8
            };

            if bit_seq_end > bit_seq_start {
                let bits = unsafe { *self.data.add(byte_index as usize) };
                let output = bits << bit_seq_start >> (bit_seq_start + 8 - bit_seq_end);
                let output_width = (bit_seq_end - bit_seq_start) as usize;
                write!(f, "{:0output_width$b}", output)?;
            }
    
            if new_node_in_this_byte {
                bit_index += bit_seq_end - bit_seq_start;
                adj_count += 1;
                next_node_index = bit_index + adj_count;
                if bit_index == self.n_bits {
                    break;
                } else {
                    write!(f, "\n");
                }
            } else {
                bit_index = (bit_index >> 3 << 3) + 8;
            }
        }

        Ok(())
    }
}

impl Drop for AdjMatrix {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.data,
                alloc::Layout::array::<u8>(self.n_bytes)
                    .expect("failed to deallocate AdjMatrix")
            )
        }
    }
}
