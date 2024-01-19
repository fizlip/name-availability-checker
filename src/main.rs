
fn website_exists(url: &str) -> bool{
    let body = reqwest::blocking::get(url);
    let res:bool = match body {
        Ok(b) => true,
        Err(_) => false,
    };

    res
}

fn username_exists(url:&str) -> bool{
    let body = reqwest::blocking::get(url);
    let res:bool = match body {
        Ok(b) => {println!("{}", b.text().unwrap()); return true},
        Err(_) => false,
    };

    res
}

fn insta_exists(username: &str) -> bool{
    let url = format!("https://www.instagram.com/{}", username);
    username_exists(&url)
}

fn youtube_profile_exists(username: &str) -> bool {
    let url = format!("https://www.youtube.com/{}", username);
    username_exists(&url)

}

fn tiktok_profile_exists(username: &str) -> bool {
    let url = format!("https://www.tiktok.com/{}", username);
    username_exists(&url)
}

fn main() {
    let website_exists = insta_exists("kimkardashian");
    println!("Website exists {}", website_exists);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn website_does_not_exist() {
        let non_existent_website:&str = "https://some_random_website.com";
        let result:bool = website_exists(non_existent_website);

        assert_eq!(result, false);
    }

    #[test]
    fn website_should_exists() {
        let existent_website:&str = "https://google.com";
        let result:bool = website_exists(existent_website);

        assert_eq!(result, true);
    }

    #[test]
    fn insta_should_exists() {
        let existent_profile:&str = "kimkardashian";
        let result:bool = insta_exists(existent_profile);

        assert_eq!(result, true);
    }
}
