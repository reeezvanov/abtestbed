pub mod CollisionMap {
    pub struct CollisionPolicy(pub u32, pub u32);

    pub const PLAYER: CollisionPolicy = CollisionPolicy(0b00000001, 0b00000010);
    pub const BOMB: CollisionPolicy = CollisionPolicy(0b00000010, 0b00000001);
}
