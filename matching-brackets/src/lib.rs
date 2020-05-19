pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack:Vec<char> = vec![];
    let ch = string.chars();
    for c in ch {
        let remove = match stack.last() {
            Some(x) => c.is_closing(*x),
            None => false
        };
        if remove {
            stack.pop();
        } else if c.is_bracket(){
            stack.push(c);
        }
    }

    stack.is_empty()
}
trait Helper {
    fn is_bracket(&self)->bool;
    fn is_closing(&self,c:char) -> bool;
}
impl Helper for char {
    fn is_bracket(&self) -> bool {
        match *self {
            '(' | '{' | '[' | ')' | '}' | ']' => true,
            _ => false
        }
    }

    fn is_closing(&self, prev: char) -> bool {
        match *self {
            '}' if prev == '{' => true,
            ']' if prev == '[' => true,
            ')' if prev == '(' => true,
            _ => false
        }
    }
}
