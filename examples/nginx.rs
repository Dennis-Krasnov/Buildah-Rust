use buildah::container::Container;
use std::error;

fn main() {
    let mut container = Container::from("nginx:1.21");
    container.copy("html", "/usr/share/nginx/html").unwrap();
    container.commit("nginx_rust").unwrap();
}
