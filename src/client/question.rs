use std::error::Error;

use crate::client::utility;

pub struct Question {
    pub q_name: Vec<u8>,
    pub q_type: u16,
    pub q_class: u16,
}

impl Question {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut question = self.q_name.to_vec();

        let mut bytes = self.q_type.to_be_bytes().to_vec();
        question.append(&mut bytes);

        bytes = self.q_class.to_be_bytes().to_vec();
        question.append(&mut bytes);

        question
    }

    pub fn parse(message: &Vec<u8>, start: usize) -> Result<(usize, Question), Box<dyn Error>> {
        let null_pos = utility::find_first_null(&message[start..])?;
        let q_name = message[start..start + null_pos + 1].to_vec();
        let q_type = utility::to_u16(&message[start + null_pos + 1..start + null_pos + 3]);
        let q_class = utility::to_u16(&message[start + null_pos + 3..start + null_pos + 5]);

        let q = Question {
            q_name,
            q_type,
            q_class,
        };

        Ok((start + null_pos + 5, q))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_question() {
        let name = vec!['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8];
        let question = Question {
            q_name: name,
            q_type: 1,
            q_class: 1,
        };

        let mut bytes = question.q_name.to_vec();
        bytes.push(0x00);
        bytes.push(0x01);
        bytes.push(0x00);
        bytes.push(0x01);
        assert_eq!(question.to_be_bytes(), bytes);
    }
}
