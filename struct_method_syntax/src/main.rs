#[derive(Debug)]
struct Post {
    title: String,
    slug: String,
    body: String,
    category_data: Category
}

#[derive(Debug)]
struct Category {
    id: u32,
    category_name: String
}

impl Post {
    fn new() -> Post {
        Post {
            title: String::default(),
            slug: String::default(),
            body: String::default(),
            category_data: Category {
                id: 0,
                category_name: String::default()
            }
        }
    }

    fn with_title(mut self, title: String) -> Post {
        
        self.title = title;

        self
    }

    fn with_slug(mut self, slug: String) -> Post {
        
        self.slug = slug;

        self
    }

    fn with_body(mut self, body: String) -> Post {
        
        self.body = body;

        self
    }

    fn with_category(mut self, category: Category) -> Post {
        
        self.category_data = category;

        self
    }

    fn publish_post(self) -> Post {

        println!("{} is published in {}", &self.title, &self.category_data.category_name);

        self
    }
}

fn main() {
    
    let title = String::from("Hello World");
    let slug = String::from("hello-world");
    let body = String::from("Lorem ipsum dolor sit amet!");

    let category = Category {
        id: 5,
        category_name: String::from("General")
    };

    let post = Post::new()
                    .with_title(title)
                    .with_slug(slug)
                    .with_body(body)
                    .with_category(category)
                    .publish_post();

    println!("Post details {:?}", post);
}
