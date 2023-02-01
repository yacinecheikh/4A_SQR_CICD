/*
this file was inspired from the Rocket hello world exemple
*/

#[macro_use] extern crate rocket;
extern crate core;


use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::ptr::hash;
use std::sync::Mutex;
use std::time::SystemTime;
use rocket::form::Form;
use rocket::response::status::BadRequest;


struct Transaction {
    date: SystemTime,
    receiver: String,
    sender: String,
    amount: i64,
    hash: u64,
}

#[derive(Hash)]
// 'a is a lifetime parameter
// HashedTransaction is dependant on Transaction
struct HashedTransaction<'a> {
    receiver: &'a String,
    sender: &'a String,
    amount: i64,
}


// the lock ensures this mutable global is thread-safe
static TRANSACTIONS: Mutex<Vec<Transaction>> = Mutex::new(vec![]);


// helper function
fn format_transaction(t: &Transaction) -> String {
    format!("{}: {} -> {} ({})",
            // unwrap() is for Result<Duration, SystemTimeError>
            t.date.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string(),
            t.sender,
            t.receiver,
            t.amount)
}

fn transaction_hash(t: &Transaction) -> u64 {
    let mut hasher = DefaultHasher::new();
    let hashed = HashedTransaction{
        receiver: &t.receiver,
        sender: &t.sender,
        amount: t.amount,
    };
    hash(&hashed, &mut hasher);
    // alternative: calling hash() on each desired attribute
    // hash(&t.receiver, &mut hasher); ...
    hasher.finish()
}



// E1: enregistrer une transaction

#[derive(Debug, FromForm)]
struct InsertForm<'v> {
    receiver: &'v str,
    sender: &'v str,
    amount: &'v str,
}

#[post("/transaction", data="<formdata>")]
fn insert(formdata: Form<InsertForm>) -> Result<(), BadRequest<String>> {
    let mut transaction = Transaction {
        date: SystemTime::now(),
        receiver: formdata.receiver.to_string(),
        sender: formdata.sender.to_string(),
        amount: formdata.amount.parse().unwrap(),
        hash: 0, // uninitialized
    };
    transaction.hash = transaction_hash(&transaction);
    TRANSACTIONS.lock().unwrap().push(transaction);

    Ok(())
}



// E2: afficher la liste de toutes les transactions dans l'ordre chronologique
// les transactions sont déjà écrites dans l'ordre chronologique

#[get("/list")]
fn history() -> String {
    let mut result = String::new();
    for transaction in TRANSACTIONS.lock().unwrap().iter() {
        let line = format_transaction(transaction) + "\n";
        // line[..] is the slice (type str)
        result.push_str(&line[..]);
    }
    result
}


// E3: Afficher une liste des transactions dans l’ordre chronologique liées à une personne

#[get("/list/<user>")]
fn user_history(user: &str) -> String {
    let mut result = String::new();

    for transaction in TRANSACTIONS.lock().unwrap().iter() {
        if transaction.sender == user || transaction.receiver == user {
            let line = format_transaction(transaction) + "\n";
            result.push_str(&line[..]);
        }
    }
    result
}

// E4: Afficher le solde du compte de la personne

#[get("/balance/<user>")]

fn user_balance(user: &str) -> String {
    let mut total = 0;
    for transaction in TRANSACTIONS.lock().unwrap().iter() {
        if transaction.sender == user {
            total -= transaction.amount
        }
        if transaction.receiver == user {
            total += transaction.amount
        }
    }
    total.to_string()
}


// E5: Importer des données depuis un fichier csv. (à documenter)
// pas encore fait

// E6: vérification de l'intégrité des hash enregistrés
#[get("/integrity_check")]
fn check_integrity() -> String {
    let mut result = "ok".to_string();
    for transaction in TRANSACTIONS.lock().unwrap().iter() {
        if transaction_hash(transaction) != transaction.hash {
            result = "fail".to_string()
        }
    }
    result
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/transactions/", routes![
            history, user_history,
            insert, user_balance,
            check_integrity,
        ])
}