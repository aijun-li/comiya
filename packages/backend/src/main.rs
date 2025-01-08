use backend::{Manhuagui, Site};

async fn test<T: Site>(site: T) {
    match site.get_comic_brief("7580".to_string()).await {
        Ok(brief) => {
            println!("{:?}", brief);
        }
        _ => return,
    }
}

#[tokio::main]
async fn main() {
    test(Manhuagui).await;
}
