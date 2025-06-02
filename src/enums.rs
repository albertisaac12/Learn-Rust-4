#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
struct mcbc {
    tp: CardSuit,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String, u64, IsActive),
    DebitCard,
    PayPal { active: bool, until: u64 },
}
#[derive(Debug)]
enum IsActive {
    Active,
    Deactivated(Since),
}

#[derive(Debug)]
struct Since {
    year: u64,
    reason: String,
}

// ententies inside the enum are called variants

enum OperatingSystem {
    Windows,
    MacOs(u64),
    Linux { version: u64 },
}
fn main() {
    let mc = CardSuit::Clubs;

    let mc = mcbc {
        tp: CardSuit::Clubs,
    };

    let visa = PaymentMethodType::CreditCard("hsbc".to_string(), 0201, IsActive::Active);
    let masterCard = PaymentMethodType::CreditCard(
        "blah".to_string(),
        0211,
        IsActive::Deactivated(Since {
            year: 2025,
            reason: String::from("Expired"),
        }),
    );

    let pp = PaymentMethodType::PayPal {
        active: true,
        until: 20_26,
    };

    let os = OperatingSystem::Linux { version: 1 };
    println!("My os is {} old", years_since_release(os));
}

// Enum Memory allocation happens on the largest memory size
// pointer is has 24 bytes

fn years_since_release(os: OperatingSystem) -> u64 {
    match os {
        OperatingSystem::Linux { version } => version,
        OperatingSystem::MacOs(val) => val,
        OperatingSystem::Windows => 3,
    }
}
