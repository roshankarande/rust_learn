async fn double_it(n: u32) -> u32 {
    2*n
}


#[cfg(test)]
mod tests {
    use tokio::runtime::Builder;

    use super::*;

    // hardway
    #[test]
    fn simple_test() {

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        assert_eq!(rt.block_on(double_it(2)), 4);
    }


    // easy way
    #[tokio::test]
    async fn tokio_test(){
        assert_eq!(double_it(2).await, 4);
    }
}