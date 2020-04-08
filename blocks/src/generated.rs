#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Air {}
impl Air {
    pub fn possible_states() -> Vec<Self> {
        vec![Air {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(air {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Stone {}
impl Stone {
    pub fn possible_states() -> Vec<Self> {
        vec![Stone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(stone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Granite {}
impl Granite {
    pub fn possible_states() -> Vec<Self> {
        vec![Granite {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(granite {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PolishedGranite {}
impl PolishedGranite {
    pub fn possible_states() -> Vec<Self> {
        vec![PolishedGranite {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(polished_granite {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Diorite {}
impl Diorite {
    pub fn possible_states() -> Vec<Self> {
        vec![Diorite {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(diorite {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PolishedDiorite {}
impl PolishedDiorite {
    pub fn possible_states() -> Vec<Self> {
        vec![PolishedDiorite {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(polished_diorite {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Andesite {}
impl Andesite {
    pub fn possible_states() -> Vec<Self> {
        vec![Andesite {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(andesite {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PolishedAndesite {}
impl PolishedAndesite {
    pub fn possible_states() -> Vec<Self> {
        vec![PolishedAndesite {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(polished_andesite {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrassBlock {
    snowy: bool,
}
impl GrassBlock {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|snowy| GrassBlock { snowy })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.snowy as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let snowy = offset / 1u16;
        offset -= snowy * 1u16;
        let snowy = Ok(if snowy == 0 { false } else { true })?;
        Self { snowy }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<grass_block, u16>> = Lazy::new(|| {
            grass_block::possible_states()
                .into_iter()
                .zip([8u16, 9u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dirt {}
impl Dirt {
    pub fn possible_states() -> Vec<Self> {
        vec![Dirt {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dirt {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoarseDirt {}
impl CoarseDirt {
    pub fn possible_states() -> Vec<Self> {
        vec![CoarseDirt {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(coarse_dirt {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Podzol {
    snowy: bool,
}
impl Podzol {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|snowy| Podzol { snowy })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.snowy as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let snowy = offset / 1u16;
        offset -= snowy * 1u16;
        let snowy = Ok(if snowy == 0 { false } else { true })?;
        Self { snowy }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<podzol, u16>> = Lazy::new(|| {
            podzol::possible_states()
                .into_iter()
                .zip([12u16, 13u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cobblestone {}
impl Cobblestone {
    pub fn possible_states() -> Vec<Self> {
        vec![Cobblestone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cobblestone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OakPlanks {}
impl OakPlanks {
    pub fn possible_states() -> Vec<Self> {
        vec![OakPlanks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(oak_planks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SprucePlanks {}
impl SprucePlanks {
    pub fn possible_states() -> Vec<Self> {
        vec![SprucePlanks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(spruce_planks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BirchPlanks {}
impl BirchPlanks {
    pub fn possible_states() -> Vec<Self> {
        vec![BirchPlanks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(birch_planks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JunglePlanks {}
impl JunglePlanks {
    pub fn possible_states() -> Vec<Self> {
        vec![JunglePlanks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(jungle_planks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AcaciaPlanks {}
impl AcaciaPlanks {
    pub fn possible_states() -> Vec<Self> {
        vec![AcaciaPlanks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(acacia_planks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DarkOakPlanks {}
impl DarkOakPlanks {
    pub fn possible_states() -> Vec<Self> {
        vec![DarkOakPlanks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dark_oak_planks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sand {}
impl Sand {
    pub fn possible_states() -> Vec<Self> {
        vec![Sand {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(sand {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedSand {}
impl RedSand {
    pub fn possible_states() -> Vec<Self> {
        vec![RedSand {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_sand {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Gravel {}
impl Gravel {
    pub fn possible_states() -> Vec<Self> {
        vec![Gravel {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gravel {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GoldOre {}
impl GoldOre {
    pub fn possible_states() -> Vec<Self> {
        vec![GoldOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gold_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IronOre {}
impl IronOre {
    pub fn possible_states() -> Vec<Self> {
        vec![IronOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(iron_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoalOre {}
impl CoalOre {
    pub fn possible_states() -> Vec<Self> {
        vec![CoalOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(coal_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sponge {}
impl Sponge {
    pub fn possible_states() -> Vec<Self> {
        vec![Sponge {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(sponge {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WetSponge {}
impl WetSponge {
    pub fn possible_states() -> Vec<Self> {
        vec![WetSponge {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(wet_sponge {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Glass {}
impl Glass {
    pub fn possible_states() -> Vec<Self> {
        vec![Glass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LapisOre {}
impl LapisOre {
    pub fn possible_states() -> Vec<Self> {
        vec![LapisOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lapis_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LapisBlock {}
impl LapisBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![LapisBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lapis_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dispenser {
    triggered: bool,
    facing: Dispenser,
}
impl Dispenser {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        Dispenser::North,
                        Dispenser::East,
                        Dispenser::South,
                        Dispenser::West,
                        Dispenser::Up,
                        Dispenser::Down,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(triggered, facing)| Dispenser { triggered, facing })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.triggered as u16 * 6u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let triggered = offset / 6u16;
        offset -= triggered * 6u16;
        let triggered = Ok(if triggered == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = Dispenser::try_from(facing)?;
        Self { triggered, facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dispenser, u16>> = Lazy::new(|| {
            dispenser::possible_states()
                .into_iter()
                .zip(
                    [
                        233u16, 234u16, 235u16, 236u16, 237u16, 238u16, 239u16, 240u16, 241u16,
                        242u16, 243u16, 244u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sandstone {}
impl Sandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![Sandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChiseledSandstone {}
impl ChiseledSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![ChiseledSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(chiseled_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CutSandstone {}
impl CutSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![CutSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cut_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NoteBlock {
    note: i32,
    powered: bool,
    instrument: NoteBlock,
}
impl NoteBlock {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=24i32 }
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        NoteBlock::Harp,
                        NoteBlock::Basedrum,
                        NoteBlock::Snare,
                        NoteBlock::Hat,
                        NoteBlock::Bass,
                        NoteBlock::Flute,
                        NoteBlock::Bell,
                        NoteBlock::Guitar,
                        NoteBlock::Chime,
                        NoteBlock::Xylophone,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((note, powered), instrument)| NoteBlock {
                note,
                powered,
                instrument,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.note as u16 * 20u16 + self.powered as u16 * 10u16 + self.instrument as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let note = offset / 20u16;
        offset -= note * 20u16;
        let note = Ok(note as i32)?;
        let powered = offset / 10u16;
        offset -= powered * 10u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let instrument = offset / 1u16;
        offset -= instrument * 1u16;
        let instrument = NoteBlock::try_from(instrument)?;
        Self {
            note,
            powered,
            instrument,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<note_block, u16>> = Lazy::new(|| {
            note_block::possible_states()
                .into_iter()
                .zip(
                    [
                        248u16, 249u16, 250u16, 251u16, 252u16, 253u16, 254u16, 255u16, 256u16,
                        257u16, 258u16, 259u16, 260u16, 261u16, 262u16, 263u16, 264u16, 265u16,
                        266u16, 267u16, 268u16, 269u16, 270u16, 271u16, 272u16, 273u16, 274u16,
                        275u16, 276u16, 277u16, 278u16, 279u16, 280u16, 281u16, 282u16, 283u16,
                        284u16, 285u16, 286u16, 287u16, 288u16, 289u16, 290u16, 291u16, 292u16,
                        293u16, 294u16, 295u16, 296u16, 297u16, 298u16, 299u16, 300u16, 301u16,
                        302u16, 303u16, 304u16, 305u16, 306u16, 307u16, 308u16, 309u16, 310u16,
                        311u16, 312u16, 313u16, 314u16, 315u16, 316u16, 317u16, 318u16, 319u16,
                        320u16, 321u16, 322u16, 323u16, 324u16, 325u16, 326u16, 327u16, 328u16,
                        329u16, 330u16, 331u16, 332u16, 333u16, 334u16, 335u16, 336u16, 337u16,
                        338u16, 339u16, 340u16, 341u16, 342u16, 343u16, 344u16, 345u16, 346u16,
                        347u16, 348u16, 349u16, 350u16, 351u16, 352u16, 353u16, 354u16, 355u16,
                        356u16, 357u16, 358u16, 359u16, 360u16, 361u16, 362u16, 363u16, 364u16,
                        365u16, 366u16, 367u16, 368u16, 369u16, 370u16, 371u16, 372u16, 373u16,
                        374u16, 375u16, 376u16, 377u16, 378u16, 379u16, 380u16, 381u16, 382u16,
                        383u16, 384u16, 385u16, 386u16, 387u16, 388u16, 389u16, 390u16, 391u16,
                        392u16, 393u16, 394u16, 395u16, 396u16, 397u16, 398u16, 399u16, 400u16,
                        401u16, 402u16, 403u16, 404u16, 405u16, 406u16, 407u16, 408u16, 409u16,
                        410u16, 411u16, 412u16, 413u16, 414u16, 415u16, 416u16, 417u16, 418u16,
                        419u16, 420u16, 421u16, 422u16, 423u16, 424u16, 425u16, 426u16, 427u16,
                        428u16, 429u16, 430u16, 431u16, 432u16, 433u16, 434u16, 435u16, 436u16,
                        437u16, 438u16, 439u16, 440u16, 441u16, 442u16, 443u16, 444u16, 445u16,
                        446u16, 447u16, 448u16, 449u16, 450u16, 451u16, 452u16, 453u16, 454u16,
                        455u16, 456u16, 457u16, 458u16, 459u16, 460u16, 461u16, 462u16, 463u16,
                        464u16, 465u16, 466u16, 467u16, 468u16, 469u16, 470u16, 471u16, 472u16,
                        473u16, 474u16, 475u16, 476u16, 477u16, 478u16, 479u16, 480u16, 481u16,
                        482u16, 483u16, 484u16, 485u16, 486u16, 487u16, 488u16, 489u16, 490u16,
                        491u16, 492u16, 493u16, 494u16, 495u16, 496u16, 497u16, 498u16, 499u16,
                        500u16, 501u16, 502u16, 503u16, 504u16, 505u16, 506u16, 507u16, 508u16,
                        509u16, 510u16, 511u16, 512u16, 513u16, 514u16, 515u16, 516u16, 517u16,
                        518u16, 519u16, 520u16, 521u16, 522u16, 523u16, 524u16, 525u16, 526u16,
                        527u16, 528u16, 529u16, 530u16, 531u16, 532u16, 533u16, 534u16, 535u16,
                        536u16, 537u16, 538u16, 539u16, 540u16, 541u16, 542u16, 543u16, 544u16,
                        545u16, 546u16, 547u16, 548u16, 549u16, 550u16, 551u16, 552u16, 553u16,
                        554u16, 555u16, 556u16, 557u16, 558u16, 559u16, 560u16, 561u16, 562u16,
                        563u16, 564u16, 565u16, 566u16, 567u16, 568u16, 569u16, 570u16, 571u16,
                        572u16, 573u16, 574u16, 575u16, 576u16, 577u16, 578u16, 579u16, 580u16,
                        581u16, 582u16, 583u16, 584u16, 585u16, 586u16, 587u16, 588u16, 589u16,
                        590u16, 591u16, 592u16, 593u16, 594u16, 595u16, 596u16, 597u16, 598u16,
                        599u16, 600u16, 601u16, 602u16, 603u16, 604u16, 605u16, 606u16, 607u16,
                        608u16, 609u16, 610u16, 611u16, 612u16, 613u16, 614u16, 615u16, 616u16,
                        617u16, 618u16, 619u16, 620u16, 621u16, 622u16, 623u16, 624u16, 625u16,
                        626u16, 627u16, 628u16, 629u16, 630u16, 631u16, 632u16, 633u16, 634u16,
                        635u16, 636u16, 637u16, 638u16, 639u16, 640u16, 641u16, 642u16, 643u16,
                        644u16, 645u16, 646u16, 647u16, 648u16, 649u16, 650u16, 651u16, 652u16,
                        653u16, 654u16, 655u16, 656u16, 657u16, 658u16, 659u16, 660u16, 661u16,
                        662u16, 663u16, 664u16, 665u16, 666u16, 667u16, 668u16, 669u16, 670u16,
                        671u16, 672u16, 673u16, 674u16, 675u16, 676u16, 677u16, 678u16, 679u16,
                        680u16, 681u16, 682u16, 683u16, 684u16, 685u16, 686u16, 687u16, 688u16,
                        689u16, 690u16, 691u16, 692u16, 693u16, 694u16, 695u16, 696u16, 697u16,
                        698u16, 699u16, 700u16, 701u16, 702u16, 703u16, 704u16, 705u16, 706u16,
                        707u16, 708u16, 709u16, 710u16, 711u16, 712u16, 713u16, 714u16, 715u16,
                        716u16, 717u16, 718u16, 719u16, 720u16, 721u16, 722u16, 723u16, 724u16,
                        725u16, 726u16, 727u16, 728u16, 729u16, 730u16, 731u16, 732u16, 733u16,
                        734u16, 735u16, 736u16, 737u16, 738u16, 739u16, 740u16, 741u16, 742u16,
                        743u16, 744u16, 745u16, 746u16, 747u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PoweredRail {
    shape: PoweredRail,
    powered: bool,
}
impl PoweredRail {
    pub fn possible_states() -> Vec<Self> {
        [
            PoweredRail::NorthSouth,
            PoweredRail::EastWest,
            PoweredRail::AscendingEast,
            PoweredRail::AscendingWest,
            PoweredRail::AscendingNorth,
            PoweredRail::AscendingSouth,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(shape, powered)| PoweredRail { shape, powered })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.shape as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let shape = offset / 2u16;
        offset -= shape * 2u16;
        let shape = PoweredRail::try_from(shape)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { shape, powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<powered_rail, u16>> = Lazy::new(|| {
            powered_rail::possible_states()
                .into_iter()
                .zip(
                    [
                        1004u16, 1005u16, 1006u16, 1007u16, 1008u16, 1009u16, 1010u16, 1011u16,
                        1012u16, 1013u16, 1014u16, 1015u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DetectorRail {
    shape: DetectorRail,
    powered: bool,
}
impl DetectorRail {
    pub fn possible_states() -> Vec<Self> {
        [
            DetectorRail::NorthSouth,
            DetectorRail::EastWest,
            DetectorRail::AscendingEast,
            DetectorRail::AscendingWest,
            DetectorRail::AscendingNorth,
            DetectorRail::AscendingSouth,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(shape, powered)| DetectorRail { shape, powered })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.shape as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let shape = offset / 2u16;
        offset -= shape * 2u16;
        let shape = DetectorRail::try_from(shape)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { shape, powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<detector_rail, u16>> = Lazy::new(|| {
            detector_rail::possible_states()
                .into_iter()
                .zip(
                    [
                        1016u16, 1017u16, 1018u16, 1019u16, 1020u16, 1021u16, 1022u16, 1023u16,
                        1024u16, 1025u16, 1026u16, 1027u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StickyPiston {
    facing: StickyPiston,
    extended: bool,
}
impl StickyPiston {
    pub fn possible_states() -> Vec<Self> {
        [
            StickyPiston::North,
            StickyPiston::East,
            StickyPiston::South,
            StickyPiston::West,
            StickyPiston::Up,
            StickyPiston::Down,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, extended)| StickyPiston { facing, extended })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.extended as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = StickyPiston::try_from(facing)?;
        let extended = offset / 1u16;
        offset -= extended * 1u16;
        let extended = Ok(if extended == 0 { false } else { true })?;
        Self { facing, extended }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<sticky_piston, u16>> = Lazy::new(|| {
            sticky_piston::possible_states()
                .into_iter()
                .zip(
                    [
                        1028u16, 1029u16, 1030u16, 1031u16, 1032u16, 1033u16, 1034u16, 1035u16,
                        1036u16, 1037u16, 1038u16, 1039u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cobweb {}
impl Cobweb {
    pub fn possible_states() -> Vec<Self> {
        vec![Cobweb {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cobweb {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grass {}
impl Grass {
    pub fn possible_states() -> Vec<Self> {
        vec![Grass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(grass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fern {}
impl Fern {
    pub fn possible_states() -> Vec<Self> {
        vec![Fern {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(fern {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBush {}
impl DeadBush {
    pub fn possible_states() -> Vec<Self> {
        vec![DeadBush {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dead_bush {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Seagrass {}
impl Seagrass {
    pub fn possible_states() -> Vec<Self> {
        vec![Seagrass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(seagrass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TallSeagrass {
    half: TallSeagrass,
}
impl TallSeagrass {
    pub fn possible_states() -> Vec<Self> {
        [TallSeagrass::Upper, TallSeagrass::Lower]
            .iter()
            .copied()
            .map(|half| TallSeagrass { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = TallSeagrass::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tall_seagrass, u16>> = Lazy::new(|| {
            tall_seagrass::possible_states()
                .into_iter()
                .zip([1045u16, 1046u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Piston {
    facing: Piston,
    extended: bool,
}
impl Piston {
    pub fn possible_states() -> Vec<Self> {
        [
            Piston::North,
            Piston::East,
            Piston::South,
            Piston::West,
            Piston::Up,
            Piston::Down,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, extended)| Piston { facing, extended })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.extended as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = Piston::try_from(facing)?;
        let extended = offset / 1u16;
        offset -= extended * 1u16;
        let extended = Ok(if extended == 0 { false } else { true })?;
        Self { facing, extended }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<piston, u16>> = Lazy::new(|| {
            piston::possible_states()
                .into_iter()
                .zip(
                    [
                        1047u16, 1048u16, 1049u16, 1050u16, 1051u16, 1052u16, 1053u16, 1054u16,
                        1055u16, 1056u16, 1057u16, 1058u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PistonHead {
    facing: PistonHead,
    short: bool,
    kind: PistonHead,
}
impl PistonHead {
    pub fn possible_states() -> Vec<Self> {
        [
            PistonHead::North,
            PistonHead::East,
            PistonHead::South,
            PistonHead::West,
            PistonHead::Up,
            PistonHead::Down,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| {
            std::iter::repeat(state).zip([PistonHead::Normal, PistonHead::Sticky].iter().copied())
        })
        .map(|((facing, short), kind)| PistonHead {
            facing,
            short,
            kind,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 4u16 + self.short as u16 * 2u16 + self.kind as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 4u16;
        offset -= facing * 4u16;
        let facing = PistonHead::try_from(facing)?;
        let short = offset / 2u16;
        offset -= short * 2u16;
        let short = Ok(if short == 0 { false } else { true })?;
        let kind = offset / 1u16;
        offset -= kind * 1u16;
        let kind = PistonHead::try_from(kind)?;
        Self {
            facing,
            short,
            kind,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<piston_head, u16>> = Lazy::new(|| {
            piston_head::possible_states()
                .into_iter()
                .zip(
                    [
                        1059u16, 1060u16, 1061u16, 1062u16, 1063u16, 1064u16, 1065u16, 1066u16,
                        1067u16, 1068u16, 1069u16, 1070u16, 1071u16, 1072u16, 1073u16, 1074u16,
                        1075u16, 1076u16, 1077u16, 1078u16, 1079u16, 1080u16, 1081u16, 1082u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteWool {}
impl WhiteWool {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeWool {}
impl OrangeWool {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaWool {}
impl MagentaWool {
    pub fn possible_states() -> Vec<Self> {
        vec![MagentaWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magenta_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueWool {}
impl LightBlueWool {
    pub fn possible_states() -> Vec<Self> {
        vec![LightBlueWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_blue_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowWool {}
impl YellowWool {
    pub fn possible_states() -> Vec<Self> {
        vec![YellowWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(yellow_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeWool {}
impl LimeWool {
    pub fn possible_states() -> Vec<Self> {
        vec![LimeWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lime_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkWool {}
impl PinkWool {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayWool {}
impl GrayWool {
    pub fn possible_states() -> Vec<Self> {
        vec![GrayWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gray_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayWool {}
impl LightGrayWool {
    pub fn possible_states() -> Vec<Self> {
        vec![LightGrayWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_gray_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanWool {}
impl CyanWool {
    pub fn possible_states() -> Vec<Self> {
        vec![CyanWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cyan_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleWool {}
impl PurpleWool {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpleWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purple_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueWool {}
impl BlueWool {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownWool {}
impl BrownWool {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenWool {}
impl GreenWool {
    pub fn possible_states() -> Vec<Self> {
        vec![GreenWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(green_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedWool {}
impl RedWool {
    pub fn possible_states() -> Vec<Self> {
        vec![RedWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackWool {}
impl BlackWool {
    pub fn possible_states() -> Vec<Self> {
        vec![BlackWool {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(black_wool {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MovingPiston {
    facing: MovingPiston,
    kind: MovingPiston,
}
impl MovingPiston {
    pub fn possible_states() -> Vec<Self> {
        [
            MovingPiston::North,
            MovingPiston::East,
            MovingPiston::South,
            MovingPiston::West,
            MovingPiston::Up,
            MovingPiston::Down,
        ]
        .iter()
        .copied()
        .flat_map(|state| {
            std::iter::repeat(state)
                .zip([MovingPiston::Normal, MovingPiston::Sticky].iter().copied())
        })
        .map(|(facing, kind)| MovingPiston { facing, kind })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.kind as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = MovingPiston::try_from(facing)?;
        let kind = offset / 1u16;
        offset -= kind * 1u16;
        let kind = MovingPiston::try_from(kind)?;
        Self { facing, kind }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<moving_piston, u16>> = Lazy::new(|| {
            moving_piston::possible_states()
                .into_iter()
                .zip(
                    [
                        1099u16, 1100u16, 1101u16, 1102u16, 1103u16, 1104u16, 1105u16, 1106u16,
                        1107u16, 1108u16, 1109u16, 1110u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dandelion {}
impl Dandelion {
    pub fn possible_states() -> Vec<Self> {
        vec![Dandelion {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dandelion {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Poppy {}
impl Poppy {
    pub fn possible_states() -> Vec<Self> {
        vec![Poppy {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(poppy {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueOrchid {}
impl BlueOrchid {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueOrchid {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_orchid {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Allium {}
impl Allium {
    pub fn possible_states() -> Vec<Self> {
        vec![Allium {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(allium {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AzureBluet {}
impl AzureBluet {
    pub fn possible_states() -> Vec<Self> {
        vec![AzureBluet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(azure_bluet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedTulip {}
impl RedTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![RedTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeTulip {}
impl OrangeTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteTulip {}
impl WhiteTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkTulip {}
impl PinkTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OxeyeDaisy {}
impl OxeyeDaisy {
    pub fn possible_states() -> Vec<Self> {
        vec![OxeyeDaisy {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(oxeye_daisy {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownMushroom {}
impl BrownMushroom {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownMushroom {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_mushroom {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedMushroom {}
impl RedMushroom {
    pub fn possible_states() -> Vec<Self> {
        vec![RedMushroom {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_mushroom {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GoldBlock {}
impl GoldBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![GoldBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gold_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IronBlock {}
impl IronBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![IronBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(iron_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bricks {}
impl Bricks {
    pub fn possible_states() -> Vec<Self> {
        vec![Bricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tnt {
    unstable: bool,
}
impl Tnt {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|unstable| Tnt { unstable })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.unstable as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let unstable = offset / 1u16;
        offset -= unstable * 1u16;
        let unstable = Ok(if unstable == 0 { false } else { true })?;
        Self { unstable }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tnt, u16>> = Lazy::new(|| {
            tnt::possible_states()
                .into_iter()
                .zip([1126u16, 1127u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bookshelf {}
impl Bookshelf {
    pub fn possible_states() -> Vec<Self> {
        vec![Bookshelf {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(bookshelf {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MossyCobblestone {}
impl MossyCobblestone {
    pub fn possible_states() -> Vec<Self> {
        vec![MossyCobblestone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(mossy_cobblestone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Obsidian {}
impl Obsidian {
    pub fn possible_states() -> Vec<Self> {
        vec![Obsidian {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(obsidian {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Torch {}
impl Torch {
    pub fn possible_states() -> Vec<Self> {
        vec![Torch {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(torch {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WallTorch {
    facing: WallTorch,
}
impl WallTorch {
    pub fn possible_states() -> Vec<Self> {
        [
            WallTorch::North,
            WallTorch::South,
            WallTorch::West,
            WallTorch::East,
        ]
        .iter()
        .copied()
        .map(|facing| WallTorch { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = WallTorch::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<wall_torch, u16>> = Lazy::new(|| {
            wall_torch::possible_states()
                .into_iter()
                .zip([1132u16, 1133u16, 1134u16, 1135u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fire {
    south: bool,
    age: i32,
    east: bool,
    up: bool,
    north: bool,
    west: bool,
}
impl Fire {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 0i32..=15i32 }))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((((south, age), east), up), north), west)| Fire {
                south,
                age,
                east,
                up,
                north,
                west,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 256u16
            + self.age as u16 * 16u16
            + self.east as u16 * 8u16
            + self.up as u16 * 4u16
            + self.north as u16 * 2u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 256u16;
        offset -= south * 256u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let age = offset / 16u16;
        offset -= age * 16u16;
        let age = Ok(age as i32)?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let up = offset / 4u16;
        offset -= up * 4u16;
        let up = Ok(if up == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = Ok(if west == 0 { false } else { true })?;
        Self {
            south,
            age,
            east,
            up,
            north,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<fire, u16>> = Lazy::new(|| {
            fire::possible_states()
                .into_iter()
                .zip(
                    [
                        1136u16, 1137u16, 1138u16, 1139u16, 1140u16, 1141u16, 1142u16, 1143u16,
                        1144u16, 1145u16, 1146u16, 1147u16, 1148u16, 1149u16, 1150u16, 1151u16,
                        1152u16, 1153u16, 1154u16, 1155u16, 1156u16, 1157u16, 1158u16, 1159u16,
                        1160u16, 1161u16, 1162u16, 1163u16, 1164u16, 1165u16, 1166u16, 1167u16,
                        1168u16, 1169u16, 1170u16, 1171u16, 1172u16, 1173u16, 1174u16, 1175u16,
                        1176u16, 1177u16, 1178u16, 1179u16, 1180u16, 1181u16, 1182u16, 1183u16,
                        1184u16, 1185u16, 1186u16, 1187u16, 1188u16, 1189u16, 1190u16, 1191u16,
                        1192u16, 1193u16, 1194u16, 1195u16, 1196u16, 1197u16, 1198u16, 1199u16,
                        1200u16, 1201u16, 1202u16, 1203u16, 1204u16, 1205u16, 1206u16, 1207u16,
                        1208u16, 1209u16, 1210u16, 1211u16, 1212u16, 1213u16, 1214u16, 1215u16,
                        1216u16, 1217u16, 1218u16, 1219u16, 1220u16, 1221u16, 1222u16, 1223u16,
                        1224u16, 1225u16, 1226u16, 1227u16, 1228u16, 1229u16, 1230u16, 1231u16,
                        1232u16, 1233u16, 1234u16, 1235u16, 1236u16, 1237u16, 1238u16, 1239u16,
                        1240u16, 1241u16, 1242u16, 1243u16, 1244u16, 1245u16, 1246u16, 1247u16,
                        1248u16, 1249u16, 1250u16, 1251u16, 1252u16, 1253u16, 1254u16, 1255u16,
                        1256u16, 1257u16, 1258u16, 1259u16, 1260u16, 1261u16, 1262u16, 1263u16,
                        1264u16, 1265u16, 1266u16, 1267u16, 1268u16, 1269u16, 1270u16, 1271u16,
                        1272u16, 1273u16, 1274u16, 1275u16, 1276u16, 1277u16, 1278u16, 1279u16,
                        1280u16, 1281u16, 1282u16, 1283u16, 1284u16, 1285u16, 1286u16, 1287u16,
                        1288u16, 1289u16, 1290u16, 1291u16, 1292u16, 1293u16, 1294u16, 1295u16,
                        1296u16, 1297u16, 1298u16, 1299u16, 1300u16, 1301u16, 1302u16, 1303u16,
                        1304u16, 1305u16, 1306u16, 1307u16, 1308u16, 1309u16, 1310u16, 1311u16,
                        1312u16, 1313u16, 1314u16, 1315u16, 1316u16, 1317u16, 1318u16, 1319u16,
                        1320u16, 1321u16, 1322u16, 1323u16, 1324u16, 1325u16, 1326u16, 1327u16,
                        1328u16, 1329u16, 1330u16, 1331u16, 1332u16, 1333u16, 1334u16, 1335u16,
                        1336u16, 1337u16, 1338u16, 1339u16, 1340u16, 1341u16, 1342u16, 1343u16,
                        1344u16, 1345u16, 1346u16, 1347u16, 1348u16, 1349u16, 1350u16, 1351u16,
                        1352u16, 1353u16, 1354u16, 1355u16, 1356u16, 1357u16, 1358u16, 1359u16,
                        1360u16, 1361u16, 1362u16, 1363u16, 1364u16, 1365u16, 1366u16, 1367u16,
                        1368u16, 1369u16, 1370u16, 1371u16, 1372u16, 1373u16, 1374u16, 1375u16,
                        1376u16, 1377u16, 1378u16, 1379u16, 1380u16, 1381u16, 1382u16, 1383u16,
                        1384u16, 1385u16, 1386u16, 1387u16, 1388u16, 1389u16, 1390u16, 1391u16,
                        1392u16, 1393u16, 1394u16, 1395u16, 1396u16, 1397u16, 1398u16, 1399u16,
                        1400u16, 1401u16, 1402u16, 1403u16, 1404u16, 1405u16, 1406u16, 1407u16,
                        1408u16, 1409u16, 1410u16, 1411u16, 1412u16, 1413u16, 1414u16, 1415u16,
                        1416u16, 1417u16, 1418u16, 1419u16, 1420u16, 1421u16, 1422u16, 1423u16,
                        1424u16, 1425u16, 1426u16, 1427u16, 1428u16, 1429u16, 1430u16, 1431u16,
                        1432u16, 1433u16, 1434u16, 1435u16, 1436u16, 1437u16, 1438u16, 1439u16,
                        1440u16, 1441u16, 1442u16, 1443u16, 1444u16, 1445u16, 1446u16, 1447u16,
                        1448u16, 1449u16, 1450u16, 1451u16, 1452u16, 1453u16, 1454u16, 1455u16,
                        1456u16, 1457u16, 1458u16, 1459u16, 1460u16, 1461u16, 1462u16, 1463u16,
                        1464u16, 1465u16, 1466u16, 1467u16, 1468u16, 1469u16, 1470u16, 1471u16,
                        1472u16, 1473u16, 1474u16, 1475u16, 1476u16, 1477u16, 1478u16, 1479u16,
                        1480u16, 1481u16, 1482u16, 1483u16, 1484u16, 1485u16, 1486u16, 1487u16,
                        1488u16, 1489u16, 1490u16, 1491u16, 1492u16, 1493u16, 1494u16, 1495u16,
                        1496u16, 1497u16, 1498u16, 1499u16, 1500u16, 1501u16, 1502u16, 1503u16,
                        1504u16, 1505u16, 1506u16, 1507u16, 1508u16, 1509u16, 1510u16, 1511u16,
                        1512u16, 1513u16, 1514u16, 1515u16, 1516u16, 1517u16, 1518u16, 1519u16,
                        1520u16, 1521u16, 1522u16, 1523u16, 1524u16, 1525u16, 1526u16, 1527u16,
                        1528u16, 1529u16, 1530u16, 1531u16, 1532u16, 1533u16, 1534u16, 1535u16,
                        1536u16, 1537u16, 1538u16, 1539u16, 1540u16, 1541u16, 1542u16, 1543u16,
                        1544u16, 1545u16, 1546u16, 1547u16, 1548u16, 1549u16, 1550u16, 1551u16,
                        1552u16, 1553u16, 1554u16, 1555u16, 1556u16, 1557u16, 1558u16, 1559u16,
                        1560u16, 1561u16, 1562u16, 1563u16, 1564u16, 1565u16, 1566u16, 1567u16,
                        1568u16, 1569u16, 1570u16, 1571u16, 1572u16, 1573u16, 1574u16, 1575u16,
                        1576u16, 1577u16, 1578u16, 1579u16, 1580u16, 1581u16, 1582u16, 1583u16,
                        1584u16, 1585u16, 1586u16, 1587u16, 1588u16, 1589u16, 1590u16, 1591u16,
                        1592u16, 1593u16, 1594u16, 1595u16, 1596u16, 1597u16, 1598u16, 1599u16,
                        1600u16, 1601u16, 1602u16, 1603u16, 1604u16, 1605u16, 1606u16, 1607u16,
                        1608u16, 1609u16, 1610u16, 1611u16, 1612u16, 1613u16, 1614u16, 1615u16,
                        1616u16, 1617u16, 1618u16, 1619u16, 1620u16, 1621u16, 1622u16, 1623u16,
                        1624u16, 1625u16, 1626u16, 1627u16, 1628u16, 1629u16, 1630u16, 1631u16,
                        1632u16, 1633u16, 1634u16, 1635u16, 1636u16, 1637u16, 1638u16, 1639u16,
                        1640u16, 1641u16, 1642u16, 1643u16, 1644u16, 1645u16, 1646u16, 1647u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Spawner {}
impl Spawner {
    pub fn possible_states() -> Vec<Self> {
        vec![Spawner {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(spawner {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chest {
    waterlogged: bool,
    facing: Chest,
    kind: Chest,
}
impl Chest {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [Chest::North, Chest::South, Chest::West, Chest::East]
                        .iter()
                        .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state)
                    .zip([Chest::Single, Chest::Left, Chest::Right].iter().copied())
            })
            .map(|((waterlogged, facing), kind)| Chest {
                waterlogged,
                facing,
                kind,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 12u16 + self.facing as u16 * 3u16 + self.kind as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 12u16;
        offset -= waterlogged * 12u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 3u16;
        offset -= facing * 3u16;
        let facing = Chest::try_from(facing)?;
        let kind = offset / 1u16;
        offset -= kind * 1u16;
        let kind = Chest::try_from(kind)?;
        Self {
            waterlogged,
            facing,
            kind,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<chest, u16>> = Lazy::new(|| {
            chest::possible_states()
                .into_iter()
                .zip(
                    [
                        1729u16, 1730u16, 1731u16, 1732u16, 1733u16, 1734u16, 1735u16, 1736u16,
                        1737u16, 1738u16, 1739u16, 1740u16, 1741u16, 1742u16, 1743u16, 1744u16,
                        1745u16, 1746u16, 1747u16, 1748u16, 1749u16, 1750u16, 1751u16, 1752u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedstoneWire {
    south: RedstoneWire,
    power: i32,
    east: RedstoneWire,
    north: RedstoneWire,
    west: RedstoneWire,
}
impl RedstoneWire {
    pub fn possible_states() -> Vec<Self> {
        [RedstoneWire::Up, RedstoneWire::Side, RedstoneWire::None]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 0i32..=15i32 }))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [RedstoneWire::Up, RedstoneWire::Side, RedstoneWire::None]
                        .iter()
                        .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [RedstoneWire::Up, RedstoneWire::Side, RedstoneWire::None]
                        .iter()
                        .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [RedstoneWire::Up, RedstoneWire::Side, RedstoneWire::None]
                        .iter()
                        .copied(),
                )
            })
            .map(|((((south, power), east), north), west)| RedstoneWire {
                south,
                power,
                east,
                north,
                west,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 432u16
            + self.power as u16 * 27u16
            + self.east as u16 * 9u16
            + self.north as u16 * 3u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 432u16;
        offset -= south * 432u16;
        let south = RedstoneWire::try_from(south)?;
        let power = offset / 27u16;
        offset -= power * 27u16;
        let power = Ok(power as i32)?;
        let east = offset / 9u16;
        offset -= east * 9u16;
        let east = RedstoneWire::try_from(east)?;
        let north = offset / 3u16;
        offset -= north * 3u16;
        let north = RedstoneWire::try_from(north)?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = RedstoneWire::try_from(west)?;
        Self {
            south,
            power,
            east,
            north,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<redstone_wire, u16>> = Lazy::new(|| {
            redstone_wire::possible_states()
                .into_iter()
                .zip(
                    [
                        1753u16, 1754u16, 1755u16, 1756u16, 1757u16, 1758u16, 1759u16, 1760u16,
                        1761u16, 1762u16, 1763u16, 1764u16, 1765u16, 1766u16, 1767u16, 1768u16,
                        1769u16, 1770u16, 1771u16, 1772u16, 1773u16, 1774u16, 1775u16, 1776u16,
                        1777u16, 1778u16, 1779u16, 1780u16, 1781u16, 1782u16, 1783u16, 1784u16,
                        1785u16, 1786u16, 1787u16, 1788u16, 1789u16, 1790u16, 1791u16, 1792u16,
                        1793u16, 1794u16, 1795u16, 1796u16, 1797u16, 1798u16, 1799u16, 1800u16,
                        1801u16, 1802u16, 1803u16, 1804u16, 1805u16, 1806u16, 1807u16, 1808u16,
                        1809u16, 1810u16, 1811u16, 1812u16, 1813u16, 1814u16, 1815u16, 1816u16,
                        1817u16, 1818u16, 1819u16, 1820u16, 1821u16, 1822u16, 1823u16, 1824u16,
                        1825u16, 1826u16, 1827u16, 1828u16, 1829u16, 1830u16, 1831u16, 1832u16,
                        1833u16, 1834u16, 1835u16, 1836u16, 1837u16, 1838u16, 1839u16, 1840u16,
                        1841u16, 1842u16, 1843u16, 1844u16, 1845u16, 1846u16, 1847u16, 1848u16,
                        1849u16, 1850u16, 1851u16, 1852u16, 1853u16, 1854u16, 1855u16, 1856u16,
                        1857u16, 1858u16, 1859u16, 1860u16, 1861u16, 1862u16, 1863u16, 1864u16,
                        1865u16, 1866u16, 1867u16, 1868u16, 1869u16, 1870u16, 1871u16, 1872u16,
                        1873u16, 1874u16, 1875u16, 1876u16, 1877u16, 1878u16, 1879u16, 1880u16,
                        1881u16, 1882u16, 1883u16, 1884u16, 1885u16, 1886u16, 1887u16, 1888u16,
                        1889u16, 1890u16, 1891u16, 1892u16, 1893u16, 1894u16, 1895u16, 1896u16,
                        1897u16, 1898u16, 1899u16, 1900u16, 1901u16, 1902u16, 1903u16, 1904u16,
                        1905u16, 1906u16, 1907u16, 1908u16, 1909u16, 1910u16, 1911u16, 1912u16,
                        1913u16, 1914u16, 1915u16, 1916u16, 1917u16, 1918u16, 1919u16, 1920u16,
                        1921u16, 1922u16, 1923u16, 1924u16, 1925u16, 1926u16, 1927u16, 1928u16,
                        1929u16, 1930u16, 1931u16, 1932u16, 1933u16, 1934u16, 1935u16, 1936u16,
                        1937u16, 1938u16, 1939u16, 1940u16, 1941u16, 1942u16, 1943u16, 1944u16,
                        1945u16, 1946u16, 1947u16, 1948u16, 1949u16, 1950u16, 1951u16, 1952u16,
                        1953u16, 1954u16, 1955u16, 1956u16, 1957u16, 1958u16, 1959u16, 1960u16,
                        1961u16, 1962u16, 1963u16, 1964u16, 1965u16, 1966u16, 1967u16, 1968u16,
                        1969u16, 1970u16, 1971u16, 1972u16, 1973u16, 1974u16, 1975u16, 1976u16,
                        1977u16, 1978u16, 1979u16, 1980u16, 1981u16, 1982u16, 1983u16, 1984u16,
                        1985u16, 1986u16, 1987u16, 1988u16, 1989u16, 1990u16, 1991u16, 1992u16,
                        1993u16, 1994u16, 1995u16, 1996u16, 1997u16, 1998u16, 1999u16, 2000u16,
                        2001u16, 2002u16, 2003u16, 2004u16, 2005u16, 2006u16, 2007u16, 2008u16,
                        2009u16, 2010u16, 2011u16, 2012u16, 2013u16, 2014u16, 2015u16, 2016u16,
                        2017u16, 2018u16, 2019u16, 2020u16, 2021u16, 2022u16, 2023u16, 2024u16,
                        2025u16, 2026u16, 2027u16, 2028u16, 2029u16, 2030u16, 2031u16, 2032u16,
                        2033u16, 2034u16, 2035u16, 2036u16, 2037u16, 2038u16, 2039u16, 2040u16,
                        2041u16, 2042u16, 2043u16, 2044u16, 2045u16, 2046u16, 2047u16, 2048u16,
                        2049u16, 2050u16, 2051u16, 2052u16, 2053u16, 2054u16, 2055u16, 2056u16,
                        2057u16, 2058u16, 2059u16, 2060u16, 2061u16, 2062u16, 2063u16, 2064u16,
                        2065u16, 2066u16, 2067u16, 2068u16, 2069u16, 2070u16, 2071u16, 2072u16,
                        2073u16, 2074u16, 2075u16, 2076u16, 2077u16, 2078u16, 2079u16, 2080u16,
                        2081u16, 2082u16, 2083u16, 2084u16, 2085u16, 2086u16, 2087u16, 2088u16,
                        2089u16, 2090u16, 2091u16, 2092u16, 2093u16, 2094u16, 2095u16, 2096u16,
                        2097u16, 2098u16, 2099u16, 2100u16, 2101u16, 2102u16, 2103u16, 2104u16,
                        2105u16, 2106u16, 2107u16, 2108u16, 2109u16, 2110u16, 2111u16, 2112u16,
                        2113u16, 2114u16, 2115u16, 2116u16, 2117u16, 2118u16, 2119u16, 2120u16,
                        2121u16, 2122u16, 2123u16, 2124u16, 2125u16, 2126u16, 2127u16, 2128u16,
                        2129u16, 2130u16, 2131u16, 2132u16, 2133u16, 2134u16, 2135u16, 2136u16,
                        2137u16, 2138u16, 2139u16, 2140u16, 2141u16, 2142u16, 2143u16, 2144u16,
                        2145u16, 2146u16, 2147u16, 2148u16, 2149u16, 2150u16, 2151u16, 2152u16,
                        2153u16, 2154u16, 2155u16, 2156u16, 2157u16, 2158u16, 2159u16, 2160u16,
                        2161u16, 2162u16, 2163u16, 2164u16, 2165u16, 2166u16, 2167u16, 2168u16,
                        2169u16, 2170u16, 2171u16, 2172u16, 2173u16, 2174u16, 2175u16, 2176u16,
                        2177u16, 2178u16, 2179u16, 2180u16, 2181u16, 2182u16, 2183u16, 2184u16,
                        2185u16, 2186u16, 2187u16, 2188u16, 2189u16, 2190u16, 2191u16, 2192u16,
                        2193u16, 2194u16, 2195u16, 2196u16, 2197u16, 2198u16, 2199u16, 2200u16,
                        2201u16, 2202u16, 2203u16, 2204u16, 2205u16, 2206u16, 2207u16, 2208u16,
                        2209u16, 2210u16, 2211u16, 2212u16, 2213u16, 2214u16, 2215u16, 2216u16,
                        2217u16, 2218u16, 2219u16, 2220u16, 2221u16, 2222u16, 2223u16, 2224u16,
                        2225u16, 2226u16, 2227u16, 2228u16, 2229u16, 2230u16, 2231u16, 2232u16,
                        2233u16, 2234u16, 2235u16, 2236u16, 2237u16, 2238u16, 2239u16, 2240u16,
                        2241u16, 2242u16, 2243u16, 2244u16, 2245u16, 2246u16, 2247u16, 2248u16,
                        2249u16, 2250u16, 2251u16, 2252u16, 2253u16, 2254u16, 2255u16, 2256u16,
                        2257u16, 2258u16, 2259u16, 2260u16, 2261u16, 2262u16, 2263u16, 2264u16,
                        2265u16, 2266u16, 2267u16, 2268u16, 2269u16, 2270u16, 2271u16, 2272u16,
                        2273u16, 2274u16, 2275u16, 2276u16, 2277u16, 2278u16, 2279u16, 2280u16,
                        2281u16, 2282u16, 2283u16, 2284u16, 2285u16, 2286u16, 2287u16, 2288u16,
                        2289u16, 2290u16, 2291u16, 2292u16, 2293u16, 2294u16, 2295u16, 2296u16,
                        2297u16, 2298u16, 2299u16, 2300u16, 2301u16, 2302u16, 2303u16, 2304u16,
                        2305u16, 2306u16, 2307u16, 2308u16, 2309u16, 2310u16, 2311u16, 2312u16,
                        2313u16, 2314u16, 2315u16, 2316u16, 2317u16, 2318u16, 2319u16, 2320u16,
                        2321u16, 2322u16, 2323u16, 2324u16, 2325u16, 2326u16, 2327u16, 2328u16,
                        2329u16, 2330u16, 2331u16, 2332u16, 2333u16, 2334u16, 2335u16, 2336u16,
                        2337u16, 2338u16, 2339u16, 2340u16, 2341u16, 2342u16, 2343u16, 2344u16,
                        2345u16, 2346u16, 2347u16, 2348u16, 2349u16, 2350u16, 2351u16, 2352u16,
                        2353u16, 2354u16, 2355u16, 2356u16, 2357u16, 2358u16, 2359u16, 2360u16,
                        2361u16, 2362u16, 2363u16, 2364u16, 2365u16, 2366u16, 2367u16, 2368u16,
                        2369u16, 2370u16, 2371u16, 2372u16, 2373u16, 2374u16, 2375u16, 2376u16,
                        2377u16, 2378u16, 2379u16, 2380u16, 2381u16, 2382u16, 2383u16, 2384u16,
                        2385u16, 2386u16, 2387u16, 2388u16, 2389u16, 2390u16, 2391u16, 2392u16,
                        2393u16, 2394u16, 2395u16, 2396u16, 2397u16, 2398u16, 2399u16, 2400u16,
                        2401u16, 2402u16, 2403u16, 2404u16, 2405u16, 2406u16, 2407u16, 2408u16,
                        2409u16, 2410u16, 2411u16, 2412u16, 2413u16, 2414u16, 2415u16, 2416u16,
                        2417u16, 2418u16, 2419u16, 2420u16, 2421u16, 2422u16, 2423u16, 2424u16,
                        2425u16, 2426u16, 2427u16, 2428u16, 2429u16, 2430u16, 2431u16, 2432u16,
                        2433u16, 2434u16, 2435u16, 2436u16, 2437u16, 2438u16, 2439u16, 2440u16,
                        2441u16, 2442u16, 2443u16, 2444u16, 2445u16, 2446u16, 2447u16, 2448u16,
                        2449u16, 2450u16, 2451u16, 2452u16, 2453u16, 2454u16, 2455u16, 2456u16,
                        2457u16, 2458u16, 2459u16, 2460u16, 2461u16, 2462u16, 2463u16, 2464u16,
                        2465u16, 2466u16, 2467u16, 2468u16, 2469u16, 2470u16, 2471u16, 2472u16,
                        2473u16, 2474u16, 2475u16, 2476u16, 2477u16, 2478u16, 2479u16, 2480u16,
                        2481u16, 2482u16, 2483u16, 2484u16, 2485u16, 2486u16, 2487u16, 2488u16,
                        2489u16, 2490u16, 2491u16, 2492u16, 2493u16, 2494u16, 2495u16, 2496u16,
                        2497u16, 2498u16, 2499u16, 2500u16, 2501u16, 2502u16, 2503u16, 2504u16,
                        2505u16, 2506u16, 2507u16, 2508u16, 2509u16, 2510u16, 2511u16, 2512u16,
                        2513u16, 2514u16, 2515u16, 2516u16, 2517u16, 2518u16, 2519u16, 2520u16,
                        2521u16, 2522u16, 2523u16, 2524u16, 2525u16, 2526u16, 2527u16, 2528u16,
                        2529u16, 2530u16, 2531u16, 2532u16, 2533u16, 2534u16, 2535u16, 2536u16,
                        2537u16, 2538u16, 2539u16, 2540u16, 2541u16, 2542u16, 2543u16, 2544u16,
                        2545u16, 2546u16, 2547u16, 2548u16, 2549u16, 2550u16, 2551u16, 2552u16,
                        2553u16, 2554u16, 2555u16, 2556u16, 2557u16, 2558u16, 2559u16, 2560u16,
                        2561u16, 2562u16, 2563u16, 2564u16, 2565u16, 2566u16, 2567u16, 2568u16,
                        2569u16, 2570u16, 2571u16, 2572u16, 2573u16, 2574u16, 2575u16, 2576u16,
                        2577u16, 2578u16, 2579u16, 2580u16, 2581u16, 2582u16, 2583u16, 2584u16,
                        2585u16, 2586u16, 2587u16, 2588u16, 2589u16, 2590u16, 2591u16, 2592u16,
                        2593u16, 2594u16, 2595u16, 2596u16, 2597u16, 2598u16, 2599u16, 2600u16,
                        2601u16, 2602u16, 2603u16, 2604u16, 2605u16, 2606u16, 2607u16, 2608u16,
                        2609u16, 2610u16, 2611u16, 2612u16, 2613u16, 2614u16, 2615u16, 2616u16,
                        2617u16, 2618u16, 2619u16, 2620u16, 2621u16, 2622u16, 2623u16, 2624u16,
                        2625u16, 2626u16, 2627u16, 2628u16, 2629u16, 2630u16, 2631u16, 2632u16,
                        2633u16, 2634u16, 2635u16, 2636u16, 2637u16, 2638u16, 2639u16, 2640u16,
                        2641u16, 2642u16, 2643u16, 2644u16, 2645u16, 2646u16, 2647u16, 2648u16,
                        2649u16, 2650u16, 2651u16, 2652u16, 2653u16, 2654u16, 2655u16, 2656u16,
                        2657u16, 2658u16, 2659u16, 2660u16, 2661u16, 2662u16, 2663u16, 2664u16,
                        2665u16, 2666u16, 2667u16, 2668u16, 2669u16, 2670u16, 2671u16, 2672u16,
                        2673u16, 2674u16, 2675u16, 2676u16, 2677u16, 2678u16, 2679u16, 2680u16,
                        2681u16, 2682u16, 2683u16, 2684u16, 2685u16, 2686u16, 2687u16, 2688u16,
                        2689u16, 2690u16, 2691u16, 2692u16, 2693u16, 2694u16, 2695u16, 2696u16,
                        2697u16, 2698u16, 2699u16, 2700u16, 2701u16, 2702u16, 2703u16, 2704u16,
                        2705u16, 2706u16, 2707u16, 2708u16, 2709u16, 2710u16, 2711u16, 2712u16,
                        2713u16, 2714u16, 2715u16, 2716u16, 2717u16, 2718u16, 2719u16, 2720u16,
                        2721u16, 2722u16, 2723u16, 2724u16, 2725u16, 2726u16, 2727u16, 2728u16,
                        2729u16, 2730u16, 2731u16, 2732u16, 2733u16, 2734u16, 2735u16, 2736u16,
                        2737u16, 2738u16, 2739u16, 2740u16, 2741u16, 2742u16, 2743u16, 2744u16,
                        2745u16, 2746u16, 2747u16, 2748u16, 2749u16, 2750u16, 2751u16, 2752u16,
                        2753u16, 2754u16, 2755u16, 2756u16, 2757u16, 2758u16, 2759u16, 2760u16,
                        2761u16, 2762u16, 2763u16, 2764u16, 2765u16, 2766u16, 2767u16, 2768u16,
                        2769u16, 2770u16, 2771u16, 2772u16, 2773u16, 2774u16, 2775u16, 2776u16,
                        2777u16, 2778u16, 2779u16, 2780u16, 2781u16, 2782u16, 2783u16, 2784u16,
                        2785u16, 2786u16, 2787u16, 2788u16, 2789u16, 2790u16, 2791u16, 2792u16,
                        2793u16, 2794u16, 2795u16, 2796u16, 2797u16, 2798u16, 2799u16, 2800u16,
                        2801u16, 2802u16, 2803u16, 2804u16, 2805u16, 2806u16, 2807u16, 2808u16,
                        2809u16, 2810u16, 2811u16, 2812u16, 2813u16, 2814u16, 2815u16, 2816u16,
                        2817u16, 2818u16, 2819u16, 2820u16, 2821u16, 2822u16, 2823u16, 2824u16,
                        2825u16, 2826u16, 2827u16, 2828u16, 2829u16, 2830u16, 2831u16, 2832u16,
                        2833u16, 2834u16, 2835u16, 2836u16, 2837u16, 2838u16, 2839u16, 2840u16,
                        2841u16, 2842u16, 2843u16, 2844u16, 2845u16, 2846u16, 2847u16, 2848u16,
                        2849u16, 2850u16, 2851u16, 2852u16, 2853u16, 2854u16, 2855u16, 2856u16,
                        2857u16, 2858u16, 2859u16, 2860u16, 2861u16, 2862u16, 2863u16, 2864u16,
                        2865u16, 2866u16, 2867u16, 2868u16, 2869u16, 2870u16, 2871u16, 2872u16,
                        2873u16, 2874u16, 2875u16, 2876u16, 2877u16, 2878u16, 2879u16, 2880u16,
                        2881u16, 2882u16, 2883u16, 2884u16, 2885u16, 2886u16, 2887u16, 2888u16,
                        2889u16, 2890u16, 2891u16, 2892u16, 2893u16, 2894u16, 2895u16, 2896u16,
                        2897u16, 2898u16, 2899u16, 2900u16, 2901u16, 2902u16, 2903u16, 2904u16,
                        2905u16, 2906u16, 2907u16, 2908u16, 2909u16, 2910u16, 2911u16, 2912u16,
                        2913u16, 2914u16, 2915u16, 2916u16, 2917u16, 2918u16, 2919u16, 2920u16,
                        2921u16, 2922u16, 2923u16, 2924u16, 2925u16, 2926u16, 2927u16, 2928u16,
                        2929u16, 2930u16, 2931u16, 2932u16, 2933u16, 2934u16, 2935u16, 2936u16,
                        2937u16, 2938u16, 2939u16, 2940u16, 2941u16, 2942u16, 2943u16, 2944u16,
                        2945u16, 2946u16, 2947u16, 2948u16, 2949u16, 2950u16, 2951u16, 2952u16,
                        2953u16, 2954u16, 2955u16, 2956u16, 2957u16, 2958u16, 2959u16, 2960u16,
                        2961u16, 2962u16, 2963u16, 2964u16, 2965u16, 2966u16, 2967u16, 2968u16,
                        2969u16, 2970u16, 2971u16, 2972u16, 2973u16, 2974u16, 2975u16, 2976u16,
                        2977u16, 2978u16, 2979u16, 2980u16, 2981u16, 2982u16, 2983u16, 2984u16,
                        2985u16, 2986u16, 2987u16, 2988u16, 2989u16, 2990u16, 2991u16, 2992u16,
                        2993u16, 2994u16, 2995u16, 2996u16, 2997u16, 2998u16, 2999u16, 3000u16,
                        3001u16, 3002u16, 3003u16, 3004u16, 3005u16, 3006u16, 3007u16, 3008u16,
                        3009u16, 3010u16, 3011u16, 3012u16, 3013u16, 3014u16, 3015u16, 3016u16,
                        3017u16, 3018u16, 3019u16, 3020u16, 3021u16, 3022u16, 3023u16, 3024u16,
                        3025u16, 3026u16, 3027u16, 3028u16, 3029u16, 3030u16, 3031u16, 3032u16,
                        3033u16, 3034u16, 3035u16, 3036u16, 3037u16, 3038u16, 3039u16, 3040u16,
                        3041u16, 3042u16, 3043u16, 3044u16, 3045u16, 3046u16, 3047u16, 3048u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiamondOre {}
impl DiamondOre {
    pub fn possible_states() -> Vec<Self> {
        vec![DiamondOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(diamond_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiamondBlock {}
impl DiamondBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DiamondBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(diamond_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CraftingTable {}
impl CraftingTable {
    pub fn possible_states() -> Vec<Self> {
        vec![CraftingTable {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(crafting_table {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Wheat {
    age: i32,
}
impl Wheat {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=7i32 }.map(|age| Wheat { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<wheat, u16>> = Lazy::new(|| {
            wheat::possible_states()
                .into_iter()
                .zip(
                    [
                        3052u16, 3053u16, 3054u16, 3055u16, 3056u16, 3057u16, 3058u16, 3059u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Farmland {
    moisture: i32,
}
impl Farmland {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=7i32 }
            .map(|moisture| Farmland { moisture })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.moisture as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let moisture = offset / 1u16;
        offset -= moisture * 1u16;
        let moisture = Ok(moisture as i32)?;
        Self { moisture }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<farmland, u16>> = Lazy::new(|| {
            farmland::possible_states()
                .into_iter()
                .zip(
                    [
                        3060u16, 3061u16, 3062u16, 3063u16, 3064u16, 3065u16, 3066u16, 3067u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Furnace {
    lit: bool,
    facing: Furnace,
}
impl Furnace {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [Furnace::North, Furnace::South, Furnace::West, Furnace::East]
                        .iter()
                        .copied(),
                )
            })
            .map(|(lit, facing)| Furnace { lit, facing })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.lit as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let lit = offset / 4u16;
        offset -= lit * 4u16;
        let lit = Ok(if lit == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = Furnace::try_from(facing)?;
        Self { lit, facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<furnace, u16>> = Lazy::new(|| {
            furnace::possible_states()
                .into_iter()
                .zip(
                    [
                        3068u16, 3069u16, 3070u16, 3071u16, 3072u16, 3073u16, 3074u16, 3075u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sign {
    waterlogged: bool,
    rotation: i32,
}
impl Sign {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 0i32..=15i32 }))
            .map(|(waterlogged, rotation)| Sign {
                waterlogged,
                rotation,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 16u16 + self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 16u16;
        offset -= waterlogged * 16u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self {
            waterlogged,
            rotation,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<sign, u16>> = Lazy::new(|| {
            sign::possible_states()
                .into_iter()
                .zip(
                    [
                        3076u16, 3077u16, 3078u16, 3079u16, 3080u16, 3081u16, 3082u16, 3083u16,
                        3084u16, 3085u16, 3086u16, 3087u16, 3088u16, 3089u16, 3090u16, 3091u16,
                        3092u16, 3093u16, 3094u16, 3095u16, 3096u16, 3097u16, 3098u16, 3099u16,
                        3100u16, 3101u16, 3102u16, 3103u16, 3104u16, 3105u16, 3106u16, 3107u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OakDoor {
    half: OakDoor,
    powered: bool,
    facing: OakDoor,
    hinge: OakDoor,
    open: bool,
}
impl OakDoor {
    pub fn possible_states() -> Vec<Self> {
        [OakDoor::Upper, OakDoor::Lower]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [OakDoor::North, OakDoor::South, OakDoor::West, OakDoor::East]
                        .iter()
                        .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip([OakDoor::Left, OakDoor::Right].iter().copied())
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((((half, powered), facing), hinge), open)| OakDoor {
                half,
                powered,
                facing,
                hinge,
                open,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 32u16
            + self.powered as u16 * 16u16
            + self.facing as u16 * 4u16
            + self.hinge as u16 * 2u16
            + self.open as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 32u16;
        offset -= half * 32u16;
        let half = OakDoor::try_from(half)?;
        let powered = offset / 16u16;
        offset -= powered * 16u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 4u16;
        offset -= facing * 4u16;
        let facing = OakDoor::try_from(facing)?;
        let hinge = offset / 2u16;
        offset -= hinge * 2u16;
        let hinge = OakDoor::try_from(hinge)?;
        let open = offset / 1u16;
        offset -= open * 1u16;
        let open = Ok(if open == 0 { false } else { true })?;
        Self {
            half,
            powered,
            facing,
            hinge,
            open,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<oak_door, u16>> = Lazy::new(|| {
            oak_door::possible_states()
                .into_iter()
                .zip(
                    [
                        3108u16, 3109u16, 3110u16, 3111u16, 3112u16, 3113u16, 3114u16, 3115u16,
                        3116u16, 3117u16, 3118u16, 3119u16, 3120u16, 3121u16, 3122u16, 3123u16,
                        3124u16, 3125u16, 3126u16, 3127u16, 3128u16, 3129u16, 3130u16, 3131u16,
                        3132u16, 3133u16, 3134u16, 3135u16, 3136u16, 3137u16, 3138u16, 3139u16,
                        3140u16, 3141u16, 3142u16, 3143u16, 3144u16, 3145u16, 3146u16, 3147u16,
                        3148u16, 3149u16, 3150u16, 3151u16, 3152u16, 3153u16, 3154u16, 3155u16,
                        3156u16, 3157u16, 3158u16, 3159u16, 3160u16, 3161u16, 3162u16, 3163u16,
                        3164u16, 3165u16, 3166u16, 3167u16, 3168u16, 3169u16, 3170u16, 3171u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ladder {
    waterlogged: bool,
    facing: Ladder,
}
impl Ladder {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [Ladder::North, Ladder::South, Ladder::West, Ladder::East]
                        .iter()
                        .copied(),
                )
            })
            .map(|(waterlogged, facing)| Ladder {
                waterlogged,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = Ladder::try_from(facing)?;
        Self {
            waterlogged,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<ladder, u16>> = Lazy::new(|| {
            ladder::possible_states()
                .into_iter()
                .zip(
                    [
                        3172u16, 3173u16, 3174u16, 3175u16, 3176u16, 3177u16, 3178u16, 3179u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rail {
    shape: Rail,
}
impl Rail {
    pub fn possible_states() -> Vec<Self> {
        [
            Rail::NorthSouth,
            Rail::EastWest,
            Rail::AscendingEast,
            Rail::AscendingWest,
            Rail::AscendingNorth,
            Rail::AscendingSouth,
            Rail::SouthEast,
            Rail::SouthWest,
            Rail::NorthWest,
            Rail::NorthEast,
        ]
        .iter()
        .copied()
        .map(|shape| Rail { shape })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.shape as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let shape = offset / 1u16;
        offset -= shape * 1u16;
        let shape = Rail::try_from(shape)?;
        Self { shape }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<rail, u16>> = Lazy::new(|| {
            rail::possible_states()
                .into_iter()
                .zip(
                    [
                        3180u16, 3181u16, 3182u16, 3183u16, 3184u16, 3185u16, 3186u16, 3187u16,
                        3188u16, 3189u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WallSign {
    facing: WallSign,
    waterlogged: bool,
}
impl WallSign {
    pub fn possible_states() -> Vec<Self> {
        [
            WallSign::North,
            WallSign::South,
            WallSign::West,
            WallSign::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| WallSign {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = WallSign::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<wall_sign, u16>> = Lazy::new(|| {
            wall_sign::possible_states()
                .into_iter()
                .zip(
                    [
                        3270u16, 3271u16, 3272u16, 3273u16, 3274u16, 3275u16, 3276u16, 3277u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Lever {
    face: Lever,
    powered: bool,
    facing: Lever,
}
impl Lever {
    pub fn possible_states() -> Vec<Self> {
        [Lever::Floor, Lever::Wall, Lever::Ceiling]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [Lever::North, Lever::South, Lever::West, Lever::East]
                        .iter()
                        .copied(),
                )
            })
            .map(|((face, powered), facing)| Lever {
                face,
                powered,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.face as u16 * 8u16 + self.powered as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let face = offset / 8u16;
        offset -= face * 8u16;
        let face = Lever::try_from(face)?;
        let powered = offset / 4u16;
        offset -= powered * 4u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = Lever::try_from(facing)?;
        Self {
            face,
            powered,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lever, u16>> = Lazy::new(|| {
            lever::possible_states()
                .into_iter()
                .zip(
                    [
                        3278u16, 3279u16, 3280u16, 3281u16, 3282u16, 3283u16, 3284u16, 3285u16,
                        3286u16, 3287u16, 3288u16, 3289u16, 3290u16, 3291u16, 3292u16, 3293u16,
                        3294u16, 3295u16, 3296u16, 3297u16, 3298u16, 3299u16, 3300u16, 3301u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StonePressurePlate {
    powered: bool,
}
impl StonePressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| StonePressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<stone_pressure_plate, u16>> = Lazy::new(|| {
            stone_pressure_plate::possible_states()
                .into_iter()
                .zip([3302u16, 3303u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IronDoor {
    half: IronDoor,
    hinge: IronDoor,
    facing: IronDoor,
    open: bool,
    powered: bool,
}
impl IronDoor {
    pub fn possible_states() -> Vec<Self> {
        [IronDoor::Upper, IronDoor::Lower]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip([IronDoor::Left, IronDoor::Right].iter().copied())
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        IronDoor::North,
                        IronDoor::South,
                        IronDoor::West,
                        IronDoor::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((((half, hinge), facing), open), powered)| IronDoor {
                half,
                hinge,
                facing,
                open,
                powered,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 32u16
            + self.hinge as u16 * 16u16
            + self.facing as u16 * 4u16
            + self.open as u16 * 2u16
            + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 32u16;
        offset -= half * 32u16;
        let half = IronDoor::try_from(half)?;
        let hinge = offset / 16u16;
        offset -= hinge * 16u16;
        let hinge = IronDoor::try_from(hinge)?;
        let facing = offset / 4u16;
        offset -= facing * 4u16;
        let facing = IronDoor::try_from(facing)?;
        let open = offset / 2u16;
        offset -= open * 2u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self {
            half,
            hinge,
            facing,
            open,
            powered,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<iron_door, u16>> = Lazy::new(|| {
            iron_door::possible_states()
                .into_iter()
                .zip(
                    [
                        3304u16, 3305u16, 3306u16, 3307u16, 3308u16, 3309u16, 3310u16, 3311u16,
                        3312u16, 3313u16, 3314u16, 3315u16, 3316u16, 3317u16, 3318u16, 3319u16,
                        3320u16, 3321u16, 3322u16, 3323u16, 3324u16, 3325u16, 3326u16, 3327u16,
                        3328u16, 3329u16, 3330u16, 3331u16, 3332u16, 3333u16, 3334u16, 3335u16,
                        3336u16, 3337u16, 3338u16, 3339u16, 3340u16, 3341u16, 3342u16, 3343u16,
                        3344u16, 3345u16, 3346u16, 3347u16, 3348u16, 3349u16, 3350u16, 3351u16,
                        3352u16, 3353u16, 3354u16, 3355u16, 3356u16, 3357u16, 3358u16, 3359u16,
                        3360u16, 3361u16, 3362u16, 3363u16, 3364u16, 3365u16, 3366u16, 3367u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OakPressurePlate {
    powered: bool,
}
impl OakPressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| OakPressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<oak_pressure_plate, u16>> = Lazy::new(|| {
            oak_pressure_plate::possible_states()
                .into_iter()
                .zip([3368u16, 3369u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SprucePressurePlate {
    powered: bool,
}
impl SprucePressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| SprucePressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<spruce_pressure_plate, u16>> = Lazy::new(|| {
            spruce_pressure_plate::possible_states()
                .into_iter()
                .zip([3370u16, 3371u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BirchPressurePlate {
    powered: bool,
}
impl BirchPressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| BirchPressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<birch_pressure_plate, u16>> = Lazy::new(|| {
            birch_pressure_plate::possible_states()
                .into_iter()
                .zip([3372u16, 3373u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JunglePressurePlate {
    powered: bool,
}
impl JunglePressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| JunglePressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<jungle_pressure_plate, u16>> = Lazy::new(|| {
            jungle_pressure_plate::possible_states()
                .into_iter()
                .zip([3374u16, 3375u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AcaciaPressurePlate {
    powered: bool,
}
impl AcaciaPressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| AcaciaPressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<acacia_pressure_plate, u16>> = Lazy::new(|| {
            acacia_pressure_plate::possible_states()
                .into_iter()
                .zip([3376u16, 3377u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DarkOakPressurePlate {
    powered: bool,
}
impl DarkOakPressurePlate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|powered| DarkOakPressurePlate { powered })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dark_oak_pressure_plate, u16>> = Lazy::new(|| {
            dark_oak_pressure_plate::possible_states()
                .into_iter()
                .zip([3378u16, 3379u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedstoneOre {
    lit: bool,
}
impl RedstoneOre {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|lit| RedstoneOre { lit })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.lit as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let lit = offset / 1u16;
        offset -= lit * 1u16;
        let lit = Ok(if lit == 0 { false } else { true })?;
        Self { lit }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<redstone_ore, u16>> = Lazy::new(|| {
            redstone_ore::possible_states()
                .into_iter()
                .zip([3380u16, 3381u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedstoneTorch {
    lit: bool,
}
impl RedstoneTorch {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|lit| RedstoneTorch { lit })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.lit as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let lit = offset / 1u16;
        offset -= lit * 1u16;
        let lit = Ok(if lit == 0 { false } else { true })?;
        Self { lit }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<redstone_torch, u16>> = Lazy::new(|| {
            redstone_torch::possible_states()
                .into_iter()
                .zip([3382u16, 3383u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedstoneWallTorch {
    facing: RedstoneWallTorch,
    lit: bool,
}
impl RedstoneWallTorch {
    pub fn possible_states() -> Vec<Self> {
        [
            RedstoneWallTorch::North,
            RedstoneWallTorch::South,
            RedstoneWallTorch::West,
            RedstoneWallTorch::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, lit)| RedstoneWallTorch { facing, lit })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.lit as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = RedstoneWallTorch::try_from(facing)?;
        let lit = offset / 1u16;
        offset -= lit * 1u16;
        let lit = Ok(if lit == 0 { false } else { true })?;
        Self { facing, lit }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<redstone_wall_torch, u16>> = Lazy::new(|| {
            redstone_wall_torch::possible_states()
                .into_iter()
                .zip(
                    [
                        3384u16, 3385u16, 3386u16, 3387u16, 3388u16, 3389u16, 3390u16, 3391u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StoneButton {
    facing: StoneButton,
    face: StoneButton,
    powered: bool,
}
impl StoneButton {
    pub fn possible_states() -> Vec<Self> {
        [
            StoneButton::North,
            StoneButton::South,
            StoneButton::West,
            StoneButton::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| {
            std::iter::repeat(state).zip(
                [StoneButton::Floor, StoneButton::Wall, StoneButton::Ceiling]
                    .iter()
                    .copied(),
            )
        })
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|((facing, face), powered)| StoneButton {
            facing,
            face,
            powered,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 6u16 + self.face as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 6u16;
        offset -= facing * 6u16;
        let facing = StoneButton::try_from(facing)?;
        let face = offset / 2u16;
        offset -= face * 2u16;
        let face = StoneButton::try_from(face)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self {
            facing,
            face,
            powered,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<stone_button, u16>> = Lazy::new(|| {
            stone_button::possible_states()
                .into_iter()
                .zip(
                    [
                        3392u16, 3393u16, 3394u16, 3395u16, 3396u16, 3397u16, 3398u16, 3399u16,
                        3400u16, 3401u16, 3402u16, 3403u16, 3404u16, 3405u16, 3406u16, 3407u16,
                        3408u16, 3409u16, 3410u16, 3411u16, 3412u16, 3413u16, 3414u16, 3415u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Snow {
    layers: i32,
}
impl Snow {
    pub fn possible_states() -> Vec<Self> {
        { 1i32..=8i32 }.map(|layers| Snow { layers }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.layers as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let layers = offset / 1u16;
        offset -= layers * 1u16;
        let layers = Ok(layers as i32)?;
        Self { layers }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<snow, u16>> = Lazy::new(|| {
            snow::possible_states()
                .into_iter()
                .zip(
                    [
                        3416u16, 3417u16, 3418u16, 3419u16, 3420u16, 3421u16, 3422u16, 3423u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ice {}
impl Ice {
    pub fn possible_states() -> Vec<Self> {
        vec![Ice {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(ice {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnowBlock {}
impl SnowBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![SnowBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(snow_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cactus {
    age: i32,
}
impl Cactus {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }.map(|age| Cactus { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cactus, u16>> = Lazy::new(|| {
            cactus::possible_states()
                .into_iter()
                .zip(
                    [
                        3426u16, 3427u16, 3428u16, 3429u16, 3430u16, 3431u16, 3432u16, 3433u16,
                        3434u16, 3435u16, 3436u16, 3437u16, 3438u16, 3439u16, 3440u16, 3441u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Clay {}
impl Clay {
    pub fn possible_states() -> Vec<Self> {
        vec![Clay {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(clay {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SugarCane {
    age: i32,
}
impl SugarCane {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }.map(|age| SugarCane { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<sugar_cane, u16>> = Lazy::new(|| {
            sugar_cane::possible_states()
                .into_iter()
                .zip(
                    [
                        3443u16, 3444u16, 3445u16, 3446u16, 3447u16, 3448u16, 3449u16, 3450u16,
                        3451u16, 3452u16, 3453u16, 3454u16, 3455u16, 3456u16, 3457u16, 3458u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Jukebox {
    has_record: bool,
}
impl Jukebox {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|has_record| Jukebox { has_record })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.has_record as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let has_record = offset / 1u16;
        offset -= has_record * 1u16;
        let has_record = Ok(if has_record == 0 { false } else { true })?;
        Self { has_record }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<jukebox, u16>> = Lazy::new(|| {
            jukebox::possible_states()
                .into_iter()
                .zip([3459u16, 3460u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pumpkin {}
impl Pumpkin {
    pub fn possible_states() -> Vec<Self> {
        vec![Pumpkin {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pumpkin {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Netherrack {}
impl Netherrack {
    pub fn possible_states() -> Vec<Self> {
        vec![Netherrack {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(netherrack {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SoulSand {}
impl SoulSand {
    pub fn possible_states() -> Vec<Self> {
        vec![SoulSand {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(soul_sand {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Glowstone {}
impl Glowstone {
    pub fn possible_states() -> Vec<Self> {
        vec![Glowstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(glowstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetherPortal {
    axis: NetherPortal,
}
impl NetherPortal {
    pub fn possible_states() -> Vec<Self> {
        [NetherPortal::X, NetherPortal::Z]
            .iter()
            .copied()
            .map(|axis| NetherPortal { axis })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.axis as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let axis = offset / 1u16;
        offset -= axis * 1u16;
        let axis = NetherPortal::try_from(axis)?;
        Self { axis }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<nether_portal, u16>> = Lazy::new(|| {
            nether_portal::possible_states()
                .into_iter()
                .zip([3497u16, 3498u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CarvedPumpkin {
    facing: CarvedPumpkin,
}
impl CarvedPumpkin {
    pub fn possible_states() -> Vec<Self> {
        [
            CarvedPumpkin::North,
            CarvedPumpkin::South,
            CarvedPumpkin::West,
            CarvedPumpkin::East,
        ]
        .iter()
        .copied()
        .map(|facing| CarvedPumpkin { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = CarvedPumpkin::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<carved_pumpkin, u16>> = Lazy::new(|| {
            carved_pumpkin::possible_states()
                .into_iter()
                .zip([3499u16, 3500u16, 3501u16, 3502u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JackOLantern {
    facing: JackOLantern,
}
impl JackOLantern {
    pub fn possible_states() -> Vec<Self> {
        [
            JackOLantern::North,
            JackOLantern::South,
            JackOLantern::West,
            JackOLantern::East,
        ]
        .iter()
        .copied()
        .map(|facing| JackOLantern { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = JackOLantern::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<jack_o_lantern, u16>> = Lazy::new(|| {
            jack_o_lantern::possible_states()
                .into_iter()
                .zip([3503u16, 3504u16, 3505u16, 3506u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cake {
    bites: i32,
}
impl Cake {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=6i32 }.map(|bites| Cake { bites }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.bites as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let bites = offset / 1u16;
        offset -= bites * 1u16;
        let bites = Ok(bites as i32)?;
        Self { bites }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cake, u16>> = Lazy::new(|| {
            cake::possible_states()
                .into_iter()
                .zip(
                    [
                        3507u16, 3508u16, 3509u16, 3510u16, 3511u16, 3512u16, 3513u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Repeater {
    powered: bool,
    delay: i32,
    facing: Repeater,
    locked: bool,
}
impl Repeater {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 1i32..=4i32 }))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        Repeater::North,
                        Repeater::South,
                        Repeater::West,
                        Repeater::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((powered, delay), facing), locked)| Repeater {
                powered,
                delay,
                facing,
                locked,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 32u16
            + self.delay as u16 * 8u16
            + self.facing as u16 * 2u16
            + self.locked as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 32u16;
        offset -= powered * 32u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let delay = offset / 8u16;
        offset -= delay * 8u16;
        let delay = Ok(delay as i32)?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = Repeater::try_from(facing)?;
        let locked = offset / 1u16;
        offset -= locked * 1u16;
        let locked = Ok(if locked == 0 { false } else { true })?;
        Self {
            powered,
            delay,
            facing,
            locked,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<repeater, u16>> = Lazy::new(|| {
            repeater::possible_states()
                .into_iter()
                .zip(
                    [
                        3514u16, 3515u16, 3516u16, 3517u16, 3518u16, 3519u16, 3520u16, 3521u16,
                        3522u16, 3523u16, 3524u16, 3525u16, 3526u16, 3527u16, 3528u16, 3529u16,
                        3530u16, 3531u16, 3532u16, 3533u16, 3534u16, 3535u16, 3536u16, 3537u16,
                        3538u16, 3539u16, 3540u16, 3541u16, 3542u16, 3543u16, 3544u16, 3545u16,
                        3546u16, 3547u16, 3548u16, 3549u16, 3550u16, 3551u16, 3552u16, 3553u16,
                        3554u16, 3555u16, 3556u16, 3557u16, 3558u16, 3559u16, 3560u16, 3561u16,
                        3562u16, 3563u16, 3564u16, 3565u16, 3566u16, 3567u16, 3568u16, 3569u16,
                        3570u16, 3571u16, 3572u16, 3573u16, 3574u16, 3575u16, 3576u16, 3577u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteStainedGlass {}
impl WhiteStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeStainedGlass {}
impl OrangeStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaStainedGlass {}
impl MagentaStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![MagentaStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magenta_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueStainedGlass {}
impl LightBlueStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![LightBlueStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_blue_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowStainedGlass {}
impl YellowStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![YellowStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(yellow_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeStainedGlass {}
impl LimeStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![LimeStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lime_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkStainedGlass {}
impl PinkStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayStainedGlass {}
impl GrayStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![GrayStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gray_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayStainedGlass {}
impl LightGrayStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![LightGrayStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_gray_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanStainedGlass {}
impl CyanStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![CyanStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cyan_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleStainedGlass {}
impl PurpleStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpleStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purple_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueStainedGlass {}
impl BlueStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownStainedGlass {}
impl BrownStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenStainedGlass {}
impl GreenStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![GreenStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(green_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedStainedGlass {}
impl RedStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![RedStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackStainedGlass {}
impl BlackStainedGlass {
    pub fn possible_states() -> Vec<Self> {
        vec![BlackStainedGlass {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(black_stained_glass {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfestedStone {}
impl InfestedStone {
    pub fn possible_states() -> Vec<Self> {
        vec![InfestedStone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(infested_stone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfestedCobblestone {}
impl InfestedCobblestone {
    pub fn possible_states() -> Vec<Self> {
        vec![InfestedCobblestone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(infested_cobblestone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfestedStoneBricks {}
impl InfestedStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![InfestedStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(infested_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfestedMossyStoneBricks {}
impl InfestedMossyStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![InfestedMossyStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(infested_mossy_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfestedCrackedStoneBricks {}
impl InfestedCrackedStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![InfestedCrackedStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(infested_cracked_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InfestedChiseledStoneBricks {}
impl InfestedChiseledStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![InfestedChiseledStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(infested_chiseled_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StoneBricks {}
impl StoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![StoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MossyStoneBricks {}
impl MossyStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![MossyStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(mossy_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CrackedStoneBricks {}
impl CrackedStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![CrackedStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cracked_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChiseledStoneBricks {}
impl ChiseledStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![ChiseledStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(chiseled_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownMushroomBlock {
    east: bool,
    down: bool,
    north: bool,
    up: bool,
    west: bool,
    south: bool,
}
impl BrownMushroomBlock {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |(((((east, down), north), up), west), south)| BrownMushroomBlock {
                    east,
                    down,
                    north,
                    up,
                    west,
                    south,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.east as u16 * 32u16
            + self.down as u16 * 16u16
            + self.north as u16 * 8u16
            + self.up as u16 * 4u16
            + self.west as u16 * 2u16
            + self.south as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let east = offset / 32u16;
        offset -= east * 32u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let down = offset / 16u16;
        offset -= down * 16u16;
        let down = Ok(if down == 0 { false } else { true })?;
        let north = offset / 8u16;
        offset -= north * 8u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let up = offset / 4u16;
        offset -= up * 4u16;
        let up = Ok(if up == 0 { false } else { true })?;
        let west = offset / 2u16;
        offset -= west * 2u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let south = offset / 1u16;
        offset -= south * 1u16;
        let south = Ok(if south == 0 { false } else { true })?;
        Self {
            east,
            down,
            north,
            up,
            west,
            south,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brown_mushroom_block, u16>> = Lazy::new(|| {
            brown_mushroom_block::possible_states()
                .into_iter()
                .zip(
                    [
                        3988u16, 3989u16, 3990u16, 3991u16, 3992u16, 3993u16, 3994u16, 3995u16,
                        3996u16, 3997u16, 3998u16, 3999u16, 4000u16, 4001u16, 4002u16, 4003u16,
                        4004u16, 4005u16, 4006u16, 4007u16, 4008u16, 4009u16, 4010u16, 4011u16,
                        4012u16, 4013u16, 4014u16, 4015u16, 4016u16, 4017u16, 4018u16, 4019u16,
                        4020u16, 4021u16, 4022u16, 4023u16, 4024u16, 4025u16, 4026u16, 4027u16,
                        4028u16, 4029u16, 4030u16, 4031u16, 4032u16, 4033u16, 4034u16, 4035u16,
                        4036u16, 4037u16, 4038u16, 4039u16, 4040u16, 4041u16, 4042u16, 4043u16,
                        4044u16, 4045u16, 4046u16, 4047u16, 4048u16, 4049u16, 4050u16, 4051u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedMushroomBlock {
    east: bool,
    north: bool,
    west: bool,
    down: bool,
    south: bool,
    up: bool,
}
impl RedMushroomBlock {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |(((((east, north), west), down), south), up)| RedMushroomBlock {
                    east,
                    north,
                    west,
                    down,
                    south,
                    up,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.east as u16 * 32u16
            + self.north as u16 * 16u16
            + self.west as u16 * 8u16
            + self.down as u16 * 4u16
            + self.south as u16 * 2u16
            + self.up as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let east = offset / 32u16;
        offset -= east * 32u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let west = offset / 8u16;
        offset -= west * 8u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let down = offset / 4u16;
        offset -= down * 4u16;
        let down = Ok(if down == 0 { false } else { true })?;
        let south = offset / 2u16;
        offset -= south * 2u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let up = offset / 1u16;
        offset -= up * 1u16;
        let up = Ok(if up == 0 { false } else { true })?;
        Self {
            east,
            north,
            west,
            down,
            south,
            up,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<red_mushroom_block, u16>> = Lazy::new(|| {
            red_mushroom_block::possible_states()
                .into_iter()
                .zip(
                    [
                        4052u16, 4053u16, 4054u16, 4055u16, 4056u16, 4057u16, 4058u16, 4059u16,
                        4060u16, 4061u16, 4062u16, 4063u16, 4064u16, 4065u16, 4066u16, 4067u16,
                        4068u16, 4069u16, 4070u16, 4071u16, 4072u16, 4073u16, 4074u16, 4075u16,
                        4076u16, 4077u16, 4078u16, 4079u16, 4080u16, 4081u16, 4082u16, 4083u16,
                        4084u16, 4085u16, 4086u16, 4087u16, 4088u16, 4089u16, 4090u16, 4091u16,
                        4092u16, 4093u16, 4094u16, 4095u16, 4096u16, 4097u16, 4098u16, 4099u16,
                        4100u16, 4101u16, 4102u16, 4103u16, 4104u16, 4105u16, 4106u16, 4107u16,
                        4108u16, 4109u16, 4110u16, 4111u16, 4112u16, 4113u16, 4114u16, 4115u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MushroomStem {
    east: bool,
    south: bool,
    down: bool,
    west: bool,
    north: bool,
    up: bool,
}
impl MushroomStem {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |(((((east, south), down), west), north), up)| MushroomStem {
                    east,
                    south,
                    down,
                    west,
                    north,
                    up,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.east as u16 * 32u16
            + self.south as u16 * 16u16
            + self.down as u16 * 8u16
            + self.west as u16 * 4u16
            + self.north as u16 * 2u16
            + self.up as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let east = offset / 32u16;
        offset -= east * 32u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let down = offset / 8u16;
        offset -= down * 8u16;
        let down = Ok(if down == 0 { false } else { true })?;
        let west = offset / 4u16;
        offset -= west * 4u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let up = offset / 1u16;
        offset -= up * 1u16;
        let up = Ok(if up == 0 { false } else { true })?;
        Self {
            east,
            south,
            down,
            west,
            north,
            up,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<mushroom_stem, u16>> = Lazy::new(|| {
            mushroom_stem::possible_states()
                .into_iter()
                .zip(
                    [
                        4116u16, 4117u16, 4118u16, 4119u16, 4120u16, 4121u16, 4122u16, 4123u16,
                        4124u16, 4125u16, 4126u16, 4127u16, 4128u16, 4129u16, 4130u16, 4131u16,
                        4132u16, 4133u16, 4134u16, 4135u16, 4136u16, 4137u16, 4138u16, 4139u16,
                        4140u16, 4141u16, 4142u16, 4143u16, 4144u16, 4145u16, 4146u16, 4147u16,
                        4148u16, 4149u16, 4150u16, 4151u16, 4152u16, 4153u16, 4154u16, 4155u16,
                        4156u16, 4157u16, 4158u16, 4159u16, 4160u16, 4161u16, 4162u16, 4163u16,
                        4164u16, 4165u16, 4166u16, 4167u16, 4168u16, 4169u16, 4170u16, 4171u16,
                        4172u16, 4173u16, 4174u16, 4175u16, 4176u16, 4177u16, 4178u16, 4179u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IronBars {
    east: bool,
    south: bool,
    waterlogged: bool,
    north: bool,
    west: bool,
}
impl IronBars {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((((east, south), waterlogged), north), west)| IronBars {
                east,
                south,
                waterlogged,
                north,
                west,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.east as u16 * 16u16
            + self.south as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.north as u16 * 2u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let east = offset / 16u16;
        offset -= east * 16u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = Ok(if west == 0 { false } else { true })?;
        Self {
            east,
            south,
            waterlogged,
            north,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<iron_bars, u16>> = Lazy::new(|| {
            iron_bars::possible_states()
                .into_iter()
                .zip(
                    [
                        4180u16, 4181u16, 4182u16, 4183u16, 4184u16, 4185u16, 4186u16, 4187u16,
                        4188u16, 4189u16, 4190u16, 4191u16, 4192u16, 4193u16, 4194u16, 4195u16,
                        4196u16, 4197u16, 4198u16, 4199u16, 4200u16, 4201u16, 4202u16, 4203u16,
                        4204u16, 4205u16, 4206u16, 4207u16, 4208u16, 4209u16, 4210u16, 4211u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlassPane {
    west: bool,
    north: bool,
    east: bool,
    south: bool,
    waterlogged: bool,
}
impl GlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((((west, north), east), south), waterlogged)| GlassPane {
                west,
                north,
                east,
                south,
                waterlogged,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.west as u16 * 16u16
            + self.north as u16 * 8u16
            + self.east as u16 * 4u16
            + self.south as u16 * 2u16
            + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let west = offset / 16u16;
        offset -= west * 16u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 8u16;
        offset -= north * 8u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 4u16;
        offset -= east * 4u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let south = offset / 2u16;
        offset -= south * 2u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            west,
            north,
            east,
            south,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<glass_pane, u16>> = Lazy::new(|| {
            glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        4212u16, 4213u16, 4214u16, 4215u16, 4216u16, 4217u16, 4218u16, 4219u16,
                        4220u16, 4221u16, 4222u16, 4223u16, 4224u16, 4225u16, 4226u16, 4227u16,
                        4228u16, 4229u16, 4230u16, 4231u16, 4232u16, 4233u16, 4234u16, 4235u16,
                        4236u16, 4237u16, 4238u16, 4239u16, 4240u16, 4241u16, 4242u16, 4243u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Melon {}
impl Melon {
    pub fn possible_states() -> Vec<Self> {
        vec![Melon {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(melon {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttachedPumpkinStem {
    facing: AttachedPumpkinStem,
}
impl AttachedPumpkinStem {
    pub fn possible_states() -> Vec<Self> {
        [
            AttachedPumpkinStem::North,
            AttachedPumpkinStem::South,
            AttachedPumpkinStem::West,
            AttachedPumpkinStem::East,
        ]
        .iter()
        .copied()
        .map(|facing| AttachedPumpkinStem { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = AttachedPumpkinStem::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<attached_pumpkin_stem, u16>> = Lazy::new(|| {
            attached_pumpkin_stem::possible_states()
                .into_iter()
                .zip([4245u16, 4246u16, 4247u16, 4248u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttachedMelonStem {
    facing: AttachedMelonStem,
}
impl AttachedMelonStem {
    pub fn possible_states() -> Vec<Self> {
        [
            AttachedMelonStem::North,
            AttachedMelonStem::South,
            AttachedMelonStem::West,
            AttachedMelonStem::East,
        ]
        .iter()
        .copied()
        .map(|facing| AttachedMelonStem { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = AttachedMelonStem::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<attached_melon_stem, u16>> = Lazy::new(|| {
            attached_melon_stem::possible_states()
                .into_iter()
                .zip([4249u16, 4250u16, 4251u16, 4252u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PumpkinStem {
    age: i32,
}
impl PumpkinStem {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=7i32 }.map(|age| PumpkinStem { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<pumpkin_stem, u16>> = Lazy::new(|| {
            pumpkin_stem::possible_states()
                .into_iter()
                .zip(
                    [
                        4253u16, 4254u16, 4255u16, 4256u16, 4257u16, 4258u16, 4259u16, 4260u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MelonStem {
    age: i32,
}
impl MelonStem {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=7i32 }.map(|age| MelonStem { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<melon_stem, u16>> = Lazy::new(|| {
            melon_stem::possible_states()
                .into_iter()
                .zip(
                    [
                        4261u16, 4262u16, 4263u16, 4264u16, 4265u16, 4266u16, 4267u16, 4268u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vine {
    up: bool,
    south: bool,
    west: bool,
    north: bool,
    east: bool,
}
impl Vine {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((((up, south), west), north), east)| Vine {
                up,
                south,
                west,
                north,
                east,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.up as u16 * 16u16
            + self.south as u16 * 8u16
            + self.west as u16 * 4u16
            + self.north as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let up = offset / 16u16;
        offset -= up * 16u16;
        let up = Ok(if up == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 4u16;
        offset -= west * 4u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            up,
            south,
            west,
            north,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<vine, u16>> = Lazy::new(|| {
            vine::possible_states()
                .into_iter()
                .zip(
                    [
                        4269u16, 4270u16, 4271u16, 4272u16, 4273u16, 4274u16, 4275u16, 4276u16,
                        4277u16, 4278u16, 4279u16, 4280u16, 4281u16, 4282u16, 4283u16, 4284u16,
                        4285u16, 4286u16, 4287u16, 4288u16, 4289u16, 4290u16, 4291u16, 4292u16,
                        4293u16, 4294u16, 4295u16, 4296u16, 4297u16, 4298u16, 4299u16, 4300u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OakFenceGate {
    powered: bool,
    facing: OakFenceGate,
    in_wall: bool,
    open: bool,
}
impl OakFenceGate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        OakFenceGate::North,
                        OakFenceGate::South,
                        OakFenceGate::West,
                        OakFenceGate::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((powered, facing), in_wall), open)| OakFenceGate {
                powered,
                facing,
                in_wall,
                open,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 16u16
            + self.facing as u16 * 4u16
            + self.in_wall as u16 * 2u16
            + self.open as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 16u16;
        offset -= powered * 16u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 4u16;
        offset -= facing * 4u16;
        let facing = OakFenceGate::try_from(facing)?;
        let in_wall = offset / 2u16;
        offset -= in_wall * 2u16;
        let in_wall = Ok(if in_wall == 0 { false } else { true })?;
        let open = offset / 1u16;
        offset -= open * 1u16;
        let open = Ok(if open == 0 { false } else { true })?;
        Self {
            powered,
            facing,
            in_wall,
            open,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<oak_fence_gate, u16>> = Lazy::new(|| {
            oak_fence_gate::possible_states()
                .into_iter()
                .zip(
                    [
                        4301u16, 4302u16, 4303u16, 4304u16, 4305u16, 4306u16, 4307u16, 4308u16,
                        4309u16, 4310u16, 4311u16, 4312u16, 4313u16, 4314u16, 4315u16, 4316u16,
                        4317u16, 4318u16, 4319u16, 4320u16, 4321u16, 4322u16, 4323u16, 4324u16,
                        4325u16, 4326u16, 4327u16, 4328u16, 4329u16, 4330u16, 4331u16, 4332u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mycelium {
    snowy: bool,
}
impl Mycelium {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|snowy| Mycelium { snowy })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.snowy as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let snowy = offset / 1u16;
        offset -= snowy * 1u16;
        let snowy = Ok(if snowy == 0 { false } else { true })?;
        Self { snowy }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<mycelium, u16>> = Lazy::new(|| {
            mycelium::possible_states()
                .into_iter()
                .zip([4493u16, 4494u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LilyPad {}
impl LilyPad {
    pub fn possible_states() -> Vec<Self> {
        vec![LilyPad {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lily_pad {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetherBricks {}
impl NetherBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![NetherBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(nether_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetherWart {
    age: i32,
}
impl NetherWart {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=3i32 }.map(|age| NetherWart { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<nether_wart, u16>> = Lazy::new(|| {
            nether_wart::possible_states()
                .into_iter()
                .zip([4609u16, 4610u16, 4611u16, 4612u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnchantingTable {}
impl EnchantingTable {
    pub fn possible_states() -> Vec<Self> {
        vec![EnchantingTable {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(enchanting_table {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrewingStand {
    has_bottle_0: bool,
    has_bottle_2: bool,
    has_bottle_1: bool,
}
impl BrewingStand {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((has_bottle_0, has_bottle_2), has_bottle_1)| BrewingStand {
                    has_bottle_0,
                    has_bottle_2,
                    has_bottle_1,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.has_bottle_0 as u16 * 4u16
            + self.has_bottle_2 as u16 * 2u16
            + self.has_bottle_1 as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let has_bottle_0 = offset / 4u16;
        offset -= has_bottle_0 * 4u16;
        let has_bottle_0 = Ok(if has_bottle_0 == 0 { false } else { true })?;
        let has_bottle_2 = offset / 2u16;
        offset -= has_bottle_2 * 2u16;
        let has_bottle_2 = Ok(if has_bottle_2 == 0 { false } else { true })?;
        let has_bottle_1 = offset / 1u16;
        offset -= has_bottle_1 * 1u16;
        let has_bottle_1 = Ok(if has_bottle_1 == 0 { false } else { true })?;
        Self {
            has_bottle_0,
            has_bottle_2,
            has_bottle_1,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brewing_stand, u16>> = Lazy::new(|| {
            brewing_stand::possible_states()
                .into_iter()
                .zip(
                    [
                        4614u16, 4615u16, 4616u16, 4617u16, 4618u16, 4619u16, 4620u16, 4621u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cauldron {
    level: i32,
}
impl Cauldron {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=3i32 }.map(|level| Cauldron { level }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.level as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let level = offset / 1u16;
        offset -= level * 1u16;
        let level = Ok(level as i32)?;
        Self { level }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cauldron, u16>> = Lazy::new(|| {
            cauldron::possible_states()
                .into_iter()
                .zip([4622u16, 4623u16, 4624u16, 4625u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndPortal {}
impl EndPortal {
    pub fn possible_states() -> Vec<Self> {
        vec![EndPortal {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(end_portal {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndPortalFrame {
    eye: bool,
    facing: EndPortalFrame,
}
impl EndPortalFrame {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        EndPortalFrame::North,
                        EndPortalFrame::South,
                        EndPortalFrame::West,
                        EndPortalFrame::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(eye, facing)| EndPortalFrame { eye, facing })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.eye as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let eye = offset / 4u16;
        offset -= eye * 4u16;
        let eye = Ok(if eye == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = EndPortalFrame::try_from(facing)?;
        Self { eye, facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<end_portal_frame, u16>> = Lazy::new(|| {
            end_portal_frame::possible_states()
                .into_iter()
                .zip(
                    [
                        4627u16, 4628u16, 4629u16, 4630u16, 4631u16, 4632u16, 4633u16, 4634u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndStone {}
impl EndStone {
    pub fn possible_states() -> Vec<Self> {
        vec![EndStone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(end_stone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DragonEgg {}
impl DragonEgg {
    pub fn possible_states() -> Vec<Self> {
        vec![DragonEgg {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dragon_egg {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedstoneLamp {
    lit: bool,
}
impl RedstoneLamp {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|lit| RedstoneLamp { lit })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.lit as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let lit = offset / 1u16;
        offset -= lit * 1u16;
        let lit = Ok(if lit == 0 { false } else { true })?;
        Self { lit }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<redstone_lamp, u16>> = Lazy::new(|| {
            redstone_lamp::possible_states()
                .into_iter()
                .zip([4637u16, 4638u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cocoa {
    facing: Cocoa,
    age: i32,
}
impl Cocoa {
    pub fn possible_states() -> Vec<Self> {
        [Cocoa::North, Cocoa::South, Cocoa::West, Cocoa::East]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 0i32..=2i32 }))
            .map(|(facing, age)| Cocoa { facing, age })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 3u16 + self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 3u16;
        offset -= facing * 3u16;
        let facing = Cocoa::try_from(facing)?;
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { facing, age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cocoa, u16>> = Lazy::new(|| {
            cocoa::possible_states()
                .into_iter()
                .zip(
                    [
                        4639u16, 4640u16, 4641u16, 4642u16, 4643u16, 4644u16, 4645u16, 4646u16,
                        4647u16, 4648u16, 4649u16, 4650u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmeraldOre {}
impl EmeraldOre {
    pub fn possible_states() -> Vec<Self> {
        vec![EmeraldOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(emerald_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnderChest {
    facing: EnderChest,
    waterlogged: bool,
}
impl EnderChest {
    pub fn possible_states() -> Vec<Self> {
        [
            EnderChest::North,
            EnderChest::South,
            EnderChest::West,
            EnderChest::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| EnderChest {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = EnderChest::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<ender_chest, u16>> = Lazy::new(|| {
            ender_chest::possible_states()
                .into_iter()
                .zip(
                    [
                        4732u16, 4733u16, 4734u16, 4735u16, 4736u16, 4737u16, 4738u16, 4739u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TripwireHook {
    facing: TripwireHook,
    powered: bool,
    attached: bool,
}
impl TripwireHook {
    pub fn possible_states() -> Vec<Self> {
        [
            TripwireHook::North,
            TripwireHook::South,
            TripwireHook::West,
            TripwireHook::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|((facing, powered), attached)| TripwireHook {
            facing,
            powered,
            attached,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 4u16 + self.powered as u16 * 2u16 + self.attached as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 4u16;
        offset -= facing * 4u16;
        let facing = TripwireHook::try_from(facing)?;
        let powered = offset / 2u16;
        offset -= powered * 2u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let attached = offset / 1u16;
        offset -= attached * 1u16;
        let attached = Ok(if attached == 0 { false } else { true })?;
        Self {
            facing,
            powered,
            attached,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tripwire_hook, u16>> = Lazy::new(|| {
            tripwire_hook::possible_states()
                .into_iter()
                .zip(
                    [
                        4740u16, 4741u16, 4742u16, 4743u16, 4744u16, 4745u16, 4746u16, 4747u16,
                        4748u16, 4749u16, 4750u16, 4751u16, 4752u16, 4753u16, 4754u16, 4755u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tripwire {
    powered: bool,
    disarmed: bool,
    attached: bool,
    north: bool,
    east: bool,
    west: bool,
    south: bool,
}
impl Tripwire {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((((powered, disarmed), attached), north), east), west), south)| Tripwire {
                    powered,
                    disarmed,
                    attached,
                    north,
                    east,
                    west,
                    south,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 64u16
            + self.disarmed as u16 * 32u16
            + self.attached as u16 * 16u16
            + self.north as u16 * 8u16
            + self.east as u16 * 4u16
            + self.west as u16 * 2u16
            + self.south as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 64u16;
        offset -= powered * 64u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let disarmed = offset / 32u16;
        offset -= disarmed * 32u16;
        let disarmed = Ok(if disarmed == 0 { false } else { true })?;
        let attached = offset / 16u16;
        offset -= attached * 16u16;
        let attached = Ok(if attached == 0 { false } else { true })?;
        let north = offset / 8u16;
        offset -= north * 8u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 4u16;
        offset -= east * 4u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let west = offset / 2u16;
        offset -= west * 2u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let south = offset / 1u16;
        offset -= south * 1u16;
        let south = Ok(if south == 0 { false } else { true })?;
        Self {
            powered,
            disarmed,
            attached,
            north,
            east,
            west,
            south,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tripwire, u16>> = Lazy::new(|| {
            tripwire::possible_states()
                .into_iter()
                .zip(
                    [
                        4756u16, 4757u16, 4758u16, 4759u16, 4760u16, 4761u16, 4762u16, 4763u16,
                        4764u16, 4765u16, 4766u16, 4767u16, 4768u16, 4769u16, 4770u16, 4771u16,
                        4772u16, 4773u16, 4774u16, 4775u16, 4776u16, 4777u16, 4778u16, 4779u16,
                        4780u16, 4781u16, 4782u16, 4783u16, 4784u16, 4785u16, 4786u16, 4787u16,
                        4788u16, 4789u16, 4790u16, 4791u16, 4792u16, 4793u16, 4794u16, 4795u16,
                        4796u16, 4797u16, 4798u16, 4799u16, 4800u16, 4801u16, 4802u16, 4803u16,
                        4804u16, 4805u16, 4806u16, 4807u16, 4808u16, 4809u16, 4810u16, 4811u16,
                        4812u16, 4813u16, 4814u16, 4815u16, 4816u16, 4817u16, 4818u16, 4819u16,
                        4820u16, 4821u16, 4822u16, 4823u16, 4824u16, 4825u16, 4826u16, 4827u16,
                        4828u16, 4829u16, 4830u16, 4831u16, 4832u16, 4833u16, 4834u16, 4835u16,
                        4836u16, 4837u16, 4838u16, 4839u16, 4840u16, 4841u16, 4842u16, 4843u16,
                        4844u16, 4845u16, 4846u16, 4847u16, 4848u16, 4849u16, 4850u16, 4851u16,
                        4852u16, 4853u16, 4854u16, 4855u16, 4856u16, 4857u16, 4858u16, 4859u16,
                        4860u16, 4861u16, 4862u16, 4863u16, 4864u16, 4865u16, 4866u16, 4867u16,
                        4868u16, 4869u16, 4870u16, 4871u16, 4872u16, 4873u16, 4874u16, 4875u16,
                        4876u16, 4877u16, 4878u16, 4879u16, 4880u16, 4881u16, 4882u16, 4883u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EmeraldBlock {}
impl EmeraldBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![EmeraldBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(emerald_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandBlock {
    conditional: bool,
    facing: CommandBlock,
}
impl CommandBlock {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        CommandBlock::North,
                        CommandBlock::East,
                        CommandBlock::South,
                        CommandBlock::West,
                        CommandBlock::Up,
                        CommandBlock::Down,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(conditional, facing)| CommandBlock {
                conditional,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.conditional as u16 * 6u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let conditional = offset / 6u16;
        offset -= conditional * 6u16;
        let conditional = Ok(if conditional == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = CommandBlock::try_from(facing)?;
        Self {
            conditional,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<command_block, u16>> = Lazy::new(|| {
            command_block::possible_states()
                .into_iter()
                .zip(
                    [
                        5125u16, 5126u16, 5127u16, 5128u16, 5129u16, 5130u16, 5131u16, 5132u16,
                        5133u16, 5134u16, 5135u16, 5136u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Beacon {}
impl Beacon {
    pub fn possible_states() -> Vec<Self> {
        vec![Beacon {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(beacon {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CobblestoneWall {
    south: bool,
    north: bool,
    waterlogged: bool,
    up: bool,
    east: bool,
    west: bool,
}
impl CobblestoneWall {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |(((((south, north), waterlogged), up), east), west)| CobblestoneWall {
                    south,
                    north,
                    waterlogged,
                    up,
                    east,
                    west,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 32u16
            + self.north as u16 * 16u16
            + self.waterlogged as u16 * 8u16
            + self.up as u16 * 4u16
            + self.east as u16 * 2u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 32u16;
        offset -= south * 32u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let waterlogged = offset / 8u16;
        offset -= waterlogged * 8u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let up = offset / 4u16;
        offset -= up * 4u16;
        let up = Ok(if up == 0 { false } else { true })?;
        let east = offset / 2u16;
        offset -= east * 2u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = Ok(if west == 0 { false } else { true })?;
        Self {
            south,
            north,
            waterlogged,
            up,
            east,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cobblestone_wall, u16>> = Lazy::new(|| {
            cobblestone_wall::possible_states()
                .into_iter()
                .zip(
                    [
                        5138u16, 5139u16, 5140u16, 5141u16, 5142u16, 5143u16, 5144u16, 5145u16,
                        5146u16, 5147u16, 5148u16, 5149u16, 5150u16, 5151u16, 5152u16, 5153u16,
                        5154u16, 5155u16, 5156u16, 5157u16, 5158u16, 5159u16, 5160u16, 5161u16,
                        5162u16, 5163u16, 5164u16, 5165u16, 5166u16, 5167u16, 5168u16, 5169u16,
                        5170u16, 5171u16, 5172u16, 5173u16, 5174u16, 5175u16, 5176u16, 5177u16,
                        5178u16, 5179u16, 5180u16, 5181u16, 5182u16, 5183u16, 5184u16, 5185u16,
                        5186u16, 5187u16, 5188u16, 5189u16, 5190u16, 5191u16, 5192u16, 5193u16,
                        5194u16, 5195u16, 5196u16, 5197u16, 5198u16, 5199u16, 5200u16, 5201u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MossyCobblestoneWall {
    waterlogged: bool,
    up: bool,
    south: bool,
    west: bool,
    north: bool,
    east: bool,
}
impl MossyCobblestoneWall {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |(((((waterlogged, up), south), west), north), east)| MossyCobblestoneWall {
                    waterlogged,
                    up,
                    south,
                    west,
                    north,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 32u16
            + self.up as u16 * 16u16
            + self.south as u16 * 8u16
            + self.west as u16 * 4u16
            + self.north as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 32u16;
        offset -= waterlogged * 32u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let up = offset / 16u16;
        offset -= up * 16u16;
        let up = Ok(if up == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 4u16;
        offset -= west * 4u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            waterlogged,
            up,
            south,
            west,
            north,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<mossy_cobblestone_wall, u16>> = Lazy::new(|| {
            mossy_cobblestone_wall::possible_states()
                .into_iter()
                .zip(
                    [
                        5202u16, 5203u16, 5204u16, 5205u16, 5206u16, 5207u16, 5208u16, 5209u16,
                        5210u16, 5211u16, 5212u16, 5213u16, 5214u16, 5215u16, 5216u16, 5217u16,
                        5218u16, 5219u16, 5220u16, 5221u16, 5222u16, 5223u16, 5224u16, 5225u16,
                        5226u16, 5227u16, 5228u16, 5229u16, 5230u16, 5231u16, 5232u16, 5233u16,
                        5234u16, 5235u16, 5236u16, 5237u16, 5238u16, 5239u16, 5240u16, 5241u16,
                        5242u16, 5243u16, 5244u16, 5245u16, 5246u16, 5247u16, 5248u16, 5249u16,
                        5250u16, 5251u16, 5252u16, 5253u16, 5254u16, 5255u16, 5256u16, 5257u16,
                        5258u16, 5259u16, 5260u16, 5261u16, 5262u16, 5263u16, 5264u16, 5265u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FlowerPot {}
impl FlowerPot {
    pub fn possible_states() -> Vec<Self> {
        vec![FlowerPot {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(flower_pot {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedFern {}
impl PottedFern {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedFern {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_fern {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedDandelion {}
impl PottedDandelion {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedDandelion {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_dandelion {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedPoppy {}
impl PottedPoppy {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedPoppy {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_poppy {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedBlueOrchid {}
impl PottedBlueOrchid {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedBlueOrchid {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_blue_orchid {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedAllium {}
impl PottedAllium {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedAllium {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_allium {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedAzureBluet {}
impl PottedAzureBluet {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedAzureBluet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_azure_bluet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedRedTulip {}
impl PottedRedTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedRedTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_red_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedOrangeTulip {}
impl PottedOrangeTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedOrangeTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_orange_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedWhiteTulip {}
impl PottedWhiteTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedWhiteTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_white_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedPinkTulip {}
impl PottedPinkTulip {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedPinkTulip {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_pink_tulip {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedOxeyeDaisy {}
impl PottedOxeyeDaisy {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedOxeyeDaisy {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_oxeye_daisy {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedRedMushroom {}
impl PottedRedMushroom {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedRedMushroom {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_red_mushroom {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedBrownMushroom {}
impl PottedBrownMushroom {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedBrownMushroom {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_brown_mushroom {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedDeadBush {}
impl PottedDeadBush {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedDeadBush {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_dead_bush {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PottedCactus {}
impl PottedCactus {
    pub fn possible_states() -> Vec<Self> {
        vec![PottedCactus {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(potted_cactus {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Carrots {
    age: i32,
}
impl Carrots {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=7i32 }.map(|age| Carrots { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<carrots, u16>> = Lazy::new(|| {
            carrots::possible_states()
                .into_iter()
                .zip(
                    [
                        5288u16, 5289u16, 5290u16, 5291u16, 5292u16, 5293u16, 5294u16, 5295u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Potatoes {
    age: i32,
}
impl Potatoes {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=7i32 }.map(|age| Potatoes { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<potatoes, u16>> = Lazy::new(|| {
            potatoes::possible_states()
                .into_iter()
                .zip(
                    [
                        5296u16, 5297u16, 5298u16, 5299u16, 5300u16, 5301u16, 5302u16, 5303u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OakButton {
    powered: bool,
    face: OakButton,
    facing: OakButton,
}
impl OakButton {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [OakButton::Floor, OakButton::Wall, OakButton::Ceiling]
                        .iter()
                        .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        OakButton::North,
                        OakButton::South,
                        OakButton::West,
                        OakButton::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((powered, face), facing)| OakButton {
                powered,
                face,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 12u16 + self.face as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 12u16;
        offset -= powered * 12u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let face = offset / 4u16;
        offset -= face * 4u16;
        let face = OakButton::try_from(face)?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = OakButton::try_from(facing)?;
        Self {
            powered,
            face,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<oak_button, u16>> = Lazy::new(|| {
            oak_button::possible_states()
                .into_iter()
                .zip(
                    [
                        5304u16, 5305u16, 5306u16, 5307u16, 5308u16, 5309u16, 5310u16, 5311u16,
                        5312u16, 5313u16, 5314u16, 5315u16, 5316u16, 5317u16, 5318u16, 5319u16,
                        5320u16, 5321u16, 5322u16, 5323u16, 5324u16, 5325u16, 5326u16, 5327u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpruceButton {
    powered: bool,
    facing: SpruceButton,
    face: SpruceButton,
}
impl SpruceButton {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        SpruceButton::North,
                        SpruceButton::South,
                        SpruceButton::West,
                        SpruceButton::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        SpruceButton::Floor,
                        SpruceButton::Wall,
                        SpruceButton::Ceiling,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((powered, facing), face)| SpruceButton {
                powered,
                facing,
                face,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 12u16 + self.facing as u16 * 3u16 + self.face as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 12u16;
        offset -= powered * 12u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 3u16;
        offset -= facing * 3u16;
        let facing = SpruceButton::try_from(facing)?;
        let face = offset / 1u16;
        offset -= face * 1u16;
        let face = SpruceButton::try_from(face)?;
        Self {
            powered,
            facing,
            face,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<spruce_button, u16>> = Lazy::new(|| {
            spruce_button::possible_states()
                .into_iter()
                .zip(
                    [
                        5328u16, 5329u16, 5330u16, 5331u16, 5332u16, 5333u16, 5334u16, 5335u16,
                        5336u16, 5337u16, 5338u16, 5339u16, 5340u16, 5341u16, 5342u16, 5343u16,
                        5344u16, 5345u16, 5346u16, 5347u16, 5348u16, 5349u16, 5350u16, 5351u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BirchButton {
    face: BirchButton,
    powered: bool,
    facing: BirchButton,
}
impl BirchButton {
    pub fn possible_states() -> Vec<Self> {
        [BirchButton::Floor, BirchButton::Wall, BirchButton::Ceiling]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        BirchButton::North,
                        BirchButton::South,
                        BirchButton::West,
                        BirchButton::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((face, powered), facing)| BirchButton {
                face,
                powered,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.face as u16 * 8u16 + self.powered as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let face = offset / 8u16;
        offset -= face * 8u16;
        let face = BirchButton::try_from(face)?;
        let powered = offset / 4u16;
        offset -= powered * 4u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BirchButton::try_from(facing)?;
        Self {
            face,
            powered,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<birch_button, u16>> = Lazy::new(|| {
            birch_button::possible_states()
                .into_iter()
                .zip(
                    [
                        5352u16, 5353u16, 5354u16, 5355u16, 5356u16, 5357u16, 5358u16, 5359u16,
                        5360u16, 5361u16, 5362u16, 5363u16, 5364u16, 5365u16, 5366u16, 5367u16,
                        5368u16, 5369u16, 5370u16, 5371u16, 5372u16, 5373u16, 5374u16, 5375u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JungleButton {
    face: JungleButton,
    facing: JungleButton,
    powered: bool,
}
impl JungleButton {
    pub fn possible_states() -> Vec<Self> {
        [
            JungleButton::Floor,
            JungleButton::Wall,
            JungleButton::Ceiling,
        ]
        .iter()
        .copied()
        .flat_map(|state| {
            std::iter::repeat(state).zip(
                [
                    JungleButton::North,
                    JungleButton::South,
                    JungleButton::West,
                    JungleButton::East,
                ]
                .iter()
                .copied(),
            )
        })
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|((face, facing), powered)| JungleButton {
            face,
            facing,
            powered,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.face as u16 * 8u16 + self.facing as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let face = offset / 8u16;
        offset -= face * 8u16;
        let face = JungleButton::try_from(face)?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = JungleButton::try_from(facing)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self {
            face,
            facing,
            powered,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<jungle_button, u16>> = Lazy::new(|| {
            jungle_button::possible_states()
                .into_iter()
                .zip(
                    [
                        5376u16, 5377u16, 5378u16, 5379u16, 5380u16, 5381u16, 5382u16, 5383u16,
                        5384u16, 5385u16, 5386u16, 5387u16, 5388u16, 5389u16, 5390u16, 5391u16,
                        5392u16, 5393u16, 5394u16, 5395u16, 5396u16, 5397u16, 5398u16, 5399u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AcaciaButton {
    face: AcaciaButton,
    facing: AcaciaButton,
    powered: bool,
}
impl AcaciaButton {
    pub fn possible_states() -> Vec<Self> {
        [
            AcaciaButton::Floor,
            AcaciaButton::Wall,
            AcaciaButton::Ceiling,
        ]
        .iter()
        .copied()
        .flat_map(|state| {
            std::iter::repeat(state).zip(
                [
                    AcaciaButton::North,
                    AcaciaButton::South,
                    AcaciaButton::West,
                    AcaciaButton::East,
                ]
                .iter()
                .copied(),
            )
        })
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|((face, facing), powered)| AcaciaButton {
            face,
            facing,
            powered,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.face as u16 * 8u16 + self.facing as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let face = offset / 8u16;
        offset -= face * 8u16;
        let face = AcaciaButton::try_from(face)?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = AcaciaButton::try_from(facing)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self {
            face,
            facing,
            powered,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<acacia_button, u16>> = Lazy::new(|| {
            acacia_button::possible_states()
                .into_iter()
                .zip(
                    [
                        5400u16, 5401u16, 5402u16, 5403u16, 5404u16, 5405u16, 5406u16, 5407u16,
                        5408u16, 5409u16, 5410u16, 5411u16, 5412u16, 5413u16, 5414u16, 5415u16,
                        5416u16, 5417u16, 5418u16, 5419u16, 5420u16, 5421u16, 5422u16, 5423u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DarkOakButton {
    powered: bool,
    face: DarkOakButton,
    facing: DarkOakButton,
}
impl DarkOakButton {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        DarkOakButton::Floor,
                        DarkOakButton::Wall,
                        DarkOakButton::Ceiling,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        DarkOakButton::North,
                        DarkOakButton::South,
                        DarkOakButton::West,
                        DarkOakButton::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((powered, face), facing)| DarkOakButton {
                powered,
                face,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 12u16 + self.face as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 12u16;
        offset -= powered * 12u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let face = offset / 4u16;
        offset -= face * 4u16;
        let face = DarkOakButton::try_from(face)?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = DarkOakButton::try_from(facing)?;
        Self {
            powered,
            face,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dark_oak_button, u16>> = Lazy::new(|| {
            dark_oak_button::possible_states()
                .into_iter()
                .zip(
                    [
                        5424u16, 5425u16, 5426u16, 5427u16, 5428u16, 5429u16, 5430u16, 5431u16,
                        5432u16, 5433u16, 5434u16, 5435u16, 5436u16, 5437u16, 5438u16, 5439u16,
                        5440u16, 5441u16, 5442u16, 5443u16, 5444u16, 5445u16, 5446u16, 5447u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SkeletonWallSkull {
    facing: SkeletonWallSkull,
}
impl SkeletonWallSkull {
    pub fn possible_states() -> Vec<Self> {
        [
            SkeletonWallSkull::North,
            SkeletonWallSkull::South,
            SkeletonWallSkull::West,
            SkeletonWallSkull::East,
        ]
        .iter()
        .copied()
        .map(|facing| SkeletonWallSkull { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = SkeletonWallSkull::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<skeleton_wall_skull, u16>> = Lazy::new(|| {
            skeleton_wall_skull::possible_states()
                .into_iter()
                .zip([5448u16, 5449u16, 5450u16, 5451u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SkeletonSkull {
    rotation: i32,
}
impl SkeletonSkull {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| SkeletonSkull { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<skeleton_skull, u16>> = Lazy::new(|| {
            skeleton_skull::possible_states()
                .into_iter()
                .zip(
                    [
                        5452u16, 5453u16, 5454u16, 5455u16, 5456u16, 5457u16, 5458u16, 5459u16,
                        5460u16, 5461u16, 5462u16, 5463u16, 5464u16, 5465u16, 5466u16, 5467u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WitherSkeletonWallSkull {
    facing: WitherSkeletonWallSkull,
}
impl WitherSkeletonWallSkull {
    pub fn possible_states() -> Vec<Self> {
        [
            WitherSkeletonWallSkull::North,
            WitherSkeletonWallSkull::South,
            WitherSkeletonWallSkull::West,
            WitherSkeletonWallSkull::East,
        ]
        .iter()
        .copied()
        .map(|facing| WitherSkeletonWallSkull { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = WitherSkeletonWallSkull::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<wither_skeleton_wall_skull, u16>> = Lazy::new(|| {
            wither_skeleton_wall_skull::possible_states()
                .into_iter()
                .zip([5468u16, 5469u16, 5470u16, 5471u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WitherSkeletonSkull {
    rotation: i32,
}
impl WitherSkeletonSkull {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| WitherSkeletonSkull { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<wither_skeleton_skull, u16>> = Lazy::new(|| {
            wither_skeleton_skull::possible_states()
                .into_iter()
                .zip(
                    [
                        5472u16, 5473u16, 5474u16, 5475u16, 5476u16, 5477u16, 5478u16, 5479u16,
                        5480u16, 5481u16, 5482u16, 5483u16, 5484u16, 5485u16, 5486u16, 5487u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ZombieWallHead {
    facing: ZombieWallHead,
}
impl ZombieWallHead {
    pub fn possible_states() -> Vec<Self> {
        [
            ZombieWallHead::North,
            ZombieWallHead::South,
            ZombieWallHead::West,
            ZombieWallHead::East,
        ]
        .iter()
        .copied()
        .map(|facing| ZombieWallHead { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = ZombieWallHead::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<zombie_wall_head, u16>> = Lazy::new(|| {
            zombie_wall_head::possible_states()
                .into_iter()
                .zip([5488u16, 5489u16, 5490u16, 5491u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ZombieHead {
    rotation: i32,
}
impl ZombieHead {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| ZombieHead { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<zombie_head, u16>> = Lazy::new(|| {
            zombie_head::possible_states()
                .into_iter()
                .zip(
                    [
                        5492u16, 5493u16, 5494u16, 5495u16, 5496u16, 5497u16, 5498u16, 5499u16,
                        5500u16, 5501u16, 5502u16, 5503u16, 5504u16, 5505u16, 5506u16, 5507u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerWallHead {
    facing: PlayerWallHead,
}
impl PlayerWallHead {
    pub fn possible_states() -> Vec<Self> {
        [
            PlayerWallHead::North,
            PlayerWallHead::South,
            PlayerWallHead::West,
            PlayerWallHead::East,
        ]
        .iter()
        .copied()
        .map(|facing| PlayerWallHead { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PlayerWallHead::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<player_wall_head, u16>> = Lazy::new(|| {
            player_wall_head::possible_states()
                .into_iter()
                .zip([5508u16, 5509u16, 5510u16, 5511u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerHead {
    rotation: i32,
}
impl PlayerHead {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| PlayerHead { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<player_head, u16>> = Lazy::new(|| {
            player_head::possible_states()
                .into_iter()
                .zip(
                    [
                        5512u16, 5513u16, 5514u16, 5515u16, 5516u16, 5517u16, 5518u16, 5519u16,
                        5520u16, 5521u16, 5522u16, 5523u16, 5524u16, 5525u16, 5526u16, 5527u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CreeperWallHead {
    facing: CreeperWallHead,
}
impl CreeperWallHead {
    pub fn possible_states() -> Vec<Self> {
        [
            CreeperWallHead::North,
            CreeperWallHead::South,
            CreeperWallHead::West,
            CreeperWallHead::East,
        ]
        .iter()
        .copied()
        .map(|facing| CreeperWallHead { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = CreeperWallHead::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<creeper_wall_head, u16>> = Lazy::new(|| {
            creeper_wall_head::possible_states()
                .into_iter()
                .zip([5528u16, 5529u16, 5530u16, 5531u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CreeperHead {
    rotation: i32,
}
impl CreeperHead {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| CreeperHead { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<creeper_head, u16>> = Lazy::new(|| {
            creeper_head::possible_states()
                .into_iter()
                .zip(
                    [
                        5532u16, 5533u16, 5534u16, 5535u16, 5536u16, 5537u16, 5538u16, 5539u16,
                        5540u16, 5541u16, 5542u16, 5543u16, 5544u16, 5545u16, 5546u16, 5547u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DragonWallHead {
    facing: DragonWallHead,
}
impl DragonWallHead {
    pub fn possible_states() -> Vec<Self> {
        [
            DragonWallHead::North,
            DragonWallHead::South,
            DragonWallHead::West,
            DragonWallHead::East,
        ]
        .iter()
        .copied()
        .map(|facing| DragonWallHead { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = DragonWallHead::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dragon_wall_head, u16>> = Lazy::new(|| {
            dragon_wall_head::possible_states()
                .into_iter()
                .zip([5548u16, 5549u16, 5550u16, 5551u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DragonHead {
    rotation: i32,
}
impl DragonHead {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| DragonHead { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dragon_head, u16>> = Lazy::new(|| {
            dragon_head::possible_states()
                .into_iter()
                .zip(
                    [
                        5552u16, 5553u16, 5554u16, 5555u16, 5556u16, 5557u16, 5558u16, 5559u16,
                        5560u16, 5561u16, 5562u16, 5563u16, 5564u16, 5565u16, 5566u16, 5567u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Anvil {
    facing: Anvil,
}
impl Anvil {
    pub fn possible_states() -> Vec<Self> {
        [Anvil::North, Anvil::South, Anvil::West, Anvil::East]
            .iter()
            .copied()
            .map(|facing| Anvil { facing })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = Anvil::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<anvil, u16>> = Lazy::new(|| {
            anvil::possible_states()
                .into_iter()
                .zip([5568u16, 5569u16, 5570u16, 5571u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChippedAnvil {
    facing: ChippedAnvil,
}
impl ChippedAnvil {
    pub fn possible_states() -> Vec<Self> {
        [
            ChippedAnvil::North,
            ChippedAnvil::South,
            ChippedAnvil::West,
            ChippedAnvil::East,
        ]
        .iter()
        .copied()
        .map(|facing| ChippedAnvil { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = ChippedAnvil::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<chipped_anvil, u16>> = Lazy::new(|| {
            chipped_anvil::possible_states()
                .into_iter()
                .zip([5572u16, 5573u16, 5574u16, 5575u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DamagedAnvil {
    facing: DamagedAnvil,
}
impl DamagedAnvil {
    pub fn possible_states() -> Vec<Self> {
        [
            DamagedAnvil::North,
            DamagedAnvil::South,
            DamagedAnvil::West,
            DamagedAnvil::East,
        ]
        .iter()
        .copied()
        .map(|facing| DamagedAnvil { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = DamagedAnvil::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<damaged_anvil, u16>> = Lazy::new(|| {
            damaged_anvil::possible_states()
                .into_iter()
                .zip([5576u16, 5577u16, 5578u16, 5579u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrappedChest {
    waterlogged: bool,
    facing: TrappedChest,
    kind: TrappedChest,
}
impl TrappedChest {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        TrappedChest::North,
                        TrappedChest::South,
                        TrappedChest::West,
                        TrappedChest::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        TrappedChest::Single,
                        TrappedChest::Left,
                        TrappedChest::Right,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((waterlogged, facing), kind)| TrappedChest {
                waterlogged,
                facing,
                kind,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 12u16 + self.facing as u16 * 3u16 + self.kind as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 12u16;
        offset -= waterlogged * 12u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 3u16;
        offset -= facing * 3u16;
        let facing = TrappedChest::try_from(facing)?;
        let kind = offset / 1u16;
        offset -= kind * 1u16;
        let kind = TrappedChest::try_from(kind)?;
        Self {
            waterlogged,
            facing,
            kind,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<trapped_chest, u16>> = Lazy::new(|| {
            trapped_chest::possible_states()
                .into_iter()
                .zip(
                    [
                        5580u16, 5581u16, 5582u16, 5583u16, 5584u16, 5585u16, 5586u16, 5587u16,
                        5588u16, 5589u16, 5590u16, 5591u16, 5592u16, 5593u16, 5594u16, 5595u16,
                        5596u16, 5597u16, 5598u16, 5599u16, 5600u16, 5601u16, 5602u16, 5603u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightWeightedPressurePlate {
    power: i32,
}
impl LightWeightedPressurePlate {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|power| LightWeightedPressurePlate { power })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.power as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let power = offset / 1u16;
        offset -= power * 1u16;
        let power = Ok(power as i32)?;
        Self { power }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_weighted_pressure_plate, u16>> = Lazy::new(|| {
            light_weighted_pressure_plate::possible_states()
                .into_iter()
                .zip(
                    [
                        5604u16, 5605u16, 5606u16, 5607u16, 5608u16, 5609u16, 5610u16, 5611u16,
                        5612u16, 5613u16, 5614u16, 5615u16, 5616u16, 5617u16, 5618u16, 5619u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HeavyWeightedPressurePlate {
    power: i32,
}
impl HeavyWeightedPressurePlate {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|power| HeavyWeightedPressurePlate { power })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.power as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let power = offset / 1u16;
        offset -= power * 1u16;
        let power = Ok(power as i32)?;
        Self { power }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<heavy_weighted_pressure_plate, u16>> = Lazy::new(|| {
            heavy_weighted_pressure_plate::possible_states()
                .into_iter()
                .zip(
                    [
                        5620u16, 5621u16, 5622u16, 5623u16, 5624u16, 5625u16, 5626u16, 5627u16,
                        5628u16, 5629u16, 5630u16, 5631u16, 5632u16, 5633u16, 5634u16, 5635u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Comparator {
    mode: Comparator,
    facing: Comparator,
    powered: bool,
}
impl Comparator {
    pub fn possible_states() -> Vec<Self> {
        [Comparator::Compare, Comparator::Subtract]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        Comparator::North,
                        Comparator::South,
                        Comparator::West,
                        Comparator::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((mode, facing), powered)| Comparator {
                mode,
                facing,
                powered,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.mode as u16 * 8u16 + self.facing as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let mode = offset / 8u16;
        offset -= mode * 8u16;
        let mode = Comparator::try_from(mode)?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = Comparator::try_from(facing)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self {
            mode,
            facing,
            powered,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<comparator, u16>> = Lazy::new(|| {
            comparator::possible_states()
                .into_iter()
                .zip(
                    [
                        5636u16, 5637u16, 5638u16, 5639u16, 5640u16, 5641u16, 5642u16, 5643u16,
                        5644u16, 5645u16, 5646u16, 5647u16, 5648u16, 5649u16, 5650u16, 5651u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DaylightDetector {
    inverted: bool,
    power: i32,
}
impl DaylightDetector {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 0i32..=15i32 }))
            .map(|(inverted, power)| DaylightDetector { inverted, power })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.inverted as u16 * 16u16 + self.power as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let inverted = offset / 16u16;
        offset -= inverted * 16u16;
        let inverted = Ok(if inverted == 0 { false } else { true })?;
        let power = offset / 1u16;
        offset -= power * 1u16;
        let power = Ok(power as i32)?;
        Self { inverted, power }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<daylight_detector, u16>> = Lazy::new(|| {
            daylight_detector::possible_states()
                .into_iter()
                .zip(
                    [
                        5652u16, 5653u16, 5654u16, 5655u16, 5656u16, 5657u16, 5658u16, 5659u16,
                        5660u16, 5661u16, 5662u16, 5663u16, 5664u16, 5665u16, 5666u16, 5667u16,
                        5668u16, 5669u16, 5670u16, 5671u16, 5672u16, 5673u16, 5674u16, 5675u16,
                        5676u16, 5677u16, 5678u16, 5679u16, 5680u16, 5681u16, 5682u16, 5683u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedstoneBlock {}
impl RedstoneBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![RedstoneBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(redstone_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetherQuartzOre {}
impl NetherQuartzOre {
    pub fn possible_states() -> Vec<Self> {
        vec![NetherQuartzOre {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(nether_quartz_ore {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hopper {
    facing: Hopper,
    enabled: bool,
}
impl Hopper {
    pub fn possible_states() -> Vec<Self> {
        [
            Hopper::Down,
            Hopper::North,
            Hopper::South,
            Hopper::West,
            Hopper::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, enabled)| Hopper { facing, enabled })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.enabled as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = Hopper::try_from(facing)?;
        let enabled = offset / 1u16;
        offset -= enabled * 1u16;
        let enabled = Ok(if enabled == 0 { false } else { true })?;
        Self { facing, enabled }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<hopper, u16>> = Lazy::new(|| {
            hopper::possible_states()
                .into_iter()
                .zip(
                    [
                        5686u16, 5687u16, 5688u16, 5689u16, 5690u16, 5691u16, 5692u16, 5693u16,
                        5694u16, 5695u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuartzBlock {}
impl QuartzBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![QuartzBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(quartz_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChiseledQuartzBlock {}
impl ChiseledQuartzBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![ChiseledQuartzBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(chiseled_quartz_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuartzPillar {
    axis: QuartzPillar,
}
impl QuartzPillar {
    pub fn possible_states() -> Vec<Self> {
        [QuartzPillar::X, QuartzPillar::Y, QuartzPillar::Z]
            .iter()
            .copied()
            .map(|axis| QuartzPillar { axis })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.axis as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let axis = offset / 1u16;
        offset -= axis * 1u16;
        let axis = QuartzPillar::try_from(axis)?;
        Self { axis }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<quartz_pillar, u16>> = Lazy::new(|| {
            quartz_pillar::possible_states()
                .into_iter()
                .zip([5698u16, 5699u16, 5700u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ActivatorRail {
    shape: ActivatorRail,
    powered: bool,
}
impl ActivatorRail {
    pub fn possible_states() -> Vec<Self> {
        [
            ActivatorRail::NorthSouth,
            ActivatorRail::EastWest,
            ActivatorRail::AscendingEast,
            ActivatorRail::AscendingWest,
            ActivatorRail::AscendingNorth,
            ActivatorRail::AscendingSouth,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(shape, powered)| ActivatorRail { shape, powered })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.shape as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let shape = offset / 2u16;
        offset -= shape * 2u16;
        let shape = ActivatorRail::try_from(shape)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { shape, powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<activator_rail, u16>> = Lazy::new(|| {
            activator_rail::possible_states()
                .into_iter()
                .zip(
                    [
                        5781u16, 5782u16, 5783u16, 5784u16, 5785u16, 5786u16, 5787u16, 5788u16,
                        5789u16, 5790u16, 5791u16, 5792u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dropper {
    triggered: bool,
    facing: Dropper,
}
impl Dropper {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        Dropper::North,
                        Dropper::East,
                        Dropper::South,
                        Dropper::West,
                        Dropper::Up,
                        Dropper::Down,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(triggered, facing)| Dropper { triggered, facing })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.triggered as u16 * 6u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let triggered = offset / 6u16;
        offset -= triggered * 6u16;
        let triggered = Ok(if triggered == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = Dropper::try_from(facing)?;
        Self { triggered, facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dropper, u16>> = Lazy::new(|| {
            dropper::possible_states()
                .into_iter()
                .zip(
                    [
                        5793u16, 5794u16, 5795u16, 5796u16, 5797u16, 5798u16, 5799u16, 5800u16,
                        5801u16, 5802u16, 5803u16, 5804u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteTerracotta {}
impl WhiteTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeTerracotta {}
impl OrangeTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaTerracotta {}
impl MagentaTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![MagentaTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magenta_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueTerracotta {}
impl LightBlueTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![LightBlueTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_blue_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowTerracotta {}
impl YellowTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![YellowTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(yellow_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeTerracotta {}
impl LimeTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![LimeTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lime_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkTerracotta {}
impl PinkTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayTerracotta {}
impl GrayTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![GrayTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gray_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayTerracotta {}
impl LightGrayTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![LightGrayTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_gray_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanTerracotta {}
impl CyanTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![CyanTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cyan_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleTerracotta {}
impl PurpleTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpleTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purple_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueTerracotta {}
impl BlueTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownTerracotta {}
impl BrownTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenTerracotta {}
impl GreenTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![GreenTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(green_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedTerracotta {}
impl RedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![RedTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackTerracotta {}
impl BlackTerracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![BlackTerracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(black_terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteStainedGlassPane {
    east: bool,
    south: bool,
    waterlogged: bool,
    west: bool,
    north: bool,
}
impl WhiteStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((east, south), waterlogged), west), north)| WhiteStainedGlassPane {
                    east,
                    south,
                    waterlogged,
                    west,
                    north,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.east as u16 * 16u16
            + self.south as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.west as u16 * 2u16
            + self.north as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let east = offset / 16u16;
        offset -= east * 16u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let west = offset / 2u16;
        offset -= west * 2u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 1u16;
        offset -= north * 1u16;
        let north = Ok(if north == 0 { false } else { true })?;
        Self {
            east,
            south,
            waterlogged,
            west,
            north,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<white_stained_glass_pane, u16>> = Lazy::new(|| {
            white_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        5821u16, 5822u16, 5823u16, 5824u16, 5825u16, 5826u16, 5827u16, 5828u16,
                        5829u16, 5830u16, 5831u16, 5832u16, 5833u16, 5834u16, 5835u16, 5836u16,
                        5837u16, 5838u16, 5839u16, 5840u16, 5841u16, 5842u16, 5843u16, 5844u16,
                        5845u16, 5846u16, 5847u16, 5848u16, 5849u16, 5850u16, 5851u16, 5852u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeStainedGlassPane {
    north: bool,
    west: bool,
    south: bool,
    waterlogged: bool,
    east: bool,
}
impl OrangeStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((north, west), south), waterlogged), east)| OrangeStainedGlassPane {
                    north,
                    west,
                    south,
                    waterlogged,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.north as u16 * 16u16
            + self.west as u16 * 8u16
            + self.south as u16 * 4u16
            + self.waterlogged as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let west = offset / 8u16;
        offset -= west * 8u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let south = offset / 4u16;
        offset -= south * 4u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let waterlogged = offset / 2u16;
        offset -= waterlogged * 2u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            north,
            west,
            south,
            waterlogged,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<orange_stained_glass_pane, u16>> = Lazy::new(|| {
            orange_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        5853u16, 5854u16, 5855u16, 5856u16, 5857u16, 5858u16, 5859u16, 5860u16,
                        5861u16, 5862u16, 5863u16, 5864u16, 5865u16, 5866u16, 5867u16, 5868u16,
                        5869u16, 5870u16, 5871u16, 5872u16, 5873u16, 5874u16, 5875u16, 5876u16,
                        5877u16, 5878u16, 5879u16, 5880u16, 5881u16, 5882u16, 5883u16, 5884u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaStainedGlassPane {
    south: bool,
    north: bool,
    waterlogged: bool,
    east: bool,
    west: bool,
}
impl MagentaStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((south, north), waterlogged), east), west)| MagentaStainedGlassPane {
                    south,
                    north,
                    waterlogged,
                    east,
                    west,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 16u16
            + self.north as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.east as u16 * 2u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let north = offset / 8u16;
        offset -= north * 8u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let east = offset / 2u16;
        offset -= east * 2u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = Ok(if west == 0 { false } else { true })?;
        Self {
            south,
            north,
            waterlogged,
            east,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<magenta_stained_glass_pane, u16>> = Lazy::new(|| {
            magenta_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        5885u16, 5886u16, 5887u16, 5888u16, 5889u16, 5890u16, 5891u16, 5892u16,
                        5893u16, 5894u16, 5895u16, 5896u16, 5897u16, 5898u16, 5899u16, 5900u16,
                        5901u16, 5902u16, 5903u16, 5904u16, 5905u16, 5906u16, 5907u16, 5908u16,
                        5909u16, 5910u16, 5911u16, 5912u16, 5913u16, 5914u16, 5915u16, 5916u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueStainedGlassPane {
    south: bool,
    east: bool,
    west: bool,
    north: bool,
    waterlogged: bool,
}
impl LightBlueStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((south, east), west), north), waterlogged)| LightBlueStainedGlassPane {
                    south,
                    east,
                    west,
                    north,
                    waterlogged,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 16u16
            + self.east as u16 * 8u16
            + self.west as u16 * 4u16
            + self.north as u16 * 2u16
            + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let west = offset / 4u16;
        offset -= west * 4u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            south,
            east,
            west,
            north,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_blue_stained_glass_pane, u16>> = Lazy::new(|| {
            light_blue_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        5917u16, 5918u16, 5919u16, 5920u16, 5921u16, 5922u16, 5923u16, 5924u16,
                        5925u16, 5926u16, 5927u16, 5928u16, 5929u16, 5930u16, 5931u16, 5932u16,
                        5933u16, 5934u16, 5935u16, 5936u16, 5937u16, 5938u16, 5939u16, 5940u16,
                        5941u16, 5942u16, 5943u16, 5944u16, 5945u16, 5946u16, 5947u16, 5948u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowStainedGlassPane {
    south: bool,
    east: bool,
    north: bool,
    waterlogged: bool,
    west: bool,
}
impl YellowStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((south, east), north), waterlogged), west)| YellowStainedGlassPane {
                    south,
                    east,
                    north,
                    waterlogged,
                    west,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 16u16
            + self.east as u16 * 8u16
            + self.north as u16 * 4u16
            + self.waterlogged as u16 * 2u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let north = offset / 4u16;
        offset -= north * 4u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let waterlogged = offset / 2u16;
        offset -= waterlogged * 2u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = Ok(if west == 0 { false } else { true })?;
        Self {
            south,
            east,
            north,
            waterlogged,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<yellow_stained_glass_pane, u16>> = Lazy::new(|| {
            yellow_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        5949u16, 5950u16, 5951u16, 5952u16, 5953u16, 5954u16, 5955u16, 5956u16,
                        5957u16, 5958u16, 5959u16, 5960u16, 5961u16, 5962u16, 5963u16, 5964u16,
                        5965u16, 5966u16, 5967u16, 5968u16, 5969u16, 5970u16, 5971u16, 5972u16,
                        5973u16, 5974u16, 5975u16, 5976u16, 5977u16, 5978u16, 5979u16, 5980u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeStainedGlassPane {
    west: bool,
    south: bool,
    north: bool,
    waterlogged: bool,
    east: bool,
}
impl LimeStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((west, south), north), waterlogged), east)| LimeStainedGlassPane {
                    west,
                    south,
                    north,
                    waterlogged,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.west as u16 * 16u16
            + self.south as u16 * 8u16
            + self.north as u16 * 4u16
            + self.waterlogged as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let west = offset / 16u16;
        offset -= west * 16u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let north = offset / 4u16;
        offset -= north * 4u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let waterlogged = offset / 2u16;
        offset -= waterlogged * 2u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            west,
            south,
            north,
            waterlogged,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lime_stained_glass_pane, u16>> = Lazy::new(|| {
            lime_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        5981u16, 5982u16, 5983u16, 5984u16, 5985u16, 5986u16, 5987u16, 5988u16,
                        5989u16, 5990u16, 5991u16, 5992u16, 5993u16, 5994u16, 5995u16, 5996u16,
                        5997u16, 5998u16, 5999u16, 6000u16, 6001u16, 6002u16, 6003u16, 6004u16,
                        6005u16, 6006u16, 6007u16, 6008u16, 6009u16, 6010u16, 6011u16, 6012u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkStainedGlassPane {
    west: bool,
    east: bool,
    north: bool,
    south: bool,
    waterlogged: bool,
}
impl PinkStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((west, east), north), south), waterlogged)| PinkStainedGlassPane {
                    west,
                    east,
                    north,
                    south,
                    waterlogged,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.west as u16 * 16u16
            + self.east as u16 * 8u16
            + self.north as u16 * 4u16
            + self.south as u16 * 2u16
            + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let west = offset / 16u16;
        offset -= west * 16u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let north = offset / 4u16;
        offset -= north * 4u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let south = offset / 2u16;
        offset -= south * 2u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            west,
            east,
            north,
            south,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<pink_stained_glass_pane, u16>> = Lazy::new(|| {
            pink_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6013u16, 6014u16, 6015u16, 6016u16, 6017u16, 6018u16, 6019u16, 6020u16,
                        6021u16, 6022u16, 6023u16, 6024u16, 6025u16, 6026u16, 6027u16, 6028u16,
                        6029u16, 6030u16, 6031u16, 6032u16, 6033u16, 6034u16, 6035u16, 6036u16,
                        6037u16, 6038u16, 6039u16, 6040u16, 6041u16, 6042u16, 6043u16, 6044u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayStainedGlassPane {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    waterlogged: bool,
}
impl GrayStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((north, east), south), west), waterlogged)| GrayStainedGlassPane {
                    north,
                    east,
                    south,
                    west,
                    waterlogged,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.north as u16 * 16u16
            + self.east as u16 * 8u16
            + self.south as u16 * 4u16
            + self.west as u16 * 2u16
            + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let south = offset / 4u16;
        offset -= south * 4u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 2u16;
        offset -= west * 2u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            north,
            east,
            south,
            west,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<gray_stained_glass_pane, u16>> = Lazy::new(|| {
            gray_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6045u16, 6046u16, 6047u16, 6048u16, 6049u16, 6050u16, 6051u16, 6052u16,
                        6053u16, 6054u16, 6055u16, 6056u16, 6057u16, 6058u16, 6059u16, 6060u16,
                        6061u16, 6062u16, 6063u16, 6064u16, 6065u16, 6066u16, 6067u16, 6068u16,
                        6069u16, 6070u16, 6071u16, 6072u16, 6073u16, 6074u16, 6075u16, 6076u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayStainedGlassPane {
    south: bool,
    east: bool,
    waterlogged: bool,
    west: bool,
    north: bool,
}
impl LightGrayStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((south, east), waterlogged), west), north)| LightGrayStainedGlassPane {
                    south,
                    east,
                    waterlogged,
                    west,
                    north,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 16u16
            + self.east as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.west as u16 * 2u16
            + self.north as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let west = offset / 2u16;
        offset -= west * 2u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let north = offset / 1u16;
        offset -= north * 1u16;
        let north = Ok(if north == 0 { false } else { true })?;
        Self {
            south,
            east,
            waterlogged,
            west,
            north,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_gray_stained_glass_pane, u16>> = Lazy::new(|| {
            light_gray_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6077u16, 6078u16, 6079u16, 6080u16, 6081u16, 6082u16, 6083u16, 6084u16,
                        6085u16, 6086u16, 6087u16, 6088u16, 6089u16, 6090u16, 6091u16, 6092u16,
                        6093u16, 6094u16, 6095u16, 6096u16, 6097u16, 6098u16, 6099u16, 6100u16,
                        6101u16, 6102u16, 6103u16, 6104u16, 6105u16, 6106u16, 6107u16, 6108u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanStainedGlassPane {
    north: bool,
    west: bool,
    waterlogged: bool,
    south: bool,
    east: bool,
}
impl CyanStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((north, west), waterlogged), south), east)| CyanStainedGlassPane {
                    north,
                    west,
                    waterlogged,
                    south,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.north as u16 * 16u16
            + self.west as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.south as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let west = offset / 8u16;
        offset -= west * 8u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let south = offset / 2u16;
        offset -= south * 2u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            north,
            west,
            waterlogged,
            south,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cyan_stained_glass_pane, u16>> = Lazy::new(|| {
            cyan_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6109u16, 6110u16, 6111u16, 6112u16, 6113u16, 6114u16, 6115u16, 6116u16,
                        6117u16, 6118u16, 6119u16, 6120u16, 6121u16, 6122u16, 6123u16, 6124u16,
                        6125u16, 6126u16, 6127u16, 6128u16, 6129u16, 6130u16, 6131u16, 6132u16,
                        6133u16, 6134u16, 6135u16, 6136u16, 6137u16, 6138u16, 6139u16, 6140u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleStainedGlassPane {
    south: bool,
    west: bool,
    waterlogged: bool,
    north: bool,
    east: bool,
}
impl PurpleStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((south, west), waterlogged), north), east)| PurpleStainedGlassPane {
                    south,
                    west,
                    waterlogged,
                    north,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 16u16
            + self.west as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.north as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 8u16;
        offset -= west * 8u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            south,
            west,
            waterlogged,
            north,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<purple_stained_glass_pane, u16>> = Lazy::new(|| {
            purple_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6141u16, 6142u16, 6143u16, 6144u16, 6145u16, 6146u16, 6147u16, 6148u16,
                        6149u16, 6150u16, 6151u16, 6152u16, 6153u16, 6154u16, 6155u16, 6156u16,
                        6157u16, 6158u16, 6159u16, 6160u16, 6161u16, 6162u16, 6163u16, 6164u16,
                        6165u16, 6166u16, 6167u16, 6168u16, 6169u16, 6170u16, 6171u16, 6172u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueStainedGlassPane {
    north: bool,
    south: bool,
    west: bool,
    waterlogged: bool,
    east: bool,
}
impl BlueStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((north, south), west), waterlogged), east)| BlueStainedGlassPane {
                    north,
                    south,
                    west,
                    waterlogged,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.north as u16 * 16u16
            + self.south as u16 * 8u16
            + self.west as u16 * 4u16
            + self.waterlogged as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 4u16;
        offset -= west * 4u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let waterlogged = offset / 2u16;
        offset -= waterlogged * 2u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            north,
            south,
            west,
            waterlogged,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<blue_stained_glass_pane, u16>> = Lazy::new(|| {
            blue_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6173u16, 6174u16, 6175u16, 6176u16, 6177u16, 6178u16, 6179u16, 6180u16,
                        6181u16, 6182u16, 6183u16, 6184u16, 6185u16, 6186u16, 6187u16, 6188u16,
                        6189u16, 6190u16, 6191u16, 6192u16, 6193u16, 6194u16, 6195u16, 6196u16,
                        6197u16, 6198u16, 6199u16, 6200u16, 6201u16, 6202u16, 6203u16, 6204u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownStainedGlassPane {
    waterlogged: bool,
    east: bool,
    north: bool,
    south: bool,
    west: bool,
}
impl BrownStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((waterlogged, east), north), south), west)| BrownStainedGlassPane {
                    waterlogged,
                    east,
                    north,
                    south,
                    west,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 16u16
            + self.east as u16 * 8u16
            + self.north as u16 * 4u16
            + self.south as u16 * 2u16
            + self.west as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 16u16;
        offset -= waterlogged * 16u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let east = offset / 8u16;
        offset -= east * 8u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let north = offset / 4u16;
        offset -= north * 4u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let south = offset / 2u16;
        offset -= south * 2u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 1u16;
        offset -= west * 1u16;
        let west = Ok(if west == 0 { false } else { true })?;
        Self {
            waterlogged,
            east,
            north,
            south,
            west,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brown_stained_glass_pane, u16>> = Lazy::new(|| {
            brown_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6205u16, 6206u16, 6207u16, 6208u16, 6209u16, 6210u16, 6211u16, 6212u16,
                        6213u16, 6214u16, 6215u16, 6216u16, 6217u16, 6218u16, 6219u16, 6220u16,
                        6221u16, 6222u16, 6223u16, 6224u16, 6225u16, 6226u16, 6227u16, 6228u16,
                        6229u16, 6230u16, 6231u16, 6232u16, 6233u16, 6234u16, 6235u16, 6236u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenStainedGlassPane {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    waterlogged: bool,
}
impl GreenStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((north, south), east), west), waterlogged)| GreenStainedGlassPane {
                    north,
                    south,
                    east,
                    west,
                    waterlogged,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.north as u16 * 16u16
            + self.south as u16 * 8u16
            + self.east as u16 * 4u16
            + self.west as u16 * 2u16
            + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let north = offset / 16u16;
        offset -= north * 16u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let east = offset / 4u16;
        offset -= east * 4u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let west = offset / 2u16;
        offset -= west * 2u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            north,
            south,
            east,
            west,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<green_stained_glass_pane, u16>> = Lazy::new(|| {
            green_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6237u16, 6238u16, 6239u16, 6240u16, 6241u16, 6242u16, 6243u16, 6244u16,
                        6245u16, 6246u16, 6247u16, 6248u16, 6249u16, 6250u16, 6251u16, 6252u16,
                        6253u16, 6254u16, 6255u16, 6256u16, 6257u16, 6258u16, 6259u16, 6260u16,
                        6261u16, 6262u16, 6263u16, 6264u16, 6265u16, 6266u16, 6267u16, 6268u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedStainedGlassPane {
    west: bool,
    south: bool,
    waterlogged: bool,
    north: bool,
    east: bool,
}
impl RedStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((west, south), waterlogged), north), east)| RedStainedGlassPane {
                    west,
                    south,
                    waterlogged,
                    north,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.west as u16 * 16u16
            + self.south as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.north as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let west = offset / 16u16;
        offset -= west * 16u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            west,
            south,
            waterlogged,
            north,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<red_stained_glass_pane, u16>> = Lazy::new(|| {
            red_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6269u16, 6270u16, 6271u16, 6272u16, 6273u16, 6274u16, 6275u16, 6276u16,
                        6277u16, 6278u16, 6279u16, 6280u16, 6281u16, 6282u16, 6283u16, 6284u16,
                        6285u16, 6286u16, 6287u16, 6288u16, 6289u16, 6290u16, 6291u16, 6292u16,
                        6293u16, 6294u16, 6295u16, 6296u16, 6297u16, 6298u16, 6299u16, 6300u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackStainedGlassPane {
    south: bool,
    west: bool,
    waterlogged: bool,
    north: bool,
    east: bool,
}
impl BlackStainedGlassPane {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(
                |((((south, west), waterlogged), north), east)| BlackStainedGlassPane {
                    south,
                    west,
                    waterlogged,
                    north,
                    east,
                },
            )
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.south as u16 * 16u16
            + self.west as u16 * 8u16
            + self.waterlogged as u16 * 4u16
            + self.north as u16 * 2u16
            + self.east as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let south = offset / 16u16;
        offset -= south * 16u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 8u16;
        offset -= west * 8u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let north = offset / 2u16;
        offset -= north * 2u16;
        let north = Ok(if north == 0 { false } else { true })?;
        let east = offset / 1u16;
        offset -= east * 1u16;
        let east = Ok(if east == 0 { false } else { true })?;
        Self {
            south,
            west,
            waterlogged,
            north,
            east,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<black_stained_glass_pane, u16>> = Lazy::new(|| {
            black_stained_glass_pane::possible_states()
                .into_iter()
                .zip(
                    [
                        6301u16, 6302u16, 6303u16, 6304u16, 6305u16, 6306u16, 6307u16, 6308u16,
                        6309u16, 6310u16, 6311u16, 6312u16, 6313u16, 6314u16, 6315u16, 6316u16,
                        6317u16, 6318u16, 6319u16, 6320u16, 6321u16, 6322u16, 6323u16, 6324u16,
                        6325u16, 6326u16, 6327u16, 6328u16, 6329u16, 6330u16, 6331u16, 6332u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlimeBlock {}
impl SlimeBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![SlimeBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(slime_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Barrier {}
impl Barrier {
    pub fn possible_states() -> Vec<Self> {
        vec![Barrier {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(barrier {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Prismarine {}
impl Prismarine {
    pub fn possible_states() -> Vec<Self> {
        vec![Prismarine {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(prismarine {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrismarineBricks {}
impl PrismarineBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![PrismarineBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(prismarine_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DarkPrismarine {}
impl DarkPrismarine {
    pub fn possible_states() -> Vec<Self> {
        vec![DarkPrismarine {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dark_prismarine {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SeaLantern {}
impl SeaLantern {
    pub fn possible_states() -> Vec<Self> {
        vec![SeaLantern {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(sea_lantern {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HayBlock {
    axis: HayBlock,
}
impl HayBlock {
    pub fn possible_states() -> Vec<Self> {
        [HayBlock::X, HayBlock::Y, HayBlock::Z]
            .iter()
            .copied()
            .map(|axis| HayBlock { axis })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.axis as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let axis = offset / 1u16;
        offset -= axis * 1u16;
        let axis = HayBlock::try_from(axis)?;
        Self { axis }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<hay_block, u16>> = Lazy::new(|| {
            hay_block::possible_states()
                .into_iter()
                .zip([6821u16, 6822u16, 6823u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteCarpet {}
impl WhiteCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeCarpet {}
impl OrangeCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaCarpet {}
impl MagentaCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![MagentaCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magenta_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueCarpet {}
impl LightBlueCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![LightBlueCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_blue_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowCarpet {}
impl YellowCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![YellowCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(yellow_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeCarpet {}
impl LimeCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![LimeCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lime_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkCarpet {}
impl PinkCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayCarpet {}
impl GrayCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![GrayCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gray_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayCarpet {}
impl LightGrayCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![LightGrayCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_gray_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanCarpet {}
impl CyanCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![CyanCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cyan_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleCarpet {}
impl PurpleCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpleCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purple_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueCarpet {}
impl BlueCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownCarpet {}
impl BrownCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenCarpet {}
impl GreenCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![GreenCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(green_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedCarpet {}
impl RedCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![RedCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackCarpet {}
impl BlackCarpet {
    pub fn possible_states() -> Vec<Self> {
        vec![BlackCarpet {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(black_carpet {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Terracotta {}
impl Terracotta {
    pub fn possible_states() -> Vec<Self> {
        vec![Terracotta {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(terracotta {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoalBlock {}
impl CoalBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![CoalBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(coal_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackedIce {}
impl PackedIce {
    pub fn possible_states() -> Vec<Self> {
        vec![PackedIce {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(packed_ice {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sunflower {
    half: Sunflower,
}
impl Sunflower {
    pub fn possible_states() -> Vec<Self> {
        [Sunflower::Upper, Sunflower::Lower]
            .iter()
            .copied()
            .map(|half| Sunflower { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = Sunflower::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<sunflower, u16>> = Lazy::new(|| {
            sunflower::possible_states()
                .into_iter()
                .zip([6843u16, 6844u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Lilac {
    half: Lilac,
}
impl Lilac {
    pub fn possible_states() -> Vec<Self> {
        [Lilac::Upper, Lilac::Lower]
            .iter()
            .copied()
            .map(|half| Lilac { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = Lilac::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lilac, u16>> = Lazy::new(|| {
            lilac::possible_states()
                .into_iter()
                .zip([6845u16, 6846u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoseBush {
    half: RoseBush,
}
impl RoseBush {
    pub fn possible_states() -> Vec<Self> {
        [RoseBush::Upper, RoseBush::Lower]
            .iter()
            .copied()
            .map(|half| RoseBush { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = RoseBush::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<rose_bush, u16>> = Lazy::new(|| {
            rose_bush::possible_states()
                .into_iter()
                .zip([6847u16, 6848u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Peony {
    half: Peony,
}
impl Peony {
    pub fn possible_states() -> Vec<Self> {
        [Peony::Upper, Peony::Lower]
            .iter()
            .copied()
            .map(|half| Peony { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = Peony::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<peony, u16>> = Lazy::new(|| {
            peony::possible_states()
                .into_iter()
                .zip([6849u16, 6850u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TallGrass {
    half: TallGrass,
}
impl TallGrass {
    pub fn possible_states() -> Vec<Self> {
        [TallGrass::Upper, TallGrass::Lower]
            .iter()
            .copied()
            .map(|half| TallGrass { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = TallGrass::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tall_grass, u16>> = Lazy::new(|| {
            tall_grass::possible_states()
                .into_iter()
                .zip([6851u16, 6852u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LargeFern {
    half: LargeFern,
}
impl LargeFern {
    pub fn possible_states() -> Vec<Self> {
        [LargeFern::Upper, LargeFern::Lower]
            .iter()
            .copied()
            .map(|half| LargeFern { half })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = LargeFern::try_from(half)?;
        Self { half }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<large_fern, u16>> = Lazy::new(|| {
            large_fern::possible_states()
                .into_iter()
                .zip([6853u16, 6854u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteBanner {
    rotation: i32,
}
impl WhiteBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| WhiteBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<white_banner, u16>> = Lazy::new(|| {
            white_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6855u16, 6856u16, 6857u16, 6858u16, 6859u16, 6860u16, 6861u16, 6862u16,
                        6863u16, 6864u16, 6865u16, 6866u16, 6867u16, 6868u16, 6869u16, 6870u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeBanner {
    rotation: i32,
}
impl OrangeBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| OrangeBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<orange_banner, u16>> = Lazy::new(|| {
            orange_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6871u16, 6872u16, 6873u16, 6874u16, 6875u16, 6876u16, 6877u16, 6878u16,
                        6879u16, 6880u16, 6881u16, 6882u16, 6883u16, 6884u16, 6885u16, 6886u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaBanner {
    rotation: i32,
}
impl MagentaBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| MagentaBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<magenta_banner, u16>> = Lazy::new(|| {
            magenta_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6887u16, 6888u16, 6889u16, 6890u16, 6891u16, 6892u16, 6893u16, 6894u16,
                        6895u16, 6896u16, 6897u16, 6898u16, 6899u16, 6900u16, 6901u16, 6902u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueBanner {
    rotation: i32,
}
impl LightBlueBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| LightBlueBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_blue_banner, u16>> = Lazy::new(|| {
            light_blue_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6903u16, 6904u16, 6905u16, 6906u16, 6907u16, 6908u16, 6909u16, 6910u16,
                        6911u16, 6912u16, 6913u16, 6914u16, 6915u16, 6916u16, 6917u16, 6918u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowBanner {
    rotation: i32,
}
impl YellowBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| YellowBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<yellow_banner, u16>> = Lazy::new(|| {
            yellow_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6919u16, 6920u16, 6921u16, 6922u16, 6923u16, 6924u16, 6925u16, 6926u16,
                        6927u16, 6928u16, 6929u16, 6930u16, 6931u16, 6932u16, 6933u16, 6934u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeBanner {
    rotation: i32,
}
impl LimeBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| LimeBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lime_banner, u16>> = Lazy::new(|| {
            lime_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6935u16, 6936u16, 6937u16, 6938u16, 6939u16, 6940u16, 6941u16, 6942u16,
                        6943u16, 6944u16, 6945u16, 6946u16, 6947u16, 6948u16, 6949u16, 6950u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkBanner {
    rotation: i32,
}
impl PinkBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| PinkBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<pink_banner, u16>> = Lazy::new(|| {
            pink_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6951u16, 6952u16, 6953u16, 6954u16, 6955u16, 6956u16, 6957u16, 6958u16,
                        6959u16, 6960u16, 6961u16, 6962u16, 6963u16, 6964u16, 6965u16, 6966u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayBanner {
    rotation: i32,
}
impl GrayBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| GrayBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<gray_banner, u16>> = Lazy::new(|| {
            gray_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6967u16, 6968u16, 6969u16, 6970u16, 6971u16, 6972u16, 6973u16, 6974u16,
                        6975u16, 6976u16, 6977u16, 6978u16, 6979u16, 6980u16, 6981u16, 6982u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayBanner {
    rotation: i32,
}
impl LightGrayBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| LightGrayBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_gray_banner, u16>> = Lazy::new(|| {
            light_gray_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6983u16, 6984u16, 6985u16, 6986u16, 6987u16, 6988u16, 6989u16, 6990u16,
                        6991u16, 6992u16, 6993u16, 6994u16, 6995u16, 6996u16, 6997u16, 6998u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanBanner {
    rotation: i32,
}
impl CyanBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| CyanBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cyan_banner, u16>> = Lazy::new(|| {
            cyan_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        6999u16, 7000u16, 7001u16, 7002u16, 7003u16, 7004u16, 7005u16, 7006u16,
                        7007u16, 7008u16, 7009u16, 7010u16, 7011u16, 7012u16, 7013u16, 7014u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleBanner {
    rotation: i32,
}
impl PurpleBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| PurpleBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<purple_banner, u16>> = Lazy::new(|| {
            purple_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        7015u16, 7016u16, 7017u16, 7018u16, 7019u16, 7020u16, 7021u16, 7022u16,
                        7023u16, 7024u16, 7025u16, 7026u16, 7027u16, 7028u16, 7029u16, 7030u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueBanner {
    rotation: i32,
}
impl BlueBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| BlueBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<blue_banner, u16>> = Lazy::new(|| {
            blue_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        7031u16, 7032u16, 7033u16, 7034u16, 7035u16, 7036u16, 7037u16, 7038u16,
                        7039u16, 7040u16, 7041u16, 7042u16, 7043u16, 7044u16, 7045u16, 7046u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownBanner {
    rotation: i32,
}
impl BrownBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| BrownBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brown_banner, u16>> = Lazy::new(|| {
            brown_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        7047u16, 7048u16, 7049u16, 7050u16, 7051u16, 7052u16, 7053u16, 7054u16,
                        7055u16, 7056u16, 7057u16, 7058u16, 7059u16, 7060u16, 7061u16, 7062u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenBanner {
    rotation: i32,
}
impl GreenBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| GreenBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<green_banner, u16>> = Lazy::new(|| {
            green_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        7063u16, 7064u16, 7065u16, 7066u16, 7067u16, 7068u16, 7069u16, 7070u16,
                        7071u16, 7072u16, 7073u16, 7074u16, 7075u16, 7076u16, 7077u16, 7078u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedBanner {
    rotation: i32,
}
impl RedBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| RedBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<red_banner, u16>> = Lazy::new(|| {
            red_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        7079u16, 7080u16, 7081u16, 7082u16, 7083u16, 7084u16, 7085u16, 7086u16,
                        7087u16, 7088u16, 7089u16, 7090u16, 7091u16, 7092u16, 7093u16, 7094u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackBanner {
    rotation: i32,
}
impl BlackBanner {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=15i32 }
            .map(|rotation| BlackBanner { rotation })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.rotation as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let rotation = offset / 1u16;
        offset -= rotation * 1u16;
        let rotation = Ok(rotation as i32)?;
        Self { rotation }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<black_banner, u16>> = Lazy::new(|| {
            black_banner::possible_states()
                .into_iter()
                .zip(
                    [
                        7095u16, 7096u16, 7097u16, 7098u16, 7099u16, 7100u16, 7101u16, 7102u16,
                        7103u16, 7104u16, 7105u16, 7106u16, 7107u16, 7108u16, 7109u16, 7110u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteWallBanner {
    facing: WhiteWallBanner,
}
impl WhiteWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            WhiteWallBanner::North,
            WhiteWallBanner::South,
            WhiteWallBanner::West,
            WhiteWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| WhiteWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = WhiteWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<white_wall_banner, u16>> = Lazy::new(|| {
            white_wall_banner::possible_states()
                .into_iter()
                .zip([7111u16, 7112u16, 7113u16, 7114u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeWallBanner {
    facing: OrangeWallBanner,
}
impl OrangeWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            OrangeWallBanner::North,
            OrangeWallBanner::South,
            OrangeWallBanner::West,
            OrangeWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| OrangeWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = OrangeWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<orange_wall_banner, u16>> = Lazy::new(|| {
            orange_wall_banner::possible_states()
                .into_iter()
                .zip([7115u16, 7116u16, 7117u16, 7118u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaWallBanner {
    facing: MagentaWallBanner,
}
impl MagentaWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            MagentaWallBanner::North,
            MagentaWallBanner::South,
            MagentaWallBanner::West,
            MagentaWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| MagentaWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = MagentaWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<magenta_wall_banner, u16>> = Lazy::new(|| {
            magenta_wall_banner::possible_states()
                .into_iter()
                .zip([7119u16, 7120u16, 7121u16, 7122u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueWallBanner {
    facing: LightBlueWallBanner,
}
impl LightBlueWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            LightBlueWallBanner::North,
            LightBlueWallBanner::South,
            LightBlueWallBanner::West,
            LightBlueWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| LightBlueWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LightBlueWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_blue_wall_banner, u16>> = Lazy::new(|| {
            light_blue_wall_banner::possible_states()
                .into_iter()
                .zip([7123u16, 7124u16, 7125u16, 7126u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowWallBanner {
    facing: YellowWallBanner,
}
impl YellowWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            YellowWallBanner::North,
            YellowWallBanner::South,
            YellowWallBanner::West,
            YellowWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| YellowWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = YellowWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<yellow_wall_banner, u16>> = Lazy::new(|| {
            yellow_wall_banner::possible_states()
                .into_iter()
                .zip([7127u16, 7128u16, 7129u16, 7130u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeWallBanner {
    facing: LimeWallBanner,
}
impl LimeWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            LimeWallBanner::North,
            LimeWallBanner::South,
            LimeWallBanner::West,
            LimeWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| LimeWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LimeWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lime_wall_banner, u16>> = Lazy::new(|| {
            lime_wall_banner::possible_states()
                .into_iter()
                .zip([7131u16, 7132u16, 7133u16, 7134u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkWallBanner {
    facing: PinkWallBanner,
}
impl PinkWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            PinkWallBanner::North,
            PinkWallBanner::South,
            PinkWallBanner::West,
            PinkWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| PinkWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PinkWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<pink_wall_banner, u16>> = Lazy::new(|| {
            pink_wall_banner::possible_states()
                .into_iter()
                .zip([7135u16, 7136u16, 7137u16, 7138u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayWallBanner {
    facing: GrayWallBanner,
}
impl GrayWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            GrayWallBanner::North,
            GrayWallBanner::South,
            GrayWallBanner::West,
            GrayWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| GrayWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = GrayWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<gray_wall_banner, u16>> = Lazy::new(|| {
            gray_wall_banner::possible_states()
                .into_iter()
                .zip([7139u16, 7140u16, 7141u16, 7142u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayWallBanner {
    facing: LightGrayWallBanner,
}
impl LightGrayWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            LightGrayWallBanner::North,
            LightGrayWallBanner::South,
            LightGrayWallBanner::West,
            LightGrayWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| LightGrayWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LightGrayWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_gray_wall_banner, u16>> = Lazy::new(|| {
            light_gray_wall_banner::possible_states()
                .into_iter()
                .zip([7143u16, 7144u16, 7145u16, 7146u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanWallBanner {
    facing: CyanWallBanner,
}
impl CyanWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            CyanWallBanner::North,
            CyanWallBanner::South,
            CyanWallBanner::West,
            CyanWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| CyanWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = CyanWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cyan_wall_banner, u16>> = Lazy::new(|| {
            cyan_wall_banner::possible_states()
                .into_iter()
                .zip([7147u16, 7148u16, 7149u16, 7150u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleWallBanner {
    facing: PurpleWallBanner,
}
impl PurpleWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            PurpleWallBanner::North,
            PurpleWallBanner::South,
            PurpleWallBanner::West,
            PurpleWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| PurpleWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PurpleWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<purple_wall_banner, u16>> = Lazy::new(|| {
            purple_wall_banner::possible_states()
                .into_iter()
                .zip([7151u16, 7152u16, 7153u16, 7154u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueWallBanner {
    facing: BlueWallBanner,
}
impl BlueWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            BlueWallBanner::North,
            BlueWallBanner::South,
            BlueWallBanner::West,
            BlueWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| BlueWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BlueWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<blue_wall_banner, u16>> = Lazy::new(|| {
            blue_wall_banner::possible_states()
                .into_iter()
                .zip([7155u16, 7156u16, 7157u16, 7158u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownWallBanner {
    facing: BrownWallBanner,
}
impl BrownWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            BrownWallBanner::North,
            BrownWallBanner::South,
            BrownWallBanner::West,
            BrownWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| BrownWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BrownWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brown_wall_banner, u16>> = Lazy::new(|| {
            brown_wall_banner::possible_states()
                .into_iter()
                .zip([7159u16, 7160u16, 7161u16, 7162u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenWallBanner {
    facing: GreenWallBanner,
}
impl GreenWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            GreenWallBanner::North,
            GreenWallBanner::South,
            GreenWallBanner::West,
            GreenWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| GreenWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = GreenWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<green_wall_banner, u16>> = Lazy::new(|| {
            green_wall_banner::possible_states()
                .into_iter()
                .zip([7163u16, 7164u16, 7165u16, 7166u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedWallBanner {
    facing: RedWallBanner,
}
impl RedWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            RedWallBanner::North,
            RedWallBanner::South,
            RedWallBanner::West,
            RedWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| RedWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = RedWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<red_wall_banner, u16>> = Lazy::new(|| {
            red_wall_banner::possible_states()
                .into_iter()
                .zip([7167u16, 7168u16, 7169u16, 7170u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackWallBanner {
    facing: BlackWallBanner,
}
impl BlackWallBanner {
    pub fn possible_states() -> Vec<Self> {
        [
            BlackWallBanner::North,
            BlackWallBanner::South,
            BlackWallBanner::West,
            BlackWallBanner::East,
        ]
        .iter()
        .copied()
        .map(|facing| BlackWallBanner { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BlackWallBanner::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<black_wall_banner, u16>> = Lazy::new(|| {
            black_wall_banner::possible_states()
                .into_iter()
                .zip([7171u16, 7172u16, 7173u16, 7174u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedSandstone {}
impl RedSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![RedSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChiseledRedSandstone {}
impl ChiseledRedSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![ChiseledRedSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(chiseled_red_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CutRedSandstone {}
impl CutRedSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![CutRedSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cut_red_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SmoothStone {}
impl SmoothStone {
    pub fn possible_states() -> Vec<Self> {
        vec![SmoothStone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(smooth_stone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SmoothSandstone {}
impl SmoothSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![SmoothSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(smooth_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SmoothQuartz {}
impl SmoothQuartz {
    pub fn possible_states() -> Vec<Self> {
        vec![SmoothQuartz {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(smooth_quartz {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SmoothRedSandstone {}
impl SmoothRedSandstone {
    pub fn possible_states() -> Vec<Self> {
        vec![SmoothRedSandstone {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(smooth_red_sandstone {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpruceFenceGate {
    facing: SpruceFenceGate,
    in_wall: bool,
    powered: bool,
    open: bool,
}
impl SpruceFenceGate {
    pub fn possible_states() -> Vec<Self> {
        [
            SpruceFenceGate::North,
            SpruceFenceGate::South,
            SpruceFenceGate::West,
            SpruceFenceGate::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(((facing, in_wall), powered), open)| SpruceFenceGate {
            facing,
            in_wall,
            powered,
            open,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 8u16
            + self.in_wall as u16 * 4u16
            + self.powered as u16 * 2u16
            + self.open as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 8u16;
        offset -= facing * 8u16;
        let facing = SpruceFenceGate::try_from(facing)?;
        let in_wall = offset / 4u16;
        offset -= in_wall * 4u16;
        let in_wall = Ok(if in_wall == 0 { false } else { true })?;
        let powered = offset / 2u16;
        offset -= powered * 2u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let open = offset / 1u16;
        offset -= open * 1u16;
        let open = Ok(if open == 0 { false } else { true })?;
        Self {
            facing,
            in_wall,
            powered,
            open,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<spruce_fence_gate, u16>> = Lazy::new(|| {
            spruce_fence_gate::possible_states()
                .into_iter()
                .zip(
                    [
                        7358u16, 7359u16, 7360u16, 7361u16, 7362u16, 7363u16, 7364u16, 7365u16,
                        7366u16, 7367u16, 7368u16, 7369u16, 7370u16, 7371u16, 7372u16, 7373u16,
                        7374u16, 7375u16, 7376u16, 7377u16, 7378u16, 7379u16, 7380u16, 7381u16,
                        7382u16, 7383u16, 7384u16, 7385u16, 7386u16, 7387u16, 7388u16, 7389u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BirchFenceGate {
    open: bool,
    powered: bool,
    facing: BirchFenceGate,
    in_wall: bool,
}
impl BirchFenceGate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        BirchFenceGate::North,
                        BirchFenceGate::South,
                        BirchFenceGate::West,
                        BirchFenceGate::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((open, powered), facing), in_wall)| BirchFenceGate {
                open,
                powered,
                facing,
                in_wall,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.open as u16 * 16u16
            + self.powered as u16 * 8u16
            + self.facing as u16 * 2u16
            + self.in_wall as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let open = offset / 16u16;
        offset -= open * 16u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let powered = offset / 8u16;
        offset -= powered * 8u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = BirchFenceGate::try_from(facing)?;
        let in_wall = offset / 1u16;
        offset -= in_wall * 1u16;
        let in_wall = Ok(if in_wall == 0 { false } else { true })?;
        Self {
            open,
            powered,
            facing,
            in_wall,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<birch_fence_gate, u16>> = Lazy::new(|| {
            birch_fence_gate::possible_states()
                .into_iter()
                .zip(
                    [
                        7390u16, 7391u16, 7392u16, 7393u16, 7394u16, 7395u16, 7396u16, 7397u16,
                        7398u16, 7399u16, 7400u16, 7401u16, 7402u16, 7403u16, 7404u16, 7405u16,
                        7406u16, 7407u16, 7408u16, 7409u16, 7410u16, 7411u16, 7412u16, 7413u16,
                        7414u16, 7415u16, 7416u16, 7417u16, 7418u16, 7419u16, 7420u16, 7421u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JungleFenceGate {
    facing: JungleFenceGate,
    open: bool,
    powered: bool,
    in_wall: bool,
}
impl JungleFenceGate {
    pub fn possible_states() -> Vec<Self> {
        [
            JungleFenceGate::North,
            JungleFenceGate::South,
            JungleFenceGate::West,
            JungleFenceGate::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(((facing, open), powered), in_wall)| JungleFenceGate {
            facing,
            open,
            powered,
            in_wall,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 8u16
            + self.open as u16 * 4u16
            + self.powered as u16 * 2u16
            + self.in_wall as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 8u16;
        offset -= facing * 8u16;
        let facing = JungleFenceGate::try_from(facing)?;
        let open = offset / 4u16;
        offset -= open * 4u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let powered = offset / 2u16;
        offset -= powered * 2u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let in_wall = offset / 1u16;
        offset -= in_wall * 1u16;
        let in_wall = Ok(if in_wall == 0 { false } else { true })?;
        Self {
            facing,
            open,
            powered,
            in_wall,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<jungle_fence_gate, u16>> = Lazy::new(|| {
            jungle_fence_gate::possible_states()
                .into_iter()
                .zip(
                    [
                        7422u16, 7423u16, 7424u16, 7425u16, 7426u16, 7427u16, 7428u16, 7429u16,
                        7430u16, 7431u16, 7432u16, 7433u16, 7434u16, 7435u16, 7436u16, 7437u16,
                        7438u16, 7439u16, 7440u16, 7441u16, 7442u16, 7443u16, 7444u16, 7445u16,
                        7446u16, 7447u16, 7448u16, 7449u16, 7450u16, 7451u16, 7452u16, 7453u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AcaciaFenceGate {
    powered: bool,
    facing: AcaciaFenceGate,
    open: bool,
    in_wall: bool,
}
impl AcaciaFenceGate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        AcaciaFenceGate::North,
                        AcaciaFenceGate::South,
                        AcaciaFenceGate::West,
                        AcaciaFenceGate::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((powered, facing), open), in_wall)| AcaciaFenceGate {
                powered,
                facing,
                open,
                in_wall,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.powered as u16 * 16u16
            + self.facing as u16 * 4u16
            + self.open as u16 * 2u16
            + self.in_wall as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let powered = offset / 16u16;
        offset -= powered * 16u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let facing = offset / 4u16;
        offset -= facing * 4u16;
        let facing = AcaciaFenceGate::try_from(facing)?;
        let open = offset / 2u16;
        offset -= open * 2u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let in_wall = offset / 1u16;
        offset -= in_wall * 1u16;
        let in_wall = Ok(if in_wall == 0 { false } else { true })?;
        Self {
            powered,
            facing,
            open,
            in_wall,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<acacia_fence_gate, u16>> = Lazy::new(|| {
            acacia_fence_gate::possible_states()
                .into_iter()
                .zip(
                    [
                        7454u16, 7455u16, 7456u16, 7457u16, 7458u16, 7459u16, 7460u16, 7461u16,
                        7462u16, 7463u16, 7464u16, 7465u16, 7466u16, 7467u16, 7468u16, 7469u16,
                        7470u16, 7471u16, 7472u16, 7473u16, 7474u16, 7475u16, 7476u16, 7477u16,
                        7478u16, 7479u16, 7480u16, 7481u16, 7482u16, 7483u16, 7484u16, 7485u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DarkOakFenceGate {
    in_wall: bool,
    open: bool,
    facing: DarkOakFenceGate,
    powered: bool,
}
impl DarkOakFenceGate {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        DarkOakFenceGate::North,
                        DarkOakFenceGate::South,
                        DarkOakFenceGate::West,
                        DarkOakFenceGate::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((in_wall, open), facing), powered)| DarkOakFenceGate {
                in_wall,
                open,
                facing,
                powered,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.in_wall as u16 * 16u16
            + self.open as u16 * 8u16
            + self.facing as u16 * 2u16
            + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let in_wall = offset / 16u16;
        offset -= in_wall * 16u16;
        let in_wall = Ok(if in_wall == 0 { false } else { true })?;
        let open = offset / 8u16;
        offset -= open * 8u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = DarkOakFenceGate::try_from(facing)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self {
            in_wall,
            open,
            facing,
            powered,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dark_oak_fence_gate, u16>> = Lazy::new(|| {
            dark_oak_fence_gate::possible_states()
                .into_iter()
                .zip(
                    [
                        7486u16, 7487u16, 7488u16, 7489u16, 7490u16, 7491u16, 7492u16, 7493u16,
                        7494u16, 7495u16, 7496u16, 7497u16, 7498u16, 7499u16, 7500u16, 7501u16,
                        7502u16, 7503u16, 7504u16, 7505u16, 7506u16, 7507u16, 7508u16, 7509u16,
                        7510u16, 7511u16, 7512u16, 7513u16, 7514u16, 7515u16, 7516u16, 7517u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpruceDoor {
    open: bool,
    powered: bool,
    hinge: SpruceDoor,
    half: SpruceDoor,
    facing: SpruceDoor,
}
impl SpruceDoor {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip([SpruceDoor::Left, SpruceDoor::Right].iter().copied())
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip([SpruceDoor::Upper, SpruceDoor::Lower].iter().copied())
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        SpruceDoor::North,
                        SpruceDoor::South,
                        SpruceDoor::West,
                        SpruceDoor::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|((((open, powered), hinge), half), facing)| SpruceDoor {
                open,
                powered,
                hinge,
                half,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.open as u16 * 32u16
            + self.powered as u16 * 16u16
            + self.hinge as u16 * 8u16
            + self.half as u16 * 4u16
            + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let open = offset / 32u16;
        offset -= open * 32u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let powered = offset / 16u16;
        offset -= powered * 16u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let hinge = offset / 8u16;
        offset -= hinge * 8u16;
        let hinge = SpruceDoor::try_from(hinge)?;
        let half = offset / 4u16;
        offset -= half * 4u16;
        let half = SpruceDoor::try_from(half)?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = SpruceDoor::try_from(facing)?;
        Self {
            open,
            powered,
            hinge,
            half,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<spruce_door, u16>> = Lazy::new(|| {
            spruce_door::possible_states()
                .into_iter()
                .zip(
                    [
                        7678u16, 7679u16, 7680u16, 7681u16, 7682u16, 7683u16, 7684u16, 7685u16,
                        7686u16, 7687u16, 7688u16, 7689u16, 7690u16, 7691u16, 7692u16, 7693u16,
                        7694u16, 7695u16, 7696u16, 7697u16, 7698u16, 7699u16, 7700u16, 7701u16,
                        7702u16, 7703u16, 7704u16, 7705u16, 7706u16, 7707u16, 7708u16, 7709u16,
                        7710u16, 7711u16, 7712u16, 7713u16, 7714u16, 7715u16, 7716u16, 7717u16,
                        7718u16, 7719u16, 7720u16, 7721u16, 7722u16, 7723u16, 7724u16, 7725u16,
                        7726u16, 7727u16, 7728u16, 7729u16, 7730u16, 7731u16, 7732u16, 7733u16,
                        7734u16, 7735u16, 7736u16, 7737u16, 7738u16, 7739u16, 7740u16, 7741u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BirchDoor {
    half: BirchDoor,
    facing: BirchDoor,
    powered: bool,
    hinge: BirchDoor,
    open: bool,
}
impl BirchDoor {
    pub fn possible_states() -> Vec<Self> {
        [BirchDoor::Upper, BirchDoor::Lower]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        BirchDoor::North,
                        BirchDoor::South,
                        BirchDoor::West,
                        BirchDoor::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip([BirchDoor::Left, BirchDoor::Right].iter().copied())
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|((((half, facing), powered), hinge), open)| BirchDoor {
                half,
                facing,
                powered,
                hinge,
                open,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 32u16
            + self.facing as u16 * 8u16
            + self.powered as u16 * 4u16
            + self.hinge as u16 * 2u16
            + self.open as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 32u16;
        offset -= half * 32u16;
        let half = BirchDoor::try_from(half)?;
        let facing = offset / 8u16;
        offset -= facing * 8u16;
        let facing = BirchDoor::try_from(facing)?;
        let powered = offset / 4u16;
        offset -= powered * 4u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let hinge = offset / 2u16;
        offset -= hinge * 2u16;
        let hinge = BirchDoor::try_from(hinge)?;
        let open = offset / 1u16;
        offset -= open * 1u16;
        let open = Ok(if open == 0 { false } else { true })?;
        Self {
            half,
            facing,
            powered,
            hinge,
            open,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<birch_door, u16>> = Lazy::new(|| {
            birch_door::possible_states()
                .into_iter()
                .zip(
                    [
                        7742u16, 7743u16, 7744u16, 7745u16, 7746u16, 7747u16, 7748u16, 7749u16,
                        7750u16, 7751u16, 7752u16, 7753u16, 7754u16, 7755u16, 7756u16, 7757u16,
                        7758u16, 7759u16, 7760u16, 7761u16, 7762u16, 7763u16, 7764u16, 7765u16,
                        7766u16, 7767u16, 7768u16, 7769u16, 7770u16, 7771u16, 7772u16, 7773u16,
                        7774u16, 7775u16, 7776u16, 7777u16, 7778u16, 7779u16, 7780u16, 7781u16,
                        7782u16, 7783u16, 7784u16, 7785u16, 7786u16, 7787u16, 7788u16, 7789u16,
                        7790u16, 7791u16, 7792u16, 7793u16, 7794u16, 7795u16, 7796u16, 7797u16,
                        7798u16, 7799u16, 7800u16, 7801u16, 7802u16, 7803u16, 7804u16, 7805u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JungleDoor {
    hinge: JungleDoor,
    facing: JungleDoor,
    open: bool,
    powered: bool,
    half: JungleDoor,
}
impl JungleDoor {
    pub fn possible_states() -> Vec<Self> {
        [JungleDoor::Left, JungleDoor::Right]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        JungleDoor::North,
                        JungleDoor::South,
                        JungleDoor::West,
                        JungleDoor::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip([JungleDoor::Upper, JungleDoor::Lower].iter().copied())
            })
            .map(|((((hinge, facing), open), powered), half)| JungleDoor {
                hinge,
                facing,
                open,
                powered,
                half,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.hinge as u16 * 32u16
            + self.facing as u16 * 8u16
            + self.open as u16 * 4u16
            + self.powered as u16 * 2u16
            + self.half as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let hinge = offset / 32u16;
        offset -= hinge * 32u16;
        let hinge = JungleDoor::try_from(hinge)?;
        let facing = offset / 8u16;
        offset -= facing * 8u16;
        let facing = JungleDoor::try_from(facing)?;
        let open = offset / 4u16;
        offset -= open * 4u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let powered = offset / 2u16;
        offset -= powered * 2u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let half = offset / 1u16;
        offset -= half * 1u16;
        let half = JungleDoor::try_from(half)?;
        Self {
            hinge,
            facing,
            open,
            powered,
            half,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<jungle_door, u16>> = Lazy::new(|| {
            jungle_door::possible_states()
                .into_iter()
                .zip(
                    [
                        7806u16, 7807u16, 7808u16, 7809u16, 7810u16, 7811u16, 7812u16, 7813u16,
                        7814u16, 7815u16, 7816u16, 7817u16, 7818u16, 7819u16, 7820u16, 7821u16,
                        7822u16, 7823u16, 7824u16, 7825u16, 7826u16, 7827u16, 7828u16, 7829u16,
                        7830u16, 7831u16, 7832u16, 7833u16, 7834u16, 7835u16, 7836u16, 7837u16,
                        7838u16, 7839u16, 7840u16, 7841u16, 7842u16, 7843u16, 7844u16, 7845u16,
                        7846u16, 7847u16, 7848u16, 7849u16, 7850u16, 7851u16, 7852u16, 7853u16,
                        7854u16, 7855u16, 7856u16, 7857u16, 7858u16, 7859u16, 7860u16, 7861u16,
                        7862u16, 7863u16, 7864u16, 7865u16, 7866u16, 7867u16, 7868u16, 7869u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AcaciaDoor {
    half: AcaciaDoor,
    powered: bool,
    open: bool,
    facing: AcaciaDoor,
    hinge: AcaciaDoor,
}
impl AcaciaDoor {
    pub fn possible_states() -> Vec<Self> {
        [AcaciaDoor::Upper, AcaciaDoor::Lower]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        AcaciaDoor::North,
                        AcaciaDoor::South,
                        AcaciaDoor::West,
                        AcaciaDoor::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .flat_map(|state| {
                std::iter::repeat(state).zip([AcaciaDoor::Left, AcaciaDoor::Right].iter().copied())
            })
            .map(|((((half, powered), open), facing), hinge)| AcaciaDoor {
                half,
                powered,
                open,
                facing,
                hinge,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.half as u16 * 32u16
            + self.powered as u16 * 16u16
            + self.open as u16 * 8u16
            + self.facing as u16 * 2u16
            + self.hinge as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let half = offset / 32u16;
        offset -= half * 32u16;
        let half = AcaciaDoor::try_from(half)?;
        let powered = offset / 16u16;
        offset -= powered * 16u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let open = offset / 8u16;
        offset -= open * 8u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = AcaciaDoor::try_from(facing)?;
        let hinge = offset / 1u16;
        offset -= hinge * 1u16;
        let hinge = AcaciaDoor::try_from(hinge)?;
        Self {
            half,
            powered,
            open,
            facing,
            hinge,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<acacia_door, u16>> = Lazy::new(|| {
            acacia_door::possible_states()
                .into_iter()
                .zip(
                    [
                        7870u16, 7871u16, 7872u16, 7873u16, 7874u16, 7875u16, 7876u16, 7877u16,
                        7878u16, 7879u16, 7880u16, 7881u16, 7882u16, 7883u16, 7884u16, 7885u16,
                        7886u16, 7887u16, 7888u16, 7889u16, 7890u16, 7891u16, 7892u16, 7893u16,
                        7894u16, 7895u16, 7896u16, 7897u16, 7898u16, 7899u16, 7900u16, 7901u16,
                        7902u16, 7903u16, 7904u16, 7905u16, 7906u16, 7907u16, 7908u16, 7909u16,
                        7910u16, 7911u16, 7912u16, 7913u16, 7914u16, 7915u16, 7916u16, 7917u16,
                        7918u16, 7919u16, 7920u16, 7921u16, 7922u16, 7923u16, 7924u16, 7925u16,
                        7926u16, 7927u16, 7928u16, 7929u16, 7930u16, 7931u16, 7932u16, 7933u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DarkOakDoor {
    facing: DarkOakDoor,
    half: DarkOakDoor,
    open: bool,
    powered: bool,
    hinge: DarkOakDoor,
}
impl DarkOakDoor {
    pub fn possible_states() -> Vec<Self> {
        [
            DarkOakDoor::North,
            DarkOakDoor::South,
            DarkOakDoor::West,
            DarkOakDoor::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| {
            std::iter::repeat(state).zip([DarkOakDoor::Upper, DarkOakDoor::Lower].iter().copied())
        })
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .flat_map(|state| {
            std::iter::repeat(state).zip([DarkOakDoor::Left, DarkOakDoor::Right].iter().copied())
        })
        .map(|((((facing, half), open), powered), hinge)| DarkOakDoor {
            facing,
            half,
            open,
            powered,
            hinge,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 16u16
            + self.half as u16 * 8u16
            + self.open as u16 * 4u16
            + self.powered as u16 * 2u16
            + self.hinge as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 16u16;
        offset -= facing * 16u16;
        let facing = DarkOakDoor::try_from(facing)?;
        let half = offset / 8u16;
        offset -= half * 8u16;
        let half = DarkOakDoor::try_from(half)?;
        let open = offset / 4u16;
        offset -= open * 4u16;
        let open = Ok(if open == 0 { false } else { true })?;
        let powered = offset / 2u16;
        offset -= powered * 2u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        let hinge = offset / 1u16;
        offset -= hinge * 1u16;
        let hinge = DarkOakDoor::try_from(hinge)?;
        Self {
            facing,
            half,
            open,
            powered,
            hinge,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dark_oak_door, u16>> = Lazy::new(|| {
            dark_oak_door::possible_states()
                .into_iter()
                .zip(
                    [
                        7934u16, 7935u16, 7936u16, 7937u16, 7938u16, 7939u16, 7940u16, 7941u16,
                        7942u16, 7943u16, 7944u16, 7945u16, 7946u16, 7947u16, 7948u16, 7949u16,
                        7950u16, 7951u16, 7952u16, 7953u16, 7954u16, 7955u16, 7956u16, 7957u16,
                        7958u16, 7959u16, 7960u16, 7961u16, 7962u16, 7963u16, 7964u16, 7965u16,
                        7966u16, 7967u16, 7968u16, 7969u16, 7970u16, 7971u16, 7972u16, 7973u16,
                        7974u16, 7975u16, 7976u16, 7977u16, 7978u16, 7979u16, 7980u16, 7981u16,
                        7982u16, 7983u16, 7984u16, 7985u16, 7986u16, 7987u16, 7988u16, 7989u16,
                        7990u16, 7991u16, 7992u16, 7993u16, 7994u16, 7995u16, 7996u16, 7997u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndRod {
    facing: EndRod,
}
impl EndRod {
    pub fn possible_states() -> Vec<Self> {
        [
            EndRod::North,
            EndRod::East,
            EndRod::South,
            EndRod::West,
            EndRod::Up,
            EndRod::Down,
        ]
        .iter()
        .copied()
        .map(|facing| EndRod { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = EndRod::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<end_rod, u16>> = Lazy::new(|| {
            end_rod::possible_states()
                .into_iter()
                .zip(
                    [7998u16, 7999u16, 8000u16, 8001u16, 8002u16, 8003u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChorusPlant {
    up: bool,
    east: bool,
    south: bool,
    west: bool,
    down: bool,
    north: bool,
}
impl ChorusPlant {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
            .map(|(((((up, east), south), west), down), north)| ChorusPlant {
                up,
                east,
                south,
                west,
                down,
                north,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.up as u16 * 32u16
            + self.east as u16 * 16u16
            + self.south as u16 * 8u16
            + self.west as u16 * 4u16
            + self.down as u16 * 2u16
            + self.north as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let up = offset / 32u16;
        offset -= up * 32u16;
        let up = Ok(if up == 0 { false } else { true })?;
        let east = offset / 16u16;
        offset -= east * 16u16;
        let east = Ok(if east == 0 { false } else { true })?;
        let south = offset / 8u16;
        offset -= south * 8u16;
        let south = Ok(if south == 0 { false } else { true })?;
        let west = offset / 4u16;
        offset -= west * 4u16;
        let west = Ok(if west == 0 { false } else { true })?;
        let down = offset / 2u16;
        offset -= down * 2u16;
        let down = Ok(if down == 0 { false } else { true })?;
        let north = offset / 1u16;
        offset -= north * 1u16;
        let north = Ok(if north == 0 { false } else { true })?;
        Self {
            up,
            east,
            south,
            west,
            down,
            north,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<chorus_plant, u16>> = Lazy::new(|| {
            chorus_plant::possible_states()
                .into_iter()
                .zip(
                    [
                        8004u16, 8005u16, 8006u16, 8007u16, 8008u16, 8009u16, 8010u16, 8011u16,
                        8012u16, 8013u16, 8014u16, 8015u16, 8016u16, 8017u16, 8018u16, 8019u16,
                        8020u16, 8021u16, 8022u16, 8023u16, 8024u16, 8025u16, 8026u16, 8027u16,
                        8028u16, 8029u16, 8030u16, 8031u16, 8032u16, 8033u16, 8034u16, 8035u16,
                        8036u16, 8037u16, 8038u16, 8039u16, 8040u16, 8041u16, 8042u16, 8043u16,
                        8044u16, 8045u16, 8046u16, 8047u16, 8048u16, 8049u16, 8050u16, 8051u16,
                        8052u16, 8053u16, 8054u16, 8055u16, 8056u16, 8057u16, 8058u16, 8059u16,
                        8060u16, 8061u16, 8062u16, 8063u16, 8064u16, 8065u16, 8066u16, 8067u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChorusFlower {
    age: i32,
}
impl ChorusFlower {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=5i32 }.map(|age| ChorusFlower { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<chorus_flower, u16>> = Lazy::new(|| {
            chorus_flower::possible_states()
                .into_iter()
                .zip(
                    [8068u16, 8069u16, 8070u16, 8071u16, 8072u16, 8073u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpurBlock {}
impl PurpurBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpurBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purpur_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpurPillar {
    axis: PurpurPillar,
}
impl PurpurPillar {
    pub fn possible_states() -> Vec<Self> {
        [PurpurPillar::X, PurpurPillar::Y, PurpurPillar::Z]
            .iter()
            .copied()
            .map(|axis| PurpurPillar { axis })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.axis as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let axis = offset / 1u16;
        offset -= axis * 1u16;
        let axis = PurpurPillar::try_from(axis)?;
        Self { axis }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<purpur_pillar, u16>> = Lazy::new(|| {
            purpur_pillar::possible_states()
                .into_iter()
                .zip([8075u16, 8076u16, 8077u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndStoneBricks {}
impl EndStoneBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![EndStoneBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(end_stone_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Beetroots {
    age: i32,
}
impl Beetroots {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=3i32 }.map(|age| Beetroots { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<beetroots, u16>> = Lazy::new(|| {
            beetroots::possible_states()
                .into_iter()
                .zip([8159u16, 8160u16, 8161u16, 8162u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrassPath {}
impl GrassPath {
    pub fn possible_states() -> Vec<Self> {
        vec![GrassPath {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(grass_path {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndGateway {}
impl EndGateway {
    pub fn possible_states() -> Vec<Self> {
        vec![EndGateway {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(end_gateway {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RepeatingCommandBlock {
    conditional: bool,
    facing: RepeatingCommandBlock,
}
impl RepeatingCommandBlock {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        RepeatingCommandBlock::North,
                        RepeatingCommandBlock::East,
                        RepeatingCommandBlock::South,
                        RepeatingCommandBlock::West,
                        RepeatingCommandBlock::Up,
                        RepeatingCommandBlock::Down,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(conditional, facing)| RepeatingCommandBlock {
                conditional,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.conditional as u16 * 6u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let conditional = offset / 6u16;
        offset -= conditional * 6u16;
        let conditional = Ok(if conditional == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = RepeatingCommandBlock::try_from(facing)?;
        Self {
            conditional,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<repeating_command_block, u16>> = Lazy::new(|| {
            repeating_command_block::possible_states()
                .into_iter()
                .zip(
                    [
                        8165u16, 8166u16, 8167u16, 8168u16, 8169u16, 8170u16, 8171u16, 8172u16,
                        8173u16, 8174u16, 8175u16, 8176u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChainCommandBlock {
    conditional: bool,
    facing: ChainCommandBlock,
}
impl ChainCommandBlock {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        ChainCommandBlock::North,
                        ChainCommandBlock::East,
                        ChainCommandBlock::South,
                        ChainCommandBlock::West,
                        ChainCommandBlock::Up,
                        ChainCommandBlock::Down,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(conditional, facing)| ChainCommandBlock {
                conditional,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.conditional as u16 * 6u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let conditional = offset / 6u16;
        offset -= conditional * 6u16;
        let conditional = Ok(if conditional == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = ChainCommandBlock::try_from(facing)?;
        Self {
            conditional,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<chain_command_block, u16>> = Lazy::new(|| {
            chain_command_block::possible_states()
                .into_iter()
                .zip(
                    [
                        8177u16, 8178u16, 8179u16, 8180u16, 8181u16, 8182u16, 8183u16, 8184u16,
                        8185u16, 8186u16, 8187u16, 8188u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FrostedIce {
    age: i32,
}
impl FrostedIce {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=3i32 }.map(|age| FrostedIce { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<frosted_ice, u16>> = Lazy::new(|| {
            frosted_ice::possible_states()
                .into_iter()
                .zip([8189u16, 8190u16, 8191u16, 8192u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagmaBlock {}
impl MagmaBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![MagmaBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magma_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetherWartBlock {}
impl NetherWartBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![NetherWartBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(nether_wart_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedNetherBricks {}
impl RedNetherBricks {
    pub fn possible_states() -> Vec<Self> {
        vec![RedNetherBricks {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_nether_bricks {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoneBlock {
    axis: BoneBlock,
}
impl BoneBlock {
    pub fn possible_states() -> Vec<Self> {
        [BoneBlock::X, BoneBlock::Y, BoneBlock::Z]
            .iter()
            .copied()
            .map(|axis| BoneBlock { axis })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.axis as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let axis = offset / 1u16;
        offset -= axis * 1u16;
        let axis = BoneBlock::try_from(axis)?;
        Self { axis }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<bone_block, u16>> = Lazy::new(|| {
            bone_block::possible_states()
                .into_iter()
                .zip([8196u16, 8197u16, 8198u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructureVoid {}
impl StructureVoid {
    pub fn possible_states() -> Vec<Self> {
        vec![StructureVoid {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(structure_void {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Observer {
    facing: Observer,
    powered: bool,
}
impl Observer {
    pub fn possible_states() -> Vec<Self> {
        [
            Observer::North,
            Observer::East,
            Observer::South,
            Observer::West,
            Observer::Up,
            Observer::Down,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, powered)| Observer { facing, powered })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.powered as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = Observer::try_from(facing)?;
        let powered = offset / 1u16;
        offset -= powered * 1u16;
        let powered = Ok(if powered == 0 { false } else { true })?;
        Self { facing, powered }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<observer, u16>> = Lazy::new(|| {
            observer::possible_states()
                .into_iter()
                .zip(
                    [
                        8200u16, 8201u16, 8202u16, 8203u16, 8204u16, 8205u16, 8206u16, 8207u16,
                        8208u16, 8209u16, 8210u16, 8211u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShulkerBox {
    facing: ShulkerBox,
}
impl ShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            ShulkerBox::North,
            ShulkerBox::East,
            ShulkerBox::South,
            ShulkerBox::West,
            ShulkerBox::Up,
            ShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| ShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = ShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<shulker_box, u16>> = Lazy::new(|| {
            shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8212u16, 8213u16, 8214u16, 8215u16, 8216u16, 8217u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteShulkerBox {
    facing: WhiteShulkerBox,
}
impl WhiteShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            WhiteShulkerBox::North,
            WhiteShulkerBox::East,
            WhiteShulkerBox::South,
            WhiteShulkerBox::West,
            WhiteShulkerBox::Up,
            WhiteShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| WhiteShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = WhiteShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<white_shulker_box, u16>> = Lazy::new(|| {
            white_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8218u16, 8219u16, 8220u16, 8221u16, 8222u16, 8223u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeShulkerBox {
    facing: OrangeShulkerBox,
}
impl OrangeShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            OrangeShulkerBox::North,
            OrangeShulkerBox::East,
            OrangeShulkerBox::South,
            OrangeShulkerBox::West,
            OrangeShulkerBox::Up,
            OrangeShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| OrangeShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = OrangeShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<orange_shulker_box, u16>> = Lazy::new(|| {
            orange_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8224u16, 8225u16, 8226u16, 8227u16, 8228u16, 8229u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaShulkerBox {
    facing: MagentaShulkerBox,
}
impl MagentaShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            MagentaShulkerBox::North,
            MagentaShulkerBox::East,
            MagentaShulkerBox::South,
            MagentaShulkerBox::West,
            MagentaShulkerBox::Up,
            MagentaShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| MagentaShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = MagentaShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<magenta_shulker_box, u16>> = Lazy::new(|| {
            magenta_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8230u16, 8231u16, 8232u16, 8233u16, 8234u16, 8235u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueShulkerBox {
    facing: LightBlueShulkerBox,
}
impl LightBlueShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            LightBlueShulkerBox::North,
            LightBlueShulkerBox::East,
            LightBlueShulkerBox::South,
            LightBlueShulkerBox::West,
            LightBlueShulkerBox::Up,
            LightBlueShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| LightBlueShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LightBlueShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_blue_shulker_box, u16>> = Lazy::new(|| {
            light_blue_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8236u16, 8237u16, 8238u16, 8239u16, 8240u16, 8241u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowShulkerBox {
    facing: YellowShulkerBox,
}
impl YellowShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            YellowShulkerBox::North,
            YellowShulkerBox::East,
            YellowShulkerBox::South,
            YellowShulkerBox::West,
            YellowShulkerBox::Up,
            YellowShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| YellowShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = YellowShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<yellow_shulker_box, u16>> = Lazy::new(|| {
            yellow_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8242u16, 8243u16, 8244u16, 8245u16, 8246u16, 8247u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeShulkerBox {
    facing: LimeShulkerBox,
}
impl LimeShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            LimeShulkerBox::North,
            LimeShulkerBox::East,
            LimeShulkerBox::South,
            LimeShulkerBox::West,
            LimeShulkerBox::Up,
            LimeShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| LimeShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LimeShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lime_shulker_box, u16>> = Lazy::new(|| {
            lime_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8248u16, 8249u16, 8250u16, 8251u16, 8252u16, 8253u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkShulkerBox {
    facing: PinkShulkerBox,
}
impl PinkShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            PinkShulkerBox::North,
            PinkShulkerBox::East,
            PinkShulkerBox::South,
            PinkShulkerBox::West,
            PinkShulkerBox::Up,
            PinkShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| PinkShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PinkShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<pink_shulker_box, u16>> = Lazy::new(|| {
            pink_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8254u16, 8255u16, 8256u16, 8257u16, 8258u16, 8259u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayShulkerBox {
    facing: GrayShulkerBox,
}
impl GrayShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            GrayShulkerBox::North,
            GrayShulkerBox::East,
            GrayShulkerBox::South,
            GrayShulkerBox::West,
            GrayShulkerBox::Up,
            GrayShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| GrayShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = GrayShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<gray_shulker_box, u16>> = Lazy::new(|| {
            gray_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8260u16, 8261u16, 8262u16, 8263u16, 8264u16, 8265u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayShulkerBox {
    facing: LightGrayShulkerBox,
}
impl LightGrayShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            LightGrayShulkerBox::North,
            LightGrayShulkerBox::East,
            LightGrayShulkerBox::South,
            LightGrayShulkerBox::West,
            LightGrayShulkerBox::Up,
            LightGrayShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| LightGrayShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LightGrayShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_gray_shulker_box, u16>> = Lazy::new(|| {
            light_gray_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8266u16, 8267u16, 8268u16, 8269u16, 8270u16, 8271u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanShulkerBox {
    facing: CyanShulkerBox,
}
impl CyanShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            CyanShulkerBox::North,
            CyanShulkerBox::East,
            CyanShulkerBox::South,
            CyanShulkerBox::West,
            CyanShulkerBox::Up,
            CyanShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| CyanShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = CyanShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cyan_shulker_box, u16>> = Lazy::new(|| {
            cyan_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8272u16, 8273u16, 8274u16, 8275u16, 8276u16, 8277u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleShulkerBox {
    facing: PurpleShulkerBox,
}
impl PurpleShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            PurpleShulkerBox::North,
            PurpleShulkerBox::East,
            PurpleShulkerBox::South,
            PurpleShulkerBox::West,
            PurpleShulkerBox::Up,
            PurpleShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| PurpleShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PurpleShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<purple_shulker_box, u16>> = Lazy::new(|| {
            purple_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8278u16, 8279u16, 8280u16, 8281u16, 8282u16, 8283u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueShulkerBox {
    facing: BlueShulkerBox,
}
impl BlueShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            BlueShulkerBox::North,
            BlueShulkerBox::East,
            BlueShulkerBox::South,
            BlueShulkerBox::West,
            BlueShulkerBox::Up,
            BlueShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| BlueShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BlueShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<blue_shulker_box, u16>> = Lazy::new(|| {
            blue_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8284u16, 8285u16, 8286u16, 8287u16, 8288u16, 8289u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownShulkerBox {
    facing: BrownShulkerBox,
}
impl BrownShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            BrownShulkerBox::North,
            BrownShulkerBox::East,
            BrownShulkerBox::South,
            BrownShulkerBox::West,
            BrownShulkerBox::Up,
            BrownShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| BrownShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BrownShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brown_shulker_box, u16>> = Lazy::new(|| {
            brown_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8290u16, 8291u16, 8292u16, 8293u16, 8294u16, 8295u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenShulkerBox {
    facing: GreenShulkerBox,
}
impl GreenShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            GreenShulkerBox::North,
            GreenShulkerBox::East,
            GreenShulkerBox::South,
            GreenShulkerBox::West,
            GreenShulkerBox::Up,
            GreenShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| GreenShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = GreenShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<green_shulker_box, u16>> = Lazy::new(|| {
            green_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8296u16, 8297u16, 8298u16, 8299u16, 8300u16, 8301u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedShulkerBox {
    facing: RedShulkerBox,
}
impl RedShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            RedShulkerBox::North,
            RedShulkerBox::East,
            RedShulkerBox::South,
            RedShulkerBox::West,
            RedShulkerBox::Up,
            RedShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| RedShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = RedShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<red_shulker_box, u16>> = Lazy::new(|| {
            red_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8302u16, 8303u16, 8304u16, 8305u16, 8306u16, 8307u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackShulkerBox {
    facing: BlackShulkerBox,
}
impl BlackShulkerBox {
    pub fn possible_states() -> Vec<Self> {
        [
            BlackShulkerBox::North,
            BlackShulkerBox::East,
            BlackShulkerBox::South,
            BlackShulkerBox::West,
            BlackShulkerBox::Up,
            BlackShulkerBox::Down,
        ]
        .iter()
        .copied()
        .map(|facing| BlackShulkerBox { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BlackShulkerBox::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<black_shulker_box, u16>> = Lazy::new(|| {
            black_shulker_box::possible_states()
                .into_iter()
                .zip(
                    [8308u16, 8309u16, 8310u16, 8311u16, 8312u16, 8313u16]
                        .iter()
                        .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteGlazedTerracotta {
    facing: WhiteGlazedTerracotta,
}
impl WhiteGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            WhiteGlazedTerracotta::North,
            WhiteGlazedTerracotta::South,
            WhiteGlazedTerracotta::West,
            WhiteGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| WhiteGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = WhiteGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<white_glazed_terracotta, u16>> = Lazy::new(|| {
            white_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8314u16, 8315u16, 8316u16, 8317u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeGlazedTerracotta {
    facing: OrangeGlazedTerracotta,
}
impl OrangeGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            OrangeGlazedTerracotta::North,
            OrangeGlazedTerracotta::South,
            OrangeGlazedTerracotta::West,
            OrangeGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| OrangeGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = OrangeGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<orange_glazed_terracotta, u16>> = Lazy::new(|| {
            orange_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8318u16, 8319u16, 8320u16, 8321u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaGlazedTerracotta {
    facing: MagentaGlazedTerracotta,
}
impl MagentaGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            MagentaGlazedTerracotta::North,
            MagentaGlazedTerracotta::South,
            MagentaGlazedTerracotta::West,
            MagentaGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| MagentaGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = MagentaGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<magenta_glazed_terracotta, u16>> = Lazy::new(|| {
            magenta_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8322u16, 8323u16, 8324u16, 8325u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueGlazedTerracotta {
    facing: LightBlueGlazedTerracotta,
}
impl LightBlueGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            LightBlueGlazedTerracotta::North,
            LightBlueGlazedTerracotta::South,
            LightBlueGlazedTerracotta::West,
            LightBlueGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| LightBlueGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LightBlueGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_blue_glazed_terracotta, u16>> = Lazy::new(|| {
            light_blue_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8326u16, 8327u16, 8328u16, 8329u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowGlazedTerracotta {
    facing: YellowGlazedTerracotta,
}
impl YellowGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            YellowGlazedTerracotta::North,
            YellowGlazedTerracotta::South,
            YellowGlazedTerracotta::West,
            YellowGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| YellowGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = YellowGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<yellow_glazed_terracotta, u16>> = Lazy::new(|| {
            yellow_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8330u16, 8331u16, 8332u16, 8333u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeGlazedTerracotta {
    facing: LimeGlazedTerracotta,
}
impl LimeGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            LimeGlazedTerracotta::North,
            LimeGlazedTerracotta::South,
            LimeGlazedTerracotta::West,
            LimeGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| LimeGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LimeGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<lime_glazed_terracotta, u16>> = Lazy::new(|| {
            lime_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8334u16, 8335u16, 8336u16, 8337u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkGlazedTerracotta {
    facing: PinkGlazedTerracotta,
}
impl PinkGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            PinkGlazedTerracotta::North,
            PinkGlazedTerracotta::South,
            PinkGlazedTerracotta::West,
            PinkGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| PinkGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PinkGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<pink_glazed_terracotta, u16>> = Lazy::new(|| {
            pink_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8338u16, 8339u16, 8340u16, 8341u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayGlazedTerracotta {
    facing: GrayGlazedTerracotta,
}
impl GrayGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            GrayGlazedTerracotta::North,
            GrayGlazedTerracotta::South,
            GrayGlazedTerracotta::West,
            GrayGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| GrayGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = GrayGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<gray_glazed_terracotta, u16>> = Lazy::new(|| {
            gray_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8342u16, 8343u16, 8344u16, 8345u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayGlazedTerracotta {
    facing: LightGrayGlazedTerracotta,
}
impl LightGrayGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            LightGrayGlazedTerracotta::North,
            LightGrayGlazedTerracotta::South,
            LightGrayGlazedTerracotta::West,
            LightGrayGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| LightGrayGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = LightGrayGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<light_gray_glazed_terracotta, u16>> = Lazy::new(|| {
            light_gray_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8346u16, 8347u16, 8348u16, 8349u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanGlazedTerracotta {
    facing: CyanGlazedTerracotta,
}
impl CyanGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            CyanGlazedTerracotta::North,
            CyanGlazedTerracotta::South,
            CyanGlazedTerracotta::West,
            CyanGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| CyanGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = CyanGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<cyan_glazed_terracotta, u16>> = Lazy::new(|| {
            cyan_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8350u16, 8351u16, 8352u16, 8353u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleGlazedTerracotta {
    facing: PurpleGlazedTerracotta,
}
impl PurpleGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            PurpleGlazedTerracotta::North,
            PurpleGlazedTerracotta::South,
            PurpleGlazedTerracotta::West,
            PurpleGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| PurpleGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = PurpleGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<purple_glazed_terracotta, u16>> = Lazy::new(|| {
            purple_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8354u16, 8355u16, 8356u16, 8357u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueGlazedTerracotta {
    facing: BlueGlazedTerracotta,
}
impl BlueGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            BlueGlazedTerracotta::North,
            BlueGlazedTerracotta::South,
            BlueGlazedTerracotta::West,
            BlueGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| BlueGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BlueGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<blue_glazed_terracotta, u16>> = Lazy::new(|| {
            blue_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8358u16, 8359u16, 8360u16, 8361u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownGlazedTerracotta {
    facing: BrownGlazedTerracotta,
}
impl BrownGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            BrownGlazedTerracotta::North,
            BrownGlazedTerracotta::South,
            BrownGlazedTerracotta::West,
            BrownGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| BrownGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BrownGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brown_glazed_terracotta, u16>> = Lazy::new(|| {
            brown_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8362u16, 8363u16, 8364u16, 8365u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenGlazedTerracotta {
    facing: GreenGlazedTerracotta,
}
impl GreenGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            GreenGlazedTerracotta::North,
            GreenGlazedTerracotta::South,
            GreenGlazedTerracotta::West,
            GreenGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| GreenGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = GreenGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<green_glazed_terracotta, u16>> = Lazy::new(|| {
            green_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8366u16, 8367u16, 8368u16, 8369u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedGlazedTerracotta {
    facing: RedGlazedTerracotta,
}
impl RedGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            RedGlazedTerracotta::North,
            RedGlazedTerracotta::South,
            RedGlazedTerracotta::West,
            RedGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| RedGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = RedGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<red_glazed_terracotta, u16>> = Lazy::new(|| {
            red_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8370u16, 8371u16, 8372u16, 8373u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackGlazedTerracotta {
    facing: BlackGlazedTerracotta,
}
impl BlackGlazedTerracotta {
    pub fn possible_states() -> Vec<Self> {
        [
            BlackGlazedTerracotta::North,
            BlackGlazedTerracotta::South,
            BlackGlazedTerracotta::West,
            BlackGlazedTerracotta::East,
        ]
        .iter()
        .copied()
        .map(|facing| BlackGlazedTerracotta { facing })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BlackGlazedTerracotta::try_from(facing)?;
        Self { facing }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<black_glazed_terracotta, u16>> = Lazy::new(|| {
            black_glazed_terracotta::possible_states()
                .into_iter()
                .zip([8374u16, 8375u16, 8376u16, 8377u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteConcrete {}
impl WhiteConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeConcrete {}
impl OrangeConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaConcrete {}
impl MagentaConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![MagentaConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magenta_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueConcrete {}
impl LightBlueConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![LightBlueConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_blue_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowConcrete {}
impl YellowConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![YellowConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(yellow_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeConcrete {}
impl LimeConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![LimeConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lime_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkConcrete {}
impl PinkConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayConcrete {}
impl GrayConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![GrayConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gray_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayConcrete {}
impl LightGrayConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![LightGrayConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_gray_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanConcrete {}
impl CyanConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![CyanConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cyan_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleConcrete {}
impl PurpleConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpleConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purple_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueConcrete {}
impl BlueConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownConcrete {}
impl BrownConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenConcrete {}
impl GreenConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![GreenConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(green_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedConcrete {}
impl RedConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![RedConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackConcrete {}
impl BlackConcrete {
    pub fn possible_states() -> Vec<Self> {
        vec![BlackConcrete {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(black_concrete {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhiteConcretePowder {}
impl WhiteConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![WhiteConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(white_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrangeConcretePowder {}
impl OrangeConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![OrangeConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(orange_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagentaConcretePowder {}
impl MagentaConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![MagentaConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(magenta_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightBlueConcretePowder {}
impl LightBlueConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![LightBlueConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_blue_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YellowConcretePowder {}
impl YellowConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![YellowConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(yellow_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimeConcretePowder {}
impl LimeConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![LimeConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(lime_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PinkConcretePowder {}
impl PinkConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![PinkConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(pink_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrayConcretePowder {}
impl GrayConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![GrayConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(gray_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LightGrayConcretePowder {}
impl LightGrayConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![LightGrayConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(light_gray_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CyanConcretePowder {}
impl CyanConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![CyanConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cyan_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PurpleConcretePowder {}
impl PurpleConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![PurpleConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(purple_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueConcretePowder {}
impl BlueConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrownConcretePowder {}
impl BrownConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![BrownConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brown_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenConcretePowder {}
impl GreenConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![GreenConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(green_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RedConcretePowder {}
impl RedConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![RedConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(red_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlackConcretePowder {}
impl BlackConcretePowder {
    pub fn possible_states() -> Vec<Self> {
        vec![BlackConcretePowder {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(black_concrete_powder {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Kelp {
    age: i32,
}
impl Kelp {
    pub fn possible_states() -> Vec<Self> {
        { 0i32..=25i32 }.map(|age| Kelp { age }).collect()
    }
    pub fn id_offset(self) -> u16 {
        self.age as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let age = offset / 1u16;
        offset -= age * 1u16;
        let age = Ok(age as i32)?;
        Self { age }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<kelp, u16>> = Lazy::new(|| {
            kelp::possible_states()
                .into_iter()
                .zip(
                    [
                        8410u16, 8411u16, 8412u16, 8413u16, 8414u16, 8415u16, 8416u16, 8417u16,
                        8418u16, 8419u16, 8420u16, 8421u16, 8422u16, 8423u16, 8424u16, 8425u16,
                        8426u16, 8427u16, 8428u16, 8429u16, 8430u16, 8431u16, 8432u16, 8433u16,
                        8434u16, 8435u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KelpPlant {}
impl KelpPlant {
    pub fn possible_states() -> Vec<Self> {
        vec![KelpPlant {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(kelp_plant {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DriedKelpBlock {}
impl DriedKelpBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DriedKelpBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dried_kelp_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TurtleEgg {
    eggs: i32,
    hatch: i32,
}
impl TurtleEgg {
    pub fn possible_states() -> Vec<Self> {
        { 1i32..=4i32 }
            .flat_map(|state| std::iter::repeat(state).zip({ 0i32..=2i32 }))
            .map(|(eggs, hatch)| TurtleEgg { eggs, hatch })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.eggs as u16 * 3u16 + self.hatch as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let eggs = offset / 3u16;
        offset -= eggs * 3u16;
        let eggs = Ok(eggs as i32)?;
        let hatch = offset / 1u16;
        offset -= hatch * 1u16;
        let hatch = Ok(hatch as i32)?;
        Self { eggs, hatch }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<turtle_egg, u16>> = Lazy::new(|| {
            turtle_egg::possible_states()
                .into_iter()
                .zip(
                    [
                        8438u16, 8439u16, 8440u16, 8441u16, 8442u16, 8443u16, 8444u16, 8445u16,
                        8446u16, 8447u16, 8448u16, 8449u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadTubeCoralBlock {}
impl DeadTubeCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DeadTubeCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dead_tube_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBrainCoralBlock {}
impl DeadBrainCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DeadBrainCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dead_brain_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoralBlock {}
impl DeadBubbleCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DeadBubbleCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dead_bubble_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadFireCoralBlock {}
impl DeadFireCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DeadFireCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dead_fire_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadHornCoralBlock {}
impl DeadHornCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![DeadHornCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(dead_horn_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TubeCoralBlock {}
impl TubeCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![TubeCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(tube_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrainCoralBlock {}
impl BrainCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![BrainCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(brain_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BubbleCoralBlock {}
impl BubbleCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![BubbleCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(bubble_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FireCoralBlock {}
impl FireCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![FireCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(fire_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HornCoralBlock {}
impl HornCoralBlock {
    pub fn possible_states() -> Vec<Self> {
        vec![HornCoralBlock {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(horn_coral_block {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadTubeCoral {
    waterlogged: bool,
}
impl DeadTubeCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadTubeCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_tube_coral, u16>> = Lazy::new(|| {
            dead_tube_coral::possible_states()
                .into_iter()
                .zip([8460u16, 8461u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBrainCoral {
    waterlogged: bool,
}
impl DeadBrainCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadBrainCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_brain_coral, u16>> = Lazy::new(|| {
            dead_brain_coral::possible_states()
                .into_iter()
                .zip([8462u16, 8463u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoral {
    waterlogged: bool,
}
impl DeadBubbleCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadBubbleCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_bubble_coral, u16>> = Lazy::new(|| {
            dead_bubble_coral::possible_states()
                .into_iter()
                .zip([8464u16, 8465u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadFireCoral {
    waterlogged: bool,
}
impl DeadFireCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadFireCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_fire_coral, u16>> = Lazy::new(|| {
            dead_fire_coral::possible_states()
                .into_iter()
                .zip([8466u16, 8467u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadHornCoral {
    waterlogged: bool,
}
impl DeadHornCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadHornCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_horn_coral, u16>> = Lazy::new(|| {
            dead_horn_coral::possible_states()
                .into_iter()
                .zip([8468u16, 8469u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TubeCoral {
    waterlogged: bool,
}
impl TubeCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| TubeCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tube_coral, u16>> = Lazy::new(|| {
            tube_coral::possible_states()
                .into_iter()
                .zip([8470u16, 8471u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrainCoral {
    waterlogged: bool,
}
impl BrainCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| BrainCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brain_coral, u16>> = Lazy::new(|| {
            brain_coral::possible_states()
                .into_iter()
                .zip([8472u16, 8473u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BubbleCoral {
    waterlogged: bool,
}
impl BubbleCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| BubbleCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<bubble_coral, u16>> = Lazy::new(|| {
            bubble_coral::possible_states()
                .into_iter()
                .zip([8474u16, 8475u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FireCoral {
    waterlogged: bool,
}
impl FireCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| FireCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<fire_coral, u16>> = Lazy::new(|| {
            fire_coral::possible_states()
                .into_iter()
                .zip([8476u16, 8477u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HornCoral {
    waterlogged: bool,
}
impl HornCoral {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| HornCoral { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<horn_coral, u16>> = Lazy::new(|| {
            horn_coral::possible_states()
                .into_iter()
                .zip([8478u16, 8479u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadTubeCoralWallFan {
    waterlogged: bool,
    facing: DeadTubeCoralWallFan,
}
impl DeadTubeCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        DeadTubeCoralWallFan::North,
                        DeadTubeCoralWallFan::South,
                        DeadTubeCoralWallFan::West,
                        DeadTubeCoralWallFan::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(waterlogged, facing)| DeadTubeCoralWallFan {
                waterlogged,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = DeadTubeCoralWallFan::try_from(facing)?;
        Self {
            waterlogged,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_tube_coral_wall_fan, u16>> = Lazy::new(|| {
            dead_tube_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8480u16, 8481u16, 8482u16, 8483u16, 8484u16, 8485u16, 8486u16, 8487u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBrainCoralWallFan {
    waterlogged: bool,
    facing: DeadBrainCoralWallFan,
}
impl DeadBrainCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        DeadBrainCoralWallFan::North,
                        DeadBrainCoralWallFan::South,
                        DeadBrainCoralWallFan::West,
                        DeadBrainCoralWallFan::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(waterlogged, facing)| DeadBrainCoralWallFan {
                waterlogged,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = DeadBrainCoralWallFan::try_from(facing)?;
        Self {
            waterlogged,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_brain_coral_wall_fan, u16>> = Lazy::new(|| {
            dead_brain_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8488u16, 8489u16, 8490u16, 8491u16, 8492u16, 8493u16, 8494u16, 8495u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoralWallFan {
    facing: DeadBubbleCoralWallFan,
    waterlogged: bool,
}
impl DeadBubbleCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [
            DeadBubbleCoralWallFan::North,
            DeadBubbleCoralWallFan::South,
            DeadBubbleCoralWallFan::West,
            DeadBubbleCoralWallFan::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| DeadBubbleCoralWallFan {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = DeadBubbleCoralWallFan::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_bubble_coral_wall_fan, u16>> = Lazy::new(|| {
            dead_bubble_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8496u16, 8497u16, 8498u16, 8499u16, 8500u16, 8501u16, 8502u16, 8503u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadFireCoralWallFan {
    waterlogged: bool,
    facing: DeadFireCoralWallFan,
}
impl DeadFireCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        DeadFireCoralWallFan::North,
                        DeadFireCoralWallFan::South,
                        DeadFireCoralWallFan::West,
                        DeadFireCoralWallFan::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(waterlogged, facing)| DeadFireCoralWallFan {
                waterlogged,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = DeadFireCoralWallFan::try_from(facing)?;
        Self {
            waterlogged,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_fire_coral_wall_fan, u16>> = Lazy::new(|| {
            dead_fire_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8504u16, 8505u16, 8506u16, 8507u16, 8508u16, 8509u16, 8510u16, 8511u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadHornCoralWallFan {
    facing: DeadHornCoralWallFan,
    waterlogged: bool,
}
impl DeadHornCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [
            DeadHornCoralWallFan::North,
            DeadHornCoralWallFan::South,
            DeadHornCoralWallFan::West,
            DeadHornCoralWallFan::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| DeadHornCoralWallFan {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = DeadHornCoralWallFan::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_horn_coral_wall_fan, u16>> = Lazy::new(|| {
            dead_horn_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8512u16, 8513u16, 8514u16, 8515u16, 8516u16, 8517u16, 8518u16, 8519u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TubeCoralWallFan {
    facing: TubeCoralWallFan,
    waterlogged: bool,
}
impl TubeCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [
            TubeCoralWallFan::North,
            TubeCoralWallFan::South,
            TubeCoralWallFan::West,
            TubeCoralWallFan::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| TubeCoralWallFan {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = TubeCoralWallFan::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tube_coral_wall_fan, u16>> = Lazy::new(|| {
            tube_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8520u16, 8521u16, 8522u16, 8523u16, 8524u16, 8525u16, 8526u16, 8527u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrainCoralWallFan {
    facing: BrainCoralWallFan,
    waterlogged: bool,
}
impl BrainCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [
            BrainCoralWallFan::North,
            BrainCoralWallFan::South,
            BrainCoralWallFan::West,
            BrainCoralWallFan::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| BrainCoralWallFan {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = BrainCoralWallFan::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brain_coral_wall_fan, u16>> = Lazy::new(|| {
            brain_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8528u16, 8529u16, 8530u16, 8531u16, 8532u16, 8533u16, 8534u16, 8535u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BubbleCoralWallFan {
    waterlogged: bool,
    facing: BubbleCoralWallFan,
}
impl BubbleCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        BubbleCoralWallFan::North,
                        BubbleCoralWallFan::South,
                        BubbleCoralWallFan::West,
                        BubbleCoralWallFan::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(waterlogged, facing)| BubbleCoralWallFan {
                waterlogged,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = BubbleCoralWallFan::try_from(facing)?;
        Self {
            waterlogged,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<bubble_coral_wall_fan, u16>> = Lazy::new(|| {
            bubble_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8536u16, 8537u16, 8538u16, 8539u16, 8540u16, 8541u16, 8542u16, 8543u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FireCoralWallFan {
    waterlogged: bool,
    facing: FireCoralWallFan,
}
impl FireCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| {
                std::iter::repeat(state).zip(
                    [
                        FireCoralWallFan::North,
                        FireCoralWallFan::South,
                        FireCoralWallFan::West,
                        FireCoralWallFan::East,
                    ]
                    .iter()
                    .copied(),
                )
            })
            .map(|(waterlogged, facing)| FireCoralWallFan {
                waterlogged,
                facing,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.facing as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let facing = offset / 1u16;
        offset -= facing * 1u16;
        let facing = FireCoralWallFan::try_from(facing)?;
        Self {
            waterlogged,
            facing,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<fire_coral_wall_fan, u16>> = Lazy::new(|| {
            fire_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8544u16, 8545u16, 8546u16, 8547u16, 8548u16, 8549u16, 8550u16, 8551u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HornCoralWallFan {
    facing: HornCoralWallFan,
    waterlogged: bool,
}
impl HornCoralWallFan {
    pub fn possible_states() -> Vec<Self> {
        [
            HornCoralWallFan::North,
            HornCoralWallFan::South,
            HornCoralWallFan::West,
            HornCoralWallFan::East,
        ]
        .iter()
        .copied()
        .flat_map(|state| std::iter::repeat(state).zip([false, true].iter().copied()))
        .map(|(facing, waterlogged)| HornCoralWallFan {
            facing,
            waterlogged,
        })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.facing as u16 * 2u16 + self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing = offset / 2u16;
        offset -= facing * 2u16;
        let facing = HornCoralWallFan::try_from(facing)?;
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self {
            facing,
            waterlogged,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<horn_coral_wall_fan, u16>> = Lazy::new(|| {
            horn_coral_wall_fan::possible_states()
                .into_iter()
                .zip(
                    [
                        8552u16, 8553u16, 8554u16, 8555u16, 8556u16, 8557u16, 8558u16, 8559u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadTubeCoralFan {
    waterlogged: bool,
}
impl DeadTubeCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadTubeCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_tube_coral_fan, u16>> = Lazy::new(|| {
            dead_tube_coral_fan::possible_states()
                .into_iter()
                .zip([8560u16, 8561u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBrainCoralFan {
    waterlogged: bool,
}
impl DeadBrainCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadBrainCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_brain_coral_fan, u16>> = Lazy::new(|| {
            dead_brain_coral_fan::possible_states()
                .into_iter()
                .zip([8562u16, 8563u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoralFan {
    waterlogged: bool,
}
impl DeadBubbleCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadBubbleCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_bubble_coral_fan, u16>> = Lazy::new(|| {
            dead_bubble_coral_fan::possible_states()
                .into_iter()
                .zip([8564u16, 8565u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadFireCoralFan {
    waterlogged: bool,
}
impl DeadFireCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadFireCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_fire_coral_fan, u16>> = Lazy::new(|| {
            dead_fire_coral_fan::possible_states()
                .into_iter()
                .zip([8566u16, 8567u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeadHornCoralFan {
    waterlogged: bool,
}
impl DeadHornCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| DeadHornCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<dead_horn_coral_fan, u16>> = Lazy::new(|| {
            dead_horn_coral_fan::possible_states()
                .into_iter()
                .zip([8568u16, 8569u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TubeCoralFan {
    waterlogged: bool,
}
impl TubeCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| TubeCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<tube_coral_fan, u16>> = Lazy::new(|| {
            tube_coral_fan::possible_states()
                .into_iter()
                .zip([8570u16, 8571u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrainCoralFan {
    waterlogged: bool,
}
impl BrainCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| BrainCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<brain_coral_fan, u16>> = Lazy::new(|| {
            brain_coral_fan::possible_states()
                .into_iter()
                .zip([8572u16, 8573u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BubbleCoralFan {
    waterlogged: bool,
}
impl BubbleCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| BubbleCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<bubble_coral_fan, u16>> = Lazy::new(|| {
            bubble_coral_fan::possible_states()
                .into_iter()
                .zip([8574u16, 8575u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FireCoralFan {
    waterlogged: bool,
}
impl FireCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| FireCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<fire_coral_fan, u16>> = Lazy::new(|| {
            fire_coral_fan::possible_states()
                .into_iter()
                .zip([8576u16, 8577u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HornCoralFan {
    waterlogged: bool,
}
impl HornCoralFan {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| HornCoralFan { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<horn_coral_fan, u16>> = Lazy::new(|| {
            horn_coral_fan::possible_states()
                .into_iter()
                .zip([8578u16, 8579u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SeaPickle {
    waterlogged: bool,
    pickles: i32,
}
impl SeaPickle {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .flat_map(|state| std::iter::repeat(state).zip({ 1i32..=4i32 }))
            .map(|(waterlogged, pickles)| SeaPickle {
                waterlogged,
                pickles,
            })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 4u16 + self.pickles as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 4u16;
        offset -= waterlogged * 4u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        let pickles = offset / 1u16;
        offset -= pickles * 1u16;
        let pickles = Ok(pickles as i32)?;
        Self {
            waterlogged,
            pickles,
        }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<sea_pickle, u16>> = Lazy::new(|| {
            sea_pickle::possible_states()
                .into_iter()
                .zip(
                    [
                        8580u16, 8581u16, 8582u16, 8583u16, 8584u16, 8585u16, 8586u16, 8587u16,
                    ]
                    .iter()
                    .copied(),
                )
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlueIce {}
impl BlueIce {
    pub fn possible_states() -> Vec<Self> {
        vec![BlueIce {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(blue_ice {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Conduit {
    waterlogged: bool,
}
impl Conduit {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|waterlogged| Conduit { waterlogged })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.waterlogged as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let waterlogged = offset / 1u16;
        offset -= waterlogged * 1u16;
        let waterlogged = Ok(if waterlogged == 0 { false } else { true })?;
        Self { waterlogged }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<conduit, u16>> = Lazy::new(|| {
            conduit::possible_states()
                .into_iter()
                .zip([8589u16, 8590u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VoidAir {}
impl VoidAir {
    pub fn possible_states() -> Vec<Self> {
        vec![VoidAir {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(void_air {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaveAir {}
impl CaveAir {
    pub fn possible_states() -> Vec<Self> {
        vec![CaveAir {}]
    }
    pub fn id_offset(self) -> u16 {
        0
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        Ok(cave_air {})
    }
    pub fn vanilla_id_offset(self) -> u16 {
        0
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BubbleColumn {
    drag: bool,
}
impl BubbleColumn {
    pub fn possible_states() -> Vec<Self> {
        [false, true]
            .iter()
            .copied()
            .map(|drag| BubbleColumn { drag })
            .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.drag as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let drag = offset / 1u16;
        offset -= drag * 1u16;
        let drag = Ok(if drag == 0 { false } else { true })?;
        Self { drag }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<bubble_column, u16>> = Lazy::new(|| {
            bubble_column::possible_states()
                .into_iter()
                .zip([8593u16, 8594u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructureBlock {
    mode: StructureBlock,
}
impl StructureBlock {
    pub fn possible_states() -> Vec<Self> {
        [
            StructureBlock::Save,
            StructureBlock::Load,
            StructureBlock::Corner,
            StructureBlock::Data,
        ]
        .iter()
        .copied()
        .map(|mode| StructureBlock { mode })
        .collect()
    }
    pub fn id_offset(self) -> u16 {
        self.mode as u16 * 1u16
    }
    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let mode = offset / 1u16;
        offset -= mode * 1u16;
        let mode = StructureBlock::try_from(mode)?;
        Self { mode }
    }
    pub fn vanilla_id_offset(self) -> u16 {
        static MAP: Lazy<HashMap<structure_block, u16>> = Lazy::new(|| {
            structure_block::possible_states()
                .into_iter()
                .zip([8595u16, 8596u16, 8597u16, 8598u16].iter().copied())
                .collect()
        });
        *MAP.get(&self).unwrap()
    }
}
