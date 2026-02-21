#[cfg(test)]
mod tests {
    use autowrap::WrapExt;

    #[test]
    fn test_cell_wrap() {
        let c = 10u32.cell();
        assert_eq!(c.get(), 10);

        c.set(20);
        assert_eq!(c.get(), 20);
    }

    #[cfg(feature = "std")]
    mod std_tests {
        use std::rc::Rc;
        use autowrap::WrapExt;

        #[test]
        fn test_refcell_wrap() {
            let r = 5u32.refcell();
            *r.borrow_mut() = 8;
            assert_eq!(*r.borrow(), 8);
        }

        #[test]
        fn test_rc_wrap() {
            let rc = 42u32.rc();
            assert_eq!(*rc, 42);

            let rc2 = Rc::clone(&rc);
            assert_eq!(Rc::strong_count(&rc2), 2);
        }

        #[test]
        fn test_rc_refcell_wrap() {
            let value = 100u32.rc_refcell();
            *value.borrow_mut() = 200;
            assert_eq!(*value.borrow(), 200);

            let cloned = Rc::clone(&value);
            assert_eq!(Rc::strong_count(&cloned), 2);
        }

        #[cfg(feature = "sync")]
        mod sync_tests {
            use super::*;
            use std::sync::{Arc, Mutex};
            use std::thread;

            #[test]
            fn test_arc_wrap() {
                let arc = 99u32.arc();
                assert_eq!(*arc, 99);

                let arc2 = Arc::clone(&arc);
                assert_eq!(Arc::strong_count(&arc2), 2);
            }

            #[test]
            fn test_arc_mutex_wrap() {
                let value = 1u32.arc_mutex();

                {
                    let mut lock = value.lock().unwrap();
                    *lock = 10;
                }

                assert_eq!(*value.lock().unwrap(), 10);

                let cloned = Arc::clone(&value);
                let handle = thread::spawn(move || {
                    let mut data = cloned.lock().unwrap();
                    *data += 5;
                });

                handle.join().unwrap();
                assert_eq!(*value.lock().unwrap(), 15);
            }
        }
    }
}