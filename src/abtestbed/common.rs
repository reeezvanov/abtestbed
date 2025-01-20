pub mod collision {

    pub mod group {
        pub const PLAYER_GROUP: u32 = 0b00000001;
        pub const BOMB_GROUP: u32 = 0b00000010;
    }

    pub mod policy {
        pub const PLAYER: (u32, u32) = (super::group::PLAYER_GROUP, super::group::BOMB_GROUP);
        pub const BOMB: (u32, u32) = (
            super::group::BOMB_GROUP,
            super::group::PLAYER_GROUP | super::group::BOMB_GROUP,
        );
    }
}
