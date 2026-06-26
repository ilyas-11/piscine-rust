pub fn first_subword(mut s: String) -> String {
    let mut index=s.len();
   for (i,c)in s.char_indices(){
    if i!=0&&(c=='_'||c.is_uppercase()){
        index=i;
        break;
    }

   }
   s.truncate(index);
   s
}
