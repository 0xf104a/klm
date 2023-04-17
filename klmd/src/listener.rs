/**
 * This file is part of KLMd project.
 *
 *  Copyright 2022-2023 by Polar <toddot@protonmail.com>
 *
 *  Licensed under GNU General Public License 3.0 or later.
 *  Some rights reserved. See COPYING, AUTHORS.
 *
 * @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
 */


use crate::protocol;
use crate::util::log;
use crate::keyboard;

use std::os::unix::net::UnixListener;
use std::os::unix::fs::PermissionsExt;
use std::io::prelude::*;
use users::{Groups, UsersCache};
use file_owner::PathExt;
use crate::util::u8::U8Serializable;
use crate::util::u8::U8VecSerializable;

const TAG: &'static str = "listener";

fn set_socket_permissions(){
    let cache = UsersCache::new();
    let group = cache.get_group_by_name("klm");
    let perms = std::fs::Permissions::from_mode(0o660);
    if group.is_none() {
        log::w(TAG, "You do not have klm group in your system.");
        log::w(TAG, "The permissions for socket would be set, but you may be unable to access it");
    } else {
        "/var/run/klmd.sock".set_group("klm").unwrap();
    }
    std::fs::set_permissions("/var/run/klmd.sock",
                             perms).unwrap();
}
//Listeners accept UNIX-socket connections
//and reads to buffer requests. Then it passes
//buffer to protocol handler.
//TODO: check errors in listen
pub fn listen(keyboard: &mut keyboard::Keyboard){
    let listener = UnixListener::bind("/var/run/klmd.sock").unwrap();

    set_socket_permissions();

    log::i(TAG, "Started listening at /var/run/klmd.sock");

    loop{
        match listener.accept() {
            Ok((mut sock, addr)) => {
                log::d(TAG, &format!("Received connection from {:?} - {:?}", sock, addr));
                let mut size_buffer = [0; 1];
                let mut response = [0; 1];
                let mut data_buffer = [0; 1];
                sock.read_exact(&mut size_buffer).unwrap();

                let sz = size_buffer[0];
                log::d(TAG, &format!("Expecting request size to be {} bytes", sz));
                if sz > 0 {
                    let mut buffer = Vec::<u8>::with_capacity(sz as usize);
                    for _ in 0..sz {
                        sock.read_exact(&mut data_buffer).unwrap();
                        buffer.push(data_buffer[0]);
                    }
                    let result = protocol::proto::proto_handle_message(keyboard, &buffer);
                    let result_vec = result.to_u8_vec();
                    sock.write_all(&result_vec).unwrap();
                } else {
                    log::e(TAG, "Request length is zero. Responding with bad request.");
                    response[0] = protocol::response::ProtoResponseState::ResultBadRequest.to_u8();
                    sock.write_all(&response).unwrap();
                }
            },
            Err(e) => log::e(TAG, &format!("accept: {:?}", e)),
        }
    }
}
