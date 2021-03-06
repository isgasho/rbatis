#[cfg(test)]
mod test {
    use rbatis::utils::string_util::{find_convert_string, un_packing_string};
    #[test]
    fn test_find() {
        let sql = "update user set name=#{name}, password=#{password} ,sex=#{sex}, phone=#{phone}, delete_flag=#{flag}, #{name}";
        let finds = find_convert_string(sql);
        assert_eq!(finds.len(), 5);
        let mut index = 0;
        for (k, _) in &finds {
            if index == 0 {
                assert_eq!(k, "name");
            }
            if index == 1 {
                assert_eq!(k, "password");
            }
            if index == 2 {
                assert_eq!(k, "sex");
            }
            if index == 3 {
                assert_eq!(k, "phone");
            }
            if index == 4 {
                assert_eq!(k, "flag");
            }
            index += 1;
        }
        println!("{:?}", finds);
    }

    #[test]
    fn test_find_fail() {
        let sql = "select #{column   #{  }";
        let finds = find_convert_string(sql);
        println!("{:?}", finds);
    }

    #[test]
    fn test_un_pack_string() {
        assert_eq!(un_packing_string(""), "");
        assert_eq!(un_packing_string("''"), "");
        assert_eq!(un_packing_string("``"), "");
        assert_eq!(un_packing_string("\"\""), "");
        assert_eq!(un_packing_string("`a`"), "a");
        assert_eq!(un_packing_string("\""), "\"");
    }
}
