// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    sparkling,
    sweet,
    fruity
    }

struct Drink {
    flavor: Flavor,
    fluid_oz: f64, 
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::sparkling => println!("flavor: sparkling"),
        Flavor::sweet => println!("flovor: sweet"),
        Flavor::fruity => println!("flavor: fruity"),

    }

    println!("oz: {:?}", drink.fluid_oz);
}
fn main() {
    let sweet = Drink{
        flavor: Flavor::sweet,
        fluid_oz: 6.0
    };
    print_drink(sweet);
    let fruity = Drink {
        flavor: Flavor::fruity,
        fluid_oz: 10.0
    };
    print_drink(fruity);

}


