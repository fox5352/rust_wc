pub mod file_processing {
    use core::fmt;
    use std::{
        collections::HashMap, 
        fmt::Display, 
        fs::File, 
        io::{
            BufRead, 
            BufReader, 
            Error, 
            Read
        }, 
        path::Path
    };

    pub struct FileProcessingConfig {
        pub filename: String,
        pub flags: Vec<char>,
        pub query: String
    }

    impl FileProcessingConfig {
        pub fn new<T: Iterator<Item = String> + ExactSizeIterator>(mut args: T) -> Result<FileProcessingConfig, Error> {
            // eat file name
            args.next().unwrap();

            if args.len() < 1 {
                return Err(
                    Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Not enough arguments"
                    )
                );
            }

            if args.len() < 2 {
                let flags: Vec<char> = vec!['c','l', 'w'];

                let query: String = args.next().unwrap();
                let filename = query.clone();

                return Ok(FileProcessingConfig {
                    filename,
                    flags,
                    query
                });

            }else {
                let buffer: String = args.next().unwrap();

                let mut flags: Vec<char> = Vec::new();
                let query: String = args.next().unwrap();

                for (_i, item) in buffer.as_bytes().iter().enumerate() {
                    if *item != b'-' {
                        flags.push(*item as char);
                    }
                }

                let filename = query.clone();
                
                return Ok(FileProcessingConfig {
                    filename,
                    flags,
                    query
                });
            }
        }
        
        pub fn run(&self) -> Result<String, Error> {
            let mut formatted_string = String::new();
            let mut map: HashMap<char, Box<dyn Fn() -> String>>  = HashMap::new();

            // add functions
            map.insert('c', Box::new(|| -> String {self.get_byte_size()}));
            map.insert('l', Box::new(|| -> String {self.get_lines_count()}));
            map.insert('w', Box::new(|| -> String {self.get_word_count()}));
            map.insert('m', Box::new(|| -> String { self.get_char_count() }));


            // loop flags
            for char in self.flags.iter() {
                if let Some(function) = map.get(char) {
                    formatted_string.push_str(&function());
                }
            }


            return Ok(formatted_string);
        }

        fn get_byte_size(&self) -> String {
            let size: BufReader<File> = match read_file_to_buffer(&self.query) {
                Ok(data) => data,
                Err(e) => panic!("{}", e)
            };

            let bytes_count = size.bytes().count().to_string();
            return self.formatter(String::from("bytes"), bytes_count);
        }

        fn get_lines_count(&self) -> String {
            let file_buffer: BufReader<File> = match read_file_to_buffer(&self.query) {
                Ok(data) => data,
                Err(e) => panic!("{}", e)
            };

            let lines_count = file_buffer.lines().count().to_string();

            return self.formatter(String::from("lines"), lines_count);
        }

        fn get_word_count(&self) -> String{
            let file_buffer = match read_file_to_buffer(&self.query) {
                Ok(data) => data,
                Err(e) => panic!("{}", e)
            };

            let mut word_count = 0;            
            
            for line in file_buffer.lines() {
                let line_buffer = line.unwrap();
                word_count += line_buffer.split_whitespace().count()
            }

            return self.formatter(String::from("words"),word_count)
        }

        fn get_char_count(&self) -> String {
            let file_buffer = match read_file_to_buffer(&self.query) {
                Ok(data) => data,
                Err(e) => panic!("{}", e),
            };

            let mut char_count = 0;

            for line in file_buffer.lines() {
                let line_buffer = line.unwrap();

                char_count += line_buffer.len();
                // println!("{} {}", line_buffer, line_buffer.len());
            }

            return self.formatter(String::from("characters"), char_count.to_string());
        }   

        fn formatter<T: Display>(&self, data_type: String, value: T) -> String {
            return format!("{} {} in {} \n", data_type, value, self.filename);
        }

    }
    

    impl fmt::Display for FileProcessingConfig {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "FileProcessingConfig flags: {:?}, query: {:?}", self.flags, self.query)
        }
    }
    fn read_file_to_buffer(path: &str) -> Result<BufReader<File>, Error> {
        let file = File::open(Path::new(path))?;
    
        Ok(BufReader::new(file))
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
