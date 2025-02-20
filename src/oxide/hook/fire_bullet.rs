use crate::{
    call_original, cfn, get_cheat, oxide::cheat::visual::Visuals, sdk::{entity::{player::Player, weapon::Weapon}, fire_bullets_info::FireBulletsInfo}
};

pub const NAME: &str = "FireBullet";

pub type FireBullet = cfn!(
    (),
    &mut Player,
    *mut Weapon,
    &FireBulletsInfo,
    bool,
    i32,
    i32
);

pub extern "C" fn hook(
    player: &mut Player,
    weapon: *mut Weapon,
    info: &FireBulletsInfo,
    do_effects: bool,
    damage_type: i32,
    custom_damage_typ: i32,
) {
    get_cheat!(Visuals).draw_bullet_tracer(info, weapon);

    call_original!(
        NAME,
        FireBullet,
        player,
        weapon,
        info,
        do_effects,
        damage_type,
        custom_damage_typ
    );
}
