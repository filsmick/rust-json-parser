extern crate json_parser;
use json_parser::*;


#[test]
fn test_huge() {
  let input = r##"{
  "_id": "55fe8588577622a8d848e4a0",
  "index": 0,
  "guid": "4bf11885-5390-4048-807e-38adc8eeec52",
  "isActive": true,
  "balance": "$1,197.39",
  "picture": "http://placehold.it/32x32",
  "age": 29,
  "eyeColor": "brown",
  "name": "Short Huffman",
  "gender": "male",
  "company": "AUSTEX",
  "email": "shorthuffman@austex.com",
  "phone": "+1 (852) 562-3203",
  "address": "340 Ford Street, Chesterfield, California, 4255",
  "about": "Voluptate id anim magna ad sit est laborum reprehenderit velit. Duis eiusmod tempor ipsum consectetur. Enim velit nisi sint ex voluptate anim irure minim ea do esse excepteur exercitation. Occaecat mollit duis cupidatat qui elit dolore. Labore reprehenderit mollit deserunt eu voluptate ex id adipisicing non duis incididunt id eiusmod officia. Anim magna do in irure anim consequat officia Lorem fugiat elit laborum voluptate. Et ipsum qui incididunt ex tempor tempor labore id ad.\r\n",
  "registered": "2014-06-13T12:13:56 -02:00",
  "latitude": -79.472824,
  "longitude": -66.216522,
  "tags": [
    "id",
    "nulla",
    "mollit",
    "elit",
    "ad",
    "aute",
    "aute"
  ],
  "friends": [
    {
      "id": 0,
      "name": "Bridges Kirkland"
    },
    {
      "id": 1,
      "name": "Julie Contreras"
    },
    {
      "id": 2,
      "name": "Ruth Tanner"
    }
  ],
  "greeting": "Hello, Short Huffman! You have 3 unread messages.",
  "favoriteFruit": "strawberry"
}"##;

  parse_json(input);
}
