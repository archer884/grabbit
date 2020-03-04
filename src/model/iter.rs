use super::{Post, PostsResponse};
use hashbrown::HashMap;
use std::{slice, vec};

#[derive(Clone, Debug)]
pub struct PostsIter<'response> {
    response: &'response PostsResponse,
    post_ids: slice::Iter<'response, String>,
}

impl<'response> Iterator for PostsIter<'response> {
    type Item = &'response Post;

    fn next(&mut self) -> Option<Self::Item> {
        self.post_ids
            .next()
            .and_then(|id| self.response.posts.get(id))
    }
}

impl<'response> IntoIterator for &'response PostsResponse {
    type Item = &'response Post;
    type IntoIter = PostsIter<'response>;

    fn into_iter(self) -> Self::IntoIter {
        PostsIter {
            post_ids: self.post_ids.iter(),
            response: self,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PostsIntoIter {
    posts: HashMap<String, Post>,
    post_ids: vec::IntoIter<String>,
}

impl Iterator for PostsIntoIter {
    type Item = Post;

    fn next(&mut self) -> Option<Self::Item> {
        self.post_ids.next().and_then(|id| self.posts.remove(&id))
    }
}
