use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    UserHasVoted = 101,
    GameNotFound = 102,
    GameNameCannotBeEmpty = 103,
}
