use option_chain_tool::opt;

mod option_ext;
#[derive(Debug, Clone)]
struct User {
    profile: Option<Profile>,
}

#[derive(Debug, Clone)]
struct Profile {
    address: Option<Address>,
}

#[derive(Debug, Clone)]
struct Address {
    city: Option<String>,
    street: String,
    some_field: Result<String, String>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .init();

    let user = User {
        profile: Some(Profile {
            address: Some(Address {
                city: Some("New York".to_string()),
                street: "5th Avenue".to_string(),
                some_field: Ok("Some value".to_string()),
            }),
        }),
    };

    let a = opt!(user.profile?.address?.city?);
    let a = if let Some(____v) = user.profile.as_ref() {
        if let Some(____v) = ____v.address.as_ref() {
            ____v.city.as_ref()
        } else {
            None
        }
    } else {
        None
    };

    let a = opt!(user.profile?.address?.street);
    let b = if let Some(____v) = user.profile.as_ref() {
        if let Some(____v) = ____v.address.as_ref() {
            Some(&____v.street)
        } else {
            None
        }
    } else {
        None
    };

    let a = opt!(user.profile?.address?.some_field?Err);
    let c = if let Some(____v) = &user.profile {
        if let Some(____v) = &____v.address {
            Some(____v)
        } else {
            None
        }
    } else {
        None
    };

    let c = if let Some(____v) = user.profile.as_ref() {
        if let Some(____v) = ____v.address.as_ref() {
            ____v.some_field.as_ref().ok()
        } else {
            None
        }
    } else {
        None
    };

    println!("City: {:?}", c);
    // println!("Street: {:?}", b);
    println!("Some Field: {:?}", c);
}
