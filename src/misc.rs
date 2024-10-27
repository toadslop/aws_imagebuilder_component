use either::Either;

pub type User = Either<String, u32>;
pub type Group = User;
pub type Permissions = Either<String, u32>;
