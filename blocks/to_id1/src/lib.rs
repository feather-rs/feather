use feather_blocks_enum::*;

const NOT_FOUND: u16 = 65535;

pub fn to_id(block: Block) -> u16 {
    match block {
        Block::OakFence(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 3461;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 3462;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 3463;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 3464;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 3465;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 3466;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 3467;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 3468;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 3469;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 3470;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 3471;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 3472;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 3473;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 3474;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 3475;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 3476;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 3477;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 3478;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 3479;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 3480;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 3481;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 3482;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 3483;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 3484;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 3485;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 3486;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 3487;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 3488;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 3489;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 3490;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 3491;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 3492;
            }
        }
        Block::Pumpkin => return 3493,
        Block::Netherrack => return 3494,
        Block::SoulSand => return 3495,
        Block::Glowstone => return 3496,
        Block::NetherPortal(data) => {
            if data.axis == Axis::X {
                return 3497;
            }
            if data.axis == Axis::Z {
                return 3498;
            }
        }
        Block::CarvedPumpkin(data) => {
            if data.facing == Facing::North {
                return 3499;
            }
            if data.facing == Facing::South {
                return 3500;
            }
            if data.facing == Facing::West {
                return 3501;
            }
            if data.facing == Facing::East {
                return 3502;
            }
        }
        Block::JackOLantern(data) => {
            if data.facing == Facing::North {
                return 3503;
            }
            if data.facing == Facing::South {
                return 3504;
            }
            if data.facing == Facing::West {
                return 3505;
            }
            if data.facing == Facing::East {
                return 3506;
            }
        }
        Block::Cake(data) => {
            if data.bites == 0 {
                return 3507;
            }
            if data.bites == 1 {
                return 3508;
            }
            if data.bites == 2 {
                return 3509;
            }
            if data.bites == 3 {
                return 3510;
            }
            if data.bites == 4 {
                return 3511;
            }
            if data.bites == 5 {
                return 3512;
            }
            if data.bites == 6 {
                return 3513;
            }
        }
        Block::Repeater(data) => {
            if data.delay == 1
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == true
            {
                return 3514;
            }
            if data.delay == 1
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == false
            {
                return 3515;
            }
            if data.delay == 1
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == true
            {
                return 3516;
            }
            if data.delay == 1
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == false
            {
                return 3517;
            }
            if data.delay == 1
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == true
            {
                return 3518;
            }
            if data.delay == 1
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == false
            {
                return 3519;
            }
            if data.delay == 1
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == true
            {
                return 3520;
            }
            if data.delay == 1
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == false
            {
                return 3521;
            }
            if data.delay == 1
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == true
            {
                return 3522;
            }
            if data.delay == 1
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == false
            {
                return 3523;
            }
            if data.delay == 1
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == true
            {
                return 3524;
            }
            if data.delay == 1
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == false
            {
                return 3525;
            }
            if data.delay == 1
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == true
            {
                return 3526;
            }
            if data.delay == 1
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == false
            {
                return 3527;
            }
            if data.delay == 1
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == true
            {
                return 3528;
            }
            if data.delay == 1
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == false
            {
                return 3529;
            }
            if data.delay == 2
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == true
            {
                return 3530;
            }
            if data.delay == 2
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == false
            {
                return 3531;
            }
            if data.delay == 2
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == true
            {
                return 3532;
            }
            if data.delay == 2
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == false
            {
                return 3533;
            }
            if data.delay == 2
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == true
            {
                return 3534;
            }
            if data.delay == 2
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == false
            {
                return 3535;
            }
            if data.delay == 2
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == true
            {
                return 3536;
            }
            if data.delay == 2
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == false
            {
                return 3537;
            }
            if data.delay == 2
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == true
            {
                return 3538;
            }
            if data.delay == 2
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == false
            {
                return 3539;
            }
            if data.delay == 2
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == true
            {
                return 3540;
            }
            if data.delay == 2
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == false
            {
                return 3541;
            }
            if data.delay == 2
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == true
            {
                return 3542;
            }
            if data.delay == 2
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == false
            {
                return 3543;
            }
            if data.delay == 2
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == true
            {
                return 3544;
            }
            if data.delay == 2
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == false
            {
                return 3545;
            }
            if data.delay == 3
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == true
            {
                return 3546;
            }
            if data.delay == 3
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == false
            {
                return 3547;
            }
            if data.delay == 3
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == true
            {
                return 3548;
            }
            if data.delay == 3
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == false
            {
                return 3549;
            }
            if data.delay == 3
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == true
            {
                return 3550;
            }
            if data.delay == 3
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == false
            {
                return 3551;
            }
            if data.delay == 3
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == true
            {
                return 3552;
            }
            if data.delay == 3
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == false
            {
                return 3553;
            }
            if data.delay == 3
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == true
            {
                return 3554;
            }
            if data.delay == 3
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == false
            {
                return 3555;
            }
            if data.delay == 3
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == true
            {
                return 3556;
            }
            if data.delay == 3
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == false
            {
                return 3557;
            }
            if data.delay == 3
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == true
            {
                return 3558;
            }
            if data.delay == 3
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == false
            {
                return 3559;
            }
            if data.delay == 3
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == true
            {
                return 3560;
            }
            if data.delay == 3
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == false
            {
                return 3561;
            }
            if data.delay == 4
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == true
            {
                return 3562;
            }
            if data.delay == 4
                && data.facing == Facing::North
                && data.locked == true
                && data.powered == false
            {
                return 3563;
            }
            if data.delay == 4
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == true
            {
                return 3564;
            }
            if data.delay == 4
                && data.facing == Facing::North
                && data.locked == false
                && data.powered == false
            {
                return 3565;
            }
            if data.delay == 4
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == true
            {
                return 3566;
            }
            if data.delay == 4
                && data.facing == Facing::South
                && data.locked == true
                && data.powered == false
            {
                return 3567;
            }
            if data.delay == 4
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == true
            {
                return 3568;
            }
            if data.delay == 4
                && data.facing == Facing::South
                && data.locked == false
                && data.powered == false
            {
                return 3569;
            }
            if data.delay == 4
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == true
            {
                return 3570;
            }
            if data.delay == 4
                && data.facing == Facing::West
                && data.locked == true
                && data.powered == false
            {
                return 3571;
            }
            if data.delay == 4
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == true
            {
                return 3572;
            }
            if data.delay == 4
                && data.facing == Facing::West
                && data.locked == false
                && data.powered == false
            {
                return 3573;
            }
            if data.delay == 4
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == true
            {
                return 3574;
            }
            if data.delay == 4
                && data.facing == Facing::East
                && data.locked == true
                && data.powered == false
            {
                return 3575;
            }
            if data.delay == 4
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == true
            {
                return 3576;
            }
            if data.delay == 4
                && data.facing == Facing::East
                && data.locked == false
                && data.powered == false
            {
                return 3577;
            }
        }
        Block::WhiteStainedGlass => return 3578,
        Block::OrangeStainedGlass => return 3579,
        Block::MagentaStainedGlass => return 3580,
        Block::LightBlueStainedGlass => return 3581,
        Block::YellowStainedGlass => return 3582,
        Block::LimeStainedGlass => return 3583,
        Block::PinkStainedGlass => return 3584,
        Block::GrayStainedGlass => return 3585,
        Block::LightGrayStainedGlass => return 3586,
        Block::CyanStainedGlass => return 3587,
        Block::PurpleStainedGlass => return 3588,
        Block::BlueStainedGlass => return 3589,
        Block::BrownStainedGlass => return 3590,
        Block::GreenStainedGlass => return 3591,
        Block::RedStainedGlass => return 3592,
        Block::BlackStainedGlass => return 3593,
        Block::OakTrapdoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3594;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3595;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3596;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3597;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3598;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3599;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3600;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3601;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3602;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3603;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3604;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3605;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3606;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3607;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3608;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3609;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3610;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3611;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3612;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3613;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3614;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3615;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3616;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3617;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3618;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3619;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3620;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3621;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3622;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3623;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3624;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3625;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3626;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3627;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3628;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3629;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3630;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3631;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3632;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3633;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3634;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3635;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3636;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3637;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3638;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3639;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3640;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3641;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3642;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3643;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3644;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3645;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3646;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3647;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3648;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3649;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3650;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3651;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3652;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3653;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3654;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3655;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3656;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3657;
            }
        }
        Block::SpruceTrapdoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3658;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3659;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3660;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3661;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3662;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3663;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3664;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3665;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3666;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3667;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3668;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3669;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3670;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3671;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3672;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3673;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3674;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3675;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3676;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3677;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3678;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3679;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3680;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3681;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3682;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3683;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3684;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3685;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3686;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3687;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3688;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3689;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3690;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3691;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3692;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3693;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3694;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3695;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3696;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3697;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3698;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3699;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3700;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3701;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3702;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3703;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3704;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3705;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3706;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3707;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3708;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3709;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3710;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3711;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3712;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3713;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3714;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3715;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3716;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3717;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3718;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3719;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3720;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3721;
            }
        }
        Block::BirchTrapdoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3722;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3723;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3724;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3725;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3726;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3727;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3728;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3729;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3730;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3731;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3732;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3733;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3734;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3735;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3736;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3737;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3738;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3739;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3740;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3741;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3742;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3743;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3744;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3745;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3746;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3747;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3748;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3749;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3750;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3751;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3752;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3753;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3754;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3755;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3756;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3757;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3758;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3759;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3760;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3761;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3762;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3763;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3764;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3765;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3766;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3767;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3768;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3769;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3770;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3771;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3772;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3773;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3774;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3775;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3776;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3777;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3778;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3779;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3780;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3781;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3782;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3783;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3784;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3785;
            }
        }
        Block::JungleTrapdoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3786;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3787;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3788;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3789;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3790;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3791;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3792;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3793;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3794;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3795;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3796;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3797;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3798;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3799;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3800;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3801;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3802;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3803;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3804;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3805;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3806;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3807;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3808;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3809;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3810;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3811;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3812;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3813;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3814;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3815;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3816;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3817;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3818;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3819;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3820;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3821;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3822;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3823;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3824;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3825;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3826;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3827;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3828;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3829;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3830;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3831;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3832;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3833;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3834;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3835;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3836;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3837;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3838;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3839;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3840;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3841;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3842;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3843;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3844;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3845;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3846;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3847;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3848;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3849;
            }
        }
        Block::AcaciaTrapdoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3850;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3851;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3852;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3853;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3854;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3855;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3856;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3857;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3858;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3859;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3860;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3861;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3862;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3863;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3864;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3865;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3866;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3867;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3868;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3869;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3870;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3871;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3872;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3873;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3874;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3875;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3876;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3877;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3878;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3879;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3880;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3881;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3882;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3883;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3884;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3885;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3886;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3887;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3888;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3889;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3890;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3891;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3892;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3893;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3894;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3895;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3896;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3897;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3898;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3899;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3900;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3901;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3902;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3903;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3904;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3905;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3906;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3907;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3908;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3909;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3910;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3911;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3912;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3913;
            }
        }
        Block::DarkOakTrapdoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3914;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3915;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3916;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3917;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3918;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3919;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3920;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3921;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3922;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3923;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3924;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3925;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3926;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3927;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3928;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3929;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3930;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3931;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3932;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3933;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3934;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3935;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3936;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3937;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3938;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3939;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3940;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3941;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3942;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3943;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3944;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3945;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3946;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3947;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3948;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3949;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3950;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3951;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3952;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3953;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3954;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3955;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3956;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3957;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3958;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3959;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3960;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3961;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3962;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3963;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3964;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3965;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3966;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3967;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3968;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3969;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == true
            {
                return 3970;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == true
                && data.waterlogged == false
            {
                return 3971;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == true
            {
                return 3972;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == true
                && data.powered == false
                && data.waterlogged == false
            {
                return 3973;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == true
            {
                return 3974;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == true
                && data.waterlogged == false
            {
                return 3975;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == true
            {
                return 3976;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.open == false
                && data.powered == false
                && data.waterlogged == false
            {
                return 3977;
            }
        }
        Block::InfestedStone => return 3978,
        Block::InfestedCobblestone => return 3979,
        Block::InfestedStoneBricks => return 3980,
        Block::InfestedMossyStoneBricks => return 3981,
        Block::InfestedCrackedStoneBricks => return 3982,
        Block::InfestedChiseledStoneBricks => return 3983,
        Block::StoneBricks => return 3984,
        Block::MossyStoneBricks => return 3985,
        Block::CrackedStoneBricks => return 3986,
        Block::ChiseledStoneBricks => return 3987,
        Block::BrownMushroomBlock(data) => {
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 3988;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 3989;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 3990;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 3991;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 3992;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 3993;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 3994;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 3995;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 3996;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 3997;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 3998;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 3999;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4000;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4001;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4002;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4003;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4004;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4005;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4006;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4007;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4008;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4009;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4010;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4011;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4012;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4013;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4014;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4015;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4016;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4017;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4018;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4019;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4020;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4021;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4022;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4023;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4024;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4025;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4026;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4027;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4028;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4029;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4030;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4031;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4032;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4033;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4034;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4035;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4036;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4037;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4038;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4039;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4040;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4041;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4042;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4043;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4044;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4045;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4046;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4047;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4048;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4049;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4050;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4051;
            }
        }
        Block::RedMushroomBlock(data) => {
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4052;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4053;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4054;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4055;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4056;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4057;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4058;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4059;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4060;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4061;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4062;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4063;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4064;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4065;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4066;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4067;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4068;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4069;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4070;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4071;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4072;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4073;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4074;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4075;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4076;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4077;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4078;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4079;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4080;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4081;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4082;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4083;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4084;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4085;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4086;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4087;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4088;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4089;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4090;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4091;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4092;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4093;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4094;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4095;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4096;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4097;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4098;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4099;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4100;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4101;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4102;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4103;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4104;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4105;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4106;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4107;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4108;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4109;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4110;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4111;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4112;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4113;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4114;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4115;
            }
        }
        Block::MushroomStem(data) => {
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4116;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4117;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4118;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4119;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4120;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4121;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4122;
            }
            if data.down == true
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4123;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4124;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4125;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4126;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4127;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4128;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4129;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4130;
            }
            if data.down == true
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4131;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4132;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4133;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4134;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4135;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4136;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4137;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4138;
            }
            if data.down == true
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4139;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4140;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4141;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4142;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4143;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4144;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4145;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4146;
            }
            if data.down == true
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4147;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4148;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4149;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4150;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4151;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4152;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4153;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4154;
            }
            if data.down == false
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4155;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4156;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4157;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4158;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4159;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4160;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4161;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4162;
            }
            if data.down == false
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4163;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4164;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4165;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4166;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4167;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4168;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4169;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4170;
            }
            if data.down == false
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4171;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4172;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4173;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4174;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4175;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4176;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4177;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4178;
            }
            if data.down == false
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4179;
            }
        }
        Block::IronBars(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4180;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4181;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4182;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4183;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4184;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4185;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4186;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4187;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4188;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4189;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4190;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4191;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4192;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4193;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4194;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4195;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4196;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4197;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4198;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4199;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4200;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4201;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4202;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4203;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4204;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4205;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4206;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4207;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4208;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4209;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4210;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4211;
            }
        }
        Block::GlassPane(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4212;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4213;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4214;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4215;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4216;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4217;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4218;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4219;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4220;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4221;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4222;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4223;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4224;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4225;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4226;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4227;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4228;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4229;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4230;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4231;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4232;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4233;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4234;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4235;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4236;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4237;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4238;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4239;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4240;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4241;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4242;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4243;
            }
        }
        Block::Melon => return 4244,
        Block::AttachedPumpkinStem(data) => {
            if data.facing == Facing::North {
                return 4245;
            }
            if data.facing == Facing::South {
                return 4246;
            }
            if data.facing == Facing::West {
                return 4247;
            }
            if data.facing == Facing::East {
                return 4248;
            }
        }
        Block::AttachedMelonStem(data) => {
            if data.facing == Facing::North {
                return 4249;
            }
            if data.facing == Facing::South {
                return 4250;
            }
            if data.facing == Facing::West {
                return 4251;
            }
            if data.facing == Facing::East {
                return 4252;
            }
        }
        Block::PumpkinStem(data) => {
            if data.age == 0 {
                return 4253;
            }
            if data.age == 1 {
                return 4254;
            }
            if data.age == 2 {
                return 4255;
            }
            if data.age == 3 {
                return 4256;
            }
            if data.age == 4 {
                return 4257;
            }
            if data.age == 5 {
                return 4258;
            }
            if data.age == 6 {
                return 4259;
            }
            if data.age == 7 {
                return 4260;
            }
        }
        Block::MelonStem(data) => {
            if data.age == 0 {
                return 4261;
            }
            if data.age == 1 {
                return 4262;
            }
            if data.age == 2 {
                return 4263;
            }
            if data.age == 3 {
                return 4264;
            }
            if data.age == 4 {
                return 4265;
            }
            if data.age == 5 {
                return 4266;
            }
            if data.age == 6 {
                return 4267;
            }
            if data.age == 7 {
                return 4268;
            }
        }
        Block::Vine(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4269;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4270;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4271;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4272;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4273;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4274;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4275;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4276;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4277;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4278;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4279;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4280;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4281;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4282;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4283;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4284;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4285;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4286;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4287;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4288;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4289;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4290;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4291;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4292;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 4293;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 4294;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 4295;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 4296;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 4297;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 4298;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 4299;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 4300;
            }
        }
        Block::OakFenceGate(data) => {
            if data.facing == Facing::North
                && data.in_wall == true
                && data.open == true
                && data.powered == true
            {
                return 4301;
            }
            if data.facing == Facing::North
                && data.in_wall == true
                && data.open == true
                && data.powered == false
            {
                return 4302;
            }
            if data.facing == Facing::North
                && data.in_wall == true
                && data.open == false
                && data.powered == true
            {
                return 4303;
            }
            if data.facing == Facing::North
                && data.in_wall == true
                && data.open == false
                && data.powered == false
            {
                return 4304;
            }
            if data.facing == Facing::North
                && data.in_wall == false
                && data.open == true
                && data.powered == true
            {
                return 4305;
            }
            if data.facing == Facing::North
                && data.in_wall == false
                && data.open == true
                && data.powered == false
            {
                return 4306;
            }
            if data.facing == Facing::North
                && data.in_wall == false
                && data.open == false
                && data.powered == true
            {
                return 4307;
            }
            if data.facing == Facing::North
                && data.in_wall == false
                && data.open == false
                && data.powered == false
            {
                return 4308;
            }
            if data.facing == Facing::South
                && data.in_wall == true
                && data.open == true
                && data.powered == true
            {
                return 4309;
            }
            if data.facing == Facing::South
                && data.in_wall == true
                && data.open == true
                && data.powered == false
            {
                return 4310;
            }
            if data.facing == Facing::South
                && data.in_wall == true
                && data.open == false
                && data.powered == true
            {
                return 4311;
            }
            if data.facing == Facing::South
                && data.in_wall == true
                && data.open == false
                && data.powered == false
            {
                return 4312;
            }
            if data.facing == Facing::South
                && data.in_wall == false
                && data.open == true
                && data.powered == true
            {
                return 4313;
            }
            if data.facing == Facing::South
                && data.in_wall == false
                && data.open == true
                && data.powered == false
            {
                return 4314;
            }
            if data.facing == Facing::South
                && data.in_wall == false
                && data.open == false
                && data.powered == true
            {
                return 4315;
            }
            if data.facing == Facing::South
                && data.in_wall == false
                && data.open == false
                && data.powered == false
            {
                return 4316;
            }
            if data.facing == Facing::West
                && data.in_wall == true
                && data.open == true
                && data.powered == true
            {
                return 4317;
            }
            if data.facing == Facing::West
                && data.in_wall == true
                && data.open == true
                && data.powered == false
            {
                return 4318;
            }
            if data.facing == Facing::West
                && data.in_wall == true
                && data.open == false
                && data.powered == true
            {
                return 4319;
            }
            if data.facing == Facing::West
                && data.in_wall == true
                && data.open == false
                && data.powered == false
            {
                return 4320;
            }
            if data.facing == Facing::West
                && data.in_wall == false
                && data.open == true
                && data.powered == true
            {
                return 4321;
            }
            if data.facing == Facing::West
                && data.in_wall == false
                && data.open == true
                && data.powered == false
            {
                return 4322;
            }
            if data.facing == Facing::West
                && data.in_wall == false
                && data.open == false
                && data.powered == true
            {
                return 4323;
            }
            if data.facing == Facing::West
                && data.in_wall == false
                && data.open == false
                && data.powered == false
            {
                return 4324;
            }
            if data.facing == Facing::East
                && data.in_wall == true
                && data.open == true
                && data.powered == true
            {
                return 4325;
            }
            if data.facing == Facing::East
                && data.in_wall == true
                && data.open == true
                && data.powered == false
            {
                return 4326;
            }
            if data.facing == Facing::East
                && data.in_wall == true
                && data.open == false
                && data.powered == true
            {
                return 4327;
            }
            if data.facing == Facing::East
                && data.in_wall == true
                && data.open == false
                && data.powered == false
            {
                return 4328;
            }
            if data.facing == Facing::East
                && data.in_wall == false
                && data.open == true
                && data.powered == true
            {
                return 4329;
            }
            if data.facing == Facing::East
                && data.in_wall == false
                && data.open == true
                && data.powered == false
            {
                return 4330;
            }
            if data.facing == Facing::East
                && data.in_wall == false
                && data.open == false
                && data.powered == true
            {
                return 4331;
            }
            if data.facing == Facing::East
                && data.in_wall == false
                && data.open == false
                && data.powered == false
            {
                return 4332;
            }
        }
        Block::BrickStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4333;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4334;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4335;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4336;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4337;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4338;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4339;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4340;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4341;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4342;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4343;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4344;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4345;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4346;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4347;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4348;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4349;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4350;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4351;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4352;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4353;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4354;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4355;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4356;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4357;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4358;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4359;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4360;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4361;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4362;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4363;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4364;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4365;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4366;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4367;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4368;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4369;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4370;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4371;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4372;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4373;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4374;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4375;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4376;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4377;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4378;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4379;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4380;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4381;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4382;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4383;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4384;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4385;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4386;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4387;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4388;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4389;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4390;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4391;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4392;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4393;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4394;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4395;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4396;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4397;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4398;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4399;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4400;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4401;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4402;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4403;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4404;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4405;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4406;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4407;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4408;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4409;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4410;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4411;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4412;
            }
        }
        Block::StoneBrickStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4413;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4414;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4415;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4416;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4417;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4418;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4419;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4420;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4421;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4422;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4423;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4424;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4425;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4426;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4427;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4428;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4429;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4430;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4431;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4432;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4433;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4434;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4435;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4436;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4437;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4438;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4439;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4440;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4441;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4442;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4443;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4444;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4445;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4446;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4447;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4448;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4449;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4450;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4451;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4452;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4453;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4454;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4455;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4456;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4457;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4458;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4459;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4460;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4461;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4462;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4463;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4464;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4465;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4466;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4467;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4468;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4469;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4470;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4471;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4472;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4473;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4474;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4475;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4476;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4477;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4478;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4479;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4480;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4481;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4482;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4483;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4484;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4485;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4486;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4487;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4488;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4489;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4490;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4491;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4492;
            }
        }
        Block::Mycelium(data) => {
            if data.snowy == true {
                return 4493;
            }
            if data.snowy == false {
                return 4494;
            }
        }
        Block::LilyPad => return 4495,
        Block::NetherBricks => return 4496,
        Block::NetherBrickFence(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4497;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4498;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4499;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4500;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4501;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4502;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4503;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4504;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4505;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4506;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4507;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4508;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4509;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4510;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4511;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4512;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4513;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4514;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4515;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4516;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4517;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4518;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4519;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4520;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 4521;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 4522;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 4523;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 4524;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 4525;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 4526;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 4527;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 4528;
            }
        }
        Block::NetherBrickStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4529;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4530;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4531;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4532;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4533;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4534;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4535;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4536;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4537;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4538;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4539;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4540;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4541;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4542;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4543;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4544;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4545;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4546;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4547;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4548;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4549;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4550;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4551;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4552;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4553;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4554;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4555;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4556;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4557;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4558;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4559;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4560;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4561;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4562;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4563;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4564;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4565;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4566;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4567;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4568;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4569;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4570;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4571;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4572;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4573;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4574;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4575;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4576;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4577;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4578;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4579;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4580;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4581;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4582;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4583;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4584;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4585;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4586;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4587;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4588;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4589;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4590;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4591;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4592;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4593;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4594;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4595;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4596;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4597;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4598;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4599;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4600;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4601;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4602;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4603;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4604;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4605;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4606;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4607;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4608;
            }
        }
        Block::NetherWart(data) => {
            if data.age == 0 {
                return 4609;
            }
            if data.age == 1 {
                return 4610;
            }
            if data.age == 2 {
                return 4611;
            }
            if data.age == 3 {
                return 4612;
            }
        }
        Block::EnchantingTable => return 4613,
        Block::BrewingStand(data) => {
            if data.has_bottle_0 == true && data.has_bottle_1 == true && data.has_bottle_2 == true {
                return 4614;
            }
            if data.has_bottle_0 == true && data.has_bottle_1 == true && data.has_bottle_2 == false
            {
                return 4615;
            }
            if data.has_bottle_0 == true && data.has_bottle_1 == false && data.has_bottle_2 == true
            {
                return 4616;
            }
            if data.has_bottle_0 == true && data.has_bottle_1 == false && data.has_bottle_2 == false
            {
                return 4617;
            }
            if data.has_bottle_0 == false && data.has_bottle_1 == true && data.has_bottle_2 == true
            {
                return 4618;
            }
            if data.has_bottle_0 == false && data.has_bottle_1 == true && data.has_bottle_2 == false
            {
                return 4619;
            }
            if data.has_bottle_0 == false && data.has_bottle_1 == false && data.has_bottle_2 == true
            {
                return 4620;
            }
            if data.has_bottle_0 == false
                && data.has_bottle_1 == false
                && data.has_bottle_2 == false
            {
                return 4621;
            }
        }
        Block::Cauldron(data) => {
            if data.level == 0 {
                return 4622;
            }
            if data.level == 1 {
                return 4623;
            }
            if data.level == 2 {
                return 4624;
            }
            if data.level == 3 {
                return 4625;
            }
        }
        Block::EndPortal => return 4626,
        Block::EndPortalFrame(data) => {
            if data.eye == true && data.facing == Facing::North {
                return 4627;
            }
            if data.eye == true && data.facing == Facing::South {
                return 4628;
            }
            if data.eye == true && data.facing == Facing::West {
                return 4629;
            }
            if data.eye == true && data.facing == Facing::East {
                return 4630;
            }
            if data.eye == false && data.facing == Facing::North {
                return 4631;
            }
            if data.eye == false && data.facing == Facing::South {
                return 4632;
            }
            if data.eye == false && data.facing == Facing::West {
                return 4633;
            }
            if data.eye == false && data.facing == Facing::East {
                return 4634;
            }
        }
        Block::EndStone => return 4635,
        Block::DragonEgg => return 4636,
        Block::RedstoneLamp(data) => {
            if data.lit == true {
                return 4637;
            }
            if data.lit == false {
                return 4638;
            }
        }
        Block::Cocoa(data) => {
            if data.age == 0 && data.facing == Facing::North {
                return 4639;
            }
            if data.age == 0 && data.facing == Facing::South {
                return 4640;
            }
            if data.age == 0 && data.facing == Facing::West {
                return 4641;
            }
            if data.age == 0 && data.facing == Facing::East {
                return 4642;
            }
            if data.age == 1 && data.facing == Facing::North {
                return 4643;
            }
            if data.age == 1 && data.facing == Facing::South {
                return 4644;
            }
            if data.age == 1 && data.facing == Facing::West {
                return 4645;
            }
            if data.age == 1 && data.facing == Facing::East {
                return 4646;
            }
            if data.age == 2 && data.facing == Facing::North {
                return 4647;
            }
            if data.age == 2 && data.facing == Facing::South {
                return 4648;
            }
            if data.age == 2 && data.facing == Facing::West {
                return 4649;
            }
            if data.age == 2 && data.facing == Facing::East {
                return 4650;
            }
        }
        Block::SandstoneStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4651;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4652;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4653;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4654;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4655;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4656;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4657;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4658;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4659;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4660;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4661;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4662;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4663;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4664;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4665;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4666;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4667;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4668;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4669;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4670;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4671;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4672;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4673;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4674;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4675;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4676;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4677;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4678;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4679;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4680;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4681;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4682;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4683;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4684;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4685;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4686;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4687;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4688;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4689;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4690;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4691;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4692;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4693;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4694;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4695;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4696;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4697;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4698;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4699;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4700;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4701;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4702;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4703;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4704;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4705;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4706;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4707;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4708;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4709;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4710;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4711;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4712;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4713;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4714;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4715;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4716;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4717;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4718;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4719;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4720;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4721;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4722;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4723;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4724;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4725;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4726;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4727;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4728;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4729;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4730;
            }
        }
        Block::EmeraldOre => return 4731,
        Block::EnderChest(data) => {
            if data.facing == Facing::North && data.waterlogged == true {
                return 4732;
            }
            if data.facing == Facing::North && data.waterlogged == false {
                return 4733;
            }
            if data.facing == Facing::South && data.waterlogged == true {
                return 4734;
            }
            if data.facing == Facing::South && data.waterlogged == false {
                return 4735;
            }
            if data.facing == Facing::West && data.waterlogged == true {
                return 4736;
            }
            if data.facing == Facing::West && data.waterlogged == false {
                return 4737;
            }
            if data.facing == Facing::East && data.waterlogged == true {
                return 4738;
            }
            if data.facing == Facing::East && data.waterlogged == false {
                return 4739;
            }
        }
        Block::TripwireHook(data) => {
            if data.attached == true && data.facing == Facing::North && data.powered == true {
                return 4740;
            }
            if data.attached == true && data.facing == Facing::North && data.powered == false {
                return 4741;
            }
            if data.attached == true && data.facing == Facing::South && data.powered == true {
                return 4742;
            }
            if data.attached == true && data.facing == Facing::South && data.powered == false {
                return 4743;
            }
            if data.attached == true && data.facing == Facing::West && data.powered == true {
                return 4744;
            }
            if data.attached == true && data.facing == Facing::West && data.powered == false {
                return 4745;
            }
            if data.attached == true && data.facing == Facing::East && data.powered == true {
                return 4746;
            }
            if data.attached == true && data.facing == Facing::East && data.powered == false {
                return 4747;
            }
            if data.attached == false && data.facing == Facing::North && data.powered == true {
                return 4748;
            }
            if data.attached == false && data.facing == Facing::North && data.powered == false {
                return 4749;
            }
            if data.attached == false && data.facing == Facing::South && data.powered == true {
                return 4750;
            }
            if data.attached == false && data.facing == Facing::South && data.powered == false {
                return 4751;
            }
            if data.attached == false && data.facing == Facing::West && data.powered == true {
                return 4752;
            }
            if data.attached == false && data.facing == Facing::West && data.powered == false {
                return 4753;
            }
            if data.attached == false && data.facing == Facing::East && data.powered == true {
                return 4754;
            }
            if data.attached == false && data.facing == Facing::East && data.powered == false {
                return 4755;
            }
        }
        Block::Tripwire(data) => {
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4756;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4757;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4758;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4759;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4760;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4761;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4762;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4763;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4764;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4765;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4766;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4767;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4768;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4769;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4770;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4771;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4772;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4773;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4774;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4775;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4776;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4777;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4778;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4779;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4780;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4781;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4782;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4783;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4784;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4785;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4786;
            }
            if data.attached == true
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4787;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4788;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4789;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4790;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4791;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4792;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4793;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4794;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4795;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4796;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4797;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4798;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4799;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4800;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4801;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4802;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4803;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4804;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4805;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4806;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4807;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4808;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4809;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4810;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4811;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4812;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4813;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4814;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4815;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4816;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4817;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4818;
            }
            if data.attached == true
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4819;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4820;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4821;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4822;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4823;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4824;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4825;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4826;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4827;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4828;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4829;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4830;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4831;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4832;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4833;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4834;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4835;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4836;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4837;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4838;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4839;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4840;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4841;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4842;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4843;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4844;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4845;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4846;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4847;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4848;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4849;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4850;
            }
            if data.attached == false
                && data.disarmed == true
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4851;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4852;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4853;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4854;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4855;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4856;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4857;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4858;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4859;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4860;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4861;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4862;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4863;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4864;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4865;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4866;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == true
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4867;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4868;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4869;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4870;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4871;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4872;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4873;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4874;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == true
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4875;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == true
            {
                return 4876;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == true
                && data.west == false
            {
                return 4877;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == true
            {
                return 4878;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == true
                && data.south == false
                && data.west == false
            {
                return 4879;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == true
            {
                return 4880;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == true
                && data.west == false
            {
                return 4881;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == true
            {
                return 4882;
            }
            if data.attached == false
                && data.disarmed == false
                && data.east == false
                && data.north == false
                && data.powered == false
                && data.south == false
                && data.west == false
            {
                return 4883;
            }
        }
        Block::EmeraldBlock => return 4884,
        Block::SpruceStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4885;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4886;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4887;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4888;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4889;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4890;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4891;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4892;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4893;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4894;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4895;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4896;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4897;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4898;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4899;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4900;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4901;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4902;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4903;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4904;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4905;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4906;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4907;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4908;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4909;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4910;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4911;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4912;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4913;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4914;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4915;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4916;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4917;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4918;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4919;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4920;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4921;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4922;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4923;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4924;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4925;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4926;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4927;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4928;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4929;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4930;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4931;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4932;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4933;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4934;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4935;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4936;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4937;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4938;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4939;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4940;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4941;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4942;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4943;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4944;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4945;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4946;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4947;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4948;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4949;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4950;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4951;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4952;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4953;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4954;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4955;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4956;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4957;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4958;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4959;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4960;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4961;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4962;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4963;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4964;
            }
        }
        Block::BirchStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4965;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4966;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4967;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4968;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4969;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4970;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4971;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4972;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4973;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4974;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4975;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4976;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4977;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4978;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4979;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4980;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4981;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4982;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4983;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4984;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4985;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4986;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4987;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4988;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4989;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 4990;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 4991;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 4992;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 4993;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 4994;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 4995;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 4996;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 4997;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 4998;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 4999;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5000;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5001;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5002;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5003;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5004;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5005;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5006;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5007;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5008;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5009;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5010;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5011;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5012;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5013;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5014;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5015;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5016;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5017;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5018;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5019;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5020;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5021;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5022;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5023;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5024;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5025;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5026;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5027;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5028;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5029;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5030;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5031;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5032;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5033;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5034;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5035;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5036;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5037;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5038;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5039;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5040;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5041;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5042;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5043;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5044;
            }
        }
        Block::JungleStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5045;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5046;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5047;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5048;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5049;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5050;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5051;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5052;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5053;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5054;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5055;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5056;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5057;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5058;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5059;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5060;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5061;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5062;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5063;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5064;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5065;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5066;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5067;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5068;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5069;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5070;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5071;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5072;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5073;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5074;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5075;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5076;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5077;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5078;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5079;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5080;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5081;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5082;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5083;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5084;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5085;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5086;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5087;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5088;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5089;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5090;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5091;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5092;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5093;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5094;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5095;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5096;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5097;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5098;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5099;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5100;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5101;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5102;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5103;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5104;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5105;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5106;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5107;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5108;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5109;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5110;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5111;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5112;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5113;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5114;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5115;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5116;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5117;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5118;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5119;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5120;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5121;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5122;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5123;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5124;
            }
        }
        Block::CommandBlock(data) => {
            if data.conditional == true && data.facing == Facing::North {
                return 5125;
            }
            if data.conditional == true && data.facing == Facing::East {
                return 5126;
            }
            if data.conditional == true && data.facing == Facing::South {
                return 5127;
            }
            if data.conditional == true && data.facing == Facing::West {
                return 5128;
            }
            if data.conditional == true && data.facing == Facing::Up {
                return 5129;
            }
            if data.conditional == true && data.facing == Facing::Down {
                return 5130;
            }
            if data.conditional == false && data.facing == Facing::North {
                return 5131;
            }
            if data.conditional == false && data.facing == Facing::East {
                return 5132;
            }
            if data.conditional == false && data.facing == Facing::South {
                return 5133;
            }
            if data.conditional == false && data.facing == Facing::West {
                return 5134;
            }
            if data.conditional == false && data.facing == Facing::Up {
                return 5135;
            }
            if data.conditional == false && data.facing == Facing::Down {
                return 5136;
            }
        }
        Block::Beacon => return 5137,
        Block::CobblestoneWall(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5138;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5139;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5140;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5141;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5142;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5143;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5144;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5145;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5146;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5147;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5148;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5149;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5150;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5151;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5152;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5153;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5154;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5155;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5156;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5157;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5158;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5159;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5160;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5161;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5162;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5163;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5164;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5165;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5166;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5167;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5168;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5169;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5170;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5171;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5172;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5173;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5174;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5175;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5176;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5177;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5178;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5179;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5180;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5181;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5182;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5183;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5184;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5185;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5186;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5187;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5188;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5189;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5190;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5191;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5192;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5193;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5194;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5195;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5196;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5197;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5198;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5199;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5200;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5201;
            }
        }
        Block::MossyCobblestoneWall(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5202;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5203;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5204;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5205;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5206;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5207;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5208;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5209;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5210;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5211;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5212;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5213;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5214;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5215;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5216;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5217;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5218;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5219;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5220;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5221;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5222;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5223;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5224;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5225;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5226;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5227;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5228;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5229;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5230;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5231;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5232;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5233;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5234;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5235;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5236;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5237;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5238;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5239;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5240;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5241;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5242;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5243;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5244;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5245;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5246;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5247;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5248;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5249;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5250;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5251;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5252;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5253;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5254;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5255;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5256;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5257;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5258;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5259;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5260;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5261;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5262;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5263;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5264;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5265;
            }
        }
        Block::FlowerPot => return 5266,
        Block::PottedOakSapling => return 5267,
        Block::PottedSpruceSapling => return 5268,
        Block::PottedBirchSapling => return 5269,
        Block::PottedJungleSapling => return 5270,
        Block::PottedAcaciaSapling => return 5271,
        Block::PottedDarkOakSapling => return 5272,
        Block::PottedFern => return 5273,
        Block::PottedDandelion => return 5274,
        Block::PottedPoppy => return 5275,
        Block::PottedBlueOrchid => return 5276,
        Block::PottedAllium => return 5277,
        Block::PottedAzureBluet => return 5278,
        Block::PottedRedTulip => return 5279,
        Block::PottedOrangeTulip => return 5280,
        Block::PottedWhiteTulip => return 5281,
        Block::PottedPinkTulip => return 5282,
        Block::PottedOxeyeDaisy => return 5283,
        Block::PottedRedMushroom => return 5284,
        Block::PottedBrownMushroom => return 5285,
        Block::PottedDeadBush => return 5286,
        Block::PottedCactus => return 5287,
        Block::Carrots(data) => {
            if data.age == 0 {
                return 5288;
            }
            if data.age == 1 {
                return 5289;
            }
            if data.age == 2 {
                return 5290;
            }
            if data.age == 3 {
                return 5291;
            }
            if data.age == 4 {
                return 5292;
            }
            if data.age == 5 {
                return 5293;
            }
            if data.age == 6 {
                return 5294;
            }
            if data.age == 7 {
                return 5295;
            }
        }
        Block::Potatoes(data) => {
            if data.age == 0 {
                return 5296;
            }
            if data.age == 1 {
                return 5297;
            }
            if data.age == 2 {
                return 5298;
            }
            if data.age == 3 {
                return 5299;
            }
            if data.age == 4 {
                return 5300;
            }
            if data.age == 5 {
                return 5301;
            }
            if data.age == 6 {
                return 5302;
            }
            if data.age == 7 {
                return 5303;
            }
        }
        Block::OakButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 5304;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 5305;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 5306;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 5307;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 5308;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 5309;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 5310;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 5311;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 5312;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 5313;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 5314;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 5315;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 5316;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 5317;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 5318;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 5319;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 5320;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 5321;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 5322;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 5323;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 5324;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 5325;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 5326;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 5327;
            }
        }
        Block::SpruceButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 5328;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 5329;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 5330;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 5331;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 5332;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 5333;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 5334;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 5335;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 5336;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 5337;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 5338;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 5339;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 5340;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 5341;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 5342;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 5343;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 5344;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 5345;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 5346;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 5347;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 5348;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 5349;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 5350;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 5351;
            }
        }
        Block::BirchButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 5352;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 5353;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 5354;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 5355;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 5356;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 5357;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 5358;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 5359;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 5360;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 5361;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 5362;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 5363;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 5364;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 5365;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 5366;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 5367;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 5368;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 5369;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 5370;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 5371;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 5372;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 5373;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 5374;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 5375;
            }
        }
        Block::JungleButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 5376;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 5377;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 5378;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 5379;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 5380;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 5381;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 5382;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 5383;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 5384;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 5385;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 5386;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 5387;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 5388;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 5389;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 5390;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 5391;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 5392;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 5393;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 5394;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 5395;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 5396;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 5397;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 5398;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 5399;
            }
        }
        Block::AcaciaButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 5400;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 5401;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 5402;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 5403;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 5404;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 5405;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 5406;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 5407;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 5408;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 5409;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 5410;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 5411;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 5412;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 5413;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 5414;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 5415;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 5416;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 5417;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 5418;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 5419;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 5420;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 5421;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 5422;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 5423;
            }
        }
        Block::DarkOakButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 5424;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 5425;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 5426;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 5427;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 5428;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 5429;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 5430;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 5431;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 5432;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 5433;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 5434;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 5435;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 5436;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 5437;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 5438;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 5439;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 5440;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 5441;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 5442;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 5443;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 5444;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 5445;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 5446;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 5447;
            }
        }
        Block::SkeletonWallSkull(data) => {
            if data.facing == Facing::North {
                return 5448;
            }
            if data.facing == Facing::South {
                return 5449;
            }
            if data.facing == Facing::West {
                return 5450;
            }
            if data.facing == Facing::East {
                return 5451;
            }
        }
        Block::SkeletonSkull(data) => {
            if data.rotation == 0 {
                return 5452;
            }
            if data.rotation == 1 {
                return 5453;
            }
            if data.rotation == 2 {
                return 5454;
            }
            if data.rotation == 3 {
                return 5455;
            }
            if data.rotation == 4 {
                return 5456;
            }
            if data.rotation == 5 {
                return 5457;
            }
            if data.rotation == 6 {
                return 5458;
            }
            if data.rotation == 7 {
                return 5459;
            }
            if data.rotation == 8 {
                return 5460;
            }
            if data.rotation == 9 {
                return 5461;
            }
            if data.rotation == 10 {
                return 5462;
            }
            if data.rotation == 11 {
                return 5463;
            }
            if data.rotation == 12 {
                return 5464;
            }
            if data.rotation == 13 {
                return 5465;
            }
            if data.rotation == 14 {
                return 5466;
            }
            if data.rotation == 15 {
                return 5467;
            }
        }
        Block::WitherSkeletonWallSkull(data) => {
            if data.facing == Facing::North {
                return 5468;
            }
            if data.facing == Facing::South {
                return 5469;
            }
            if data.facing == Facing::West {
                return 5470;
            }
            if data.facing == Facing::East {
                return 5471;
            }
        }
        Block::WitherSkeletonSkull(data) => {
            if data.rotation == 0 {
                return 5472;
            }
            if data.rotation == 1 {
                return 5473;
            }
            if data.rotation == 2 {
                return 5474;
            }
            if data.rotation == 3 {
                return 5475;
            }
            if data.rotation == 4 {
                return 5476;
            }
            if data.rotation == 5 {
                return 5477;
            }
            if data.rotation == 6 {
                return 5478;
            }
            if data.rotation == 7 {
                return 5479;
            }
            if data.rotation == 8 {
                return 5480;
            }
            if data.rotation == 9 {
                return 5481;
            }
            if data.rotation == 10 {
                return 5482;
            }
            if data.rotation == 11 {
                return 5483;
            }
            if data.rotation == 12 {
                return 5484;
            }
            if data.rotation == 13 {
                return 5485;
            }
            if data.rotation == 14 {
                return 5486;
            }
            if data.rotation == 15 {
                return 5487;
            }
        }
        Block::ZombieWallHead(data) => {
            if data.facing == Facing::North {
                return 5488;
            }
            if data.facing == Facing::South {
                return 5489;
            }
            if data.facing == Facing::West {
                return 5490;
            }
            if data.facing == Facing::East {
                return 5491;
            }
        }
        Block::ZombieHead(data) => {
            if data.rotation == 0 {
                return 5492;
            }
            if data.rotation == 1 {
                return 5493;
            }
            if data.rotation == 2 {
                return 5494;
            }
            if data.rotation == 3 {
                return 5495;
            }
            if data.rotation == 4 {
                return 5496;
            }
            if data.rotation == 5 {
                return 5497;
            }
            if data.rotation == 6 {
                return 5498;
            }
            if data.rotation == 7 {
                return 5499;
            }
            if data.rotation == 8 {
                return 5500;
            }
            if data.rotation == 9 {
                return 5501;
            }
            if data.rotation == 10 {
                return 5502;
            }
            if data.rotation == 11 {
                return 5503;
            }
            if data.rotation == 12 {
                return 5504;
            }
            if data.rotation == 13 {
                return 5505;
            }
            if data.rotation == 14 {
                return 5506;
            }
            if data.rotation == 15 {
                return 5507;
            }
        }
        Block::PlayerWallHead(data) => {
            if data.facing == Facing::North {
                return 5508;
            }
            if data.facing == Facing::South {
                return 5509;
            }
            if data.facing == Facing::West {
                return 5510;
            }
            if data.facing == Facing::East {
                return 5511;
            }
        }
        Block::PlayerHead(data) => {
            if data.rotation == 0 {
                return 5512;
            }
            if data.rotation == 1 {
                return 5513;
            }
            if data.rotation == 2 {
                return 5514;
            }
            if data.rotation == 3 {
                return 5515;
            }
            if data.rotation == 4 {
                return 5516;
            }
            if data.rotation == 5 {
                return 5517;
            }
            if data.rotation == 6 {
                return 5518;
            }
            if data.rotation == 7 {
                return 5519;
            }
            if data.rotation == 8 {
                return 5520;
            }
            if data.rotation == 9 {
                return 5521;
            }
            if data.rotation == 10 {
                return 5522;
            }
            if data.rotation == 11 {
                return 5523;
            }
            if data.rotation == 12 {
                return 5524;
            }
            if data.rotation == 13 {
                return 5525;
            }
            if data.rotation == 14 {
                return 5526;
            }
            if data.rotation == 15 {
                return 5527;
            }
        }
        Block::CreeperWallHead(data) => {
            if data.facing == Facing::North {
                return 5528;
            }
            if data.facing == Facing::South {
                return 5529;
            }
            if data.facing == Facing::West {
                return 5530;
            }
            if data.facing == Facing::East {
                return 5531;
            }
        }
        Block::CreeperHead(data) => {
            if data.rotation == 0 {
                return 5532;
            }
            if data.rotation == 1 {
                return 5533;
            }
            if data.rotation == 2 {
                return 5534;
            }
            if data.rotation == 3 {
                return 5535;
            }
            if data.rotation == 4 {
                return 5536;
            }
            if data.rotation == 5 {
                return 5537;
            }
            if data.rotation == 6 {
                return 5538;
            }
            if data.rotation == 7 {
                return 5539;
            }
            if data.rotation == 8 {
                return 5540;
            }
            if data.rotation == 9 {
                return 5541;
            }
            if data.rotation == 10 {
                return 5542;
            }
            if data.rotation == 11 {
                return 5543;
            }
            if data.rotation == 12 {
                return 5544;
            }
            if data.rotation == 13 {
                return 5545;
            }
            if data.rotation == 14 {
                return 5546;
            }
            if data.rotation == 15 {
                return 5547;
            }
        }
        Block::DragonWallHead(data) => {
            if data.facing == Facing::North {
                return 5548;
            }
            if data.facing == Facing::South {
                return 5549;
            }
            if data.facing == Facing::West {
                return 5550;
            }
            if data.facing == Facing::East {
                return 5551;
            }
        }
        Block::DragonHead(data) => {
            if data.rotation == 0 {
                return 5552;
            }
            if data.rotation == 1 {
                return 5553;
            }
            if data.rotation == 2 {
                return 5554;
            }
            if data.rotation == 3 {
                return 5555;
            }
            if data.rotation == 4 {
                return 5556;
            }
            if data.rotation == 5 {
                return 5557;
            }
            if data.rotation == 6 {
                return 5558;
            }
            if data.rotation == 7 {
                return 5559;
            }
            if data.rotation == 8 {
                return 5560;
            }
            if data.rotation == 9 {
                return 5561;
            }
            if data.rotation == 10 {
                return 5562;
            }
            if data.rotation == 11 {
                return 5563;
            }
            if data.rotation == 12 {
                return 5564;
            }
            if data.rotation == 13 {
                return 5565;
            }
            if data.rotation == 14 {
                return 5566;
            }
            if data.rotation == 15 {
                return 5567;
            }
        }
        Block::Anvil(data) => {
            if data.facing == Facing::North {
                return 5568;
            }
            if data.facing == Facing::South {
                return 5569;
            }
            if data.facing == Facing::West {
                return 5570;
            }
            if data.facing == Facing::East {
                return 5571;
            }
        }
        Block::ChippedAnvil(data) => {
            if data.facing == Facing::North {
                return 5572;
            }
            if data.facing == Facing::South {
                return 5573;
            }
            if data.facing == Facing::West {
                return 5574;
            }
            if data.facing == Facing::East {
                return 5575;
            }
        }
        Block::DamagedAnvil(data) => {
            if data.facing == Facing::North {
                return 5576;
            }
            if data.facing == Facing::South {
                return 5577;
            }
            if data.facing == Facing::West {
                return 5578;
            }
            if data.facing == Facing::East {
                return 5579;
            }
        }
        Block::TrappedChest(data) => {
            if data.facing == Facing::North
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == true
            {
                return 5580;
            }
            if data.facing == Facing::North
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == false
            {
                return 5581;
            }
            if data.facing == Facing::North
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == true
            {
                return 5582;
            }
            if data.facing == Facing::North
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == false
            {
                return 5583;
            }
            if data.facing == Facing::North
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == true
            {
                return 5584;
            }
            if data.facing == Facing::North
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == false
            {
                return 5585;
            }
            if data.facing == Facing::South
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == true
            {
                return 5586;
            }
            if data.facing == Facing::South
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == false
            {
                return 5587;
            }
            if data.facing == Facing::South
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == true
            {
                return 5588;
            }
            if data.facing == Facing::South
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == false
            {
                return 5589;
            }
            if data.facing == Facing::South
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == true
            {
                return 5590;
            }
            if data.facing == Facing::South
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == false
            {
                return 5591;
            }
            if data.facing == Facing::West
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == true
            {
                return 5592;
            }
            if data.facing == Facing::West
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == false
            {
                return 5593;
            }
            if data.facing == Facing::West
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == true
            {
                return 5594;
            }
            if data.facing == Facing::West
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == false
            {
                return 5595;
            }
            if data.facing == Facing::West
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == true
            {
                return 5596;
            }
            if data.facing == Facing::West
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == false
            {
                return 5597;
            }
            if data.facing == Facing::East
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == true
            {
                return 5598;
            }
            if data.facing == Facing::East
                && data.type_ == TrappedChestType::Single
                && data.waterlogged == false
            {
                return 5599;
            }
            if data.facing == Facing::East
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == true
            {
                return 5600;
            }
            if data.facing == Facing::East
                && data.type_ == TrappedChestType::Left
                && data.waterlogged == false
            {
                return 5601;
            }
            if data.facing == Facing::East
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == true
            {
                return 5602;
            }
            if data.facing == Facing::East
                && data.type_ == TrappedChestType::Right
                && data.waterlogged == false
            {
                return 5603;
            }
        }
        Block::LightWeightedPressurePlate(data) => {
            if data.power == 0 {
                return 5604;
            }
            if data.power == 1 {
                return 5605;
            }
            if data.power == 2 {
                return 5606;
            }
            if data.power == 3 {
                return 5607;
            }
            if data.power == 4 {
                return 5608;
            }
            if data.power == 5 {
                return 5609;
            }
            if data.power == 6 {
                return 5610;
            }
            if data.power == 7 {
                return 5611;
            }
            if data.power == 8 {
                return 5612;
            }
            if data.power == 9 {
                return 5613;
            }
            if data.power == 10 {
                return 5614;
            }
            if data.power == 11 {
                return 5615;
            }
            if data.power == 12 {
                return 5616;
            }
            if data.power == 13 {
                return 5617;
            }
            if data.power == 14 {
                return 5618;
            }
            if data.power == 15 {
                return 5619;
            }
        }
        Block::HeavyWeightedPressurePlate(data) => {
            if data.power == 0 {
                return 5620;
            }
            if data.power == 1 {
                return 5621;
            }
            if data.power == 2 {
                return 5622;
            }
            if data.power == 3 {
                return 5623;
            }
            if data.power == 4 {
                return 5624;
            }
            if data.power == 5 {
                return 5625;
            }
            if data.power == 6 {
                return 5626;
            }
            if data.power == 7 {
                return 5627;
            }
            if data.power == 8 {
                return 5628;
            }
            if data.power == 9 {
                return 5629;
            }
            if data.power == 10 {
                return 5630;
            }
            if data.power == 11 {
                return 5631;
            }
            if data.power == 12 {
                return 5632;
            }
            if data.power == 13 {
                return 5633;
            }
            if data.power == 14 {
                return 5634;
            }
            if data.power == 15 {
                return 5635;
            }
        }
        Block::Comparator(data) => {
            if data.facing == Facing::North
                && data.mode == ComparatorMode::Compare
                && data.powered == true
            {
                return 5636;
            }
            if data.facing == Facing::North
                && data.mode == ComparatorMode::Compare
                && data.powered == false
            {
                return 5637;
            }
            if data.facing == Facing::North
                && data.mode == ComparatorMode::Subtract
                && data.powered == true
            {
                return 5638;
            }
            if data.facing == Facing::North
                && data.mode == ComparatorMode::Subtract
                && data.powered == false
            {
                return 5639;
            }
            if data.facing == Facing::South
                && data.mode == ComparatorMode::Compare
                && data.powered == true
            {
                return 5640;
            }
            if data.facing == Facing::South
                && data.mode == ComparatorMode::Compare
                && data.powered == false
            {
                return 5641;
            }
            if data.facing == Facing::South
                && data.mode == ComparatorMode::Subtract
                && data.powered == true
            {
                return 5642;
            }
            if data.facing == Facing::South
                && data.mode == ComparatorMode::Subtract
                && data.powered == false
            {
                return 5643;
            }
            if data.facing == Facing::West
                && data.mode == ComparatorMode::Compare
                && data.powered == true
            {
                return 5644;
            }
            if data.facing == Facing::West
                && data.mode == ComparatorMode::Compare
                && data.powered == false
            {
                return 5645;
            }
            if data.facing == Facing::West
                && data.mode == ComparatorMode::Subtract
                && data.powered == true
            {
                return 5646;
            }
            if data.facing == Facing::West
                && data.mode == ComparatorMode::Subtract
                && data.powered == false
            {
                return 5647;
            }
            if data.facing == Facing::East
                && data.mode == ComparatorMode::Compare
                && data.powered == true
            {
                return 5648;
            }
            if data.facing == Facing::East
                && data.mode == ComparatorMode::Compare
                && data.powered == false
            {
                return 5649;
            }
            if data.facing == Facing::East
                && data.mode == ComparatorMode::Subtract
                && data.powered == true
            {
                return 5650;
            }
            if data.facing == Facing::East
                && data.mode == ComparatorMode::Subtract
                && data.powered == false
            {
                return 5651;
            }
        }
        Block::DaylightDetector(data) => {
            if data.inverted == true && data.power == 0 {
                return 5652;
            }
            if data.inverted == true && data.power == 1 {
                return 5653;
            }
            if data.inverted == true && data.power == 2 {
                return 5654;
            }
            if data.inverted == true && data.power == 3 {
                return 5655;
            }
            if data.inverted == true && data.power == 4 {
                return 5656;
            }
            if data.inverted == true && data.power == 5 {
                return 5657;
            }
            if data.inverted == true && data.power == 6 {
                return 5658;
            }
            if data.inverted == true && data.power == 7 {
                return 5659;
            }
            if data.inverted == true && data.power == 8 {
                return 5660;
            }
            if data.inverted == true && data.power == 9 {
                return 5661;
            }
            if data.inverted == true && data.power == 10 {
                return 5662;
            }
            if data.inverted == true && data.power == 11 {
                return 5663;
            }
            if data.inverted == true && data.power == 12 {
                return 5664;
            }
            if data.inverted == true && data.power == 13 {
                return 5665;
            }
            if data.inverted == true && data.power == 14 {
                return 5666;
            }
            if data.inverted == true && data.power == 15 {
                return 5667;
            }
            if data.inverted == false && data.power == 0 {
                return 5668;
            }
            if data.inverted == false && data.power == 1 {
                return 5669;
            }
            if data.inverted == false && data.power == 2 {
                return 5670;
            }
            if data.inverted == false && data.power == 3 {
                return 5671;
            }
            if data.inverted == false && data.power == 4 {
                return 5672;
            }
            if data.inverted == false && data.power == 5 {
                return 5673;
            }
            if data.inverted == false && data.power == 6 {
                return 5674;
            }
            if data.inverted == false && data.power == 7 {
                return 5675;
            }
            if data.inverted == false && data.power == 8 {
                return 5676;
            }
            if data.inverted == false && data.power == 9 {
                return 5677;
            }
            if data.inverted == false && data.power == 10 {
                return 5678;
            }
            if data.inverted == false && data.power == 11 {
                return 5679;
            }
            if data.inverted == false && data.power == 12 {
                return 5680;
            }
            if data.inverted == false && data.power == 13 {
                return 5681;
            }
            if data.inverted == false && data.power == 14 {
                return 5682;
            }
            if data.inverted == false && data.power == 15 {
                return 5683;
            }
        }
        Block::RedstoneBlock => return 5684,
        Block::NetherQuartzOre => return 5685,
        Block::Hopper(data) => {
            if data.enabled == true && data.facing == Facing::Down {
                return 5686;
            }
            if data.enabled == true && data.facing == Facing::North {
                return 5687;
            }
            if data.enabled == true && data.facing == Facing::South {
                return 5688;
            }
            if data.enabled == true && data.facing == Facing::West {
                return 5689;
            }
            if data.enabled == true && data.facing == Facing::East {
                return 5690;
            }
            if data.enabled == false && data.facing == Facing::Down {
                return 5691;
            }
            if data.enabled == false && data.facing == Facing::North {
                return 5692;
            }
            if data.enabled == false && data.facing == Facing::South {
                return 5693;
            }
            if data.enabled == false && data.facing == Facing::West {
                return 5694;
            }
            if data.enabled == false && data.facing == Facing::East {
                return 5695;
            }
        }
        Block::QuartzBlock => return 5696,
        Block::ChiseledQuartzBlock => return 5697,
        Block::QuartzPillar(data) => {
            if data.axis == Axis::X {
                return 5698;
            }
            if data.axis == Axis::Y {
                return 5699;
            }
            if data.axis == Axis::Z {
                return 5700;
            }
        }
        Block::QuartzStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5701;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5702;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5703;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5704;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5705;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5706;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5707;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5708;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5709;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5710;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5711;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5712;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5713;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5714;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5715;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5716;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5717;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5718;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5719;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5720;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5721;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5722;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5723;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5724;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5725;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5726;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5727;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5728;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5729;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5730;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5731;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5732;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5733;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5734;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5735;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5736;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5737;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5738;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5739;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5740;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5741;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5742;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5743;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5744;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5745;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5746;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5747;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5748;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5749;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5750;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5751;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5752;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5753;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5754;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5755;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5756;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5757;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5758;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5759;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5760;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5761;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5762;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5763;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5764;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5765;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5766;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5767;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5768;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5769;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5770;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 5771;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 5772;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 5773;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 5774;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 5775;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 5776;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 5777;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 5778;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 5779;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 5780;
            }
        }
        Block::ActivatorRail(data) => {
            if data.powered == true && data.shape == Shape::NorthSouth {
                return 5781;
            }
            if data.powered == true && data.shape == Shape::EastWest {
                return 5782;
            }
            if data.powered == true && data.shape == Shape::AscendingEast {
                return 5783;
            }
            if data.powered == true && data.shape == Shape::AscendingWest {
                return 5784;
            }
            if data.powered == true && data.shape == Shape::AscendingNorth {
                return 5785;
            }
            if data.powered == true && data.shape == Shape::AscendingSouth {
                return 5786;
            }
            if data.powered == false && data.shape == Shape::NorthSouth {
                return 5787;
            }
            if data.powered == false && data.shape == Shape::EastWest {
                return 5788;
            }
            if data.powered == false && data.shape == Shape::AscendingEast {
                return 5789;
            }
            if data.powered == false && data.shape == Shape::AscendingWest {
                return 5790;
            }
            if data.powered == false && data.shape == Shape::AscendingNorth {
                return 5791;
            }
            if data.powered == false && data.shape == Shape::AscendingSouth {
                return 5792;
            }
        }
        Block::Dropper(data) => {
            if data.facing == Facing::North && data.triggered == true {
                return 5793;
            }
            if data.facing == Facing::North && data.triggered == false {
                return 5794;
            }
            if data.facing == Facing::East && data.triggered == true {
                return 5795;
            }
            if data.facing == Facing::East && data.triggered == false {
                return 5796;
            }
            if data.facing == Facing::South && data.triggered == true {
                return 5797;
            }
            if data.facing == Facing::South && data.triggered == false {
                return 5798;
            }
            if data.facing == Facing::West && data.triggered == true {
                return 5799;
            }
            if data.facing == Facing::West && data.triggered == false {
                return 5800;
            }
            if data.facing == Facing::Up && data.triggered == true {
                return 5801;
            }
            if data.facing == Facing::Up && data.triggered == false {
                return 5802;
            }
            if data.facing == Facing::Down && data.triggered == true {
                return 5803;
            }
            if data.facing == Facing::Down && data.triggered == false {
                return 5804;
            }
        }
        Block::WhiteTerracotta => return 5805,
        Block::OrangeTerracotta => return 5806,
        Block::MagentaTerracotta => return 5807,
        Block::LightBlueTerracotta => return 5808,
        Block::YellowTerracotta => return 5809,
        Block::LimeTerracotta => return 5810,
        Block::PinkTerracotta => return 5811,
        Block::GrayTerracotta => return 5812,
        Block::LightGrayTerracotta => return 5813,
        Block::CyanTerracotta => return 5814,
        Block::PurpleTerracotta => return 5815,
        Block::BlueTerracotta => return 5816,
        Block::BrownTerracotta => return 5817,
        Block::GreenTerracotta => return 5818,
        Block::RedTerracotta => return 5819,
        Block::BlackTerracotta => return 5820,
        Block::WhiteStainedGlassPane(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5821;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5822;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5823;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5824;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5825;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5826;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5827;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5828;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5829;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5830;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5831;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5832;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5833;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5834;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5835;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5836;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5837;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5838;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5839;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5840;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5841;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5842;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5843;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5844;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5845;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5846;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5847;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5848;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5849;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5850;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5851;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5852;
            }
        }
        Block::OrangeStainedGlassPane(data) => {
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5853;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5854;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5855;
            }
            if data.east == true
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5856;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5857;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5858;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5859;
            }
            if data.east == true
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5860;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5861;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5862;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5863;
            }
            if data.east == true
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5864;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5865;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5866;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5867;
            }
            if data.east == true
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5868;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5869;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5870;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5871;
            }
            if data.east == false
                && data.north == true
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5872;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5873;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5874;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5875;
            }
            if data.east == false
                && data.north == true
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5876;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == true
            {
                return 5877;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == true
                && data.west == false
            {
                return 5878;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == true
            {
                return 5879;
            }
            if data.east == false
                && data.north == false
                && data.south == true
                && data.waterlogged == false
                && data.west == false
            {
                return 5880;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == true
            {
                return 5881;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == true
                && data.west == false
            {
                return 5882;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == true
            {
                return 5883;
            }
            if data.east == false
                && data.north == false
                && data.south == false
                && data.waterlogged == false
                && data.west == false
            {
                return 5884;
            }
        }
        _ => return NOT_FOUND,
    }

    NOT_FOUND
}
