use mini_redis::{client, Result};

#[tokio::main]

async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    //read args 

    let args: Vec<String> = std::env::args().collect();
   // dbg!(&args);
    let command = &args[1];
    let key = &args[2];
    match  command.as_str()  {
        "set" => {

            let key_val = args[3].clone();
            println!("command set executed, sended key = {}, key_val = {}", &key, &key_val);
            let mut client = client::connect("127.0.0.1:6379").await.unwrap();
            client.set(&key, key_val.into()).await.unwrap();

        },
        "get" => {

            let mut client = client::connect("127.0.0.1:6379").await.unwrap();
            let res = client.get(&key).await.unwrap();
            // match Option::from(res) {
            //     Some(result) => {
            //         println!("{:#?}", result);
            //     }
            //     None {
            //         println!("key not found");
            //     }
            // }
            println!("res = {:#?}", res);
        }
       &_ => { println!("not command executed"); }
    } 
    

    /*let mut handle = Vec::new();
    handle.push(tokio::spawn(async {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        client.set("hello", "world".into()).await.unwrap();
    }));

    handle.push(tokio::spawn(async {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        client.set("hello", "world".into()).await.unwrap();
    }));

    for h in handle {
        h.await.unwrap();
    }*/

    Ok(())
}
