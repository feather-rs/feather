
#[derive(Debug, Error)]
pub enum BoolArgumentParseError {}

#[derive(Clone, Debug)]
pub struct BoolArgument(pub String);

impl ArgumentKind<CommandCtx> for BoolArgument {
    type ParseError = BoolArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BoolArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum DoubleArgumentParseError {}

#[derive(Clone, Debug)]
pub struct DoubleArgument(pub String);

impl ArgumentKind<CommandCtx> for DoubleArgument {
    type ParseError = DoubleArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(DoubleArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgument(pub String);

impl ArgumentKind<CommandCtx> for FloatArgument {
    type ParseError = FloatArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentBetween0And1ParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentBetween0And1(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentBetween0And1 {
    type ParseError = FloatArgumentBetween0And1ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentBetween0And1(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentBetween0And2ParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentBetween0And2(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentBetween0And2 {
    type ParseError = FloatArgumentBetween0And2ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentBetween0And2(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentPositiveParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentPositive(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentPositive {
    type ParseError = FloatArgumentPositiveParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentPositive(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentGreaterThen1ParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentGreaterThen1(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentGreaterThen1 {
    type ParseError = FloatArgumentGreaterThen1ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentGreaterThen1(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgument(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgument {
    type ParseError = IntegerArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween0And1000000ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween0And1000000(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween0And1000000 {
    type ParseError = IntegerArgumentBetween0And1000000ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween0And1000000(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween0And255ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween0And255(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween0And255 {
    type ParseError = IntegerArgumentBetween0And255ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween0And255(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween0And65535ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween0And65535(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween0And65535 {
    type ParseError = IntegerArgumentBetween0And65535ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween0And65535(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentPositiveParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentPositive(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentPositive {
    type ParseError = IntegerArgumentPositiveParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentPositive(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween1And1000000ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween1And1000000(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween1And1000000 {
    type ParseError = IntegerArgumentBetween1And1000000ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween1And1000000(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween1And64ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween1And64(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween1And64 {
    type ParseError = IntegerArgumentBetween1And64ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween1And64(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentGreaterThen1ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentGreaterThen1(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentGreaterThen1 {
    type ParseError = IntegerArgumentGreaterThen1ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentGreaterThen1(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum StringArgumentGreedyParseError {}

#[derive(Clone, Debug)]
pub struct StringArgumentGreedy(pub String);

impl ArgumentKind<CommandCtx> for StringArgumentGreedy {
    type ParseError = StringArgumentGreedyParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(StringArgumentGreedy(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum StringArgumentPhraseParseError {}

#[derive(Clone, Debug)]
pub struct StringArgumentPhrase(pub String);

impl ArgumentKind<CommandCtx> for StringArgumentPhrase {
    type ParseError = StringArgumentPhraseParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(StringArgumentPhrase(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum StringArgumentWordParseError {}

#[derive(Clone, Debug)]
pub struct StringArgumentWord(pub String);

impl ArgumentKind<CommandCtx> for StringArgumentWord {
    type ParseError = StringArgumentWordParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(StringArgumentWord(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum BlockPosParseError {}

#[derive(Clone, Debug)]
pub struct BlockPos(pub String);

impl ArgumentKind<CommandCtx> for BlockPos {
    type ParseError = BlockPosParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BlockPos(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum BlockPredicateParseError {}

#[derive(Clone, Debug)]
pub struct BlockPredicate(pub String);

impl ArgumentKind<CommandCtx> for BlockPredicate {
    type ParseError = BlockPredicateParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BlockPredicate(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum BlockStateParseError {}

#[derive(Clone, Debug)]
pub struct BlockState(pub String);

impl ArgumentKind<CommandCtx> for BlockState {
    type ParseError = BlockStateParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BlockState(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ColorParseError {}

#[derive(Clone, Debug)]
pub struct Color(pub String);

impl ArgumentKind<CommandCtx> for Color {
    type ParseError = ColorParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Color(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ColumnPosParseError {}

#[derive(Clone, Debug)]
pub struct ColumnPos(pub String);

impl ArgumentKind<CommandCtx> for ColumnPos {
    type ParseError = ColumnPosParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ColumnPos(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ComponentParseError {}

#[derive(Clone, Debug)]
pub struct Component(pub String);

impl ArgumentKind<CommandCtx> for Component {
    type ParseError = ComponentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Component(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum DimensionParseError {}

#[derive(Clone, Debug)]
pub struct Dimension(pub String);

impl ArgumentKind<CommandCtx> for Dimension {
    type ParseError = DimensionParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Dimension(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum EntityAnchorParseError {}

#[derive(Clone, Debug)]
pub struct EntityAnchor(pub String);

impl ArgumentKind<CommandCtx> for EntityAnchor {
    type ParseError = EntityAnchorParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(EntityAnchor(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum EntitySummonParseError {}

#[derive(Clone, Debug)]
pub struct EntitySummon(pub String);

impl ArgumentKind<CommandCtx> for EntitySummon {
    type ParseError = EntitySummonParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(EntitySummon(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MultipleEntitiesParseError {}

#[derive(Clone, Debug)]
pub struct MultipleEntities(pub String);

impl ArgumentKind<CommandCtx> for MultipleEntities {
    type ParseError = MultipleEntitiesParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MultipleEntities(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MultiplePlayersParseError {}

#[derive(Clone, Debug)]
pub struct MultiplePlayers(pub String);

impl ArgumentKind<CommandCtx> for MultiplePlayers {
    type ParseError = MultiplePlayersParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MultiplePlayers(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SingleEntitiesParseError {}

#[derive(Clone, Debug)]
pub struct SingleEntities(pub String);

impl ArgumentKind<CommandCtx> for SingleEntities {
    type ParseError = SingleEntitiesParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(SingleEntities(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SinglePlayerParseError {}

#[derive(Clone, Debug)]
pub struct SinglePlayer(pub String);

impl ArgumentKind<CommandCtx> for SinglePlayer {
    type ParseError = SinglePlayerParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(SinglePlayer(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MinecraftFunctionParseError {}

#[derive(Clone, Debug)]
pub struct MinecraftFunction(pub String);

impl ArgumentKind<CommandCtx> for MinecraftFunction {
    type ParseError = MinecraftFunctionParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MinecraftFunction(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum GameProfileParseError {}

#[derive(Clone, Debug)]
pub struct GameProfile(pub String);

impl ArgumentKind<CommandCtx> for GameProfile {
    type ParseError = GameProfileParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(GameProfile(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntRageParseError {}

#[derive(Clone, Debug)]
pub struct IntRage(pub String);

impl ArgumentKind<CommandCtx> for IntRage {
    type ParseError = IntRageParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntRage(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum EnchantmentParseError {}

#[derive(Clone, Debug)]
pub struct Enchantment(pub String);

impl ArgumentKind<CommandCtx> for Enchantment {
    type ParseError = EnchantmentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Enchantment(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum PredicateParseError {}

#[derive(Clone, Debug)]
pub struct Predicate(pub String);

impl ArgumentKind<CommandCtx> for Predicate {
    type ParseError = PredicateParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Predicate(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ItemSlotParseError {}

#[derive(Clone, Debug)]
pub struct ItemSlot(pub String);

impl ArgumentKind<CommandCtx> for ItemSlot {
    type ParseError = ItemSlotParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ItemSlot(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ItemStackParseError {}

#[derive(Clone, Debug)]
pub struct ItemStack(pub String);

impl ArgumentKind<CommandCtx> for ItemStack {
    type ParseError = ItemStackParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ItemStack(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MessageParseError {}

#[derive(Clone, Debug)]
pub struct Message(pub String);

impl ArgumentKind<CommandCtx> for Message {
    type ParseError = MessageParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Message(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MobEffectParseError {}

#[derive(Clone, Debug)]
pub struct MobEffect(pub String);

impl ArgumentKind<CommandCtx> for MobEffect {
    type ParseError = MobEffectParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MobEffect(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum NbtCommandTagParseError {}

#[derive(Clone, Debug)]
pub struct NbtCommandTag(pub String);

impl ArgumentKind<CommandCtx> for NbtCommandTag {
    type ParseError = NbtCommandTagParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(NbtCommandTag(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum NbtPathParseError {}

#[derive(Clone, Debug)]
pub struct NbtPath(pub String);

impl ArgumentKind<CommandCtx> for NbtPath {
    type ParseError = NbtPathParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(NbtPath(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum NbtTagParseError {}

#[derive(Clone, Debug)]
pub struct NbtTag(pub String);

impl ArgumentKind<CommandCtx> for NbtTag {
    type ParseError = NbtTagParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(NbtTag(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ObjectiveParseError {}

#[derive(Clone, Debug)]
pub struct Objective(pub String);

impl ArgumentKind<CommandCtx> for Objective {
    type ParseError = ObjectiveParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Objective(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ObjectiveCriteriaParseError {}

#[derive(Clone, Debug)]
pub struct ObjectiveCriteria(pub String);

impl ArgumentKind<CommandCtx> for ObjectiveCriteria {
    type ParseError = ObjectiveCriteriaParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ObjectiveCriteria(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum OperationParseError {}

#[derive(Clone, Debug)]
pub struct Operation(pub String);

impl ArgumentKind<CommandCtx> for Operation {
    type ParseError = OperationParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Operation(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ParticleParseError {}

#[derive(Clone, Debug)]
pub struct Particle(pub String);

impl ArgumentKind<CommandCtx> for Particle {
    type ParseError = ParticleParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Particle(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ResourceLocationParseError {}

#[derive(Clone, Debug)]
pub struct ResourceLocation(pub String);

impl ArgumentKind<CommandCtx> for ResourceLocation {
    type ParseError = ResourceLocationParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ResourceLocation(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum RotationParseError {}

#[derive(Clone, Debug)]
pub struct Rotation(pub String);

impl ArgumentKind<CommandCtx> for Rotation {
    type ParseError = RotationParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Rotation(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MultipleScoreHoldersParseError {}

#[derive(Clone, Debug)]
pub struct MultipleScoreHolders(pub String);

impl ArgumentKind<CommandCtx> for MultipleScoreHolders {
    type ParseError = MultipleScoreHoldersParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MultipleScoreHolders(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SingleScoreHolderParseError {}

#[derive(Clone, Debug)]
pub struct SingleScoreHolder(pub String);

impl ArgumentKind<CommandCtx> for SingleScoreHolder {
    type ParseError = SingleScoreHolderParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(SingleScoreHolder(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ScoreboardSlotParseError {}

#[derive(Clone, Debug)]
pub struct ScoreboardSlot(pub String);

impl ArgumentKind<CommandCtx> for ScoreboardSlot {
    type ParseError = ScoreboardSlotParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ScoreboardSlot(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SwizzleParseError {}

#[derive(Clone, Debug)]
pub struct Swizzle(pub String);

impl ArgumentKind<CommandCtx> for Swizzle {
    type ParseError = SwizzleParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Swizzle(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum TeamParseError {}

#[derive(Clone, Debug)]
pub struct Team(pub String);

impl ArgumentKind<CommandCtx> for Team {
    type ParseError = TeamParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Team(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum TimeParseError {}

#[derive(Clone, Debug)]
pub struct Time(pub String);

impl ArgumentKind<CommandCtx> for Time {
    type ParseError = TimeParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Time(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum UuidParseError {}

#[derive(Clone, Debug)]
pub struct Uuid(pub String);

impl ArgumentKind<CommandCtx> for Uuid {
    type ParseError = UuidParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Uuid(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum Vec2ParseError {}

#[derive(Clone, Debug)]
pub struct Vec2(pub String);

impl ArgumentKind<CommandCtx> for Vec2 {
    type ParseError = Vec2ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Vec2(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum Vec3ParseError {}

#[derive(Clone, Debug)]
pub struct Vec3(pub String);

impl ArgumentKind<CommandCtx> for Vec3 {
    type ParseError = Vec3ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Vec3(text.to_owned()))
    }
}

