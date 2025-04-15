#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_var() {
	let mut env = Env::default();
	let slot = Slot::new(Type::Unit, Lifetime(1));
	env.insert("x", Type::Unit, Lifetime(1));
	env.insert("y", Type::Int, Lifetime(1));
	assert_eq!(env.type_lval(&Lval::new("x", 0)).unwrap(), slot);
    }

    #[test]
    fn env_lval_box() {
	let mut env = Env::default();
	let slot = Slot::new(Type::Int, Lifetime(3));
	env.insert("x", Type::boxx(Type::Int), Lifetime(3));
	assert_eq!(env.type_lval(&Lval::new("x", 1)).unwrap(), slot);
    }

    #[test]
    fn env_lval_ref() {
	let mut env = Env::default();
	let slot = Slot::new(Type::Int, Lifetime(1));
	env.insert("x", Type::imm_ref(Lval::new("y", 1)), Lifetime(3));
	env.insert("y", Type::boxx(Type::Int), Lifetime(1));
	assert_eq!(env.type_lval(&Lval::new("x", 1)).unwrap(), slot);
    }

    #[test]
    fn env_contained() {
	let mut env = Env::default();
	env.insert("y", Type::boxx(Type::boxx(Type::Int)), Lifetime(1));
	assert_eq!(*env.contained("y").unwrap(), Type::Int);
    }

    #[test]
    fn env_contained_undefined() {
	let mut env = Env::default();
	env.insert("y", Type::boxx(Type::boxx(Type::undefined(Type::Int))), Lifetime(1));
	assert_eq!(env.contained(&String::from("y")), None);
    }

    #[test]
    fn basic_read_prohibited() {
	let mut env = Env::default();
	env.insert("z", Type::boxx(Type::mut_ref(Lval::new("w", 0))), Lifetime(2));
	env.insert("x", Type::boxx(Type::mut_ref(Lval::new("y", 3))), Lifetime(30));
	assert!(env.read_prohibited(&Lval::new("y", 5)));
    }

    #[test]
    fn basic_write_prohibited() {
	let mut env = Env::default();
	env.insert("z", Type::boxx(Type::mut_ref(Lval::new("w", 0))), Lifetime(2));
	env.insert("x", Type::boxx(Type::mut_ref(Lval::new("y", 3))), Lifetime(30));
	assert!(env.write_prohibited(&Lval::new("y", 5)));
    }

    #[test]
    fn basic_write_prohibited_2() {
	let mut env = Env::default();
	env.insert("z", Type::boxx(Type::mut_ref(Lval::new("w", 0))), Lifetime(2));
	env.insert("x", Type::boxx(Type::imm_ref(Lval::new("y", 3))), Lifetime(30));
	assert!(env.write_prohibited(&Lval::new("y", 5)));
    }

    #[test]
    fn move_under_box() {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::boxx(Type::boxx(Type::Int))), Lifetime(40));
	assert!(env.moove(&Lval::new("x", 2)).is_ok());
	if let Some(slot) = env.0.get("x") {
	    assert_eq!(slot.tipe, Type::boxx(Type::boxx(Type::undefined(Type::boxx(Type::Int)))));
	} else {
	    assert!(false);
	}
    }

    #[test]
    fn move_under_ref() {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::mut_ref(Lval::new("y", 1))), Lifetime(40));
	assert!(env.moove(&Lval::new("x", 2)).is_err());
    }

    #[test]
    fn mut_succ() {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::mut_ref(Lval::new("y", 3))), Lifetime(31));
	env.insert("y", Type::mut_ref(Lval::new("z", 0)), Lifetime(24));
	env.insert("z", Type::boxx(Type::mut_ref(Lval::new("w", 2))), Lifetime(23));
	env.insert("w", Type::boxx(Type::boxx(Type::boxx(Type::Int))), Lifetime(29));
	assert!(env.muut(&Lval::new("x", 3)));
    }

    #[test]
    fn mut_fail() {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::mut_ref(Lval::new("y", 3))), Lifetime(31));
	env.insert("y", Type::mut_ref(Lval::new("z", 0)), Lifetime(24));
	env.insert("z", Type::boxx(Type::imm_ref(Lval::new("w", 2))), Lifetime(23));
	env.insert("w", Type::boxx(Type::boxx(Type::boxx(Type::Int))), Lifetime(29));
	assert!(!env.muut(&Lval::new("x", 3)));
    }

    #[test]
    fn compatible_basic() {
	let env = Env::default();
	let t1 = Type::boxx(Type::boxx(Type::undefined(Type::boxx(Type::Int))));
	let t2 = Type::boxx(Type::undefined(Type::boxx(Type::boxx(Type::Int))));
	assert!(env.compatible(&t1, &t2));
    }

    #[test]
    fn compatible_basic_fail() {
	let env = Env::default();
	let t1 = Type::boxx(Type::boxx(Type::undefined(Type::boxx(Type::Int))));
	let t2 = Type::boxx(Type::undefined(Type::boxx(Type::Int)));
	assert!(!env.compatible(&t1, &t2));
    }

    #[test]
    fn compatible_refs() {
	let mut env = Env::default();
	env.insert("y", Type::boxx(Type::undefined(Type::imm_ref(Lval::new("a", 0)))), Lifetime(1));
	env.insert("z", Type::boxx(Type::imm_ref(Lval::new("b", 1))), Lifetime(1));
	env.insert("b", Type::boxx(Type::imm_ref(Lval::new("c", 1))), Lifetime(1));
	env.insert("a", Type::Int, Lifetime(1));
	env.insert("c", Type::boxx(Type::undefined(Type::Int)), Lifetime(1));
	let t1 = Type::boxx(Type::undefined(Type::mut_ref(Lval::new("y", 1))));
	let t2 = Type::boxx(Type::mut_ref(Lval::new("z", 2)));
	assert!(env.compatible(&t1, &t2));
    }

    #[test]
    fn write_basic() {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::boxx(Type::undefined(Type::boxx(Type::Int)))), Lifetime(23));
	assert!(env.write(&Lval::new("x", 2), Type::boxx(Type::Int)).is_ok());
	if let Some(slot) = env.0.get("x") {
	    assert_eq!(slot.tipe, Type::boxx(Type::boxx(Type::boxx(Type::Int))));
	} else {
	    assert!(false);
	}
    }

    #[test]
    fn write_ref () {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::boxx(Type::mut_ref(Lval::new("y", 2)))), Lifetime(23));
	env.insert("y", Type::boxx(Type::mut_ref(Lval::new("z", 1))), Lifetime(11));
	env.insert("z", Type::mut_ref(Lval::new("w", 2)), Lifetime(1));
	env.insert("w", Type::boxx(Type::boxx(Type::boxx(Type::imm_ref(Lval::new("a", 0))))), Lifetime(87));
	env.insert("a", Type::Int, Lifetime(23));
	env.insert("b", Type::Int, Lifetime(44));
	assert!(env.write(&Lval::new("x", 3), Type::boxx(Type::imm_ref(Lval::new("b", 0)))).is_ok());
	let mut env_2 = env.clone();
	env_2.insert("w", Type::boxx(Type::boxx(Type::boxx(Type::imm_ref(Lval::new("b", 0))))), Lifetime(87));
	assert_eq!(env, env_2);
    }

    #[test]
    fn drop_basic() {
	let mut env = Env::default();
	env.insert("x", Type::boxx(Type::boxx(Type::mut_ref(Lval::new("y", 2)))), Lifetime(11));
	env.insert("y", Type::boxx(Type::mut_ref(Lval::new("z", 1))), Lifetime(11));
	env.insert("z", Type::mut_ref(Lval::new("w", 2)), Lifetime(1));
	env.insert("w", Type::boxx(Type::boxx(Type::boxx(Type::imm_ref(Lval::new("a", 0))))), Lifetime(87));
	env.insert("a", Type::Int, Lifetime(11));
	env.insert("b", Type::Int, Lifetime(44));
	env.drop(Lifetime(11));
	let mut env_2 = Env::default();
	env_2.insert("z", Type::mut_ref(Lval::new("w", 2)), Lifetime(1));
	env_2.insert("w", Type::boxx(Type::boxx(Type::boxx(Type::imm_ref(Lval::new("a", 0))))), Lifetime(87));
	env_2.insert("b", Type::Int, Lifetime(44));
	assert_eq!(env, env_2);
    }
}
