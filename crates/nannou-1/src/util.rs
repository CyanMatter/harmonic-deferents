macro_rules! zip2 {
	($v:ident, $u:ident) => {
			$v.iter().zip($u.iter())
	};
}

pub(crate) use zip2;