#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locate_var() {
	let mut store = Store::default();
	store.insert("x", Some(Value::Unit), Lifetime::global());
	assert_eq!(store.locate(&Lval::var("x")), "x");
    }

    #[test]
    fn locate_ref() {
	let mut store = Store::default();
	store.insert("1", Some(Value::Unit), Lifetime::global());
	store.insert("y", Some(Value::Ref(String::from("1"), Owned::Yes)), Lifetime::global());
	store.insert("x", Some(Value::Ref(String::from("y"), Owned::No)), Lifetime::global());
	assert_eq!(store.locate(&Lval::new("x", 2)), "1");
    }

    #[test]
    #[should_panic]
    fn locate_panic() {
	let mut store = Store::default();
	store.insert("x", Some(Value::Int(1)), Lifetime::global());
	store.locate(&Lval::new("x", 1));
    }

    #[test]
    fn read_var() {
	let mut store = Store::default();
	store.insert("x", Some(Value::Int(42)), Lifetime::global());
	assert_eq!(store.read(&Lval::var("x")).value, Some(Value::Int(42)));
    }

    #[test]
    fn read_ref_owned() {
	let mut store = Store::default();
	store.insert("1", Some(Value::Int(-30)), Lifetime::global());
	store.insert("y", Some(Value::Ref(String::from("1"), Owned::Yes)), Lifetime::global());
	store.insert("x", Some(Value::Ref(String::from("y"), Owned::No)), Lifetime::global());
	assert_eq!(store.read(&Lval::new("x", 2)).value, Some(Value::Int(-30)));
    }

    #[test]
    #[should_panic]
    fn read_panic() {
	let mut store = Store::default();
	store.insert("1", Some(Value::Int(-30)), Lifetime::global());
	store.insert("y", Some(Value::Ref(String::from("1"), Owned::Yes)), Lifetime::global());
	store.insert("x", Some(Value::Ref(String::from("y"), Owned::No)), Lifetime::global());
	let _ = store.read(&Lval::new("x", 3)).value;
    }

    #[test]
    fn write_two_deref() {
	let mut store = Store::default();
	store.insert("2", Some(Value::Int(1)), Lifetime::global());
	store.insert("x", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime::global());
	store.insert("y", Some(Value::Ref(String::from("x"), Owned::No)), Lifetime::global());
	store.insert("z", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime::global());
	assert_eq!(store.write(&Lval::new("y", 2), Some(Value::Int(5))), Some(Value::Int(1)));
	let slot_2 = store.read(&Lval::new("y", 2));
	assert_eq!(slot_2.value, Some(Value::Int(5)));
    }

    #[test]
    fn write_deref_read_diff() {
	let mut store = Store::default();
	store.insert("2", Some(Value::Int(1)), Lifetime::global());
	store.insert("x", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime::global());
	store.insert("y", Some(Value::Ref(String::from("x"), Owned::No)), Lifetime::global());
	store.insert("z", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime::global());
	assert_eq!(store.write(&Lval::new("y", 2), Some(Value::Int(5))), Some(Value::Int(1)));
	let slot_2 = store.read(&Lval::new("z", 1));
	assert_eq!(slot_2.value, Some(Value::Int(5)));
    }

    #[test]
    #[should_panic]
    fn write_panic() {
	let mut store = Store::default();
	store.insert("2", Some(Value::Int(1)), Lifetime::global());
	store.insert("x", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime::global());
	store.insert("y", Some(Value::Ref(String::from("x"), Owned::No)), Lifetime::global());
	store.insert("z", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime::global());
	store.write(&Lval::new("y", 3), Some(Value::Int(5)));
    }

    #[test]
    fn drop_owned() {
	let mut store = Store::default();
	store.insert("x", Some(Value::Ref(String::from("1"), Owned::Yes)), Lifetime::global());
	store.insert("1", Some(Value::Int(1)), Lifetime::global());
	store.drop(vec![Some(Value::Ref(String::from("x"), Owned::Yes))]);
	assert_eq!(store, Store::default());
    }

    #[test]
    fn drop_unowned() {
	let mut store = Store::default();
	store.insert("x", Some(Value::Ref(String::from("1"), Owned::No)), Lifetime::global());
	store.insert("1", Some(Value::Int(1)), Lifetime::global());
	let mut store_2 = Store::default();
	store_2.insert("1", Some(Value::Int(1)), Lifetime::global());
	store.drop(vec![Some(Value::Ref(String::from("x"), Owned::Yes))]);
	assert_eq!(store, store_2);
    }

    #[test]
    fn drop_larger_example() {
	let mut store = Store::default();
	store.insert("x", Some(Value::Ref(String::from("1"), Owned::Yes)), Lifetime(1));
	store.insert("1", Some(Value::Ref(String::from("2"), Owned::No)), Lifetime(2));
	store.insert("2", Some(Value::Int(1)), Lifetime(1));
	store.insert("y", Some(Value::Ref(String::from("x"), Owned::No)), Lifetime(2));
	store.insert("z", Some(Value::Ref(String::from("2"), Owned::Yes)), Lifetime(2));
	store.insert("w", Some(Value::Ref(String::from("3"), Owned::Yes)), Lifetime(1));
	store.insert("3", Some(Value::Int(2)), Lifetime(2));
	store.insert("v", Some(Value::Ref(String::from("1"), Owned::No)), Lifetime(1));
	store.drop(store.locs_by_lifetime(Lifetime(1)));
	let mut store_2 = Store::default();
	store_2.insert("y", Some(Value::Ref(String::from("x"), Owned::No)), Lifetime(2));
	store_2.insert("z", Some(Value::Ref(String::from("2"), Owned::Yes)), Lifetime(2));
	assert_eq!(store, store_2);
    }

    #[test]
    fn eval_lits() {
	let mut context = Context::default();
	assert_eq!(context.eval_expr(&Expr::Unit, Lifetime::global()), Value::Unit);
	assert_eq!(context.eval_expr(&Expr::Int(234), Lifetime::global()), Value::Int(234));
	assert_eq!(context.store, Store::default());
    }

    #[test]
    fn eval_copy() {
	let mut context = Context::default();
	context.store.insert("x", Some(Value::Int(34)), Lifetime(1));
	let store_2 = context.store.clone();
	assert_eq!(context.eval_expr(&Expr::Lval(Lval::new("x", 0), Copyable::Yes), Lifetime::global()), Value::Int(34));
	assert_eq!(context.store, store_2);
    }

    #[test]
    fn eval_move() {
	let mut context = Context::default();
	context.store.insert("x", Some(Value::Int(5)), Lifetime(1));
	let mut store_2 = Store::default();
	store_2.insert("x", None, Lifetime(1));
	assert_eq!(context.eval_expr(&Expr::Lval(Lval::new("x", 0), Copyable::No), Lifetime::global()), Value::Int(5));
	assert_eq!(context.store, store_2);
    }

    #[test]
    fn eval_box() {
	let mut context = Context::default();
	if let Value::Ref(loc, _) = context.eval_expr(&Expr::Box(Box::new(Expr::Int(-1))), Lifetime(5)) {
	    let mut store_2 = Store::default();
	    store_2.insert(&loc, Some(Value::Int(-1)), Lifetime::global());
	    assert_eq!(context.store.read(&Lval::new(&loc, 0)).value, Some(Value::Int(-1)));
	    assert_eq!(context.store, store_2);
	} else {
	    assert!(false);
	}
    }

    #[test]
    fn eval_box_box() {
	let mut context = Context::default();
	let box_box = Expr::Box(Box::new(Expr::Box(Box::new(Expr::Int(12)))));
	if let Value::Ref(loc, _) = context.eval_expr(&box_box, Lifetime(34)) {
	    assert_eq!(context.store.read(&Lval::new(&loc, 1)).value, Some(Value::Int(12)));
	}
    }

    #[test]
    fn eval_let_mut() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(14)))), Lifetime(4));
	assert_eq!(context.store.read(&Lval::new("x", 1)).value, Some(Value::Int(14)));
    }

    #[test]
    fn eval_assign_copy() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(14)))), Lifetime(4));
	context.eval_stmt(&Stmt::LetMut(String::from("y"), Expr::Box(Box::new(Expr::Int(15)))), Lifetime(4));
	context.eval_stmt(&Stmt::Assign(Lval::new("x", 0), Expr::Lval(Lval::new("y", 0), Copyable::Yes)), Lifetime(4));
	assert_eq!(context.store.read(&Lval::new("x", 1)).value, Some(Value::Int(15)));
	assert_eq!(context.store.read(&Lval::new("y", 1)).value, Some(Value::Int(15)));
    }

    #[test]
    fn eval_assign_move() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(14)))), Lifetime(4));
	context.eval_stmt(&Stmt::LetMut(String::from("y"), Expr::Box(Box::new(Expr::Int(15)))), Lifetime(4));
	context.eval_stmt(&Stmt::Assign(Lval::new("x", 0), Expr::Lval(Lval::new("y", 0), Copyable::No)), Lifetime(4));
	assert_eq!(context.store.read(&Lval::new("x", 1)).value, Some(Value::Int(15)));
	assert_eq!(context.store.read(&Lval::new("y", 0)).value, None);
    }

    #[test]
    fn eval_assign_replace() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(14)))), Lifetime(4));
	context.eval_stmt(&Stmt::LetMut(String::from("y"), Expr::Box(Box::new(Expr::Int(15)))), Lifetime(4));
	context.eval_stmt(&Stmt::Assign(Lval::new("x", 0), Expr::Lval(Lval::new("y", 0), Copyable::No)), Lifetime(4));
	context.eval_stmt(&Stmt::Assign(Lval::new("y", 0), Expr::Box(Box::new(Expr::Int(16)))), Lifetime(4));
	assert_eq!(context.store.read(&Lval::new("x", 1)).value, Some(Value::Int(15)));
	assert_eq!(context.store.read(&Lval::new("y", 1)).value, Some(Value::Int(16)));
    }

    #[test]
    fn eval_assign_move_deref() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(14)))), Lifetime(4));
	context.eval_stmt(&Stmt::LetMut(String::from("y"), Expr::Box(Box::new(Expr::Int(15)))), Lifetime(4));
	context.eval_stmt(&Stmt::Assign(Lval::new("x", 1), Expr::Lval(Lval::new("y", 1), Copyable::No)), Lifetime(4));
	assert_eq!(context.store.read(&Lval::new("x", 1)).value, Some(Value::Int(15)));
	assert_eq!(context.store.read(&Lval::new("y", 1)).value, None);
    }


    #[test]
    fn eval_expr_stmt() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(14)))), Lifetime(4));
	context.eval_stmt(&Stmt::Expr(Expr::Lval(Lval::new("x", 1), Copyable::No)), Lifetime(4));
	assert_eq!(context.store.read(&Lval::new("x", 1)).value, None);
    }

    #[test]
    fn eval_block() {
	let mut context = Context::default();
	let e = Expr::Block(
	    vec![Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(23))))],
	    Box::new(Expr::Unit),
	    Lifetime(3),
	);
	context.eval_expr(&e, Lifetime(5));
	assert_eq!(context.store, Store::default());
    }

    #[test]
    fn eval_block_ref() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(203)))), Lifetime(4));
	let store_2 = context.store.clone();
	let e = Expr::Block(
	    vec![Stmt::LetMut(String::from("y"), Expr::Borrow(Lval::new("x", 1), Mutable::No))],
	    Box::new(Expr::Unit),
	    Lifetime(6),
	);
	context.eval_expr(&e, Lifetime(4));
	assert_eq!(context.store, store_2);
    }

    #[test]
    fn eval_block_mut_ref() {
	let mut context = Context::default();
	context.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(203)))), Lifetime(4));
	let e = Expr::Block(
	    vec![
		Stmt::LetMut(String::from("y"), Expr::Borrow(Lval::new("x", 1), Mutable::Yes)),
		Stmt::Assign(Lval::new("y", 1), Expr::Int(-150)),
	    ],
	    Box::new(Expr::Unit),
	    Lifetime(6),
	);
	context.eval_expr(&e, Lifetime(4));
	let mut context_2 = Context::default();
	context_2.eval_stmt(&Stmt::LetMut(String::from("x"), Expr::Box(Box::new(Expr::Int(-150)))), Lifetime(4));
	assert_eq!(context.store, context_2.store);
    }

}
