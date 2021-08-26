use vpp_macros::UnionIdent;
use generic_array::{ArrayLength, GenericArray};
use serde::{Serialize, Deserialize};
use vpp_api_encoding::typ::*;
use std::convert::TryInto;
use typenum;

#[derive(Debug, Clone, UnionIdent)]
#[types(IP4Address:4)]
pub struct AddressUnion(FixedSizeArray<u8, typenum::U16>);

type IP4Address = [u8;4]; 
type IP6Address = [u8;16]; 



fn main(){
//let mut felix = AddressUnion::new_IP4Address([10,10,1,2]);
println!("{:#?}", AddressUnion::new_IP4Address([10,10,1,2]).0);

assert_eq!(32, 32);
// assert_eq!("Idiot", MyStruct::get_message_name_and_crc());
}