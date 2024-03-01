use std::path::PathBuf;

pub mod ast;

pub fn federate_names(lf_file: &str) -> Vec<String> {
    let model = ast::parse(lf_file).model;
    model
        .reactors
        .iter()
        .filter(|r| r.is_federated)
        .flat_map(|r| r.instantiations.iter().map(|i| i.name.clone()))
        .collect()
}

pub fn main_reactor_name(lf_file: &str) -> Option<String> {
    let model = ast::parse(lf_file).model;
    model
        .reactors
        .iter()
        .find(|r| r.is_main || r.is_federated)
        .map(|r| r.name.clone())
}

pub fn path(lf_file: &str) -> PathBuf {
    ast::parse(lf_file).path
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn temp() {
        let expected = expect_test::expect![[r#""#]];
        expected.assert_eq(&format!(
            "{:?}",
            ast::parse(
                    r#"
(("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 0 0 0 23 0)
 ((() model)
  (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 0 0 0 13 1)
   ((() target-decl)
    (() "C")
    (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 303 6 8 13 1)
     ((() key-value-pairs)
      (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 305 6 10 7 16)
       ((() pair)
        (() "timeout")
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 316 7 10 7 16)
         ((() element)
          (() ((() time) (() 5) (() "sec")))))))
      (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 323 7 17 8 16)
       ((() pair)
        (() "logging")
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 334 8 10 8 16)
         ((() element) (() "DEBUG")))))
      (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 341 8 17 9 27)
       ((() pair)
        (() "coordination")
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 357 9 15 9 27)
         ((() element) (() "centralized")))))
      (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 370 9 28 12 3)
       ((() pair)
        (() "docker")
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 380 10 9 12 3)
         ((() element)
          (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 380 10 9 12 3)
           ((() key-value-pairs)
            (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 382 10 11 11 26)
             ((() pair)
              (() "rti-image")
              (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 397 11 14 11 26)
               ((() element) (() "🂠rti:local🂠")))))))))))))))
  (() ((() imports)
    (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 415 13 1 15 38)
     ((() import)
      (() "../../lib/Count.lf")
      (() ((() reactors)
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 423 15 6 15 12)
         ((() imported-reactor) (() #u8(122 167 207 110 254 203 222 200 147 255 116 189 67 218 241 149 145 54 174 0)) (() "Count") (() #u8(20 44 36 137 94 248 108 19 12 187 254 130 16 52 135 123 86 159 213 99))))))))
    (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 455 15 38 16 55)
     ((() import)
      (() "../../federated/DistributedCount.lf")
      (() ((() reactors)
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 462 16 6 16 12)
         ((() imported-reactor) (() #u8(157 122 104 62 132 139 183 223 93 138 177 86 223 18 127 91 242 7 134 124)) (() "Print") (() #u8(246 135 206 84 110 36 57 249 140 57 242 119 177 224 255 62 225 82 225 216))))))))))
  (() ((() reactors)
    (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 511 16 55 22 1)
     ((() reactor)
      (() "DistributedCountContainerized")
      (() #u8(128 73 67 242 145 70 215 14 196 33 71 74 252 6 63 123 189 48 232 58))
      (() ((() is-main) (() #f)))
      (() ((() is-federated) (() #t)))
      (() ((() is-realtime) (() #f)))
      (() ((() parameters)
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 561 18 48 18 71)
         ((() parameter)
          (() "offset")
          (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 568 18 55 18 60)
           ((() type)
            (() ((() stars) (() 0)))
            (() ((() is-time) (() #t)))))
          (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 573 18 60 18 71)
           ((() initializer)
            (() ((() exprs)
              (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 575 18 62 18 71)
               ((() time) (() 200) (() "msec")))))
            (() ((() is-braces) (() #f)))
            (() ((() is-parens) (() #f)))
            (() ((() is-assign) (() #t)))
            (() ((() is-trailing-comma) (() #f)))))))))
      (() ((() host)))
      (() ((() instantiations)
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 587 18 74 19 17)
         ((() instantiation)
          (() "c")
          (() ((() reactor) (() "Count") (() #u8(20 44 36 137 94 248 108 19 12 187 254 130 16 52 135 123 86 159 213 99))))))
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 605 19 17 20 17)
         ((() instantiation)
          (() "p")
          (() ((() reactor) (() "Print") (() #u8(246 135 206 84 110 36 57 249 140 57 242 119 177 224 255 62 225 82 225 216))))))))
      (() ((() connections)
        (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 623 20 17 21 28)
         ((() connection)
          (() ((() left-ports)
            (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 623 20 17 21 7)
             ((() var-ref)
              (("/home/username/lingua-franca/test/C/src/lib/Count.lf" 88 3 22 4 17)
               ((() output) (() "out") (() #u8(149 60 206 233 93 145 46 121 86 217 10 202 191 203 43 215 224 138 163 243))))
              (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 587 18 74 19 17)
               ((() instantiation)
                (() "c")
                (() ((() reactor) (() "Count") (() #u8(20 44 36 137 94 248 108 19 12 187 254 130 16 52 135 123 86 159 213 99))))))
              (() ((() is-interleaved) (() #f)))
              (() ((() alias)))))))
          (() ((() right-ports)
            (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 634 21 10 21 15)
             ((() var-ref)
              (("/home/username/lingua-franca/test/C/src/federated/DistributedCount.lf" 407 13 15 14 15)
               ((() input) (() "in") (() #u8(244 164 38 91 225 219 148 206 69 62 232 31 141 96 7 92 6 102 125 61))))
              (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 605 19 17 20 17)
               ((() instantiation)
                (() "p")
                (() ((() reactor) (() "Print") (() #u8(246 135 206 84 110 36 57 249 140 57 242 119 177 224 255 62 225 82 225 216))))))
              (() ((() is-interleaved) (() #f)))
              (() ((() alias)))))))
          (() ((() is-iterated) (() #f)))
          (() ((() is-physical) (() #f)))
          (() ((() delay)
            (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 645 21 21 21 28)
             ((() parameter-reference)
              (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 561 18 48 18 71)
               ((() parameter)
                (() "offset")
                (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 568 18 55 18 60)
                 ((() type)
                  (() ((() stars) (() 0)))
                  (() ((() is-time) (() #t)))))
                (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 573 18 60 18 71)
                 ((() initializer)
                  (() ((() exprs)
                    (("/home/username/lingua-franca/test/C/src/docker/federated/DistributedCountContainerized.lf" 575 18 62 18 71)
                     ((() time) (() 200) (() "msec")))))
                  (() ((() is-braces) (() #f)))
                  (() ((() is-parens) (() #f)))
                  (() ((() is-assign) (() #t)))
                  (() ((() is-trailing-comma) (() #f)))))))))))))))))))))
            "#
            )
        ));
    }
    // #[test]
    // fn temp2() {
    //     let expected = expect_test::expect![[r#""#]];
    //     expected.assert_eq(&format!(
    //         "{:?}",
    //         lexpr::from_str(r#"((a . 42) (b . 43))"#).unwrap()
    //     ));
    // }
}
