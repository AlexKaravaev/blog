use leptos::*;


use std::collections::HashMap;

// https://iamkate.com/code/tree-views/
#[component]
pub fn FunList() -> impl IntoView {
    type Tree = HashMap<String, Node>;

    enum Node {
        Leaf(Vec<String>), 
        LeafLink(Vec<(String, String)>), // href, visible test
        Branch(Tree), 
    }

    let mut tree: Tree = HashMap::new();

    let zazen = Node::LeafLink(vec![
      ("http://shinzenyoung.blogspot.com/2012/02/trigger-practice.html".to_string(), "Trigge practice, but a lot of stuff from Shinzen is good.".to_string()),
      ("https://antaiji.org/en/".to_string(), "Japanese zazen retreat".to_string()),
      ]
    );
    let enlightenment = Node::LeafLink(vec![
      ("https://knowingless.com/2020/07/15/the-enlightenment-interviews/".to_string(), "Enlightenment interviews".to_string())
    ]);
    let rust_books = Node::LeafLink(vec![
        ("https://www.manning.com/books/rust-in-action".to_string(), "Rust in Action".to_string()),
        ("https://github.com/rust-lang/rustlings".to_string(), "Rustlings Exercises".to_string()),
        ("https://doc.rust-lang.org/book/".to_string(), "Official Rust Book".to_string())
    ]);
    
    let favorite_books = Node::LeafLink(vec![
        ("https://www.goodreads.com/review/list/171196858?shelf=%23ALL%23".to_string(), "My favourite books".to_string()),
    ]);
    
    let investing = Node::LeafLink(vec![
        ("https://stonks.com/best-of-stonks-fund".to_string(), "Stonks fund".to_string()),
        ("https://hedgefollow.com/top-hedge-funds.php".to_string(), "Top hedge funds to follow".to_string()),
    ]);
    
    let sports = Node::LeafLink(vec![
        ("https://www.youtube.com/watch?v=2ME8gEN54Ao&list=PLD7BE939BB32B4CAB".to_string(), "Squat".to_string()),
        ("https://www.youtube.com/watch?v=xS2wLZSdeDg&list=PLbFxzzq99IUD1bxzKq7UUBI1TGjNSKUWB".to_string(), "Deadlift".to_string()),
        ("https://www.youtube.com/watch?v=EHx1gYTA-Rw&list=PL38D8B0C3BFDB82F2".to_string(), "Bench".to_string())
    ]);
    
    let life = Node::LeafLink(vec![
        ("https://www.youtube.com/watch?v=54l8_ewcOlY".to_string(), "Awakening from the Meaning Crisis".to_string()),
        ("https://blog.samaltman.com/how-to-be-successful".to_string(), "How to Be Successful".to_string())
    ]);
    
    let software_tools = Node::LeafLink(vec![
        ("https://fig.io/".to_string(), "Fig".to_string()),
        ("https://buildkite.com/home".to_string(), "Buildkite".to_string()),
        ("https://github.com/cantino/mcfly".to_string(), "McFly".to_string()),
        ("https://rr-project.org/".to_string(), "rr-project".to_string()),
        ("https://github.com/tigerbeetle/tigerbeetle/blob/fe09404d465df46b2bdfc017633eff37b4ab2343/docs/TIGER_STYLE.md".to_string(), "Tiger Style".to_string())
    ]);
    
    let nix = Node::LeafLink(vec![
        ("https://news.ycombinator.com/item?id=39720007".to_string(), "Nix docker builder".to_string()),
        ("https://www.tweag.io/blog/2022-07-14-taming-unix-with-nix/".to_string(), "Taming Unix with Nix".to_string()),
        ("https://garnix.io/".to_string(), "Garnix".to_string())
    ]);
    
    let mut categories = Tree::new();
    categories.insert("Rust Books".to_string(), rust_books);
    categories.insert("Favorite Books".to_string(), favorite_books);
    categories.insert("Investing".to_string(), investing);
    categories.insert("Sports".to_string(), sports);
    categories.insert("Life".to_string(), life);
    categories.insert("Software Tools".to_string(), software_tools);
    categories.insert("Nix".to_string(), nix);

    let mut meditation = Tree::new();
    meditation.insert("Zazen".to_string(), zazen);
    meditation.insert("Enlightenment".to_string(), enlightenment);
    
    categories.insert("Meditation".to_string(), Node::Branch(meditation));    
    tree.insert("Resources".to_string(), Node::Branch(categories));

    fn render_tree(tree: &Tree) -> impl IntoView {
        view! {

          <div class="flex flex-row mt-4.5 justify-center items-center">
            <ul class="tree">
                {tree.iter().map(|(key, node)| {
                    view! {
                        <li>
                            <details open>
                                <summary>{key}</summary>
                                {
                                    match node {
                                        Node::Leaf(items) => view! {
                                            <ul>
                                                {items.iter().map(|item| view! {
                                                    <li>{item}</li>
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        }.into_view(),
                                        Node::LeafLink(items) => view! {
                                          <ul>
                                              {items.iter().map(|link| view! {
                                                  <li><a  class="under text-sm font-semibold leading-6 text-gray-100" href={&link.0}>{&link.1}</a></li>
                                              }).collect::<Vec<_>>()}
                                          </ul>
                                      }.into_view(),
                                        Node::Branch(subtree) => render_tree(subtree).into_view(),
                                    }
                                }
                            </details>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
          </div>
        }
    }

    render_tree(&tree)
}



#[component]
pub fn Resources() -> impl IntoView {
  view!{
   
    <div class="w-full max-w-6xl mx-auto px-4 md:px-6 py-4">
      <p class="mt-0.5 text-gray-100">
        I love the concept of endless lists of lists. This abstraction can encapsulate almost everything in the world.
        Personally, I use <a class="under" href="https://tana.inc" target="_blank">"📝 Tana"</a>, previously used Workflowy. It works for me, maybe someday I will switch to Obsidian, but it does its job and I do not want to spend a lot of time tweaking it.
        One thing I love about it is the Capture app, where you can really quickly dump your thoughts.
      </p>
      <p class="mt-0.5 text-gray-100">
        Below is a random list of books, videos, articles, and more on various themes that I really like and wanted to share with you.
      </p>


    </div>
    
    <FunList/>
  }
}
