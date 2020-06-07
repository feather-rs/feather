use crate::{BlockId, BlockKind, Face, FacingCubic};
use feather_util::BlockPosition;

/// See also: `BlockId::needed_support`
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SupportType {
    Whitelist(SupportPosition, &'static [SupportBlock]),
    Blacklist(SupportPosition, &'static [SupportBlock]),
    SatisfiesAny(&'static [SupportType]),
    SatisfiesAll(&'static [SupportType]),
}

/// See also: `BlockId::needed_support`
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SupportPosition {
    Relative(BlockPosition),
    Facing,
    FaceFacing,
}

/// See also: `BlockId::needed_support`
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SupportBlock {
    Same,
    Full,
    Specific(BlockKind),
}

macro_rules! block_array {
    ( $( $x:expr ),* ) => { const_array!([$( $x ),*], SupportBlock) };
}

macro_rules! support_array {
    ( $( $x:expr ),* ) => { const_array!([$( $x ),*], SupportType) };
}

macro_rules! const_array {
    ( [$( $x:expr ),*], $t:ty ) => {
        {
            const ARR: &'static [$t; count!($($x),*)] = &[
                $( $x ),*
            ];
            ARR
        }
    };
}

macro_rules! const_support {
    ( $x:expr ) => {{
        const ONE: &'static SupportType = &($x);
        ONE
    }};
}

macro_rules! count {
    ( $x:expr, $( $other:expr ),+ ) => { 1 + count!($($other),+) };
    ( $x:expr ) => { 1 };
    () => { 0 };
}

const NORTH: BlockPosition = BlockPosition { x: 0, y: 0, z: -1 };
const EAST: BlockPosition = BlockPosition { x: 1, y: 0, z: 0 };
const SOUTH: BlockPosition = BlockPosition { x: 0, y: 0, z: 1 };
const WEST: BlockPosition = BlockPosition { x: -1, y: 0, z: 0 };
const UP: BlockPosition = BlockPosition { x: 0, y: 1, z: 0 };
const DOWN: BlockPosition = BlockPosition { x: 0, y: -1, z: 0 };

const DOWN_NORTH: BlockPosition = BlockPosition { x: 0, y: -1, z: -1 };
const DOWN_EAST: BlockPosition = BlockPosition { x: 1, y: -1, z: 0 };
const DOWN_SOUTH: BlockPosition = BlockPosition { x: 0, y: -1, z: 1 };
const DOWN_WEST: BlockPosition = BlockPosition { x: -1, y: -1, z: 0 };

impl BlockId {
    /// This reports whether and, if so, what kind of support block
    /// is needed for a block of this BlockId.
    /// E.g.:   - WallTorch needs full block beside it
    ///         - Grass needs full block beneath it
    ///         - etc.
    pub fn needed_support(self) -> Option<&'static SupportType> {
        use SupportBlock::*;
        use SupportPosition::*;
        use SupportType::*;
        // TODO check list, add more
        match self.kind() {
            BlockKind::Torch
            | BlockKind::RedstoneTorch
            | BlockKind::RedstoneWire
            | BlockKind::Repeater
            | BlockKind::Rail
            | BlockKind::LightWeightedPressurePlate
            | BlockKind::HeavyWeightedPressurePlate
            | BlockKind::Comparator
            | BlockKind::WhiteCarpet
            | BlockKind::OrangeCarpet
            | BlockKind::MagentaCarpet
            | BlockKind::LightBlueCarpet
            | BlockKind::YellowCarpet
            | BlockKind::LimeCarpet
            | BlockKind::PinkCarpet
            | BlockKind::GrayCarpet
            | BlockKind::LightGrayCarpet
            | BlockKind::CyanCarpet
            | BlockKind::PurpleCarpet
            | BlockKind::BlueCarpet
            | BlockKind::BrownCarpet
            | BlockKind::GreenCarpet
            | BlockKind::RedCarpet
            | BlockKind::BlackCarpet
            | BlockKind::Seagrass
            | BlockKind::TallSeagrass
            | BlockKind::Kelp
            | BlockKind::KelpPlant
            | BlockKind::Sign
            | BlockKind::StonePressurePlate
            | BlockKind::OakPressurePlate
            | BlockKind::SprucePressurePlate
            | BlockKind::BirchPressurePlate
            | BlockKind::JunglePressurePlate
            | BlockKind::AcaciaPressurePlate
            | BlockKind::DarkOakPressurePlate
            | BlockKind::ActivatorRail
            | BlockKind::DetectorRail
            | BlockKind::PoweredRail
            | BlockKind::BrainCoral
            | BlockKind::BubbleCoral
            | BlockKind::FireCoral
            | BlockKind::HornCoral
            | BlockKind::TubeCoral
            | BlockKind::BrainCoralFan
            | BlockKind::BubbleCoralFan
            | BlockKind::FireCoralFan
            | BlockKind::HornCoralFan
            | BlockKind::TubeCoralFan
            | BlockKind::DeadBrainCoral
            | BlockKind::DeadBubbleCoral
            | BlockKind::DeadFireCoral
            | BlockKind::DeadHornCoral
            | BlockKind::DeadTubeCoral
            | BlockKind::DeadBrainCoralFan
            | BlockKind::DeadBubbleCoralFan
            | BlockKind::DeadFireCoralFan
            | BlockKind::DeadHornCoralFan
            | BlockKind::DeadTubeCoralFan => Some(const_support!(Whitelist(
                Relative(DOWN),
                block_array![Full]
            ))),
            BlockKind::WallTorch
            | BlockKind::RedstoneWallTorch
            | BlockKind::WhiteWallBanner
            | BlockKind::OrangeWallBanner
            | BlockKind::MagentaWallBanner
            | BlockKind::LightBlueWallBanner
            | BlockKind::YellowWallBanner
            | BlockKind::LimeWallBanner
            | BlockKind::PinkWallBanner
            | BlockKind::GrayWallBanner
            | BlockKind::LightGrayWallBanner
            | BlockKind::CyanWallBanner
            | BlockKind::PurpleWallBanner
            | BlockKind::BlueWallBanner
            | BlockKind::BrownWallBanner
            | BlockKind::GreenWallBanner
            | BlockKind::RedWallBanner
            | BlockKind::BlackWallBanner
            | BlockKind::OakButton
            | BlockKind::Ladder
            | BlockKind::WallSign
            | BlockKind::BrainCoralWallFan
            | BlockKind::BubbleCoralWallFan
            | BlockKind::FireCoralWallFan
            | BlockKind::HornCoralWallFan
            | BlockKind::TubeCoralWallFan
            | BlockKind::DeadBrainCoralWallFan
            | BlockKind::DeadBubbleCoralWallFan
            | BlockKind::DeadFireCoralWallFan
            | BlockKind::DeadHornCoralWallFan
            | BlockKind::DeadTubeCoralWallFan => {
                Some(const_support!(Whitelist(Facing, block_array![Full])))
            }
            BlockKind::SpruceButton
            | BlockKind::BirchButton
            | BlockKind::JungleButton
            | BlockKind::AcaciaButton
            | BlockKind::DarkOakButton
            | BlockKind::StoneButton
            | BlockKind::Lever => Some(const_support!(Whitelist(FaceFacing, block_array![Full]))),
            BlockKind::AttachedMelonStem
            | BlockKind::AttachedPumpkinStem
            | BlockKind::MelonStem
            | BlockKind::PumpkinStem
            | BlockKind::Carrots
            | BlockKind::Potatoes
            | BlockKind::Beetroots
            | BlockKind::Wheat => Some(const_support!(Whitelist(
                Relative(DOWN),
                block_array![Specific(BlockKind::Farmland)],
            ))),
            BlockKind::Snow => Some(const_support!(SatisfiesAll(support_array![
                Whitelist(Relative(DOWN), block_array![Full]),
                Blacklist(
                    Relative(DOWN),
                    block_array![Specific(BlockKind::Ice), Specific(BlockKind::PackedIce)]
                )
            ]))),
            BlockKind::Grass
            | BlockKind::Fern
            | BlockKind::Sunflower
            | BlockKind::Lilac
            | BlockKind::RoseBush
            | BlockKind::Peony
            | BlockKind::TallGrass
            | BlockKind::LargeFern
            | BlockKind::OakSapling
            | BlockKind::SpruceSapling
            | BlockKind::BirchSapling
            | BlockKind::JungleSapling
            | BlockKind::AcaciaSapling
            | BlockKind::DarkOakSapling
            | BlockKind::Dandelion
            | BlockKind::Poppy
            | BlockKind::BlueOrchid
            | BlockKind::Allium
            | BlockKind::AzureBluet
            | BlockKind::RedTulip
            | BlockKind::OrangeTulip
            | BlockKind::WhiteTulip
            | BlockKind::PinkTulip
            | BlockKind::OxeyeDaisy
            | BlockKind::BrownMushroom
            | BlockKind::RedMushroom => Some(const_support!(Whitelist(
                Relative(DOWN),
                block_array![
                    Specific(BlockKind::Dirt),
                    Specific(BlockKind::GrassBlock),
                    Specific(BlockKind::CoarseDirt),
                    Specific(BlockKind::Podzol),
                    Specific(BlockKind::Farmland)
                ],
            ))),
            BlockKind::DeadBush => Some(const_support!(Whitelist(
                Relative(DOWN),
                block_array![
                    Specific(BlockKind::Sand),
                    Specific(BlockKind::RedSand),
                    Specific(BlockKind::Dirt),
                    Specific(BlockKind::CoarseDirt),
                    Specific(BlockKind::Podzol),
                    Specific(BlockKind::Terracotta),
                    Specific(BlockKind::WhiteCarpet),
                    Specific(BlockKind::OrangeCarpet),
                    Specific(BlockKind::MagentaCarpet),
                    Specific(BlockKind::LightBlueCarpet),
                    Specific(BlockKind::YellowCarpet),
                    Specific(BlockKind::LimeCarpet),
                    Specific(BlockKind::PinkCarpet),
                    Specific(BlockKind::GrayCarpet),
                    Specific(BlockKind::LightGrayCarpet),
                    Specific(BlockKind::CyanCarpet),
                    Specific(BlockKind::PurpleCarpet),
                    Specific(BlockKind::BlueCarpet),
                    Specific(BlockKind::BrownCarpet),
                    Specific(BlockKind::GreenCarpet),
                    Specific(BlockKind::RedCarpet),
                    Specific(BlockKind::BlackCarpet)
                ],
            ))),
            BlockKind::SugarCane => Some(const_support!(SatisfiesAny(support_array![
                Whitelist(Relative(DOWN), block_array![Specific(BlockKind::SugarCane)]),
                SatisfiesAll(support_array![
                    Whitelist(
                        Relative(DOWN),
                        block_array![
                            Specific(BlockKind::GrassBlock),
                            Specific(BlockKind::Dirt),
                            Specific(BlockKind::CoarseDirt),
                            Specific(BlockKind::Podzol),
                            Specific(BlockKind::Sand),
                            Specific(BlockKind::RedSand)
                        ],
                    ),
                    SatisfiesAny(support_array![
                        Whitelist(
                            Relative(DOWN_NORTH),
                            block_array![
                                Specific(BlockKind::Water),
                                Specific(BlockKind::FrostedIce)
                            ],
                        ),
                        Whitelist(
                            Relative(DOWN_EAST),
                            block_array![
                                Specific(BlockKind::Water),
                                Specific(BlockKind::FrostedIce)
                            ],
                        ),
                        Whitelist(
                            Relative(DOWN_SOUTH),
                            block_array![
                                Specific(BlockKind::Water),
                                Specific(BlockKind::FrostedIce)
                            ],
                        ),
                        Whitelist(
                            Relative(DOWN_WEST),
                            block_array![
                                Specific(BlockKind::Water),
                                Specific(BlockKind::FrostedIce)
                            ],
                        )
                    ])
                ])
            ]))),
            BlockKind::Vine => Some(const_support!(SatisfiesAny(support_array![
                Whitelist(Relative(UP), block_array![Same, Full]),
                Whitelist(Relative(NORTH), block_array![Full]),
                Whitelist(Relative(EAST), block_array![Full]),
                Whitelist(Relative(SOUTH), block_array![Full]),
                Whitelist(Relative(WEST), block_array![Full])
            ]))),
            BlockKind::Cactus => Some(const_support!(SatisfiesAny(support_array![
                Whitelist(Relative(DOWN), block_array![Same]),
                SatisfiesAll(support_array![
                    Whitelist(
                        Relative(DOWN),
                        block_array![Specific(BlockKind::Sand), Specific(BlockKind::RedSand)],
                    ),
                    SatisfiesAll(support_array![
                        Blacklist(Relative(NORTH), block_array![Full, Same]),
                        Blacklist(Relative(EAST), block_array![Full, Same]),
                        Blacklist(Relative(SOUTH), block_array![Full, Same]),
                        Blacklist(Relative(WEST), block_array![Full, Same])
                    ])
                ])
            ]))),
            BlockKind::NetherWart => Some(const_support!(Whitelist(
                Relative(DOWN),
                block_array![Specific(BlockKind::SoulSand)],
            ))),
            _ => None,
        }
    }
}

impl SupportType {
    /// Reports the offset associated with this SupportType, if any
    /// Needs the BlockId of the supported block for some SupportPosition variants
    pub fn offset(self, supported_block_id: BlockId) -> Option<BlockPosition> {
        match self {
            SupportType::Whitelist(position, ..) | SupportType::Blacklist(position, ..) => {
                Some(position.offset(supported_block_id))
            }
            _ => None,
        }
    }
}

impl SupportPosition {
    /// Solves this SupportPosition for a specific supported block id
    /// Needs the BlockId of the supported block because some variants depend on it
    pub fn offset(self, supported_block_id: BlockId) -> BlockPosition {
        match self {
            SupportPosition::Relative(offset) => offset,
            SupportPosition::Facing => supported_block_id
                .facing_cardinal()
                .unwrap()
                .opposite()
                .offset(),
            SupportPosition::FaceFacing => match supported_block_id.face().unwrap() {
                Face::Ceiling => FacingCubic::Up.offset(),
                Face::Floor => FacingCubic::Down.offset(),
                Face::Wall => supported_block_id
                    .facing_cardinal()
                    .unwrap()
                    .opposite()
                    .offset(),
            },
        }
    }
}

impl SupportBlock {
    /// Solves this SupportBlock kind for a specific supported block kind
    /// and supporting block kind.
    /// Needs the BlockKind of the supported block because some variants depend on it
    pub fn supports(
        self,
        supported_block_kind: BlockKind,
        supporting_block_kind: BlockKind,
    ) -> bool {
        match self {
            SupportBlock::Same => supported_block_kind == supporting_block_kind,
            SupportBlock::Full => supporting_block_kind.full_block(),
            SupportBlock::Specific(kind) => kind == supporting_block_kind,
        }
    }
}
