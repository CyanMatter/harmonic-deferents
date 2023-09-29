macro_rules! zip_2_vecs {
	($v:ident, $u:ident) => {
			$v.iter().zip($u.iter())
	};
}

pub(crate) use zip_2_vecs;