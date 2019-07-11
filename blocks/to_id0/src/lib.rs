use feather_blocks_enum::*;

const NOT_FOUND: u16 = 65535;

pub fn to_id(block: Block) -> u16 {
    match block {
        Block::Air => return 0,
        Block::Stone => return 1,
        Block::Granite => return 2,
        Block::PolishedGranite => return 3,
        Block::Diorite => return 4,
        Block::PolishedDiorite => return 5,
        Block::Andesite => return 6,
        Block::PolishedAndesite => return 7,
        Block::GrassBlock(data) => {
            if data.snowy == true {
                return 8;
            }
            if data.snowy == false {
                return 9;
            }
        }
        Block::Dirt => return 10,
        Block::CoarseDirt => return 11,
        Block::Podzol(data) => {
            if data.snowy == true {
                return 12;
            }
            if data.snowy == false {
                return 13;
            }
        }
        Block::Cobblestone => return 14,
        Block::OakPlanks => return 15,
        Block::SprucePlanks => return 16,
        Block::BirchPlanks => return 17,
        Block::JunglePlanks => return 18,
        Block::AcaciaPlanks => return 19,
        Block::DarkOakPlanks => return 20,
        Block::OakSapling(data) => {
            if data.stage == 0 {
                return 21;
            }
            if data.stage == 1 {
                return 22;
            }
        }
        Block::SpruceSapling(data) => {
            if data.stage == 0 {
                return 23;
            }
            if data.stage == 1 {
                return 24;
            }
        }
        Block::BirchSapling(data) => {
            if data.stage == 0 {
                return 25;
            }
            if data.stage == 1 {
                return 26;
            }
        }
        Block::JungleSapling(data) => {
            if data.stage == 0 {
                return 27;
            }
            if data.stage == 1 {
                return 28;
            }
        }
        Block::AcaciaSapling(data) => {
            if data.stage == 0 {
                return 29;
            }
            if data.stage == 1 {
                return 30;
            }
        }
        Block::DarkOakSapling(data) => {
            if data.stage == 0 {
                return 31;
            }
            if data.stage == 1 {
                return 32;
            }
        }
        Block::Bedrock => return 33,
        Block::Water(data) => {
            if data.level == 0 {
                return 34;
            }
            if data.level == 1 {
                return 35;
            }
            if data.level == 2 {
                return 36;
            }
            if data.level == 3 {
                return 37;
            }
            if data.level == 4 {
                return 38;
            }
            if data.level == 5 {
                return 39;
            }
            if data.level == 6 {
                return 40;
            }
            if data.level == 7 {
                return 41;
            }
            if data.level == 8 {
                return 42;
            }
            if data.level == 9 {
                return 43;
            }
            if data.level == 10 {
                return 44;
            }
            if data.level == 11 {
                return 45;
            }
            if data.level == 12 {
                return 46;
            }
            if data.level == 13 {
                return 47;
            }
            if data.level == 14 {
                return 48;
            }
            if data.level == 15 {
                return 49;
            }
        }
        Block::Lava(data) => {
            if data.level == 0 {
                return 50;
            }
            if data.level == 1 {
                return 51;
            }
            if data.level == 2 {
                return 52;
            }
            if data.level == 3 {
                return 53;
            }
            if data.level == 4 {
                return 54;
            }
            if data.level == 5 {
                return 55;
            }
            if data.level == 6 {
                return 56;
            }
            if data.level == 7 {
                return 57;
            }
            if data.level == 8 {
                return 58;
            }
            if data.level == 9 {
                return 59;
            }
            if data.level == 10 {
                return 60;
            }
            if data.level == 11 {
                return 61;
            }
            if data.level == 12 {
                return 62;
            }
            if data.level == 13 {
                return 63;
            }
            if data.level == 14 {
                return 64;
            }
            if data.level == 15 {
                return 65;
            }
        }
        Block::Sand => return 66,
        Block::RedSand => return 67,
        Block::Gravel => return 68,
        Block::GoldOre => return 69,
        Block::IronOre => return 70,
        Block::CoalOre => return 71,
        Block::OakLog(data) => {
            if data.axis == Axis::X {
                return 72;
            }
            if data.axis == Axis::Y {
                return 73;
            }
            if data.axis == Axis::Z {
                return 74;
            }
        }
        Block::SpruceLog(data) => {
            if data.axis == Axis::X {
                return 75;
            }
            if data.axis == Axis::Y {
                return 76;
            }
            if data.axis == Axis::Z {
                return 77;
            }
        }
        Block::BirchLog(data) => {
            if data.axis == Axis::X {
                return 78;
            }
            if data.axis == Axis::Y {
                return 79;
            }
            if data.axis == Axis::Z {
                return 80;
            }
        }
        Block::JungleLog(data) => {
            if data.axis == Axis::X {
                return 81;
            }
            if data.axis == Axis::Y {
                return 82;
            }
            if data.axis == Axis::Z {
                return 83;
            }
        }
        Block::AcaciaLog(data) => {
            if data.axis == Axis::X {
                return 84;
            }
            if data.axis == Axis::Y {
                return 85;
            }
            if data.axis == Axis::Z {
                return 86;
            }
        }
        Block::DarkOakLog(data) => {
            if data.axis == Axis::X {
                return 87;
            }
            if data.axis == Axis::Y {
                return 88;
            }
            if data.axis == Axis::Z {
                return 89;
            }
        }
        Block::StrippedSpruceLog(data) => {
            if data.axis == Axis::X {
                return 90;
            }
            if data.axis == Axis::Y {
                return 91;
            }
            if data.axis == Axis::Z {
                return 92;
            }
        }
        Block::StrippedBirchLog(data) => {
            if data.axis == Axis::X {
                return 93;
            }
            if data.axis == Axis::Y {
                return 94;
            }
            if data.axis == Axis::Z {
                return 95;
            }
        }
        Block::StrippedJungleLog(data) => {
            if data.axis == Axis::X {
                return 96;
            }
            if data.axis == Axis::Y {
                return 97;
            }
            if data.axis == Axis::Z {
                return 98;
            }
        }
        Block::StrippedAcaciaLog(data) => {
            if data.axis == Axis::X {
                return 99;
            }
            if data.axis == Axis::Y {
                return 100;
            }
            if data.axis == Axis::Z {
                return 101;
            }
        }
        Block::StrippedDarkOakLog(data) => {
            if data.axis == Axis::X {
                return 102;
            }
            if data.axis == Axis::Y {
                return 103;
            }
            if data.axis == Axis::Z {
                return 104;
            }
        }
        Block::StrippedOakLog(data) => {
            if data.axis == Axis::X {
                return 105;
            }
            if data.axis == Axis::Y {
                return 106;
            }
            if data.axis == Axis::Z {
                return 107;
            }
        }
        Block::OakWood(data) => {
            if data.axis == Axis::X {
                return 108;
            }
            if data.axis == Axis::Y {
                return 109;
            }
            if data.axis == Axis::Z {
                return 110;
            }
        }
        Block::SpruceWood(data) => {
            if data.axis == Axis::X {
                return 111;
            }
            if data.axis == Axis::Y {
                return 112;
            }
            if data.axis == Axis::Z {
                return 113;
            }
        }
        Block::BirchWood(data) => {
            if data.axis == Axis::X {
                return 114;
            }
            if data.axis == Axis::Y {
                return 115;
            }
            if data.axis == Axis::Z {
                return 116;
            }
        }
        Block::JungleWood(data) => {
            if data.axis == Axis::X {
                return 117;
            }
            if data.axis == Axis::Y {
                return 118;
            }
            if data.axis == Axis::Z {
                return 119;
            }
        }
        Block::AcaciaWood(data) => {
            if data.axis == Axis::X {
                return 120;
            }
            if data.axis == Axis::Y {
                return 121;
            }
            if data.axis == Axis::Z {
                return 122;
            }
        }
        Block::DarkOakWood(data) => {
            if data.axis == Axis::X {
                return 123;
            }
            if data.axis == Axis::Y {
                return 124;
            }
            if data.axis == Axis::Z {
                return 125;
            }
        }
        Block::StrippedOakWood(data) => {
            if data.axis == Axis::X {
                return 126;
            }
            if data.axis == Axis::Y {
                return 127;
            }
            if data.axis == Axis::Z {
                return 128;
            }
        }
        Block::StrippedSpruceWood(data) => {
            if data.axis == Axis::X {
                return 129;
            }
            if data.axis == Axis::Y {
                return 130;
            }
            if data.axis == Axis::Z {
                return 131;
            }
        }
        Block::StrippedBirchWood(data) => {
            if data.axis == Axis::X {
                return 132;
            }
            if data.axis == Axis::Y {
                return 133;
            }
            if data.axis == Axis::Z {
                return 134;
            }
        }
        Block::StrippedJungleWood(data) => {
            if data.axis == Axis::X {
                return 135;
            }
            if data.axis == Axis::Y {
                return 136;
            }
            if data.axis == Axis::Z {
                return 137;
            }
        }
        Block::StrippedAcaciaWood(data) => {
            if data.axis == Axis::X {
                return 138;
            }
            if data.axis == Axis::Y {
                return 139;
            }
            if data.axis == Axis::Z {
                return 140;
            }
        }
        Block::StrippedDarkOakWood(data) => {
            if data.axis == Axis::X {
                return 141;
            }
            if data.axis == Axis::Y {
                return 142;
            }
            if data.axis == Axis::Z {
                return 143;
            }
        }
        Block::OakLeaves(data) => {
            if data.distance == 1 && data.persistent == true {
                return 144;
            }
            if data.distance == 1 && data.persistent == false {
                return 145;
            }
            if data.distance == 2 && data.persistent == true {
                return 146;
            }
            if data.distance == 2 && data.persistent == false {
                return 147;
            }
            if data.distance == 3 && data.persistent == true {
                return 148;
            }
            if data.distance == 3 && data.persistent == false {
                return 149;
            }
            if data.distance == 4 && data.persistent == true {
                return 150;
            }
            if data.distance == 4 && data.persistent == false {
                return 151;
            }
            if data.distance == 5 && data.persistent == true {
                return 152;
            }
            if data.distance == 5 && data.persistent == false {
                return 153;
            }
            if data.distance == 6 && data.persistent == true {
                return 154;
            }
            if data.distance == 6 && data.persistent == false {
                return 155;
            }
            if data.distance == 7 && data.persistent == true {
                return 156;
            }
            if data.distance == 7 && data.persistent == false {
                return 157;
            }
        }
        Block::SpruceLeaves(data) => {
            if data.distance == 1 && data.persistent == true {
                return 158;
            }
            if data.distance == 1 && data.persistent == false {
                return 159;
            }
            if data.distance == 2 && data.persistent == true {
                return 160;
            }
            if data.distance == 2 && data.persistent == false {
                return 161;
            }
            if data.distance == 3 && data.persistent == true {
                return 162;
            }
            if data.distance == 3 && data.persistent == false {
                return 163;
            }
            if data.distance == 4 && data.persistent == true {
                return 164;
            }
            if data.distance == 4 && data.persistent == false {
                return 165;
            }
            if data.distance == 5 && data.persistent == true {
                return 166;
            }
            if data.distance == 5 && data.persistent == false {
                return 167;
            }
            if data.distance == 6 && data.persistent == true {
                return 168;
            }
            if data.distance == 6 && data.persistent == false {
                return 169;
            }
            if data.distance == 7 && data.persistent == true {
                return 170;
            }
            if data.distance == 7 && data.persistent == false {
                return 171;
            }
        }
        Block::BirchLeaves(data) => {
            if data.distance == 1 && data.persistent == true {
                return 172;
            }
            if data.distance == 1 && data.persistent == false {
                return 173;
            }
            if data.distance == 2 && data.persistent == true {
                return 174;
            }
            if data.distance == 2 && data.persistent == false {
                return 175;
            }
            if data.distance == 3 && data.persistent == true {
                return 176;
            }
            if data.distance == 3 && data.persistent == false {
                return 177;
            }
            if data.distance == 4 && data.persistent == true {
                return 178;
            }
            if data.distance == 4 && data.persistent == false {
                return 179;
            }
            if data.distance == 5 && data.persistent == true {
                return 180;
            }
            if data.distance == 5 && data.persistent == false {
                return 181;
            }
            if data.distance == 6 && data.persistent == true {
                return 182;
            }
            if data.distance == 6 && data.persistent == false {
                return 183;
            }
            if data.distance == 7 && data.persistent == true {
                return 184;
            }
            if data.distance == 7 && data.persistent == false {
                return 185;
            }
        }
        Block::JungleLeaves(data) => {
            if data.distance == 1 && data.persistent == true {
                return 186;
            }
            if data.distance == 1 && data.persistent == false {
                return 187;
            }
            if data.distance == 2 && data.persistent == true {
                return 188;
            }
            if data.distance == 2 && data.persistent == false {
                return 189;
            }
            if data.distance == 3 && data.persistent == true {
                return 190;
            }
            if data.distance == 3 && data.persistent == false {
                return 191;
            }
            if data.distance == 4 && data.persistent == true {
                return 192;
            }
            if data.distance == 4 && data.persistent == false {
                return 193;
            }
            if data.distance == 5 && data.persistent == true {
                return 194;
            }
            if data.distance == 5 && data.persistent == false {
                return 195;
            }
            if data.distance == 6 && data.persistent == true {
                return 196;
            }
            if data.distance == 6 && data.persistent == false {
                return 197;
            }
            if data.distance == 7 && data.persistent == true {
                return 198;
            }
            if data.distance == 7 && data.persistent == false {
                return 199;
            }
        }
        Block::AcaciaLeaves(data) => {
            if data.distance == 1 && data.persistent == true {
                return 200;
            }
            if data.distance == 1 && data.persistent == false {
                return 201;
            }
            if data.distance == 2 && data.persistent == true {
                return 202;
            }
            if data.distance == 2 && data.persistent == false {
                return 203;
            }
            if data.distance == 3 && data.persistent == true {
                return 204;
            }
            if data.distance == 3 && data.persistent == false {
                return 205;
            }
            if data.distance == 4 && data.persistent == true {
                return 206;
            }
            if data.distance == 4 && data.persistent == false {
                return 207;
            }
            if data.distance == 5 && data.persistent == true {
                return 208;
            }
            if data.distance == 5 && data.persistent == false {
                return 209;
            }
            if data.distance == 6 && data.persistent == true {
                return 210;
            }
            if data.distance == 6 && data.persistent == false {
                return 211;
            }
            if data.distance == 7 && data.persistent == true {
                return 212;
            }
            if data.distance == 7 && data.persistent == false {
                return 213;
            }
        }
        Block::DarkOakLeaves(data) => {
            if data.distance == 1 && data.persistent == true {
                return 214;
            }
            if data.distance == 1 && data.persistent == false {
                return 215;
            }
            if data.distance == 2 && data.persistent == true {
                return 216;
            }
            if data.distance == 2 && data.persistent == false {
                return 217;
            }
            if data.distance == 3 && data.persistent == true {
                return 218;
            }
            if data.distance == 3 && data.persistent == false {
                return 219;
            }
            if data.distance == 4 && data.persistent == true {
                return 220;
            }
            if data.distance == 4 && data.persistent == false {
                return 221;
            }
            if data.distance == 5 && data.persistent == true {
                return 222;
            }
            if data.distance == 5 && data.persistent == false {
                return 223;
            }
            if data.distance == 6 && data.persistent == true {
                return 224;
            }
            if data.distance == 6 && data.persistent == false {
                return 225;
            }
            if data.distance == 7 && data.persistent == true {
                return 226;
            }
            if data.distance == 7 && data.persistent == false {
                return 227;
            }
        }
        Block::Sponge => return 228,
        Block::WetSponge => return 229,
        Block::Glass => return 230,
        Block::LapisOre => return 231,
        Block::LapisBlock => return 232,
        Block::Dispenser(data) => {
            if data.facing == Facing::North && data.triggered == true {
                return 233;
            }
            if data.facing == Facing::North && data.triggered == false {
                return 234;
            }
            if data.facing == Facing::East && data.triggered == true {
                return 235;
            }
            if data.facing == Facing::East && data.triggered == false {
                return 236;
            }
            if data.facing == Facing::South && data.triggered == true {
                return 237;
            }
            if data.facing == Facing::South && data.triggered == false {
                return 238;
            }
            if data.facing == Facing::West && data.triggered == true {
                return 239;
            }
            if data.facing == Facing::West && data.triggered == false {
                return 240;
            }
            if data.facing == Facing::Up && data.triggered == true {
                return 241;
            }
            if data.facing == Facing::Up && data.triggered == false {
                return 242;
            }
            if data.facing == Facing::Down && data.triggered == true {
                return 243;
            }
            if data.facing == Facing::Down && data.triggered == false {
                return 244;
            }
        }
        Block::Sandstone => return 245,
        Block::ChiseledSandstone => return 246,
        Block::CutSandstone => return 247,
        Block::NoteBlock(data) => {
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 0
                && data.powered == true
            {
                return 248;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 0
                && data.powered == false
            {
                return 249;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 1
                && data.powered == true
            {
                return 250;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 1
                && data.powered == false
            {
                return 251;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 2
                && data.powered == true
            {
                return 252;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 2
                && data.powered == false
            {
                return 253;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 3
                && data.powered == true
            {
                return 254;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 3
                && data.powered == false
            {
                return 255;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 4
                && data.powered == true
            {
                return 256;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 4
                && data.powered == false
            {
                return 257;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 5
                && data.powered == true
            {
                return 258;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 5
                && data.powered == false
            {
                return 259;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 6
                && data.powered == true
            {
                return 260;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 6
                && data.powered == false
            {
                return 261;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 7
                && data.powered == true
            {
                return 262;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 7
                && data.powered == false
            {
                return 263;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 8
                && data.powered == true
            {
                return 264;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 8
                && data.powered == false
            {
                return 265;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 9
                && data.powered == true
            {
                return 266;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 9
                && data.powered == false
            {
                return 267;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 10
                && data.powered == true
            {
                return 268;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 10
                && data.powered == false
            {
                return 269;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 11
                && data.powered == true
            {
                return 270;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 11
                && data.powered == false
            {
                return 271;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 12
                && data.powered == true
            {
                return 272;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 12
                && data.powered == false
            {
                return 273;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 13
                && data.powered == true
            {
                return 274;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 13
                && data.powered == false
            {
                return 275;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 14
                && data.powered == true
            {
                return 276;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 14
                && data.powered == false
            {
                return 277;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 15
                && data.powered == true
            {
                return 278;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 15
                && data.powered == false
            {
                return 279;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 16
                && data.powered == true
            {
                return 280;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 16
                && data.powered == false
            {
                return 281;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 17
                && data.powered == true
            {
                return 282;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 17
                && data.powered == false
            {
                return 283;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 18
                && data.powered == true
            {
                return 284;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 18
                && data.powered == false
            {
                return 285;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 19
                && data.powered == true
            {
                return 286;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 19
                && data.powered == false
            {
                return 287;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 20
                && data.powered == true
            {
                return 288;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 20
                && data.powered == false
            {
                return 289;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 21
                && data.powered == true
            {
                return 290;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 21
                && data.powered == false
            {
                return 291;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 22
                && data.powered == true
            {
                return 292;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 22
                && data.powered == false
            {
                return 293;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 23
                && data.powered == true
            {
                return 294;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 23
                && data.powered == false
            {
                return 295;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 24
                && data.powered == true
            {
                return 296;
            }
            if data.instrument == NoteBlockInstrument::Harp
                && data.note == 24
                && data.powered == false
            {
                return 297;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 0
                && data.powered == true
            {
                return 298;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 0
                && data.powered == false
            {
                return 299;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 1
                && data.powered == true
            {
                return 300;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 1
                && data.powered == false
            {
                return 301;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 2
                && data.powered == true
            {
                return 302;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 2
                && data.powered == false
            {
                return 303;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 3
                && data.powered == true
            {
                return 304;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 3
                && data.powered == false
            {
                return 305;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 4
                && data.powered == true
            {
                return 306;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 4
                && data.powered == false
            {
                return 307;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 5
                && data.powered == true
            {
                return 308;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 5
                && data.powered == false
            {
                return 309;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 6
                && data.powered == true
            {
                return 310;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 6
                && data.powered == false
            {
                return 311;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 7
                && data.powered == true
            {
                return 312;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 7
                && data.powered == false
            {
                return 313;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 8
                && data.powered == true
            {
                return 314;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 8
                && data.powered == false
            {
                return 315;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 9
                && data.powered == true
            {
                return 316;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 9
                && data.powered == false
            {
                return 317;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 10
                && data.powered == true
            {
                return 318;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 10
                && data.powered == false
            {
                return 319;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 11
                && data.powered == true
            {
                return 320;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 11
                && data.powered == false
            {
                return 321;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 12
                && data.powered == true
            {
                return 322;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 12
                && data.powered == false
            {
                return 323;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 13
                && data.powered == true
            {
                return 324;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 13
                && data.powered == false
            {
                return 325;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 14
                && data.powered == true
            {
                return 326;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 14
                && data.powered == false
            {
                return 327;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 15
                && data.powered == true
            {
                return 328;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 15
                && data.powered == false
            {
                return 329;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 16
                && data.powered == true
            {
                return 330;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 16
                && data.powered == false
            {
                return 331;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 17
                && data.powered == true
            {
                return 332;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 17
                && data.powered == false
            {
                return 333;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 18
                && data.powered == true
            {
                return 334;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 18
                && data.powered == false
            {
                return 335;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 19
                && data.powered == true
            {
                return 336;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 19
                && data.powered == false
            {
                return 337;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 20
                && data.powered == true
            {
                return 338;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 20
                && data.powered == false
            {
                return 339;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 21
                && data.powered == true
            {
                return 340;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 21
                && data.powered == false
            {
                return 341;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 22
                && data.powered == true
            {
                return 342;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 22
                && data.powered == false
            {
                return 343;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 23
                && data.powered == true
            {
                return 344;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 23
                && data.powered == false
            {
                return 345;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 24
                && data.powered == true
            {
                return 346;
            }
            if data.instrument == NoteBlockInstrument::Basedrum
                && data.note == 24
                && data.powered == false
            {
                return 347;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 0
                && data.powered == true
            {
                return 348;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 0
                && data.powered == false
            {
                return 349;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 1
                && data.powered == true
            {
                return 350;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 1
                && data.powered == false
            {
                return 351;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 2
                && data.powered == true
            {
                return 352;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 2
                && data.powered == false
            {
                return 353;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 3
                && data.powered == true
            {
                return 354;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 3
                && data.powered == false
            {
                return 355;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 4
                && data.powered == true
            {
                return 356;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 4
                && data.powered == false
            {
                return 357;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 5
                && data.powered == true
            {
                return 358;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 5
                && data.powered == false
            {
                return 359;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 6
                && data.powered == true
            {
                return 360;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 6
                && data.powered == false
            {
                return 361;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 7
                && data.powered == true
            {
                return 362;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 7
                && data.powered == false
            {
                return 363;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 8
                && data.powered == true
            {
                return 364;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 8
                && data.powered == false
            {
                return 365;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 9
                && data.powered == true
            {
                return 366;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 9
                && data.powered == false
            {
                return 367;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 10
                && data.powered == true
            {
                return 368;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 10
                && data.powered == false
            {
                return 369;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 11
                && data.powered == true
            {
                return 370;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 11
                && data.powered == false
            {
                return 371;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 12
                && data.powered == true
            {
                return 372;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 12
                && data.powered == false
            {
                return 373;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 13
                && data.powered == true
            {
                return 374;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 13
                && data.powered == false
            {
                return 375;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 14
                && data.powered == true
            {
                return 376;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 14
                && data.powered == false
            {
                return 377;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 15
                && data.powered == true
            {
                return 378;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 15
                && data.powered == false
            {
                return 379;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 16
                && data.powered == true
            {
                return 380;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 16
                && data.powered == false
            {
                return 381;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 17
                && data.powered == true
            {
                return 382;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 17
                && data.powered == false
            {
                return 383;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 18
                && data.powered == true
            {
                return 384;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 18
                && data.powered == false
            {
                return 385;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 19
                && data.powered == true
            {
                return 386;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 19
                && data.powered == false
            {
                return 387;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 20
                && data.powered == true
            {
                return 388;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 20
                && data.powered == false
            {
                return 389;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 21
                && data.powered == true
            {
                return 390;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 21
                && data.powered == false
            {
                return 391;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 22
                && data.powered == true
            {
                return 392;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 22
                && data.powered == false
            {
                return 393;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 23
                && data.powered == true
            {
                return 394;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 23
                && data.powered == false
            {
                return 395;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 24
                && data.powered == true
            {
                return 396;
            }
            if data.instrument == NoteBlockInstrument::Snare
                && data.note == 24
                && data.powered == false
            {
                return 397;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 0 && data.powered == true
            {
                return 398;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 0
                && data.powered == false
            {
                return 399;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 1 && data.powered == true
            {
                return 400;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 1
                && data.powered == false
            {
                return 401;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 2 && data.powered == true
            {
                return 402;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 2
                && data.powered == false
            {
                return 403;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 3 && data.powered == true
            {
                return 404;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 3
                && data.powered == false
            {
                return 405;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 4 && data.powered == true
            {
                return 406;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 4
                && data.powered == false
            {
                return 407;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 5 && data.powered == true
            {
                return 408;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 5
                && data.powered == false
            {
                return 409;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 6 && data.powered == true
            {
                return 410;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 6
                && data.powered == false
            {
                return 411;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 7 && data.powered == true
            {
                return 412;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 7
                && data.powered == false
            {
                return 413;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 8 && data.powered == true
            {
                return 414;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 8
                && data.powered == false
            {
                return 415;
            }
            if data.instrument == NoteBlockInstrument::Hat && data.note == 9 && data.powered == true
            {
                return 416;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 9
                && data.powered == false
            {
                return 417;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 10
                && data.powered == true
            {
                return 418;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 10
                && data.powered == false
            {
                return 419;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 11
                && data.powered == true
            {
                return 420;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 11
                && data.powered == false
            {
                return 421;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 12
                && data.powered == true
            {
                return 422;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 12
                && data.powered == false
            {
                return 423;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 13
                && data.powered == true
            {
                return 424;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 13
                && data.powered == false
            {
                return 425;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 14
                && data.powered == true
            {
                return 426;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 14
                && data.powered == false
            {
                return 427;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 15
                && data.powered == true
            {
                return 428;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 15
                && data.powered == false
            {
                return 429;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 16
                && data.powered == true
            {
                return 430;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 16
                && data.powered == false
            {
                return 431;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 17
                && data.powered == true
            {
                return 432;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 17
                && data.powered == false
            {
                return 433;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 18
                && data.powered == true
            {
                return 434;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 18
                && data.powered == false
            {
                return 435;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 19
                && data.powered == true
            {
                return 436;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 19
                && data.powered == false
            {
                return 437;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 20
                && data.powered == true
            {
                return 438;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 20
                && data.powered == false
            {
                return 439;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 21
                && data.powered == true
            {
                return 440;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 21
                && data.powered == false
            {
                return 441;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 22
                && data.powered == true
            {
                return 442;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 22
                && data.powered == false
            {
                return 443;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 23
                && data.powered == true
            {
                return 444;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 23
                && data.powered == false
            {
                return 445;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 24
                && data.powered == true
            {
                return 446;
            }
            if data.instrument == NoteBlockInstrument::Hat
                && data.note == 24
                && data.powered == false
            {
                return 447;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 0
                && data.powered == true
            {
                return 448;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 0
                && data.powered == false
            {
                return 449;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 1
                && data.powered == true
            {
                return 450;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 1
                && data.powered == false
            {
                return 451;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 2
                && data.powered == true
            {
                return 452;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 2
                && data.powered == false
            {
                return 453;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 3
                && data.powered == true
            {
                return 454;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 3
                && data.powered == false
            {
                return 455;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 4
                && data.powered == true
            {
                return 456;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 4
                && data.powered == false
            {
                return 457;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 5
                && data.powered == true
            {
                return 458;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 5
                && data.powered == false
            {
                return 459;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 6
                && data.powered == true
            {
                return 460;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 6
                && data.powered == false
            {
                return 461;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 7
                && data.powered == true
            {
                return 462;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 7
                && data.powered == false
            {
                return 463;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 8
                && data.powered == true
            {
                return 464;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 8
                && data.powered == false
            {
                return 465;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 9
                && data.powered == true
            {
                return 466;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 9
                && data.powered == false
            {
                return 467;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 10
                && data.powered == true
            {
                return 468;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 10
                && data.powered == false
            {
                return 469;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 11
                && data.powered == true
            {
                return 470;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 11
                && data.powered == false
            {
                return 471;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 12
                && data.powered == true
            {
                return 472;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 12
                && data.powered == false
            {
                return 473;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 13
                && data.powered == true
            {
                return 474;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 13
                && data.powered == false
            {
                return 475;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 14
                && data.powered == true
            {
                return 476;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 14
                && data.powered == false
            {
                return 477;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 15
                && data.powered == true
            {
                return 478;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 15
                && data.powered == false
            {
                return 479;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 16
                && data.powered == true
            {
                return 480;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 16
                && data.powered == false
            {
                return 481;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 17
                && data.powered == true
            {
                return 482;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 17
                && data.powered == false
            {
                return 483;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 18
                && data.powered == true
            {
                return 484;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 18
                && data.powered == false
            {
                return 485;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 19
                && data.powered == true
            {
                return 486;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 19
                && data.powered == false
            {
                return 487;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 20
                && data.powered == true
            {
                return 488;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 20
                && data.powered == false
            {
                return 489;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 21
                && data.powered == true
            {
                return 490;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 21
                && data.powered == false
            {
                return 491;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 22
                && data.powered == true
            {
                return 492;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 22
                && data.powered == false
            {
                return 493;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 23
                && data.powered == true
            {
                return 494;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 23
                && data.powered == false
            {
                return 495;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 24
                && data.powered == true
            {
                return 496;
            }
            if data.instrument == NoteBlockInstrument::Bass
                && data.note == 24
                && data.powered == false
            {
                return 497;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 0
                && data.powered == true
            {
                return 498;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 0
                && data.powered == false
            {
                return 499;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 1
                && data.powered == true
            {
                return 500;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 1
                && data.powered == false
            {
                return 501;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 2
                && data.powered == true
            {
                return 502;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 2
                && data.powered == false
            {
                return 503;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 3
                && data.powered == true
            {
                return 504;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 3
                && data.powered == false
            {
                return 505;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 4
                && data.powered == true
            {
                return 506;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 4
                && data.powered == false
            {
                return 507;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 5
                && data.powered == true
            {
                return 508;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 5
                && data.powered == false
            {
                return 509;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 6
                && data.powered == true
            {
                return 510;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 6
                && data.powered == false
            {
                return 511;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 7
                && data.powered == true
            {
                return 512;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 7
                && data.powered == false
            {
                return 513;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 8
                && data.powered == true
            {
                return 514;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 8
                && data.powered == false
            {
                return 515;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 9
                && data.powered == true
            {
                return 516;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 9
                && data.powered == false
            {
                return 517;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 10
                && data.powered == true
            {
                return 518;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 10
                && data.powered == false
            {
                return 519;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 11
                && data.powered == true
            {
                return 520;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 11
                && data.powered == false
            {
                return 521;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 12
                && data.powered == true
            {
                return 522;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 12
                && data.powered == false
            {
                return 523;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 13
                && data.powered == true
            {
                return 524;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 13
                && data.powered == false
            {
                return 525;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 14
                && data.powered == true
            {
                return 526;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 14
                && data.powered == false
            {
                return 527;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 15
                && data.powered == true
            {
                return 528;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 15
                && data.powered == false
            {
                return 529;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 16
                && data.powered == true
            {
                return 530;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 16
                && data.powered == false
            {
                return 531;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 17
                && data.powered == true
            {
                return 532;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 17
                && data.powered == false
            {
                return 533;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 18
                && data.powered == true
            {
                return 534;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 18
                && data.powered == false
            {
                return 535;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 19
                && data.powered == true
            {
                return 536;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 19
                && data.powered == false
            {
                return 537;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 20
                && data.powered == true
            {
                return 538;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 20
                && data.powered == false
            {
                return 539;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 21
                && data.powered == true
            {
                return 540;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 21
                && data.powered == false
            {
                return 541;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 22
                && data.powered == true
            {
                return 542;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 22
                && data.powered == false
            {
                return 543;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 23
                && data.powered == true
            {
                return 544;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 23
                && data.powered == false
            {
                return 545;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 24
                && data.powered == true
            {
                return 546;
            }
            if data.instrument == NoteBlockInstrument::Flute
                && data.note == 24
                && data.powered == false
            {
                return 547;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 0
                && data.powered == true
            {
                return 548;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 0
                && data.powered == false
            {
                return 549;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 1
                && data.powered == true
            {
                return 550;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 1
                && data.powered == false
            {
                return 551;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 2
                && data.powered == true
            {
                return 552;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 2
                && data.powered == false
            {
                return 553;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 3
                && data.powered == true
            {
                return 554;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 3
                && data.powered == false
            {
                return 555;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 4
                && data.powered == true
            {
                return 556;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 4
                && data.powered == false
            {
                return 557;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 5
                && data.powered == true
            {
                return 558;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 5
                && data.powered == false
            {
                return 559;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 6
                && data.powered == true
            {
                return 560;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 6
                && data.powered == false
            {
                return 561;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 7
                && data.powered == true
            {
                return 562;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 7
                && data.powered == false
            {
                return 563;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 8
                && data.powered == true
            {
                return 564;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 8
                && data.powered == false
            {
                return 565;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 9
                && data.powered == true
            {
                return 566;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 9
                && data.powered == false
            {
                return 567;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 10
                && data.powered == true
            {
                return 568;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 10
                && data.powered == false
            {
                return 569;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 11
                && data.powered == true
            {
                return 570;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 11
                && data.powered == false
            {
                return 571;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 12
                && data.powered == true
            {
                return 572;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 12
                && data.powered == false
            {
                return 573;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 13
                && data.powered == true
            {
                return 574;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 13
                && data.powered == false
            {
                return 575;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 14
                && data.powered == true
            {
                return 576;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 14
                && data.powered == false
            {
                return 577;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 15
                && data.powered == true
            {
                return 578;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 15
                && data.powered == false
            {
                return 579;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 16
                && data.powered == true
            {
                return 580;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 16
                && data.powered == false
            {
                return 581;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 17
                && data.powered == true
            {
                return 582;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 17
                && data.powered == false
            {
                return 583;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 18
                && data.powered == true
            {
                return 584;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 18
                && data.powered == false
            {
                return 585;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 19
                && data.powered == true
            {
                return 586;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 19
                && data.powered == false
            {
                return 587;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 20
                && data.powered == true
            {
                return 588;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 20
                && data.powered == false
            {
                return 589;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 21
                && data.powered == true
            {
                return 590;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 21
                && data.powered == false
            {
                return 591;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 22
                && data.powered == true
            {
                return 592;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 22
                && data.powered == false
            {
                return 593;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 23
                && data.powered == true
            {
                return 594;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 23
                && data.powered == false
            {
                return 595;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 24
                && data.powered == true
            {
                return 596;
            }
            if data.instrument == NoteBlockInstrument::Bell
                && data.note == 24
                && data.powered == false
            {
                return 597;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 0
                && data.powered == true
            {
                return 598;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 0
                && data.powered == false
            {
                return 599;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 1
                && data.powered == true
            {
                return 600;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 1
                && data.powered == false
            {
                return 601;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 2
                && data.powered == true
            {
                return 602;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 2
                && data.powered == false
            {
                return 603;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 3
                && data.powered == true
            {
                return 604;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 3
                && data.powered == false
            {
                return 605;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 4
                && data.powered == true
            {
                return 606;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 4
                && data.powered == false
            {
                return 607;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 5
                && data.powered == true
            {
                return 608;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 5
                && data.powered == false
            {
                return 609;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 6
                && data.powered == true
            {
                return 610;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 6
                && data.powered == false
            {
                return 611;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 7
                && data.powered == true
            {
                return 612;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 7
                && data.powered == false
            {
                return 613;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 8
                && data.powered == true
            {
                return 614;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 8
                && data.powered == false
            {
                return 615;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 9
                && data.powered == true
            {
                return 616;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 9
                && data.powered == false
            {
                return 617;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 10
                && data.powered == true
            {
                return 618;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 10
                && data.powered == false
            {
                return 619;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 11
                && data.powered == true
            {
                return 620;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 11
                && data.powered == false
            {
                return 621;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 12
                && data.powered == true
            {
                return 622;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 12
                && data.powered == false
            {
                return 623;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 13
                && data.powered == true
            {
                return 624;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 13
                && data.powered == false
            {
                return 625;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 14
                && data.powered == true
            {
                return 626;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 14
                && data.powered == false
            {
                return 627;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 15
                && data.powered == true
            {
                return 628;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 15
                && data.powered == false
            {
                return 629;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 16
                && data.powered == true
            {
                return 630;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 16
                && data.powered == false
            {
                return 631;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 17
                && data.powered == true
            {
                return 632;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 17
                && data.powered == false
            {
                return 633;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 18
                && data.powered == true
            {
                return 634;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 18
                && data.powered == false
            {
                return 635;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 19
                && data.powered == true
            {
                return 636;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 19
                && data.powered == false
            {
                return 637;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 20
                && data.powered == true
            {
                return 638;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 20
                && data.powered == false
            {
                return 639;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 21
                && data.powered == true
            {
                return 640;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 21
                && data.powered == false
            {
                return 641;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 22
                && data.powered == true
            {
                return 642;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 22
                && data.powered == false
            {
                return 643;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 23
                && data.powered == true
            {
                return 644;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 23
                && data.powered == false
            {
                return 645;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 24
                && data.powered == true
            {
                return 646;
            }
            if data.instrument == NoteBlockInstrument::Guitar
                && data.note == 24
                && data.powered == false
            {
                return 647;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 0
                && data.powered == true
            {
                return 648;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 0
                && data.powered == false
            {
                return 649;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 1
                && data.powered == true
            {
                return 650;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 1
                && data.powered == false
            {
                return 651;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 2
                && data.powered == true
            {
                return 652;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 2
                && data.powered == false
            {
                return 653;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 3
                && data.powered == true
            {
                return 654;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 3
                && data.powered == false
            {
                return 655;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 4
                && data.powered == true
            {
                return 656;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 4
                && data.powered == false
            {
                return 657;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 5
                && data.powered == true
            {
                return 658;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 5
                && data.powered == false
            {
                return 659;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 6
                && data.powered == true
            {
                return 660;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 6
                && data.powered == false
            {
                return 661;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 7
                && data.powered == true
            {
                return 662;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 7
                && data.powered == false
            {
                return 663;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 8
                && data.powered == true
            {
                return 664;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 8
                && data.powered == false
            {
                return 665;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 9
                && data.powered == true
            {
                return 666;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 9
                && data.powered == false
            {
                return 667;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 10
                && data.powered == true
            {
                return 668;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 10
                && data.powered == false
            {
                return 669;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 11
                && data.powered == true
            {
                return 670;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 11
                && data.powered == false
            {
                return 671;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 12
                && data.powered == true
            {
                return 672;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 12
                && data.powered == false
            {
                return 673;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 13
                && data.powered == true
            {
                return 674;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 13
                && data.powered == false
            {
                return 675;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 14
                && data.powered == true
            {
                return 676;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 14
                && data.powered == false
            {
                return 677;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 15
                && data.powered == true
            {
                return 678;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 15
                && data.powered == false
            {
                return 679;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 16
                && data.powered == true
            {
                return 680;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 16
                && data.powered == false
            {
                return 681;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 17
                && data.powered == true
            {
                return 682;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 17
                && data.powered == false
            {
                return 683;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 18
                && data.powered == true
            {
                return 684;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 18
                && data.powered == false
            {
                return 685;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 19
                && data.powered == true
            {
                return 686;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 19
                && data.powered == false
            {
                return 687;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 20
                && data.powered == true
            {
                return 688;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 20
                && data.powered == false
            {
                return 689;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 21
                && data.powered == true
            {
                return 690;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 21
                && data.powered == false
            {
                return 691;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 22
                && data.powered == true
            {
                return 692;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 22
                && data.powered == false
            {
                return 693;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 23
                && data.powered == true
            {
                return 694;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 23
                && data.powered == false
            {
                return 695;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 24
                && data.powered == true
            {
                return 696;
            }
            if data.instrument == NoteBlockInstrument::Chime
                && data.note == 24
                && data.powered == false
            {
                return 697;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 0
                && data.powered == true
            {
                return 698;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 0
                && data.powered == false
            {
                return 699;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 1
                && data.powered == true
            {
                return 700;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 1
                && data.powered == false
            {
                return 701;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 2
                && data.powered == true
            {
                return 702;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 2
                && data.powered == false
            {
                return 703;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 3
                && data.powered == true
            {
                return 704;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 3
                && data.powered == false
            {
                return 705;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 4
                && data.powered == true
            {
                return 706;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 4
                && data.powered == false
            {
                return 707;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 5
                && data.powered == true
            {
                return 708;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 5
                && data.powered == false
            {
                return 709;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 6
                && data.powered == true
            {
                return 710;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 6
                && data.powered == false
            {
                return 711;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 7
                && data.powered == true
            {
                return 712;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 7
                && data.powered == false
            {
                return 713;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 8
                && data.powered == true
            {
                return 714;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 8
                && data.powered == false
            {
                return 715;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 9
                && data.powered == true
            {
                return 716;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 9
                && data.powered == false
            {
                return 717;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 10
                && data.powered == true
            {
                return 718;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 10
                && data.powered == false
            {
                return 719;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 11
                && data.powered == true
            {
                return 720;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 11
                && data.powered == false
            {
                return 721;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 12
                && data.powered == true
            {
                return 722;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 12
                && data.powered == false
            {
                return 723;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 13
                && data.powered == true
            {
                return 724;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 13
                && data.powered == false
            {
                return 725;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 14
                && data.powered == true
            {
                return 726;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 14
                && data.powered == false
            {
                return 727;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 15
                && data.powered == true
            {
                return 728;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 15
                && data.powered == false
            {
                return 729;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 16
                && data.powered == true
            {
                return 730;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 16
                && data.powered == false
            {
                return 731;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 17
                && data.powered == true
            {
                return 732;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 17
                && data.powered == false
            {
                return 733;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 18
                && data.powered == true
            {
                return 734;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 18
                && data.powered == false
            {
                return 735;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 19
                && data.powered == true
            {
                return 736;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 19
                && data.powered == false
            {
                return 737;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 20
                && data.powered == true
            {
                return 738;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 20
                && data.powered == false
            {
                return 739;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 21
                && data.powered == true
            {
                return 740;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 21
                && data.powered == false
            {
                return 741;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 22
                && data.powered == true
            {
                return 742;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 22
                && data.powered == false
            {
                return 743;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 23
                && data.powered == true
            {
                return 744;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 23
                && data.powered == false
            {
                return 745;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 24
                && data.powered == true
            {
                return 746;
            }
            if data.instrument == NoteBlockInstrument::Xylophone
                && data.note == 24
                && data.powered == false
            {
                return 747;
            }
        }
        Block::WhiteBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 748;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 749;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 750;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 751;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 752;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 753;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 754;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 755;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 756;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 757;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 758;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 759;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 760;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 761;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 762;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 763;
            }
        }
        Block::OrangeBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 764;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 765;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 766;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 767;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 768;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 769;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 770;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 771;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 772;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 773;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 774;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 775;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 776;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 777;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 778;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 779;
            }
        }
        Block::MagentaBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 780;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 781;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 782;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 783;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 784;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 785;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 786;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 787;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 788;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 789;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 790;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 791;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 792;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 793;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 794;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 795;
            }
        }
        Block::LightBlueBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 796;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 797;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 798;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 799;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 800;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 801;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 802;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 803;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 804;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 805;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 806;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 807;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 808;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 809;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 810;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 811;
            }
        }
        Block::YellowBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 812;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 813;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 814;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 815;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 816;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 817;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 818;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 819;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 820;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 821;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 822;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 823;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 824;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 825;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 826;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 827;
            }
        }
        Block::LimeBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 828;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 829;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 830;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 831;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 832;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 833;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 834;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 835;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 836;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 837;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 838;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 839;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 840;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 841;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 842;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 843;
            }
        }
        Block::PinkBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 844;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 845;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 846;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 847;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 848;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 849;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 850;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 851;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 852;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 853;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 854;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 855;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 856;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 857;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 858;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 859;
            }
        }
        Block::GrayBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 860;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 861;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 862;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 863;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 864;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 865;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 866;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 867;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 868;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 869;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 870;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 871;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 872;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 873;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 874;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 875;
            }
        }
        Block::LightGrayBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 876;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 877;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 878;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 879;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 880;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 881;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 882;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 883;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 884;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 885;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 886;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 887;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 888;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 889;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 890;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 891;
            }
        }
        Block::CyanBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 892;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 893;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 894;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 895;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 896;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 897;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 898;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 899;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 900;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 901;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 902;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 903;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 904;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 905;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 906;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 907;
            }
        }
        Block::PurpleBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 908;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 909;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 910;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 911;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 912;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 913;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 914;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 915;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 916;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 917;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 918;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 919;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 920;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 921;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 922;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 923;
            }
        }
        Block::BlueBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 924;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 925;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 926;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 927;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 928;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 929;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 930;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 931;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 932;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 933;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 934;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 935;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 936;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 937;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 938;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 939;
            }
        }
        Block::BrownBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 940;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 941;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 942;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 943;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 944;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 945;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 946;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 947;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 948;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 949;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 950;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 951;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 952;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 953;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 954;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 955;
            }
        }
        Block::GreenBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 956;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 957;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 958;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 959;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 960;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 961;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 962;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 963;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 964;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 965;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 966;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 967;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 968;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 969;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 970;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 971;
            }
        }
        Block::RedBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 972;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 973;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 974;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 975;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 976;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 977;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 978;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 979;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 980;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 981;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 982;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 983;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 984;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 985;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 986;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 987;
            }
        }
        Block::BlackBed(data) => {
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Head {
                return 988;
            }
            if data.facing == Facing::North && data.occupied == true && data.part == Part::Foot {
                return 989;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Head {
                return 990;
            }
            if data.facing == Facing::North && data.occupied == false && data.part == Part::Foot {
                return 991;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Head {
                return 992;
            }
            if data.facing == Facing::South && data.occupied == true && data.part == Part::Foot {
                return 993;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Head {
                return 994;
            }
            if data.facing == Facing::South && data.occupied == false && data.part == Part::Foot {
                return 995;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Head {
                return 996;
            }
            if data.facing == Facing::West && data.occupied == true && data.part == Part::Foot {
                return 997;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Head {
                return 998;
            }
            if data.facing == Facing::West && data.occupied == false && data.part == Part::Foot {
                return 999;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Head {
                return 1000;
            }
            if data.facing == Facing::East && data.occupied == true && data.part == Part::Foot {
                return 1001;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Head {
                return 1002;
            }
            if data.facing == Facing::East && data.occupied == false && data.part == Part::Foot {
                return 1003;
            }
        }
        Block::PoweredRail(data) => {
            if data.powered == true && data.shape == Shape::NorthSouth {
                return 1004;
            }
            if data.powered == true && data.shape == Shape::EastWest {
                return 1005;
            }
            if data.powered == true && data.shape == Shape::AscendingEast {
                return 1006;
            }
            if data.powered == true && data.shape == Shape::AscendingWest {
                return 1007;
            }
            if data.powered == true && data.shape == Shape::AscendingNorth {
                return 1008;
            }
            if data.powered == true && data.shape == Shape::AscendingSouth {
                return 1009;
            }
            if data.powered == false && data.shape == Shape::NorthSouth {
                return 1010;
            }
            if data.powered == false && data.shape == Shape::EastWest {
                return 1011;
            }
            if data.powered == false && data.shape == Shape::AscendingEast {
                return 1012;
            }
            if data.powered == false && data.shape == Shape::AscendingWest {
                return 1013;
            }
            if data.powered == false && data.shape == Shape::AscendingNorth {
                return 1014;
            }
            if data.powered == false && data.shape == Shape::AscendingSouth {
                return 1015;
            }
        }
        Block::DetectorRail(data) => {
            if data.powered == true && data.shape == Shape::NorthSouth {
                return 1016;
            }
            if data.powered == true && data.shape == Shape::EastWest {
                return 1017;
            }
            if data.powered == true && data.shape == Shape::AscendingEast {
                return 1018;
            }
            if data.powered == true && data.shape == Shape::AscendingWest {
                return 1019;
            }
            if data.powered == true && data.shape == Shape::AscendingNorth {
                return 1020;
            }
            if data.powered == true && data.shape == Shape::AscendingSouth {
                return 1021;
            }
            if data.powered == false && data.shape == Shape::NorthSouth {
                return 1022;
            }
            if data.powered == false && data.shape == Shape::EastWest {
                return 1023;
            }
            if data.powered == false && data.shape == Shape::AscendingEast {
                return 1024;
            }
            if data.powered == false && data.shape == Shape::AscendingWest {
                return 1025;
            }
            if data.powered == false && data.shape == Shape::AscendingNorth {
                return 1026;
            }
            if data.powered == false && data.shape == Shape::AscendingSouth {
                return 1027;
            }
        }
        Block::StickyPiston(data) => {
            if data.extended == true && data.facing == Facing::North {
                return 1028;
            }
            if data.extended == true && data.facing == Facing::East {
                return 1029;
            }
            if data.extended == true && data.facing == Facing::South {
                return 1030;
            }
            if data.extended == true && data.facing == Facing::West {
                return 1031;
            }
            if data.extended == true && data.facing == Facing::Up {
                return 1032;
            }
            if data.extended == true && data.facing == Facing::Down {
                return 1033;
            }
            if data.extended == false && data.facing == Facing::North {
                return 1034;
            }
            if data.extended == false && data.facing == Facing::East {
                return 1035;
            }
            if data.extended == false && data.facing == Facing::South {
                return 1036;
            }
            if data.extended == false && data.facing == Facing::West {
                return 1037;
            }
            if data.extended == false && data.facing == Facing::Up {
                return 1038;
            }
            if data.extended == false && data.facing == Facing::Down {
                return 1039;
            }
        }
        Block::Cobweb => return 1040,
        Block::Grass => return 1041,
        Block::Fern => return 1042,
        Block::DeadBush => return 1043,
        Block::Seagrass => return 1044,
        Block::TallSeagrass(data) => {
            if data.half == Half::Upper {
                return 1045;
            }
            if data.half == Half::Lower {
                return 1046;
            }
        }
        Block::Piston(data) => {
            if data.extended == true && data.facing == Facing::North {
                return 1047;
            }
            if data.extended == true && data.facing == Facing::East {
                return 1048;
            }
            if data.extended == true && data.facing == Facing::South {
                return 1049;
            }
            if data.extended == true && data.facing == Facing::West {
                return 1050;
            }
            if data.extended == true && data.facing == Facing::Up {
                return 1051;
            }
            if data.extended == true && data.facing == Facing::Down {
                return 1052;
            }
            if data.extended == false && data.facing == Facing::North {
                return 1053;
            }
            if data.extended == false && data.facing == Facing::East {
                return 1054;
            }
            if data.extended == false && data.facing == Facing::South {
                return 1055;
            }
            if data.extended == false && data.facing == Facing::West {
                return 1056;
            }
            if data.extended == false && data.facing == Facing::Up {
                return 1057;
            }
            if data.extended == false && data.facing == Facing::Down {
                return 1058;
            }
        }
        Block::PistonHead(data) => {
            if data.facing == Facing::North
                && data.short == true
                && data.type_ == PistonHeadType::Normal
            {
                return 1059;
            }
            if data.facing == Facing::North
                && data.short == true
                && data.type_ == PistonHeadType::Sticky
            {
                return 1060;
            }
            if data.facing == Facing::North
                && data.short == false
                && data.type_ == PistonHeadType::Normal
            {
                return 1061;
            }
            if data.facing == Facing::North
                && data.short == false
                && data.type_ == PistonHeadType::Sticky
            {
                return 1062;
            }
            if data.facing == Facing::East
                && data.short == true
                && data.type_ == PistonHeadType::Normal
            {
                return 1063;
            }
            if data.facing == Facing::East
                && data.short == true
                && data.type_ == PistonHeadType::Sticky
            {
                return 1064;
            }
            if data.facing == Facing::East
                && data.short == false
                && data.type_ == PistonHeadType::Normal
            {
                return 1065;
            }
            if data.facing == Facing::East
                && data.short == false
                && data.type_ == PistonHeadType::Sticky
            {
                return 1066;
            }
            if data.facing == Facing::South
                && data.short == true
                && data.type_ == PistonHeadType::Normal
            {
                return 1067;
            }
            if data.facing == Facing::South
                && data.short == true
                && data.type_ == PistonHeadType::Sticky
            {
                return 1068;
            }
            if data.facing == Facing::South
                && data.short == false
                && data.type_ == PistonHeadType::Normal
            {
                return 1069;
            }
            if data.facing == Facing::South
                && data.short == false
                && data.type_ == PistonHeadType::Sticky
            {
                return 1070;
            }
            if data.facing == Facing::West
                && data.short == true
                && data.type_ == PistonHeadType::Normal
            {
                return 1071;
            }
            if data.facing == Facing::West
                && data.short == true
                && data.type_ == PistonHeadType::Sticky
            {
                return 1072;
            }
            if data.facing == Facing::West
                && data.short == false
                && data.type_ == PistonHeadType::Normal
            {
                return 1073;
            }
            if data.facing == Facing::West
                && data.short == false
                && data.type_ == PistonHeadType::Sticky
            {
                return 1074;
            }
            if data.facing == Facing::Up
                && data.short == true
                && data.type_ == PistonHeadType::Normal
            {
                return 1075;
            }
            if data.facing == Facing::Up
                && data.short == true
                && data.type_ == PistonHeadType::Sticky
            {
                return 1076;
            }
            if data.facing == Facing::Up
                && data.short == false
                && data.type_ == PistonHeadType::Normal
            {
                return 1077;
            }
            if data.facing == Facing::Up
                && data.short == false
                && data.type_ == PistonHeadType::Sticky
            {
                return 1078;
            }
            if data.facing == Facing::Down
                && data.short == true
                && data.type_ == PistonHeadType::Normal
            {
                return 1079;
            }
            if data.facing == Facing::Down
                && data.short == true
                && data.type_ == PistonHeadType::Sticky
            {
                return 1080;
            }
            if data.facing == Facing::Down
                && data.short == false
                && data.type_ == PistonHeadType::Normal
            {
                return 1081;
            }
            if data.facing == Facing::Down
                && data.short == false
                && data.type_ == PistonHeadType::Sticky
            {
                return 1082;
            }
        }
        Block::WhiteWool => return 1083,
        Block::OrangeWool => return 1084,
        Block::MagentaWool => return 1085,
        Block::LightBlueWool => return 1086,
        Block::YellowWool => return 1087,
        Block::LimeWool => return 1088,
        Block::PinkWool => return 1089,
        Block::GrayWool => return 1090,
        Block::LightGrayWool => return 1091,
        Block::CyanWool => return 1092,
        Block::PurpleWool => return 1093,
        Block::BlueWool => return 1094,
        Block::BrownWool => return 1095,
        Block::GreenWool => return 1096,
        Block::RedWool => return 1097,
        Block::BlackWool => return 1098,
        Block::MovingPiston(data) => {
            if data.facing == Facing::North && data.type_ == MovingPistonType::Normal {
                return 1099;
            }
            if data.facing == Facing::North && data.type_ == MovingPistonType::Sticky {
                return 1100;
            }
            if data.facing == Facing::East && data.type_ == MovingPistonType::Normal {
                return 1101;
            }
            if data.facing == Facing::East && data.type_ == MovingPistonType::Sticky {
                return 1102;
            }
            if data.facing == Facing::South && data.type_ == MovingPistonType::Normal {
                return 1103;
            }
            if data.facing == Facing::South && data.type_ == MovingPistonType::Sticky {
                return 1104;
            }
            if data.facing == Facing::West && data.type_ == MovingPistonType::Normal {
                return 1105;
            }
            if data.facing == Facing::West && data.type_ == MovingPistonType::Sticky {
                return 1106;
            }
            if data.facing == Facing::Up && data.type_ == MovingPistonType::Normal {
                return 1107;
            }
            if data.facing == Facing::Up && data.type_ == MovingPistonType::Sticky {
                return 1108;
            }
            if data.facing == Facing::Down && data.type_ == MovingPistonType::Normal {
                return 1109;
            }
            if data.facing == Facing::Down && data.type_ == MovingPistonType::Sticky {
                return 1110;
            }
        }
        Block::Dandelion => return 1111,
        Block::Poppy => return 1112,
        Block::BlueOrchid => return 1113,
        Block::Allium => return 1114,
        Block::AzureBluet => return 1115,
        Block::RedTulip => return 1116,
        Block::OrangeTulip => return 1117,
        Block::WhiteTulip => return 1118,
        Block::PinkTulip => return 1119,
        Block::OxeyeDaisy => return 1120,
        Block::BrownMushroom => return 1121,
        Block::RedMushroom => return 1122,
        Block::GoldBlock => return 1123,
        Block::IronBlock => return 1124,
        Block::Bricks => return 1125,
        Block::Tnt(data) => {
            if data.unstable == true {
                return 1126;
            }
            if data.unstable == false {
                return 1127;
            }
        }
        Block::Bookshelf => return 1128,
        Block::MossyCobblestone => return 1129,
        Block::Obsidian => return 1130,
        Block::Torch => return 1131,
        Block::WallTorch(data) => {
            if data.facing == Facing::North {
                return 1132;
            }
            if data.facing == Facing::South {
                return 1133;
            }
            if data.facing == Facing::West {
                return 1134;
            }
            if data.facing == Facing::East {
                return 1135;
            }
        }
        Block::Fire(data) => {
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1136;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1137;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1138;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1139;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1140;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1141;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1142;
            }
            if data.age == 0
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1143;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1144;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1145;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1146;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1147;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1148;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1149;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1150;
            }
            if data.age == 0
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1151;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1152;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1153;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1154;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1155;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1156;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1157;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1158;
            }
            if data.age == 0
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1159;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1160;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1161;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1162;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1163;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1164;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1165;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1166;
            }
            if data.age == 0
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1167;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1168;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1169;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1170;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1171;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1172;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1173;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1174;
            }
            if data.age == 1
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1175;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1176;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1177;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1178;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1179;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1180;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1181;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1182;
            }
            if data.age == 1
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1183;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1184;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1185;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1186;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1187;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1188;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1189;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1190;
            }
            if data.age == 1
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1191;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1192;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1193;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1194;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1195;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1196;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1197;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1198;
            }
            if data.age == 1
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1199;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1200;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1201;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1202;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1203;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1204;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1205;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1206;
            }
            if data.age == 2
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1207;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1208;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1209;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1210;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1211;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1212;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1213;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1214;
            }
            if data.age == 2
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1215;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1216;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1217;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1218;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1219;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1220;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1221;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1222;
            }
            if data.age == 2
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1223;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1224;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1225;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1226;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1227;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1228;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1229;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1230;
            }
            if data.age == 2
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1231;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1232;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1233;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1234;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1235;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1236;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1237;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1238;
            }
            if data.age == 3
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1239;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1240;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1241;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1242;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1243;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1244;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1245;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1246;
            }
            if data.age == 3
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1247;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1248;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1249;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1250;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1251;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1252;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1253;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1254;
            }
            if data.age == 3
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1255;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1256;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1257;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1258;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1259;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1260;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1261;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1262;
            }
            if data.age == 3
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1263;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1264;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1265;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1266;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1267;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1268;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1269;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1270;
            }
            if data.age == 4
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1271;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1272;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1273;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1274;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1275;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1276;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1277;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1278;
            }
            if data.age == 4
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1279;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1280;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1281;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1282;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1283;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1284;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1285;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1286;
            }
            if data.age == 4
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1287;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1288;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1289;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1290;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1291;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1292;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1293;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1294;
            }
            if data.age == 4
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1295;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1296;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1297;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1298;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1299;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1300;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1301;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1302;
            }
            if data.age == 5
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1303;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1304;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1305;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1306;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1307;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1308;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1309;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1310;
            }
            if data.age == 5
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1311;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1312;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1313;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1314;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1315;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1316;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1317;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1318;
            }
            if data.age == 5
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1319;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1320;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1321;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1322;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1323;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1324;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1325;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1326;
            }
            if data.age == 5
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1327;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1328;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1329;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1330;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1331;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1332;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1333;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1334;
            }
            if data.age == 6
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1335;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1336;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1337;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1338;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1339;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1340;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1341;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1342;
            }
            if data.age == 6
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1343;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1344;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1345;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1346;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1347;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1348;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1349;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1350;
            }
            if data.age == 6
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1351;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1352;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1353;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1354;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1355;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1356;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1357;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1358;
            }
            if data.age == 6
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1359;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1360;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1361;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1362;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1363;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1364;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1365;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1366;
            }
            if data.age == 7
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1367;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1368;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1369;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1370;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1371;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1372;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1373;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1374;
            }
            if data.age == 7
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1375;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1376;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1377;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1378;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1379;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1380;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1381;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1382;
            }
            if data.age == 7
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1383;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1384;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1385;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1386;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1387;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1388;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1389;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1390;
            }
            if data.age == 7
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1391;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1392;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1393;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1394;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1395;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1396;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1397;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1398;
            }
            if data.age == 8
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1399;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1400;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1401;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1402;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1403;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1404;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1405;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1406;
            }
            if data.age == 8
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1407;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1408;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1409;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1410;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1411;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1412;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1413;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1414;
            }
            if data.age == 8
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1415;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1416;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1417;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1418;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1419;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1420;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1421;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1422;
            }
            if data.age == 8
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1423;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1424;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1425;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1426;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1427;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1428;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1429;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1430;
            }
            if data.age == 9
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1431;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1432;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1433;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1434;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1435;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1436;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1437;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1438;
            }
            if data.age == 9
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1439;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1440;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1441;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1442;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1443;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1444;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1445;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1446;
            }
            if data.age == 9
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1447;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1448;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1449;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1450;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1451;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1452;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1453;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1454;
            }
            if data.age == 9
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1455;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1456;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1457;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1458;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1459;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1460;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1461;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1462;
            }
            if data.age == 10
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1463;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1464;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1465;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1466;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1467;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1468;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1469;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1470;
            }
            if data.age == 10
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1471;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1472;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1473;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1474;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1475;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1476;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1477;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1478;
            }
            if data.age == 10
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1479;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1480;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1481;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1482;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1483;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1484;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1485;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1486;
            }
            if data.age == 10
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1487;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1488;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1489;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1490;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1491;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1492;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1493;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1494;
            }
            if data.age == 11
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1495;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1496;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1497;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1498;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1499;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1500;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1501;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1502;
            }
            if data.age == 11
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1503;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1504;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1505;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1506;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1507;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1508;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1509;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1510;
            }
            if data.age == 11
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1511;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1512;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1513;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1514;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1515;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1516;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1517;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1518;
            }
            if data.age == 11
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1519;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1520;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1521;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1522;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1523;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1524;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1525;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1526;
            }
            if data.age == 12
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1527;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1528;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1529;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1530;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1531;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1532;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1533;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1534;
            }
            if data.age == 12
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1535;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1536;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1537;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1538;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1539;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1540;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1541;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1542;
            }
            if data.age == 12
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1543;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1544;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1545;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1546;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1547;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1548;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1549;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1550;
            }
            if data.age == 12
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1551;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1552;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1553;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1554;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1555;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1556;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1557;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1558;
            }
            if data.age == 13
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1559;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1560;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1561;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1562;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1563;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1564;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1565;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1566;
            }
            if data.age == 13
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1567;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1568;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1569;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1570;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1571;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1572;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1573;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1574;
            }
            if data.age == 13
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1575;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1576;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1577;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1578;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1579;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1580;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1581;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1582;
            }
            if data.age == 13
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1583;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1584;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1585;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1586;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1587;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1588;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1589;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1590;
            }
            if data.age == 14
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1591;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1592;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1593;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1594;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1595;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1596;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1597;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1598;
            }
            if data.age == 14
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1599;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1600;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1601;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1602;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1603;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1604;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1605;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1606;
            }
            if data.age == 14
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1607;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1608;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1609;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1610;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1611;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1612;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1613;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1614;
            }
            if data.age == 14
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1615;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1616;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1617;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1618;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1619;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1620;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1621;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1622;
            }
            if data.age == 15
                && data.east == true
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1623;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1624;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1625;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1626;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1627;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1628;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1629;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1630;
            }
            if data.age == 15
                && data.east == true
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1631;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1632;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1633;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1634;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1635;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1636;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1637;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1638;
            }
            if data.age == 15
                && data.east == false
                && data.north == true
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1639;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == true
            {
                return 1640;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == true
                && data.west == false
            {
                return 1641;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == true
            {
                return 1642;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == true
                && data.up == false
                && data.west == false
            {
                return 1643;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == true
            {
                return 1644;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == true
                && data.west == false
            {
                return 1645;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == true
            {
                return 1646;
            }
            if data.age == 15
                && data.east == false
                && data.north == false
                && data.south == false
                && data.up == false
                && data.west == false
            {
                return 1647;
            }
        }
        Block::Spawner => return 1648,
        Block::OakStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1649;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1650;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1651;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1652;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1653;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1654;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1655;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1656;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1657;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1658;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1659;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1660;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1661;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1662;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1663;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1664;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1665;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1666;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1667;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1668;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1669;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1670;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1671;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1672;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1673;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1674;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1675;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1676;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1677;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1678;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1679;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1680;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1681;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1682;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1683;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1684;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1685;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1686;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1687;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1688;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1689;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1690;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1691;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1692;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1693;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1694;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1695;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1696;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1697;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1698;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1699;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1700;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1701;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1702;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1703;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1704;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1705;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1706;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1707;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1708;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1709;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1710;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1711;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1712;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1713;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1714;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1715;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1716;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1717;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1718;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 1719;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 1720;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 1721;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 1722;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 1723;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 1724;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 1725;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 1726;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 1727;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 1728;
            }
        }
        Block::Chest(data) => {
            if data.facing == Facing::North
                && data.type_ == ChestType::Single
                && data.waterlogged == true
            {
                return 1729;
            }
            if data.facing == Facing::North
                && data.type_ == ChestType::Single
                && data.waterlogged == false
            {
                return 1730;
            }
            if data.facing == Facing::North
                && data.type_ == ChestType::Left
                && data.waterlogged == true
            {
                return 1731;
            }
            if data.facing == Facing::North
                && data.type_ == ChestType::Left
                && data.waterlogged == false
            {
                return 1732;
            }
            if data.facing == Facing::North
                && data.type_ == ChestType::Right
                && data.waterlogged == true
            {
                return 1733;
            }
            if data.facing == Facing::North
                && data.type_ == ChestType::Right
                && data.waterlogged == false
            {
                return 1734;
            }
            if data.facing == Facing::South
                && data.type_ == ChestType::Single
                && data.waterlogged == true
            {
                return 1735;
            }
            if data.facing == Facing::South
                && data.type_ == ChestType::Single
                && data.waterlogged == false
            {
                return 1736;
            }
            if data.facing == Facing::South
                && data.type_ == ChestType::Left
                && data.waterlogged == true
            {
                return 1737;
            }
            if data.facing == Facing::South
                && data.type_ == ChestType::Left
                && data.waterlogged == false
            {
                return 1738;
            }
            if data.facing == Facing::South
                && data.type_ == ChestType::Right
                && data.waterlogged == true
            {
                return 1739;
            }
            if data.facing == Facing::South
                && data.type_ == ChestType::Right
                && data.waterlogged == false
            {
                return 1740;
            }
            if data.facing == Facing::West
                && data.type_ == ChestType::Single
                && data.waterlogged == true
            {
                return 1741;
            }
            if data.facing == Facing::West
                && data.type_ == ChestType::Single
                && data.waterlogged == false
            {
                return 1742;
            }
            if data.facing == Facing::West
                && data.type_ == ChestType::Left
                && data.waterlogged == true
            {
                return 1743;
            }
            if data.facing == Facing::West
                && data.type_ == ChestType::Left
                && data.waterlogged == false
            {
                return 1744;
            }
            if data.facing == Facing::West
                && data.type_ == ChestType::Right
                && data.waterlogged == true
            {
                return 1745;
            }
            if data.facing == Facing::West
                && data.type_ == ChestType::Right
                && data.waterlogged == false
            {
                return 1746;
            }
            if data.facing == Facing::East
                && data.type_ == ChestType::Single
                && data.waterlogged == true
            {
                return 1747;
            }
            if data.facing == Facing::East
                && data.type_ == ChestType::Single
                && data.waterlogged == false
            {
                return 1748;
            }
            if data.facing == Facing::East
                && data.type_ == ChestType::Left
                && data.waterlogged == true
            {
                return 1749;
            }
            if data.facing == Facing::East
                && data.type_ == ChestType::Left
                && data.waterlogged == false
            {
                return 1750;
            }
            if data.facing == Facing::East
                && data.type_ == ChestType::Right
                && data.waterlogged == true
            {
                return 1751;
            }
            if data.facing == Facing::East
                && data.type_ == ChestType::Right
                && data.waterlogged == false
            {
                return 1752;
            }
        }
        Block::RedstoneWire(data) => {
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1753;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1754;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1755;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1756;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1757;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1758;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1759;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1760;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1761;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1762;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1763;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1764;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1765;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1766;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1767;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1768;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1769;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1770;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1771;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1772;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1773;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1774;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1775;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1776;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1777;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1778;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1779;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1780;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1781;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1782;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1783;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1784;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1785;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1786;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1787;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1788;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1789;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1790;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1791;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1792;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1793;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1794;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1795;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1796;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1797;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1798;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1799;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1800;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1801;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1802;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1803;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1804;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1805;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1806;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1807;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1808;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1809;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1810;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1811;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1812;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1813;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1814;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1815;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1816;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1817;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1818;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1819;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1820;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1821;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1822;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1823;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1824;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1825;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1826;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1827;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1828;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1829;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1830;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1831;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1832;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1833;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1834;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1835;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1836;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1837;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1838;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1839;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1840;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1841;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1842;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1843;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1844;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1845;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1846;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1847;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1848;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1849;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1850;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1851;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1852;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1853;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1854;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1855;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1856;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1857;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1858;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1859;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1860;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1861;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1862;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1863;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1864;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1865;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1866;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1867;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1868;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1869;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1870;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1871;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1872;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1873;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1874;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1875;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1876;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1877;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1878;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1879;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1880;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1881;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1882;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1883;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1884;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1885;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1886;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1887;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1888;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1889;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1890;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1891;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1892;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1893;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1894;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1895;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1896;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1897;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1898;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1899;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1900;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1901;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1902;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1903;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1904;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1905;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1906;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1907;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1908;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1909;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1910;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1911;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1912;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1913;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1914;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1915;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1916;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1917;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1918;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1919;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1920;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1921;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1922;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1923;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1924;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1925;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1926;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1927;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1928;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1929;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1930;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1931;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1932;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1933;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1934;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1935;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1936;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1937;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1938;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1939;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1940;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1941;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1942;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1943;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1944;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1945;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1946;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1947;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1948;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1949;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1950;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1951;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1952;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1953;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1954;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1955;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1956;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1957;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1958;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1959;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1960;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1961;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1962;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1963;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1964;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1965;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1966;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1967;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1968;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1969;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1970;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1971;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1972;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1973;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1974;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1975;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1976;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1977;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1978;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1979;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1980;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1981;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1982;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1983;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1984;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1985;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1986;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1987;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1988;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1989;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1990;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 1991;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 1992;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 1993;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 1994;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 1995;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 1996;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 1997;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 1998;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 1999;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2000;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2001;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2002;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2003;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2004;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2005;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2006;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2007;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2008;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2009;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2010;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2011;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2012;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2013;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2014;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2015;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2016;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2017;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2018;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2019;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2020;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2021;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2022;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2023;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2024;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2025;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2026;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2027;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2028;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2029;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2030;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2031;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2032;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2033;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2034;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2035;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2036;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2037;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2038;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2039;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2040;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2041;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2042;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2043;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2044;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2045;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2046;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2047;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2048;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2049;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2050;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2051;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2052;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2053;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2054;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2055;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2056;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2057;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2058;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2059;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2060;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2061;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2062;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2063;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2064;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2065;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2066;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2067;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2068;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2069;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2070;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2071;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2072;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2073;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2074;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2075;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2076;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2077;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2078;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2079;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2080;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2081;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2082;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2083;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2084;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2085;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2086;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2087;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2088;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2089;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2090;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2091;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2092;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2093;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2094;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2095;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2096;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2097;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2098;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2099;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2100;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2101;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2102;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2103;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2104;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2105;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2106;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2107;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2108;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2109;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2110;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2111;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2112;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2113;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2114;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2115;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2116;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2117;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2118;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2119;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2120;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2121;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2122;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2123;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2124;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2125;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2126;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2127;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2128;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2129;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2130;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2131;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2132;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2133;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2134;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2135;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2136;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2137;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2138;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2139;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2140;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2141;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2142;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2143;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2144;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2145;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2146;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2147;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2148;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2149;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2150;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2151;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2152;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2153;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2154;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2155;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2156;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2157;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2158;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2159;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2160;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2161;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2162;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2163;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2164;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2165;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2166;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2167;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2168;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2169;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2170;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2171;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2172;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2173;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2174;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2175;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2176;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2177;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2178;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2179;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2180;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2181;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2182;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2183;
            }
            if data.east == RedstoneWireEast::Up
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2184;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2185;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2186;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2187;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2188;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2189;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2190;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2191;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2192;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2193;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2194;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2195;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2196;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2197;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2198;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2199;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2200;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2201;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2202;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2203;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2204;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2205;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2206;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2207;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2208;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2209;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2210;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2211;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2212;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2213;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2214;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2215;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2216;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2217;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2218;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2219;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2220;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2221;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2222;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2223;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2224;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2225;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2226;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2227;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2228;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2229;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2230;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2231;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2232;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2233;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2234;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2235;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2236;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2237;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2238;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2239;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2240;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2241;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2242;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2243;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2244;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2245;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2246;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2247;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2248;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2249;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2250;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2251;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2252;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2253;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2254;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2255;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2256;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2257;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2258;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2259;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2260;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2261;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2262;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2263;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2264;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2265;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2266;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2267;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2268;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2269;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2270;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2271;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2272;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2273;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2274;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2275;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2276;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2277;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2278;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2279;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2280;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2281;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2282;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2283;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2284;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2285;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2286;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2287;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2288;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2289;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2290;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2291;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2292;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2293;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2294;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2295;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2296;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2297;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2298;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2299;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2300;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2301;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2302;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2303;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2304;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2305;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2306;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2307;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2308;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2309;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2310;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2311;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2312;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2313;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2314;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2315;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2316;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2317;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2318;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2319;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2320;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2321;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2322;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2323;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2324;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2325;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2326;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2327;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2328;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2329;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2330;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2331;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2332;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2333;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2334;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2335;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2336;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2337;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2338;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2339;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2340;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2341;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2342;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2343;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2344;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2345;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2346;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2347;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2348;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2349;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2350;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2351;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2352;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2353;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2354;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2355;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2356;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2357;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2358;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2359;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2360;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2361;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2362;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2363;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2364;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2365;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2366;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2367;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2368;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2369;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2370;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2371;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2372;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2373;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2374;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2375;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2376;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2377;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2378;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2379;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2380;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2381;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2382;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2383;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2384;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2385;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2386;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2387;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2388;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2389;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2390;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2391;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2392;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2393;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2394;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2395;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2396;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2397;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2398;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2399;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2400;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2401;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2402;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2403;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2404;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2405;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2406;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2407;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2408;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2409;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2410;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2411;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2412;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2413;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2414;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2415;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2416;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2417;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2418;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2419;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2420;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2421;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2422;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2423;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2424;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2425;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2426;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2427;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2428;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2429;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2430;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2431;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2432;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2433;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2434;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2435;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2436;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2437;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2438;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2439;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2440;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2441;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2442;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2443;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2444;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2445;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2446;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2447;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2448;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2449;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2450;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2451;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2452;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2453;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2454;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2455;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2456;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2457;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2458;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2459;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2460;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2461;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2462;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2463;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2464;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2465;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2466;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2467;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2468;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2469;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2470;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2471;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2472;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2473;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2474;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2475;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2476;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2477;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2478;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2479;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2480;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2481;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2482;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2483;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2484;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2485;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2486;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2487;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2488;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2489;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2490;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2491;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2492;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2493;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2494;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2495;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2496;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2497;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2498;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2499;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2500;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2501;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2502;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2503;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2504;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2505;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2506;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2507;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2508;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2509;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2510;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2511;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2512;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2513;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2514;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2515;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2516;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2517;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2518;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2519;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2520;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2521;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2522;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2523;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2524;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2525;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2526;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2527;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2528;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2529;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2530;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2531;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2532;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2533;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2534;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2535;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2536;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2537;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2538;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2539;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2540;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2541;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2542;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2543;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2544;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2545;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2546;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2547;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2548;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2549;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2550;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2551;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2552;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2553;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2554;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2555;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2556;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2557;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2558;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2559;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2560;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2561;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2562;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2563;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2564;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2565;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2566;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2567;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2568;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2569;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2570;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2571;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2572;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2573;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2574;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2575;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2576;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2577;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2578;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2579;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2580;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2581;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2582;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2583;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2584;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2585;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2586;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2587;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2588;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2589;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2590;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2591;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2592;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2593;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2594;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2595;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2596;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2597;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2598;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2599;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2600;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2601;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2602;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2603;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2604;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2605;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2606;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2607;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2608;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2609;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2610;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2611;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2612;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2613;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2614;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2615;
            }
            if data.east == RedstoneWireEast::Side
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2616;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2617;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2618;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2619;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2620;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2621;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2622;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2623;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2624;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2625;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2626;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2627;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2628;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2629;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2630;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2631;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2632;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2633;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2634;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2635;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2636;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2637;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2638;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2639;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2640;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2641;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2642;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2643;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2644;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2645;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2646;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2647;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2648;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2649;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2650;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2651;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2652;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2653;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2654;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2655;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2656;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2657;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2658;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2659;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2660;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2661;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2662;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2663;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2664;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2665;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2666;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2667;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2668;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2669;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2670;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2671;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2672;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2673;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2674;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2675;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2676;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2677;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2678;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2679;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2680;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2681;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2682;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2683;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2684;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2685;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2686;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2687;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2688;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2689;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2690;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2691;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2692;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2693;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2694;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2695;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2696;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2697;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2698;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2699;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2700;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2701;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2702;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2703;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2704;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2705;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2706;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2707;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2708;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2709;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2710;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2711;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2712;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2713;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2714;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2715;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2716;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2717;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2718;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2719;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2720;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2721;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2722;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2723;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2724;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2725;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2726;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2727;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2728;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2729;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2730;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2731;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2732;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2733;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2734;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2735;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2736;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2737;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2738;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2739;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2740;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2741;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2742;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2743;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2744;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2745;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2746;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2747;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2748;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2749;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2750;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2751;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2752;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2753;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2754;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2755;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2756;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2757;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2758;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2759;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Up
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2760;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2761;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2762;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2763;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2764;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2765;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2766;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2767;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2768;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2769;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2770;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2771;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2772;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2773;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2774;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2775;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2776;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2777;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2778;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2779;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2780;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2781;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2782;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2783;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2784;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2785;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2786;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2787;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2788;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2789;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2790;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2791;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2792;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2793;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2794;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2795;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2796;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2797;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2798;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2799;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2800;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2801;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2802;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2803;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2804;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2805;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2806;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2807;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2808;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2809;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2810;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2811;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2812;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2813;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2814;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2815;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2816;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2817;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2818;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2819;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2820;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2821;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2822;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2823;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2824;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2825;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2826;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2827;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2828;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2829;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2830;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2831;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2832;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2833;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2834;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2835;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2836;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2837;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2838;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2839;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2840;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2841;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2842;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2843;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2844;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2845;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2846;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2847;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2848;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2849;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2850;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2851;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2852;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2853;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2854;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2855;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2856;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2857;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2858;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2859;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2860;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2861;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2862;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2863;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2864;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2865;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2866;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2867;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2868;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2869;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2870;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2871;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2872;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2873;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2874;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2875;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2876;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2877;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2878;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2879;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2880;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2881;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2882;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2883;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2884;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2885;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2886;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2887;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2888;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2889;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2890;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2891;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2892;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2893;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2894;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2895;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2896;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2897;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2898;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2899;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2900;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2901;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2902;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2903;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::Side
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2904;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2905;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2906;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2907;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2908;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2909;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2910;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2911;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2912;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 0
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2913;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2914;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2915;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2916;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2917;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2918;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2919;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2920;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2921;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 1
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2922;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2923;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2924;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2925;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2926;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2927;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2928;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2929;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2930;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 2
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2931;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2932;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2933;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2934;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2935;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2936;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2937;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2938;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2939;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 3
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2940;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2941;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2942;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2943;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2944;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2945;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2946;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2947;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2948;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 4
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2949;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2950;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2951;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2952;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2953;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2954;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2955;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2956;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2957;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 5
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2958;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2959;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2960;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2961;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2962;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2963;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2964;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2965;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2966;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 6
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2967;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2968;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2969;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2970;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2971;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2972;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2973;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2974;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2975;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 7
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2976;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2977;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2978;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2979;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2980;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2981;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2982;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2983;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2984;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 8
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2985;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2986;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2987;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2988;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2989;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2990;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 2991;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 2992;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 2993;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 9
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 2994;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 2995;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 2996;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 2997;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 2998;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 2999;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 3000;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 3001;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 3002;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 10
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 3003;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 3004;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 3005;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 3006;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 3007;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 3008;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 3009;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 3010;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 3011;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 11
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 3012;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 3013;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 3014;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 3015;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 3016;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 3017;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 3018;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 3019;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 3020;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 12
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 3021;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 3022;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 3023;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 3024;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 3025;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 3026;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 3027;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 3028;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 3029;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 13
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 3030;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 3031;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 3032;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 3033;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 3034;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 3035;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 3036;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 3037;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 3038;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 14
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 3039;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Up
            {
                return 3040;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::Side
            {
                return 3041;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Up
                && data.west == RedstoneWireWest::None
            {
                return 3042;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Up
            {
                return 3043;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::Side
            {
                return 3044;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::Side
                && data.west == RedstoneWireWest::None
            {
                return 3045;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Up
            {
                return 3046;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::Side
            {
                return 3047;
            }
            if data.east == RedstoneWireEast::None
                && data.north == RedstoneWireNorth::None
                && data.power == 15
                && data.south == RedstoneWireSouth::None
                && data.west == RedstoneWireWest::None
            {
                return 3048;
            }
        }
        Block::DiamondOre => return 3049,
        Block::DiamondBlock => return 3050,
        Block::CraftingTable => return 3051,
        Block::Wheat(data) => {
            if data.age == 0 {
                return 3052;
            }
            if data.age == 1 {
                return 3053;
            }
            if data.age == 2 {
                return 3054;
            }
            if data.age == 3 {
                return 3055;
            }
            if data.age == 4 {
                return 3056;
            }
            if data.age == 5 {
                return 3057;
            }
            if data.age == 6 {
                return 3058;
            }
            if data.age == 7 {
                return 3059;
            }
        }
        Block::Farmland(data) => {
            if data.moisture == 0 {
                return 3060;
            }
            if data.moisture == 1 {
                return 3061;
            }
            if data.moisture == 2 {
                return 3062;
            }
            if data.moisture == 3 {
                return 3063;
            }
            if data.moisture == 4 {
                return 3064;
            }
            if data.moisture == 5 {
                return 3065;
            }
            if data.moisture == 6 {
                return 3066;
            }
            if data.moisture == 7 {
                return 3067;
            }
        }
        Block::Furnace(data) => {
            if data.facing == Facing::North && data.lit == true {
                return 3068;
            }
            if data.facing == Facing::North && data.lit == false {
                return 3069;
            }
            if data.facing == Facing::South && data.lit == true {
                return 3070;
            }
            if data.facing == Facing::South && data.lit == false {
                return 3071;
            }
            if data.facing == Facing::West && data.lit == true {
                return 3072;
            }
            if data.facing == Facing::West && data.lit == false {
                return 3073;
            }
            if data.facing == Facing::East && data.lit == true {
                return 3074;
            }
            if data.facing == Facing::East && data.lit == false {
                return 3075;
            }
        }
        Block::Sign(data) => {
            if data.rotation == 0 && data.waterlogged == true {
                return 3076;
            }
            if data.rotation == 0 && data.waterlogged == false {
                return 3077;
            }
            if data.rotation == 1 && data.waterlogged == true {
                return 3078;
            }
            if data.rotation == 1 && data.waterlogged == false {
                return 3079;
            }
            if data.rotation == 2 && data.waterlogged == true {
                return 3080;
            }
            if data.rotation == 2 && data.waterlogged == false {
                return 3081;
            }
            if data.rotation == 3 && data.waterlogged == true {
                return 3082;
            }
            if data.rotation == 3 && data.waterlogged == false {
                return 3083;
            }
            if data.rotation == 4 && data.waterlogged == true {
                return 3084;
            }
            if data.rotation == 4 && data.waterlogged == false {
                return 3085;
            }
            if data.rotation == 5 && data.waterlogged == true {
                return 3086;
            }
            if data.rotation == 5 && data.waterlogged == false {
                return 3087;
            }
            if data.rotation == 6 && data.waterlogged == true {
                return 3088;
            }
            if data.rotation == 6 && data.waterlogged == false {
                return 3089;
            }
            if data.rotation == 7 && data.waterlogged == true {
                return 3090;
            }
            if data.rotation == 7 && data.waterlogged == false {
                return 3091;
            }
            if data.rotation == 8 && data.waterlogged == true {
                return 3092;
            }
            if data.rotation == 8 && data.waterlogged == false {
                return 3093;
            }
            if data.rotation == 9 && data.waterlogged == true {
                return 3094;
            }
            if data.rotation == 9 && data.waterlogged == false {
                return 3095;
            }
            if data.rotation == 10 && data.waterlogged == true {
                return 3096;
            }
            if data.rotation == 10 && data.waterlogged == false {
                return 3097;
            }
            if data.rotation == 11 && data.waterlogged == true {
                return 3098;
            }
            if data.rotation == 11 && data.waterlogged == false {
                return 3099;
            }
            if data.rotation == 12 && data.waterlogged == true {
                return 3100;
            }
            if data.rotation == 12 && data.waterlogged == false {
                return 3101;
            }
            if data.rotation == 13 && data.waterlogged == true {
                return 3102;
            }
            if data.rotation == 13 && data.waterlogged == false {
                return 3103;
            }
            if data.rotation == 14 && data.waterlogged == true {
                return 3104;
            }
            if data.rotation == 14 && data.waterlogged == false {
                return 3105;
            }
            if data.rotation == 15 && data.waterlogged == true {
                return 3106;
            }
            if data.rotation == 15 && data.waterlogged == false {
                return 3107;
            }
        }
        Block::OakDoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3108;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3109;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3110;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3111;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3112;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3113;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3114;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3115;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3116;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3117;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3118;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3119;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3120;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3121;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3122;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3123;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3124;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3125;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3126;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3127;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3128;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3129;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3130;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3131;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3132;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3133;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3134;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3135;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3136;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3137;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3138;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3139;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3140;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3141;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3142;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3143;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3144;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3145;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3146;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3147;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3148;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3149;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3150;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3151;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3152;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3153;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3154;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3155;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3156;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3157;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3158;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3159;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3160;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3161;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3162;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3163;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3164;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3165;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3166;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3167;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3168;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3169;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3170;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3171;
            }
        }
        Block::Ladder(data) => {
            if data.facing == Facing::North && data.waterlogged == true {
                return 3172;
            }
            if data.facing == Facing::North && data.waterlogged == false {
                return 3173;
            }
            if data.facing == Facing::South && data.waterlogged == true {
                return 3174;
            }
            if data.facing == Facing::South && data.waterlogged == false {
                return 3175;
            }
            if data.facing == Facing::West && data.waterlogged == true {
                return 3176;
            }
            if data.facing == Facing::West && data.waterlogged == false {
                return 3177;
            }
            if data.facing == Facing::East && data.waterlogged == true {
                return 3178;
            }
            if data.facing == Facing::East && data.waterlogged == false {
                return 3179;
            }
        }
        Block::Rail(data) => {
            if data.shape == Shape::NorthSouth {
                return 3180;
            }
            if data.shape == Shape::EastWest {
                return 3181;
            }
            if data.shape == Shape::AscendingEast {
                return 3182;
            }
            if data.shape == Shape::AscendingWest {
                return 3183;
            }
            if data.shape == Shape::AscendingNorth {
                return 3184;
            }
            if data.shape == Shape::AscendingSouth {
                return 3185;
            }
            if data.shape == Shape::SouthEast {
                return 3186;
            }
            if data.shape == Shape::SouthWest {
                return 3187;
            }
            if data.shape == Shape::NorthWest {
                return 3188;
            }
            if data.shape == Shape::NorthEast {
                return 3189;
            }
        }
        Block::CobblestoneStairs(data) => {
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3190;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3191;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3192;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3193;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3194;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3195;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3196;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3197;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3198;
            }
            if data.facing == Facing::North
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3199;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3200;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3201;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3202;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3203;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3204;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3205;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3206;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3207;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3208;
            }
            if data.facing == Facing::North
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3209;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3210;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3211;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3212;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3213;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3214;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3215;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3216;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3217;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3218;
            }
            if data.facing == Facing::South
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3219;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3220;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3221;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3222;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3223;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3224;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3225;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3226;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3227;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3228;
            }
            if data.facing == Facing::South
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3229;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3230;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3231;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3232;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3233;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3234;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3235;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3236;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3237;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3238;
            }
            if data.facing == Facing::West
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3239;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3240;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3241;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3242;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3243;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3244;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3245;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3246;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3247;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3248;
            }
            if data.facing == Facing::West
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3249;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3250;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3251;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3252;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3253;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3254;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3255;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3256;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3257;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3258;
            }
            if data.facing == Facing::East
                && data.half == Half::Top
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3259;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == true
            {
                return 3260;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::Straight
                && data.waterlogged == false
            {
                return 3261;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == true
            {
                return 3262;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerLeft
                && data.waterlogged == false
            {
                return 3263;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == true
            {
                return 3264;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::InnerRight
                && data.waterlogged == false
            {
                return 3265;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == true
            {
                return 3266;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterLeft
                && data.waterlogged == false
            {
                return 3267;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == true
            {
                return 3268;
            }
            if data.facing == Facing::East
                && data.half == Half::Bottom
                && data.shape == Shape::OuterRight
                && data.waterlogged == false
            {
                return 3269;
            }
        }
        Block::WallSign(data) => {
            if data.facing == Facing::North && data.waterlogged == true {
                return 3270;
            }
            if data.facing == Facing::North && data.waterlogged == false {
                return 3271;
            }
            if data.facing == Facing::South && data.waterlogged == true {
                return 3272;
            }
            if data.facing == Facing::South && data.waterlogged == false {
                return 3273;
            }
            if data.facing == Facing::West && data.waterlogged == true {
                return 3274;
            }
            if data.facing == Facing::West && data.waterlogged == false {
                return 3275;
            }
            if data.facing == Facing::East && data.waterlogged == true {
                return 3276;
            }
            if data.facing == Facing::East && data.waterlogged == false {
                return 3277;
            }
        }
        Block::Lever(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 3278;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 3279;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 3280;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 3281;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 3282;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 3283;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 3284;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 3285;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 3286;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 3287;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 3288;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 3289;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 3290;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 3291;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 3292;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 3293;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 3294;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 3295;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 3296;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 3297;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 3298;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 3299;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 3300;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 3301;
            }
        }
        Block::StonePressurePlate(data) => {
            if data.powered == true {
                return 3302;
            }
            if data.powered == false {
                return 3303;
            }
        }
        Block::IronDoor(data) => {
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3304;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3305;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3306;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3307;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3308;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3309;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3310;
            }
            if data.facing == Facing::North
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3311;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3312;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3313;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3314;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3315;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3316;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3317;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3318;
            }
            if data.facing == Facing::North
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3319;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3320;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3321;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3322;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3323;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3324;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3325;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3326;
            }
            if data.facing == Facing::South
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3327;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3328;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3329;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3330;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3331;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3332;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3333;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3334;
            }
            if data.facing == Facing::South
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3335;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3336;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3337;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3338;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3339;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3340;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3341;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3342;
            }
            if data.facing == Facing::West
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3343;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3344;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3345;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3346;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3347;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3348;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3349;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3350;
            }
            if data.facing == Facing::West
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3351;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3352;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3353;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3354;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3355;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3356;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3357;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3358;
            }
            if data.facing == Facing::East
                && data.half == Half::Upper
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3359;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == true
            {
                return 3360;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == true
                && data.powered == false
            {
                return 3361;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == true
            {
                return 3362;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Left
                && data.open == false
                && data.powered == false
            {
                return 3363;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == true
            {
                return 3364;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == true
                && data.powered == false
            {
                return 3365;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == true
            {
                return 3366;
            }
            if data.facing == Facing::East
                && data.half == Half::Lower
                && data.hinge == Hinge::Right
                && data.open == false
                && data.powered == false
            {
                return 3367;
            }
        }
        Block::OakPressurePlate(data) => {
            if data.powered == true {
                return 3368;
            }
            if data.powered == false {
                return 3369;
            }
        }
        Block::SprucePressurePlate(data) => {
            if data.powered == true {
                return 3370;
            }
            if data.powered == false {
                return 3371;
            }
        }
        Block::BirchPressurePlate(data) => {
            if data.powered == true {
                return 3372;
            }
            if data.powered == false {
                return 3373;
            }
        }
        Block::JunglePressurePlate(data) => {
            if data.powered == true {
                return 3374;
            }
            if data.powered == false {
                return 3375;
            }
        }
        Block::AcaciaPressurePlate(data) => {
            if data.powered == true {
                return 3376;
            }
            if data.powered == false {
                return 3377;
            }
        }
        Block::DarkOakPressurePlate(data) => {
            if data.powered == true {
                return 3378;
            }
            if data.powered == false {
                return 3379;
            }
        }
        Block::RedstoneOre(data) => {
            if data.lit == true {
                return 3380;
            }
            if data.lit == false {
                return 3381;
            }
        }
        Block::RedstoneTorch(data) => {
            if data.lit == true {
                return 3382;
            }
            if data.lit == false {
                return 3383;
            }
        }
        Block::RedstoneWallTorch(data) => {
            if data.facing == Facing::North && data.lit == true {
                return 3384;
            }
            if data.facing == Facing::North && data.lit == false {
                return 3385;
            }
            if data.facing == Facing::South && data.lit == true {
                return 3386;
            }
            if data.facing == Facing::South && data.lit == false {
                return 3387;
            }
            if data.facing == Facing::West && data.lit == true {
                return 3388;
            }
            if data.facing == Facing::West && data.lit == false {
                return 3389;
            }
            if data.facing == Facing::East && data.lit == true {
                return 3390;
            }
            if data.facing == Facing::East && data.lit == false {
                return 3391;
            }
        }
        Block::StoneButton(data) => {
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == true {
                return 3392;
            }
            if data.face == Face::Floor && data.facing == Facing::North && data.powered == false {
                return 3393;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == true {
                return 3394;
            }
            if data.face == Face::Floor && data.facing == Facing::South && data.powered == false {
                return 3395;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == true {
                return 3396;
            }
            if data.face == Face::Floor && data.facing == Facing::West && data.powered == false {
                return 3397;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == true {
                return 3398;
            }
            if data.face == Face::Floor && data.facing == Facing::East && data.powered == false {
                return 3399;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == true {
                return 3400;
            }
            if data.face == Face::Wall && data.facing == Facing::North && data.powered == false {
                return 3401;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == true {
                return 3402;
            }
            if data.face == Face::Wall && data.facing == Facing::South && data.powered == false {
                return 3403;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == true {
                return 3404;
            }
            if data.face == Face::Wall && data.facing == Facing::West && data.powered == false {
                return 3405;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == true {
                return 3406;
            }
            if data.face == Face::Wall && data.facing == Facing::East && data.powered == false {
                return 3407;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == true {
                return 3408;
            }
            if data.face == Face::Ceiling && data.facing == Facing::North && data.powered == false {
                return 3409;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == true {
                return 3410;
            }
            if data.face == Face::Ceiling && data.facing == Facing::South && data.powered == false {
                return 3411;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == true {
                return 3412;
            }
            if data.face == Face::Ceiling && data.facing == Facing::West && data.powered == false {
                return 3413;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == true {
                return 3414;
            }
            if data.face == Face::Ceiling && data.facing == Facing::East && data.powered == false {
                return 3415;
            }
        }
        Block::Snow(data) => {
            if data.layers == 1 {
                return 3416;
            }
            if data.layers == 2 {
                return 3417;
            }
            if data.layers == 3 {
                return 3418;
            }
            if data.layers == 4 {
                return 3419;
            }
            if data.layers == 5 {
                return 3420;
            }
            if data.layers == 6 {
                return 3421;
            }
            if data.layers == 7 {
                return 3422;
            }
            if data.layers == 8 {
                return 3423;
            }
        }
        Block::Ice => return 3424,
        Block::SnowBlock => return 3425,
        Block::Cactus(data) => {
            if data.age == 0 {
                return 3426;
            }
            if data.age == 1 {
                return 3427;
            }
            if data.age == 2 {
                return 3428;
            }
            if data.age == 3 {
                return 3429;
            }
            if data.age == 4 {
                return 3430;
            }
            if data.age == 5 {
                return 3431;
            }
            if data.age == 6 {
                return 3432;
            }
            if data.age == 7 {
                return 3433;
            }
            if data.age == 8 {
                return 3434;
            }
            if data.age == 9 {
                return 3435;
            }
            if data.age == 10 {
                return 3436;
            }
            if data.age == 11 {
                return 3437;
            }
            if data.age == 12 {
                return 3438;
            }
            if data.age == 13 {
                return 3439;
            }
            if data.age == 14 {
                return 3440;
            }
            if data.age == 15 {
                return 3441;
            }
        }
        Block::Clay => return 3442,
        Block::SugarCane(data) => {
            if data.age == 0 {
                return 3443;
            }
            if data.age == 1 {
                return 3444;
            }
            if data.age == 2 {
                return 3445;
            }
            if data.age == 3 {
                return 3446;
            }
            if data.age == 4 {
                return 3447;
            }
            if data.age == 5 {
                return 3448;
            }
            if data.age == 6 {
                return 3449;
            }
            if data.age == 7 {
                return 3450;
            }
            if data.age == 8 {
                return 3451;
            }
            if data.age == 9 {
                return 3452;
            }
            if data.age == 10 {
                return 3453;
            }
            if data.age == 11 {
                return 3454;
            }
            if data.age == 12 {
                return 3455;
            }
            if data.age == 13 {
                return 3456;
            }
            if data.age == 14 {
                return 3457;
            }
            if data.age == 15 {
                return 3458;
            }
        }
        Block::Jukebox(data) => {
            if data.has_record == true {
                return 3459;
            }
            if data.has_record == false {
                return 3460;
            }
        }
        _ => return NOT_FOUND,
    }
    NOT_FOUND
}
