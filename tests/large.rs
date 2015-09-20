extern crate json_parser;
use json_parser::*;

const INPUT: &'static str = r##"[
  {
    "_id": "55fedc235535491c823669ae",
    "index": 0,
    "guid": "080313e0-07f2-4a48-980a-847f493a9993",
    "isActive": true,
    "balance": "$1,805.36",
    "picture": "http://placehold.it/32x32",
    "age": 25,
    "eyeColor": "blue",
    "name": "Charlotte Michael",
    "gender": "female",
    "company": "AQUASURE",
    "email": "charlottemichael@aquasure.com",
    "phone": "+1 (942) 448-2281",
    "address": "447 Brooklyn Avenue, Hickory, Georgia, 3311",
    "about": "Fugiat amet occaecat consequat ullamco incididunt ea eu tempor. Ad sit anim reprehenderit ea Lorem incididunt qui pariatur. Ullamco aliquip culpa deserunt magna deserunt nulla. Commodo enim incididunt ullamco nostrud. Nulla non laboris quis id. Irure exercitation Lorem duis laborum.\r\n",
    "registered": "2015-05-13T02:55:38 -02:00",
    "latitude": 70.093611,
    "longitude": -116.084713,
    "tags": [
      "deserunt",
      "velit",
      "enim",
      "quis",
      "voluptate",
      "commodo",
      "minim"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Ford Carrillo"
      },
      {
        "id": 1,
        "name": "Janis Travis"
      },
      {
        "id": 2,
        "name": "Colette Mccall"
      }
    ],
    "greeting": "Hello, Charlotte Michael! You have 7 unread messages.",
    "favoriteFruit": "apple"
  },
  {
    "_id": "55fedc2319d86928abf3bb6b",
    "index": 1,
    "guid": "cd429de6-49c3-4494-8223-8beac7f0c9cc",
    "isActive": false,
    "balance": "$3,638.30",
    "picture": "http://placehold.it/32x32",
    "age": 21,
    "eyeColor": "green",
    "name": "Sanchez Francis",
    "gender": "male",
    "company": "OLUCORE",
    "email": "sanchezfrancis@olucore.com",
    "phone": "+1 (908) 429-3732",
    "address": "389 Ashland Place, Roland, Alaska, 8069",
    "about": "Elit dolor voluptate duis dolor ut exercitation. Et adipisicing exercitation esse ut culpa consectetur irure cupidatat magna irure ullamco ut exercitation. Labore quis consectetur Lorem laboris ut tempor elit incididunt proident. Nulla eiusmod Lorem fugiat do incididunt velit fugiat. Reprehenderit irure quis est occaecat incididunt.\r\n",
    "registered": "2014-10-25T02:40:04 -02:00",
    "latitude": -58.837502,
    "longitude": -64.414358,
    "tags": [
      "officia",
      "deserunt",
      "reprehenderit",
      "labore",
      "ad",
      "nulla",
      "pariatur"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Antoinette Humphrey"
      },
      {
        "id": 1,
        "name": "Tonya Silva"
      },
      {
        "id": 2,
        "name": "Josefina Neal"
      }
    ],
    "greeting": "Hello, Sanchez Francis! You have 7 unread messages.",
    "favoriteFruit": "banana"
  },
  {
    "_id": "55fedc232311e33f3016ffaa",
    "index": 2,
    "guid": "263a47ff-7cf0-4210-8bbe-16b2fa144799",
    "isActive": false,
    "balance": "$1,134.91",
    "picture": "http://placehold.it/32x32",
    "age": 36,
    "eyeColor": "brown",
    "name": "Leticia Vance",
    "gender": "female",
    "company": "MAGMINA",
    "email": "leticiavance@magmina.com",
    "phone": "+1 (863) 435-2462",
    "address": "531 Ridge Boulevard, Linganore, North Carolina, 3360",
    "about": "Culpa proident ea deserunt elit et. Ipsum aliqua ad sunt in eu sunt dolore. Laborum anim excepteur sint labore cillum tempor occaecat amet aliquip ipsum est laborum quis non. Tempor qui officia sunt proident velit sunt mollit cillum. Aliquip velit ea consequat laboris deserunt aliquip dolor quis cupidatat quis adipisicing nulla Lorem qui. Officia aute velit do eu do aliqua magna officia mollit deserunt ea aliqua culpa excepteur.\r\n",
    "registered": "2014-03-26T05:54:14 -01:00",
    "latitude": 47.970994,
    "longitude": 157.41962,
    "tags": [
      "cupidatat",
      "nostrud",
      "adipisicing",
      "velit",
      "amet",
      "ut",
      "proident"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Mayer Madden"
      },
      {
        "id": 1,
        "name": "Castro Burch"
      },
      {
        "id": 2,
        "name": "Mccoy Benjamin"
      }
    ],
    "greeting": "Hello, Leticia Vance! You have 6 unread messages.",
    "favoriteFruit": "strawberry"
  },
  {
    "_id": "55fedc23e77f5213b0282956",
    "index": 3,
    "guid": "37d5116f-95ad-4ff9-b11c-b67453a14f76",
    "isActive": true,
    "balance": "$3,196.28",
    "picture": "http://placehold.it/32x32",
    "age": 33,
    "eyeColor": "green",
    "name": "Alexis Mendoza",
    "gender": "female",
    "company": "DATAGEN",
    "email": "alexismendoza@datagen.com",
    "phone": "+1 (940) 484-2402",
    "address": "634 Church Lane, Cascades, Minnesota, 3221",
    "about": "Sint ea veniam eiusmod quis qui deserunt magna aliqua ea. Occaecat adipisicing qui minim minim officia cillum minim laboris duis nostrud nisi. Qui enim aliqua magna consectetur. Do sunt esse aute nisi elit duis voluptate ullamco sit pariatur tempor eu.\r\n",
    "registered": "2014-12-09T09:33:10 -01:00",
    "latitude": 87.935534,
    "longitude": -85.293835,
    "tags": [
      "dolore",
      "dolore",
      "cillum",
      "enim",
      "nisi",
      "excepteur",
      "voluptate"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Parsons Watson"
      },
      {
        "id": 1,
        "name": "Carole Morales"
      },
      {
        "id": 2,
        "name": "Wallace Hoover"
      }
    ],
    "greeting": "Hello, Alexis Mendoza! You have 1 unread messages.",
    "favoriteFruit": "apple"
  },
  {
    "_id": "55fedc231c631fb2b50bd658",
    "index": 4,
    "guid": "4a0700c9-6c19-4a30-9247-0fe3a96250b0",
    "isActive": true,
    "balance": "$3,356.32",
    "picture": "http://placehold.it/32x32",
    "age": 31,
    "eyeColor": "blue",
    "name": "Elisa Duncan",
    "gender": "female",
    "company": "SURELOGIC",
    "email": "elisaduncan@surelogic.com",
    "phone": "+1 (854) 596-2049",
    "address": "332 Williams Avenue, Watchtower, Montana, 7704",
    "about": "Deserunt deserunt cupidatat eu incididunt esse consectetur ad occaecat eu eiusmod cupidatat exercitation minim mollit. Sit cupidatat duis ullamco elit irure nulla voluptate. Qui do est nostrud reprehenderit labore irure reprehenderit. Pariatur tempor excepteur nisi tempor cillum aliquip culpa reprehenderit ea. Proident eu mollit elit labore duis laboris sit.\r\n",
    "registered": "2015-08-15T09:43:23 -02:00",
    "latitude": 72.46233,
    "longitude": -47.296325,
    "tags": [
      "deserunt",
      "nulla",
      "magna",
      "tempor",
      "fugiat",
      "non",
      "commodo"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Teresa Graves"
      },
      {
        "id": 1,
        "name": "Jodie Erickson"
      },
      {
        "id": 2,
        "name": "Page Bennett"
      }
    ],
    "greeting": "Hello, Elisa Duncan! You have 9 unread messages.",
    "favoriteFruit": "apple"
  },
  {
    "_id": "55fedc23c16a60dbfdae5488",
    "index": 5,
    "guid": "4ca21f77-a2e3-4fc3-b374-3670aad66bba",
    "isActive": true,
    "balance": "$3,977.05",
    "picture": "http://placehold.it/32x32",
    "age": 20,
    "eyeColor": "green",
    "name": "Magdalena Dorsey",
    "gender": "female",
    "company": "HOPELI",
    "email": "magdalenadorsey@hopeli.com",
    "phone": "+1 (962) 446-3074",
    "address": "140 Allen Avenue, Boomer, Tennessee, 3492",
    "about": "Amet et laboris nostrud quis est in magna deserunt exercitation consequat. Nisi incididunt aliquip cupidatat minim officia eu exercitation eu. Excepteur sit consectetur veniam excepteur dolore dolore. Eiusmod aliquip ea enim et dolore quis ullamco irure. Ipsum dolore ad do ullamco esse adipisicing laboris in. Eiusmod occaecat consequat tempor sint reprehenderit tempor laborum quis commodo enim ea.\r\n",
    "registered": "2014-07-30T05:33:16 -02:00",
    "latitude": 5.916797,
    "longitude": 69.166058,
    "tags": [
      "commodo",
      "dolore",
      "aliqua",
      "elit",
      "deserunt",
      "excepteur",
      "elit"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Roberson Chang"
      },
      {
        "id": 1,
        "name": "Cynthia Walker"
      },
      {
        "id": 2,
        "name": "Rodriguez Chen"
      }
    ],
    "greeting": "Hello, Magdalena Dorsey! You have 9 unread messages.",
    "favoriteFruit": "banana"
  }
]"##;

#[test]
fn large() {
  parse_json(INPUT);
}
