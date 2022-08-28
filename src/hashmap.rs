
pub struct IHashMap<T> {
    lambda: usize,
    hash_table: Vec<usize>,
    value: Vec<T>,
}

impl<T> IHashMap<T>
where
    T: Default + Clone,
{
    pub fn new() -> IHashMap<T> {
        let mut hash_table = Vec::<usize>::with_capacity(11);
        let mut value = Vec::<T>::with_capacity(11);
        for _ in 0..11 {
            hash_table.push(0);
            value.push(Default::default());
        }
        return IHashMap {
            lambda: 11,
            hash_table,
            value,
        };
    }
    //计算哈希值
    #[inline]
    fn hash(&self, hash_name: &str) -> usize {
        if hash_name.is_empty() {
            return 0;
        }
        let mut hash_sum = 0;

        for va in hash_name.chars().into_iter() {
            hash_sum += va as usize;
        }
        return hash_sum;
    }
    pub fn insert(&mut self,hash_name: &str, mut value: T){
        let hash_value=self.hash(hash_name);
        

       while let Err(q)=self.hash_insert(hash_value, value){
            value=q;
       }
    }
    fn hash_insert(&mut self, hash_value: usize, value: T)->Result<(),T> {

        let mut index = hash_value % self.lambda;
        while index < self.lambda {
            if self.hash_table[index] == 0 {
                //如果在那个位置的哈希表值为空
                self.hash_table[index] = hash_value;
                self.value[index] = value;
                return Ok(());
            } else if self.hash_table[index] == hash_value {
                self.value[index] = value;
                return Ok(());
            } else {
                //往后移动
                index += 3;
            }
        }
        return Err(value);
    }
    fn rehash_table(&mut self){
        //素数计算
        let mut n=self.lambda;
   //往后跳至少比自己大一倍的素数
        self.lambda='first:loop{
            n=n+1;
            if n%6!=1||n%6!=5{
                continue 'first;
            }
            let sqrt_n=(n as f32).sqrt() as usize;
            let k=5;
            while k<=sqrt_n{
                if n%k==0||n%(k+2)==0 {
                    continue 'first;
                }
            }
            if n>self.lambda*2{
                break n;
            }
        };

        let mut table_array:Vec<usize>=Vec::with_capacity(self.hash_table.len());
        let mut table_value:Vec<T>=Vec::with_capacity(self.value.len());
        while !self.hash_table.is_empty(){
            let e=self.hash_table.pop().unwrap();
            let r=self.value.pop().unwrap();
            if e!=0{
                table_array.push(e);
                table_value.push(r);
            }
        }
        self.hash_table.resize(self.lambda,0);
        self.value.resize(self.lambda,Default::default());
        let mut is_ok:bool=true;
        while !table_array.is_empty(){
            let e=table_array.pop().unwrap();
            let q=table_value.pop().unwrap();
           if let Err(q)=self.hash_insert(e,q){
            is_ok=false;
            self.hash_table.push(e);
            self.value.push(q);
            for next in table_array.into_iter(){
                self.hash_table.push(next);
            }
            for next in table_value.into_iter(){
                self.value.push(next);
            }
            break;
           }
        }

        if is_ok{ 
            return;
        }
        return self.rehash_table();


    }
    #[inline]
    pub fn get(&self,key:&str)->Option<&T>{
        //计算哈希值
        let hash_value=self.hash(key); 
        let mut index=hash_value%self.lambda;
        while index<self.hash_table.len(){
            if hash_value==self.hash_table[index]{
                return Some(&self.value[index]);
            }else {
                index+=3;
            }
        };
        return None;
    }
}
