pub trait U8Serializable {
    fn to_u8(&self) -> u8;
}

pub trait U8VecSerializable {
    fn to_u8_vec(&self) -> Vec<u8>;
}

impl<T> U8VecSerializable for T
where T: U8Serializable
{
    fn to_u8_vec(&self) -> Vec<u8> {
        vec![self.to_u8()]
    }
}

impl<T> U8VecSerializable for Vec<T>
where T: U8VecSerializable
{
    fn to_u8_vec(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for serializable in self.iter(){
            result.extend(serializable.to_u8_vec());
        }
        result
    }
}