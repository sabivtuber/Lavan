use crate::data::traits::{Attachable, Combinable, Response};

use super::pure::Pure;

impl Response for () {}

impl<Res> Combinable<Res> for ()
where
    Res: Response,
{
    type Output = Res;

    fn combine_response<Fun>(self, response: Fun) -> Self::Output
    where
        Fun: FnOnce() -> Res,
    {
        response()
    }
}

impl<Val> Attachable<Val> for () {
    type Output = Pure<Val>;

    fn attach_to_response(self, value: Val) -> Self::Output {
        Pure(value)
    }
}
