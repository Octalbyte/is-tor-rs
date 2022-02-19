#[cfg(test)]
pub mod tests {

    use crate::istor;
    use crate::req::find_nodes;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn nodes() {
        find_nodes();
    }

    #[test]
    fn test_all() {
        assert_eq!(true, istor::istor("176.10.99.200", false)); //in the list
        assert_eq!(false, istor::istor("176.10.99.300", false)); //not in the list
        assert_eq!(false, istor::istor("176.10.99.200\n", false)); //in the list, but is not an ip
        assert_eq!(true, istor::istor("95.143.193.125", false)); //in the list
    }

    #[test]
    fn test_connect() {
        assert_eq!(true, istor::istor("176.10.99.200", false)); //in the list
        assert_eq!(false, istor::istor("176.10.99.300", false)); //not in the list
        assert_eq!(false, istor::istor("176.10.99.200\n", false)); //in the list, but is not an ip
        assert_eq!(true, istor::istor("95.143.193.125", false)); //in the list
    }

    #[test]
    fn can_get_nodes() {
        istor::get_nodes();
    }

    #[test]
    fn can_connect() {
        istor::get_nodes_real_time();
    }
}
