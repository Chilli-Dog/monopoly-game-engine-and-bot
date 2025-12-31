// This is going to be the environment for the agents to play in.
use std::collections::HashMap;
use rand::prelude::*;

// This defines both types of cards present on the board
pub struct TreasureCards {
    cards_money: Vec<i32>,
    cards_jail: Vec<String>,
    // this hold the cards with a 'per house/hotel'
    // price: i32, per_X: String
    cards_dependent: HashMap<i32, String>,
}

impl TreasureCards {
    pub fn init(num_of_players: i32) -> Self {
        let cards_treasure_money: Vec<i32> = vec![-50, 200, 10, -100, 100, 20,
                                                  25, 50, 100, -50, 100, 10 * num_of_players];

        let cards_treasure_jail: Vec<String> = vec!["go to jail".to_string(), "get out free".to_string()];

        let mut cards_treasure_dependent: HashMap<i32, String> = HashMap::new();
        cards_treasure_dependent.insert(-40, "per house".to_string());
        cards_treasure_dependent.insert(-115, "per hotel".to_string());

        let treasure_cards = TreasureCards {
            cards_money: cards_treasure_money,
            cards_jail: cards_treasure_jail,
            cards_dependent: cards_treasure_dependent,
        };

        treasure_cards
    }
}

pub struct ChanceCards {
    // this holds the money related cards
    cards_money: Vec<i32>,
    // this hold the jail related cards like in or out for free
    cards_jail: Vec<String>,
    // this hold the cards with a 'per house/hotel'
    cards_dependent: HashMap<i32, String>,
    // this contains the cards which advance you to somewhere on the board
    cards_movement: Vec<(i16, String)>
}

impl ChanceCards {
    pub fn init(num_of_players: i32) -> Self {
        let cards_chance_money: Vec<i32> = vec![-15, 50, 150, 50 * num_of_players];

        let cards_chance_jail: Vec<String> = vec!["go to jail".to_string(), "get out free".to_string()];

        let cards_chance_movement_strs: Vec<(i16, &str)> = vec![(1, "nearest station"),
                                                                (1, "pall mall"), (1, "go"), (1, "nearest utility"),
                                                                (1, "trafalgar square"), (1, "kings cross station"),
                                                                (1, "nearest station"), (0, "back three spaces"),
                                                                (1, "mayfair")];
        let mut cards_chance_movement: Vec<(i16, String)> = Vec::new();
        for (advance, location) in cards_chance_movement_strs {
            cards_chance_movement.push((advance, location.to_string()));
        }

        let mut cards_chance_dependent: HashMap<i32, String> = HashMap::new();
        cards_chance_dependent.insert(-25, "per house".to_string());
        cards_chance_dependent.insert(-100, "per hotel".to_string());

        let chance_cards = ChanceCards {
            cards_money: cards_chance_money,
            cards_jail: cards_chance_jail,
            cards_dependent: cards_chance_dependent,
            cards_movement: cards_chance_movement,
        };

        chance_cards
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ColourGroup {
    Brown, LightBlue, Pink, Orange, Red, Yellow, Green, DarkBlue
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Property {
    name: String,
    base_price: u32,
    group: ColourGroup,
    // this stores the prices of rent in ascending order from 1 house, 2 houses
    // ... to one hotel etc.
    rent_prices: [u32; 6],
    // price of houses / hotels. The prices are always the same so only need one variable rather
    // than a tuple
    price_of_extras: u32,
    mortgage_potential: i32,
}

impl Property {
    pub fn init() -> Vec<Self> {
        let title_deeds: Vec<Property> = vec![
            Property { name: "old kent road".to_string(), base_price: 60, group: ColourGroup::Brown, rent_prices: [2, 10, 30, 90, 160, 250], price_of_extras: 50, mortgage_potential: 30 },
            Property { name: "whitechapel road".to_string(), base_price: 60, group: ColourGroup::Brown, rent_prices: [4, 20, 60, 180, 320, 450], price_of_extras: 50, mortgage_potential: 30 },
            Property { name: "the angel islington".to_string(), base_price: 100, group: ColourGroup::LightBlue, rent_prices: [6, 30, 90, 270, 400, 550], price_of_extras: 50, mortgage_potential: 50 },
            Property { name: "euston road".to_string(), base_price: 100, group: ColourGroup::LightBlue, rent_prices: [6, 30, 90, 270, 400, 550], price_of_extras: 50, mortgage_potential: 50 },
            Property { name: "pentonville road".to_string(), base_price: 120, group: ColourGroup::LightBlue, rent_prices: [8, 40, 100, 300, 450, 600], price_of_extras: 50, mortgage_potential: 60 },
            Property { name: "pall mall".to_string(), base_price: 140, group: ColourGroup::Pink, rent_prices: [10, 50, 150, 450, 625, 750], price_of_extras: 100, mortgage_potential: 70 },
            Property { name: "northumberland avenue".to_string(), base_price: 140, group: ColourGroup::Pink, rent_prices: [12, 60, 180, 500, 700, 900], price_of_extras: 100, mortgage_potential: 80 },
            Property { name: "whitehall".to_string(), base_price: 160, group: ColourGroup::Pink, rent_prices: [10, 50, 150, 450, 625, 750], price_of_extras: 100, mortgage_potential: 70 },
            Property { name: "vine street".to_string(), base_price: 180, group: ColourGroup::Orange, rent_prices: [16, 80, 220, 600, 800, 1000], price_of_extras: 100, mortgage_potential: 100 },
            Property { name: "marlborough street".to_string(), base_price: 180, group: ColourGroup::Orange, rent_prices: [14, 70, 200, 550, 750, 950], price_of_extras: 100, mortgage_potential: 90 },
            Property { name: "bow street".to_string(), base_price: 200, group: ColourGroup::Orange, rent_prices: [14, 70, 200, 550, 750, 950], price_of_extras: 100, mortgage_potential: 90 },
            Property { name: "fleet street".to_string(), base_price: 220, group: ColourGroup::Red, rent_prices: [18, 90, 250, 700, 875, 1050], price_of_extras: 150, mortgage_potential: 110 },
            Property { name: "strand".to_string(), base_price: 220, group: ColourGroup::Red, rent_prices: [18, 90, 250, 700, 875, 1050], price_of_extras: 150, mortgage_potential: 110 },
            Property { name: "trafalgar square".to_string(), base_price: 240, group: ColourGroup::Red, rent_prices: [20, 100, 300, 750, 925, 1100], price_of_extras: 150, mortgage_potential: 120 },
            Property { name: "picadilly".to_string(), base_price: 260, group: ColourGroup::Yellow, rent_prices: [24, 120, 360, 850, 1025, 1200], price_of_extras: 150, mortgage_potential: 140 },
            Property { name: "coventry street".to_string(), base_price: 260, group: ColourGroup::Yellow, rent_prices: [22, 110, 330, 800, 975, 1150], price_of_extras: 150, mortgage_potential: 130 },
            Property { name: "leicester square".to_string(), base_price: 280, group: ColourGroup::Yellow, rent_prices: [22, 110, 330, 800, 975, 1150], price_of_extras: 150, mortgage_potential: 130 },
            Property { name: "oxford street".to_string(), base_price: 300, group: ColourGroup::Green, rent_prices: [26, 130, 390, 900, 110, 1275], price_of_extras: 200, mortgage_potential: 150 },
            Property { name: "bond street".to_string(), base_price: 300, group: ColourGroup::Green, rent_prices: [28, 150, 450, 1000, 1200, 1400], price_of_extras: 200, mortgage_potential: 160 },
            Property { name: "regent street".to_string(), base_price: 320, group: ColourGroup::Green, rent_prices: [26, 130, 390, 900, 110, 1275], price_of_extras: 200, mortgage_potential: 150 },
            Property { name: "park lane".to_string(), base_price: 350, group: ColourGroup::DarkBlue, rent_prices: [35, 175, 500, 110, 1300, 1500], price_of_extras: 200, mortgage_potential: 175 },
            Property { name: "mayfair".to_string(), base_price: 400, group: ColourGroup::DarkBlue, rent_prices: [50, 200, 600, 1400, 1700, 2000], price_of_extras: 200, mortgage_potential: 200 },
        ];

        title_deeds
    }
    pub fn owned(&self, game: &Game) -> (Option<Player>, bool, i32) {
        let players = game.players.clone();
        let mut already_owned: bool = false;
        for player in players {
            already_owned = player.properties.contains_key(self);
            let num_of_houses = player.properties.get(self).unwrap().0;
            let num_of_hotels = player.properties.get(self).unwrap().1;
            let mut total_rent: i32;
            if num_of_hotels == 1 {
                total_rent = self.rent_prices[5] as i32;
            }
            else {
                total_rent = self.rent_prices[num_of_houses as usize] as i32;
            }
            
            if already_owned {
                return (Some(player), true, total_rent)
            }
            else {
                continue;
            }
        }
        (None, false, 0)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RailRoad {
    name: String,
    base_price: u32,
    rent: [u32; 4],
    mortgage_potential: u32,
}

impl RailRoad {
    pub fn init() -> Vec<Self> {
        let base_price = 200;
        let mortgage_potential = 100;
        vec![
            RailRoad { name: "kings cross station".to_string(), base_price, rent: [25, 50, 100, 200], mortgage_potential, },
            RailRoad { name: "marylebone station".to_string(), base_price, rent: [25, 50, 100, 200], mortgage_potential, },
            RailRoad { name: "liverpool street station".to_string(), base_price, rent: [25, 50, 100, 200], mortgage_potential, },
            RailRoad { name: "fenchurch street station".to_string(), base_price, rent: [25, 50, 100, 200], mortgage_potential, },
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Utility {
    name: String,
    cost: i32,
    mortgage_potential: i32,
}

impl Utility {
    pub fn init() -> Vec<Self> {
        vec![
            Self { name: "electrical company".to_string(), cost: 150, mortgage_potential: 75},
            Self { name: "water works".to_string(), cost: 150, mortgage_potential: 75},
        ]
    }
}

#[derive(Clone)]
pub struct Player {
    current_pos: usize,
    another_roll: (bool, i32),
    // value of it, number of them
    money: Vec<(i32, i32)>,
    // the first i32 value is the number of houses on the property
    // the second i32 value is the number of hotels on the property
    properties: HashMap<Property, (i32, i32)>,
    // if both i32 values are 1 then the rent is *10 of the dice etc.
    // format: (electrical, water)
    utilities: (i32, i32),
    // the i32 value is how many owned
    railroads: Vec<RailRoad>,
    railroads_owned: i32,
    in_jail: bool,
}

impl Player {
    pub fn new() -> Self {
        let mut money = Vec::new();
        money.push((500, 2));
        money.push((100, 4));
        money.push((50, 1));
        money.push((20, 1));
        money.push((10, 2));
        money.push((5, 1));
        money.push((1, 5));
        let mut properties: HashMap<Property, (i32, i32)> = HashMap::new();
        let mut utilities = (0, 0);
        let mut railroads = Vec::with_capacity(4);
        let mut railroads_owned = 0;
        Self {
            current_pos: 0,
            another_roll: (false, 0),
            money,
            properties,
            utilities,
            railroads,
            railroads_owned,
            in_jail: false,
        }
    }
    pub fn roll() -> (u32, u32) {
        let mut rng = rand::rng();

        let dice_1 = rng.random_range(1..=6);
        let dice_2 = rng.random_range(1..=6);

        (dice_1, dice_2)
    }
    pub fn move_x_spaces(&mut self, x: i32) {
        if x == 0 || x == 1 {
            println!("You're trying to move 0 or 1 spaces which isn't possible with 2 die.");
            return;
        }

        let board_size: i32 = 40;
        let old_pos = self.current_pos;
        // this formula allows for the wrapping around of the board such as mayfair + 5 or old kent road - 3
        let new_pos = ((old_pos as i32 + x) % board_size + board_size) % board_size;
        // passed go
        if new_pos < old_pos as i32 {
            self.return_change(200);
        }

        self.current_pos = new_pos as usize;
    }
    pub fn check_balance(&self) -> i32 {
        let mut total_money = 0;
        for i in 0..self.money.len() {
            total_money += self.money[i].0 * self.money[i].1;
        }

        total_money
    }
    pub fn compare_price(&self, price_to_pay: i32) -> bool {
        let total_money = self.check_balance();
        if total_money - price_to_pay >= 0 {
            true
        }
        else {
            false
        }
    }
    pub fn general_pay(
        &mut self,
        mut price: i32,
        recipient: Option<&mut Player>,
        bank_recipient: Option<&mut Bank>
    ) {
        let original_price = price; // Store the total to give to the recipient later
        let denominations = [500, 100, 50, 20, 10, 5, 1];

        for i in 0..denominations.len() {
            while price >= denominations[i] && self.money[i].1 > 0 {
                self.money[i].1 -= 1;
                price -= denominations[i];
            }
        }

        if price > 0 {
            for i in (0..denominations.len()).rev() {
                if self.money[i].1 > 0 && denominations[i] > price {
                    self.money[i].1 -= 1;
                    let change = denominations[i] - price;
                    self.return_change(change);
                    price = 0;
                    break;
                }
            }
        }

        if price > 0 {
            panic!("Player cannot afford this even though check_balance passed!");
        }

        // give money to the recipients
        if let Some(other_player) = recipient {
            other_player.return_change(original_price);
            println!("Paid player {}!", original_price);
        }
        else if let Some(bank) = bank_recipient {
            // Update the bank's specific bill counts
            bank.receive_funds(original_price);
            println!("Paid bank {}!", original_price);
        }
    }
    pub fn return_change(&mut self, mut amount: i32) {
        let denominations = [500, 100, 50, 20, 10, 5, 1];
        for i in 0..denominations.len() {
            let count = amount / denominations[i];
            self.money[i].1 += count;
            amount %= denominations[i];
        }
    }
    pub fn buy_square(&mut self, square: &[Square], bank: &mut Bank) {
        let total_money = self.check_balance();
        let current_square = &square[self.current_pos];

        match current_square {
            // add the properties bought to the properties hashmap of the player
            Square::Street(prop) => {
                if self.compare_price(prop.base_price as i32) {
                    println!("Buying street: {}", prop.name);
                    let price = prop.base_price as i32;
                    self.general_pay(price, None, Some(bank));
                    self.properties.insert(prop.clone(), (0, 0));
                }
            }
            Square::Railroad(rr) => {
                if self.compare_price(rr.base_price as i32) {
                    println!("Buying railroad: {}", rr.name);
                    let price = rr.base_price as i32;
                    self.general_pay(price, None, Some(bank));
                    self.railroads.push(rr.clone());
                    self.railroads_owned += 1;
                }
            }
            Square::Utility { name, cost } => {
                if self.compare_price(*cost as i32) {
                    println!("Buying utility: {}", name);
                    self.general_pay(*cost as i32, None, Some(bank));
                    if name == "electrical company" {
                        self.utilities.0 = 1;
                    }
                    else if name == "water works" {
                        self.utilities.1 = 1;
                    }
                    else {
                        println!("The name of the utility is invalid.");
                        println!("This error can be found in the implementation of 'Player' -> buy_square()");
                    }
                }
            }
            _ => {
                println!("This square cannot be bought.");
            }
        }
    }
    pub fn mortgage_property(&mut self, property: Property) {
        for (prop_iter, _houses_num_iter) in self.properties.clone() {
            if prop_iter.name == property.name {
                let amount_to_refund = property.mortgage_potential;
                self.properties.remove(&property);
                self.return_change(amount_to_refund);
            }
        }
    }
    pub fn mortgage_railroad(&mut self, rr: RailRoad) {
        for i in 0..self.railroads.clone().len() {
            if self.railroads[i] == rr {
                let amount_to_return = rr.mortgage_potential as i32;
                self.railroads.remove(i);
                self.return_change(amount_to_return);
                break;
            }
        }
    }
    pub fn mortgage_utility(&mut self, utility: Utility) {
        if utility.name == "electrical company" && self.utilities.0 == 1 {
            self.return_change(utility.mortgage_potential);
            self.utilities.0 = 0;
        }
        else if utility.name == "water works" && self.utilities.1 == 1 {
            self.return_change(utility.mortgage_potential);
            self.utilities.1 = 0;
        }
        else {
            println!("The name of the utility you tried to mortgage isn't valid.");
            println!("The error has occurred in the implementation of 'Player' -> mortgage_utility()");
        }
    }
    pub fn buy_houses(&mut self, property_name: &str, mut num_to_add: i32, bank: &mut Bank) {
        let property = self.properties.keys().find(|p| p.name == property_name).cloned();

        if let Some(prop) = property {
            let current_houses = self.properties.get(&prop).unwrap().0;

            if bank.houses < num_to_add as u32 {
                println!("Bank only has {} houses left.", bank.houses);
                return;
            }
            if current_houses < 4 {
                let total_cost = (prop.price_of_extras as i32) * num_to_add;
                if self.compare_price(total_cost) {
                    self.general_pay(total_cost, None, Some(bank));
                    if let Some(count) = self.properties.get_mut(&prop) {
                        count.0 += num_to_add;
                    }

                    bank.houses -= num_to_add as u32;
                    println!("Successfully built {} houses on {}", num_to_add, prop.name);
                }
                else {
                    println!("Insufficient money to build {} houses", num_to_add);
                }
            }
            else {
                let total_cost = prop.price_of_extras as i32;
                if self.compare_price(total_cost) {
                    self.general_pay(total_cost, None, Some(bank));
                    if let Some(count) = self.properties.get_mut(&prop) {
                        count.1 += 1;
                        count.0 -= 4;
                    }

                    bank.hotels -= 1 as u32;
                    bank.houses += 4 as u32;
                    println!("Successfully built {} hotels on {}", 1, prop.name);
                    num_to_add -= 1;
                    if num_to_add > 0 {
                        self.buy_houses(property_name, num_to_add, bank);
                    }
                    else {
                        println!("Finished the transaction!");
                    }
                }
                else {
                    println!("Insufficient money to build {} houses", num_to_add);
                }
            }
        }
        else {
            println!("You do not own this property");
        }
    }
    pub fn go_to_jail(&mut self) {
        self.current_pos = 10;

    }
}

pub struct Bank {
    five_hundreds: i32,
    one_hundreds: i32,
    fifties: i32,
    twenties: i32,
    tens: i32,
    fives: i32,
    ones: i32,
    title_deeds: Vec<Property>,
    railroads: Vec<RailRoad>,
    houses: u32,
    hotels: u32,
}

impl Bank {
    pub fn new() -> Self {
        let title_deeds = Property::init();
        let railroads = RailRoad::init();

        Self {
            five_hundreds: 30,
            one_hundreds: 30,
            fifties: 30,
            twenties: 30,
            tens: 30,
            fives: 30,
            ones: 30,
            title_deeds,
            railroads,
            houses: 32,
            hotels: 12
        }
    }

    pub fn receive_funds(&mut self, mut amount: i32) {
        let denominations = [500, 100, 50, 20, 10, 5, 1];

        // This distributes the incoming total into the bank's bill slots
        self.five_hundreds += amount / 500;
        amount %= 500;
        self.one_hundreds += amount / 100;
        amount %= 100;
        self.fifties += amount / 50;
        amount %= 50;
        self.twenties += amount / 20;
        amount %= 20;
        self.tens += amount / 10;
        amount %= 10;
        self.fives += amount / 5;
        amount %= 5;
        self.ones += amount;
    }
}

#[derive(Clone)]
pub enum Square {
    Street(Property),
    Railroad(RailRoad),
    Utility { name: String, cost: u32 },
    Tax { name: String, amount: i32 },
    CommunityChest,
    Chance,
    Go,
    Jail,
    FreeParking,
    GoToJail,
}

pub enum Action {
    RollDice,
    BuyProperty,
    BuildHouse(Property),
    Mortgage(Property),
    EndTurn,
    UseGetOutOfJailCard,
}
pub struct Game {
    num_of_players: i32,
    players: Vec<Player>,
    rand_players_idx: usize,
    treasure_cards: TreasureCards,
    chance_cards: ChanceCards,
    board_sequence: Vec<Square>,
    bank: Bank,
}

impl Game {
    pub fn init(&mut self, num_of_players: i32) -> Self {
        // creating players
        let mut players: Vec<Player> = Vec::new();
        for i in 0..num_of_players + 1 {
            players.push(Player::new());
        }

        // creating the treasure cards
        let treasure_cards = TreasureCards::init(num_of_players);

        // creates chance cards
        let chance_cards = ChanceCards::init(num_of_players);

        // creates board sequence and the bank
        // format: (name of square, action to do)
        let bank = Bank::new();
        let mut td = bank.title_deeds.clone();
        let mut rr = RailRoad::init();

        let board_sequence: Vec<Square> = vec![
            Square::Go,
            Square::Street(td.remove(0)),
            Square::CommunityChest,
            Square::Street(td.remove(0)),
            Square::Tax { name: "income tax".to_string(), amount: 200 },
            Square::Railroad(rr.remove(0)),
            Square::Street(td.remove(0)),
            Square::Chance,
            Square::Street(td.remove(0)),
            Square::Street(td.remove(0)),
            Square::Jail,
            Square::Street(td.remove(0)),
            Square::Utility { name: "electrical company".to_string(), cost: 150 },
            Square::Street(td.remove(0)),
            Square::Street(td.remove(0)),
            Square::Railroad(rr.remove(0)),
            Square::Street(td.remove(0)),
            Square::CommunityChest,
            Square::Street(td.remove(0)),
            Square::Street(td.remove(0)),
            Square::FreeParking,
            Square::Street(td.remove(0)),
            Square::Chance,
            Square::Street(td.remove(0)),
            Square::Street(td.remove(0)),
            Square::Railroad(rr.remove(0)),
            Square::Street(td.remove(0)),
            Square::Street(td.remove(0)),
            Square::Utility { name: "water works".to_string(), cost: 150 },
            Square::Street(td.remove(0)),
            Square::GoToJail,
            Square::Street(td.remove(0)),
            Square::Street(td.remove(0)),
            Square::CommunityChest,
            Square::Street(td.remove(0)),
            Square::Railroad(rr.remove(0)),
            Square::Chance,
            Square::Street(td.remove(0)),
            Square::Tax { name: "super tax".to_string(), amount: 100 },
            Square::Street(td.remove(0)),
        ];

        let mut rng = rand::rng();
        let rand_players_idx = rng.random_range(1..players.clone().len());

        let new_game = Game {
            num_of_players,
            players,
            rand_players_idx,
            treasure_cards,
            chance_cards,
            board_sequence,
            bank,
        };

        new_game
    }

    pub fn step(&mut self, current_player_idx: usize) {
        /* 0 = no state
           1 = pre-roll
           2 = post roll
           3 = resolution
           4 = jail
        */
        let mut curr_player = self.players[current_player_idx].clone();

        // roll the dice
        let (die1, die2) = Player::roll();
        let total_roll = die1 + die2;

        // check if a double
        if die1 == die2 {
            curr_player.another_roll = (true, curr_player.another_roll.1 + 1);
        }

        curr_player.move_x_spaces(total_roll as i32);
    }
    pub fn handle_landing(&self, square: &Square, mut curr_player: Player) {
        // Street(crate::game_basics::Property),
        // Railroad(crate::game_basics::RailRoad),
        // Utility { name: String, cost: u32 },

        // Tax { name: String, amount: i32 },
        // CommunityChest,
        // Chance,
        // Go,
        // Jail,
        // FreeParking,
        // GoToJail,
        match square {
            Square::Street(prop) => {
                let (owning_player_option, owned, price) = prop.owned(self);
                let mut owning_player = owning_player_option.unwrap();
                if owned {
                    curr_player.general_pay(price, Some(&mut owning_player), None)
                }
                else {
                    // allow the bot to buy the property or do other general actions
                }
            }
            Square::Railroad(rr) => {
                // make this be able to buy, pay rent, and do the basic actions
            }

            _ => {

            }
        }
    }
}