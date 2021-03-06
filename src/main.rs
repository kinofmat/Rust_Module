use std::collections::HashMap; // To allow me to use the hashmap I created with the menu items.

/* This function will remove any newline characters or returns that are read in. 
In this the string is passed by reference. I created this as an example and initially
used both remove, and removed, but for convienence I eneded up just using one throughout.*/
fn remove(string: &mut String){
    if let Some('\n')=string.chars().next_back() {
        string.pop();
    }
    if let Some('\r')=string.chars().next_back() {
        string.pop();
    }
}

//Same function as remove but passed by value.
fn removed(mut string: String) -> String {
    if let Some('\n')=string.chars().next_back() {
        string.pop();
    }
    if let Some('\r')=string.chars().next_back() {
        string.pop();
    }
    string
 }

 /*This will set up to take input from the keyboard. 
 It will then remove any newline or return characters, by calling removed, and then return the string splice */
fn rdin() -> String{
    let mut reader = String::new();
    std::io::stdin().read_line(&mut reader).unwrap();
    reader = removed(reader.to_lowercase()); // Changes everything to lowercase so it is not a case sensetive program.
    println!();
    return reader;
}

/*Rounded takes floating point integers and rounds them to two decimal places.
With the way that Rust rounds, it first needs to be rounded to three decimal places, and then two,
in order to get an accurate rounding. */
fn rounded(mut rounder: f32) -> f32{
    rounder = format!("{:.3}", rounder).parse::<f32>().unwrap();
    rounder = format!("{:.2}", rounder).parse::<f32>().unwrap();
    return rounder;
}

/*This function was created for checking for correct input when integers were to be used. 
It is necessary before trying to convert the string to an integer.
This is implimented with the tips. */
fn strchecker(mut temp: String) -> String{
    while !temp.contains(&"1") && !temp.contains(&"2") && !temp.contains(&"3") && !temp.contains(&"4"){
        println!("It seems you entered an unrecognized value.\nYou entered, {}, please try again with either '1', '2', '3', or '4'.", temp);
        temp = rdin();
    }
    return temp;
}

/*intchecker will check the input as the actual expected ints. 
This is a necessary second layer,  since the strchecker will allow say 21, or 34 as inputs.
If the value is incorrect it will call for a new input, and the strchecker again.*/
fn intchecker(mut tip: i16) -> i16{
    let mut temp = String::new();
    while tip != 1 && tip !=2 && tip !=3 && tip !=4{
        println!("It seems you entered an unrecognized value.\nYou entered, {}, please try again with either '1', '2', '3' or '4'.", tip);
        temp = rdin();
        temp = strchecker(temp);
        tip = temp.parse::<i16>().unwrap();
    }
    return tip;
}

/*ynchecker will do everything necessary to check the for the correct input of either a y or a n.
It calls the rdin function to get input. Then it will check for empyt string so that there is no broken code.
Then it checks the chars and if it is not within the range of acceptable values, it will use recursion to do 
get a new value and run the checks again. This is done by Reference.*/
fn ynchecker(selector: &mut char){
    let mut temp = String::new();
    temp = rdin();
    //Simply error checks for incorrect values.
    if temp.is_empty(){ // Will check for an empty string.
        *selector = ' ';
        println!("You got an empty sting there");
    } else {
        *selector = temp.chars().nth(0).unwrap(); // Have to convert from a string, to a slice, to a char.
    }
    if *selector != 'y' && *selector != 'n'{
        println!("It seems you entered an unrecognized value.\nYou entered, {}, please try again with either 'y' or 'n'.", selector);
        ynchecker(selector);
    }
}

//main is necessary to run the code.
fn main() {
    //Constants declared for the tax rate, default tip rates, and special tip cases.
    const TAX: f32 = 0.06;
    const FIVE: f32 = 0.05;
    const TEN: f32 = 0.10;
    const FTN: f32 = 0.15;
    const TWN: f32 = 0.20;
    const QRT: f32 = 0.25;
    const HALF: f32 = 0.50;
    const DOLLAR: f32 = 1.00;
    //use mut to say the variable can be changed.
    let mut i: u8;
    let mut selector: char = 'y';
    let mut cost: f32 = 0.0;
    let mut taxes: f32;
    //Created a hashmap, it then is populated with the menu information
    let mut items = HashMap::new();
    items.insert(String::from("soda"), 1.95);
    items.insert(String::from("water"), 0.00);
    items.insert(String::from("burger"), 6.95);
    items.insert(String::from("pizza"), 2.95);
    items.insert(String::from("fries"), 1.95);
    items.insert(String::from("stake"), 9.95);
    //Creates a vector. It is necessary to specify the data type that will be in the vector.
    let mut itemPrice: Vec<f32> = Vec::new();
    let mut total: f32;
    let mut fivet: f32;
    let mut tent: f32;
    let mut ftnt: f32;
    let mut twnt: f32;
    //Cannot initialize a string with values already in it.
    let mut temp = String::new();
    let mut tip: i16;

    println!("Welcome to the restaurant of Rusty Lake!"); //Do you get the reference here? xD

    //Loops through the entire body of the code to allow multiple iterations of orders.
    while selector != 'n'{
        //Needs to be cleared from any past iterations.
        cost = 0.0;
        i = 0;

        //Specifically for clearing the vector, instead of wasting memory creating a new one each time.
        //Will iterate through the length of the vector using .rev, which is basically just a backwards iterator.
        for i in (0..itemPrice.len()).rev(){
            itemPrice.remove(i);
        }

        //Will loop through for each item being selected from the menu.
        while selector != 'n'{
            println!("What item from the menu would you like to order?");
            //Prints out the entire HashMap
            for (key, value) in &items {
                println!("{}: {:.2}", key, value);
            }
            temp = rdin();

            //If the input does not match with a key we need to get the correct value.
            while !items.contains_key(&temp){
                println!("It seems what you entered did not quite match one of the items from the menu.\nPlease try again.");
                for (key, value) in &items {
                    println!("{}: {:.2}", key, value);
                }
                temp = rdin();
            }
            //Checks that the input really is a key.
            if items.contains_key(&temp){
                /*A little bit of a different descision structure here.
                The match will compare the given statement to the pattern of the other types.
                In a way this reminds me of the when statement from Kotlin.*/
                match items.get(&temp){
                    Some(price) => {
                        itemPrice.push(*price);
                        println!("Item price, ${:.2}", price);
                    }
                    None => {
                        println!("Error! Something went wrong!");
                    }
                }
            }
            println!("Is there another item from the menu you wish to order? (y/n)");
            ynchecker(&mut selector);

            i += 1;
        }

        //Will add each item from the vector to the cost.
        for order in itemPrice.iter(){
            //println!("The current item is priced ${}", order);
            cost += order;
        }
        //Calculate the costs with tax and various tips.
        taxes = cost * TAX;
        taxes = rounded(taxes);
        total = taxes + cost;
        println!("Your taxes will be: ${0:.2}\nYour total with taxes will be ${1:.2}\n", taxes, total);
        fivet = cost * FIVE;
        tent = cost * TEN;
        ftnt = cost * FTN;
        twnt = cost * TWN;
        fivet = rounded(fivet);
        tent = rounded(tent);
        ftnt = rounded(ftnt);
        twnt = rounded(twnt);

        /*First check for if they ordered water, when it would brake the normal code for calculating the tips.
        If there is a large group of people, considering someone may order 2 items on average, then raise the default tip rate.*/
        if total == 0.0{
            println!("Please consider being generous today and leave a tip for your waiter.\nSelect one of the following:\n1) $0.25      2) $0.50\n3) $1.00      4) Other");
        }
        else if i < 10{
            println!("What would you like your tip to be?\nSelect one of the following:\n1) 5%:  ${0:.2} {3:<10}2) 10%: ${1:.2}\n3) 15%: ${2:.2}{3:<10} 4) Other", fivet, tent, ftnt, "");
        } else {
            println!("What would you like your tip to be?\nSelect one of the following:\n1) 10%: ${0:.2}{3:<10} 2) 15%: ${1:.2}\n3) 20%: ${2:.2}{3:<10}4) Other", tent, ftnt, twnt, "");
        }
        temp = rdin();
        temp = strchecker(temp); // Use the string checker first to make sure there aren't actually and letters read in.
        tip = temp.parse::<i16>().unwrap(); // After we have check that there are only integers, we can convert the data type to an int.
        tip = intchecker(tip); // Then we have to actually check the values for correct integers.

        // First check for the special only water condition. Then go along with normal tips.
        if total == 0.0{
            if tip == 1{
                total += QRT;
            } else if tip == 2{
                total += HALF;
            } else if tip == 3{
                total += DOLLAR;
            } else if tip == 4{
                println!("Please enter a specific amount, including the change. Ex '10.00':");
                total += rdin().parse::<f32>().unwrap(); //Will convert the string to a floating point number. Will break if letters are read in.
            } else{
                println!("It appears you got through all my checks. In other words, you broke the code! Way to go!");
            }
        } else {
            if tip == 1{
                total += fivet;
            } else if tip == 2{
                total += tent;
            } else if tip == 3{
                total += ftnt;
            } else if tip == 4{
                println!("Please enter a specific amount, including the change. Ex '10.00':");
                total += rdin().parse::<f32>().unwrap();
            } else{ // Just a random extra else. I found no situations that would enact this code, but fun for just in case.
                println!("It appears you got through all my checks. In other words, you broke the code! Way to go!");
            }
        }
        println!("Your total will be: ${:.2}", total); // The :.2 that I used in a lot of these print statements is to enforce the formatting with two decimal places.
        
        println!("Is there another order you wish to enter? (y/n)");
        ynchecker(&mut selector); // One final error check.
    }
}
