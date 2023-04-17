use crate::protocol::response::ProtoResponseState::ResultData;
use crate::util::log;
use crate::util::u8::{U8Serializable, U8VecSerializable};

const TAG: &'static str = "proto/response";

#[derive(PartialEq)]
pub enum ProtoResponseState {
    ResultOk,
    ResultError,
    ResultBadRequest,
    ResultData,
}

impl U8Serializable for ProtoResponseState {
    fn to_u8(&self) -> u8 {
        match *self {
            ProtoResponseState::ResultOk => 0x0,
            ProtoResponseState::ResultError => 0x1,
            ProtoResponseState::ResultBadRequest => 0x2,
            ProtoResponseState::ResultData => 0x3,
        }
    }
}

pub struct ProtoResponse {
    result: Vec<u8>,
    state_only: bool,
    pub(crate) state: ProtoResponseState,
}

impl U8VecSerializable for ProtoResponse {
    fn to_u8_vec(&self) -> Vec<u8> {
        if self.state_only {
            return self.state.to_u8_vec();
        }
        let state = ProtoResponseState::ResultData.to_u8();
        let size = self.result.len();
        if size > 255 {
            log::panic(TAG, "Too big response");
        }
        let size_u8: u8 = size as u8;
        let mut response: Vec<u8> = vec![state, size_u8];
        response.extend(self.result.clone());
        response
    }
}

impl ProtoResponse {
    pub fn add_response(&mut self, response: Box<dyn U8VecSerializable>) {
        self.state = ResultData;
        self.state_only = false;
        self.result.extend(response.to_u8_vec());
    }
    pub(crate) fn from_state(state: ProtoResponseState) -> ProtoResponse {
        ProtoResponse {
            result: vec![],
            state_only: true,
            state,
        }
    }
}