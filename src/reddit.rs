use crate::model::PostsResponse;

use reqwest::blocking::Client;

#[derive(Clone, Debug)]
pub struct UserPages<'client> {
    client: &'client Client,
    user: String,
    last_id: Option<String>,
}

impl<'client> UserPages<'client> {
    pub fn new(client: &'client Client, user: impl Into<String>) -> Self {
        Self {
            client,
            user: user.into(),
            last_id: None,
        }
    }

    pub fn visit_pages(&mut self, mut f: impl FnMut(&str)) -> reqwest::Result<()> {
        loop {
            let page = self.get_page()?;
            if page.posts.is_empty() {
                return Ok(());
            }
            page.media().for_each(|x| f(x.as_ref()));
        }
    }

    fn get_page(&mut self) -> reqwest::Result<PostsResponse> {
        let response: PostsResponse = self.client.get(&self.get_request()).send()?.json()?;
        self.last_id = response.last_id().map(Clone::clone);
        Ok(response)
    }

    fn get_request(&self) -> String {
        match self.last_id.as_ref() {
            Some(after) => format!(
                "https://gateway.reddit.com/desktopapi/v1/user/{}/posts?rtj=only&allow_quarantined=true&allow_over18=1&include=identity&after={}&dist=25&sort=new&t=all",
                self.user,
                after,
            ),
            None => format!(
                "https://gateway.reddit.com/desktopapi/v1/user/{}/posts?rtj=only&allow_quarantined=true&allow_over18=1&include=identity&dist=25&sort=new&t=all",
                self.user,
            ),
        }
    }
}
