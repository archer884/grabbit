use crate::model::{Post, PostsResponse};
use reqwest::Client;

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

    pub async fn visit_pages(&mut self, mut f: impl FnMut(Post)) -> reqwest::Result<()> {
        loop {
            let mut page = self.get_page().await?;
            if let Some(item) = page.next() {
                f(item);
                page.for_each(&mut f);
            } else {
                return Ok(());
            }
        }
    }

    async fn get_page(&mut self) -> reqwest::Result<impl Iterator<Item = Post>> {
        let response: PostsResponse = self
            .client
            .get(&self.get_request())
            .send()
            .await?
            .json()
            .await?;

        self.last_id = response.last_id().map(Clone::clone);
        Ok(response.posts())
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

// FIXME:
// This use case might seem like a natural fit for the concept of a stream, but in actuality it
// may be impossible to represent this as a stream. For one thing, streams support out-of-order
// or concurrent execution, which Reddit's paging model does NOT allow, as far as I can tell.

// Anyway, I have no idea how to make a stream.

// pub struct UserPagesStream<'pages, 'client> {
//     next: Option<Box<dyn Future<Output = reqwest::Result<Vec<Post>>>>>,
//     pages: &'pages UserPages<'client>,
// }

// impl<'a, 'b> Stream for UserPagesStream<'a, 'b> {
//     type Item = reqwest::Result<Vec<Post>>;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
//         unimplemented!()
//     }
// }
