pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub fn greeting_from_lib() -> String {
    let message = String::from("Hello from lib");
    message
    }

#[macro_export] //in-built Rust
macro_rules! rectangles {
    ($($rect_props_tuple:expr),*) => {
        //I want to return a Vector of Rectangles
        {
            let mut rect_vec = Vec::new();
            //take our expression recieved, get the length, width and name 
            //from each and create the appropriate rectangle and push each
            //to our rect_vec

            $(let(length, width, name) = $rect_props_tuple;
            let rect = Rect{length: length as f32, width: width as f32, name: as &str};
            rect_vec.push(rect);)*
            rect_vec
        }
        
    };
}

//Try our rectangles! declarative macro.
pub fn run9(){
    let rects = rectangles!((1,1,"rect1"), (3.5,7.0,"rect2"));
    println!(
        "Area of rectangle 1 = {}; area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area()
    )
}

#[macro_export]
macro_rules! rectangles_with_default {
    (($($rect_props_tuple:expr), *), $default_tuple:expr) => {

        let mut rect_vec = Vec::new();
        (default_length, default_width, default_name) = $default_tuple;

        let (length,width, name)
        
    };
}











    



