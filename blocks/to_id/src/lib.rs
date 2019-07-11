use feather_blocks_enum::*;

pub trait BlockToId {
    fn block_state_id(&self) -> u16;
}

impl BlockToId for Block {
    fn block_state_id(&self) -> u16 {
        let t0 = feather_blocks_to_id0::to_id(self.clone());
        if t0 != 65535 {
            return t0;
        }

        let t1 = feather_blocks_to_id1::to_id(self.clone());
        if t1 != 65535 {
            return t1;
        }

        match self {
            Block::MagentaStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5885;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5886;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5887;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5888;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5889;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5890;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5891;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5892;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5893;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5894;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5895;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5896;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5897;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5898;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5899;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5900;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5901;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5902;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5903;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5904;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5905;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5906;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5907;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5908;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5909;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5910;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5911;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5912;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5913;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5914;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5915;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5916;
                }
            }
            Block::LightBlueStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5917;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5918;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5919;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5920;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5921;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5922;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5923;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5924;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5925;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5926;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5927;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5928;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5929;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5930;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5931;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5932;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5933;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5934;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5935;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5936;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5937;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5938;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5939;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5940;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5941;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5942;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5943;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5944;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5945;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5946;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5947;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5948;
                }
            }
            Block::YellowStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5949;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5950;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5951;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5952;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5953;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5954;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5955;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5956;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5957;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5958;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5959;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5960;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5961;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5962;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5963;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5964;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5965;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5966;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5967;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5968;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5969;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5970;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5971;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5972;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5973;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5974;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5975;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5976;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5977;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5978;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5979;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5980;
                }
            }
            Block::LimeStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5981;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5982;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5983;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5984;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5985;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5986;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5987;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5988;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5989;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5990;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5991;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5992;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5993;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5994;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5995;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 5996;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 5997;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 5998;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 5999;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6000;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6001;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6002;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6003;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6004;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6005;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6006;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6007;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6008;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6009;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6010;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6011;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6012;
                }
            }
            Block::PinkStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6013;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6014;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6015;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6016;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6017;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6018;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6019;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6020;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6021;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6022;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6023;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6024;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6025;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6026;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6027;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6028;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6029;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6030;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6031;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6032;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6033;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6034;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6035;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6036;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6037;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6038;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6039;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6040;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6041;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6042;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6043;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6044;
                }
            }
            Block::GrayStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6045;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6046;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6047;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6048;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6049;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6050;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6051;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6052;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6053;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6054;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6055;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6056;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6057;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6058;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6059;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6060;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6061;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6062;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6063;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6064;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6065;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6066;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6067;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6068;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6069;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6070;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6071;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6072;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6073;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6074;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6075;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6076;
                }
            }
            Block::LightGrayStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6077;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6078;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6079;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6080;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6081;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6082;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6083;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6084;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6085;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6086;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6087;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6088;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6089;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6090;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6091;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6092;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6093;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6094;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6095;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6096;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6097;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6098;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6099;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6100;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6101;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6102;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6103;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6104;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6105;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6106;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6107;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6108;
                }
            }
            Block::CyanStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6109;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6110;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6111;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6112;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6113;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6114;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6115;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6116;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6117;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6118;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6119;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6120;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6121;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6122;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6123;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6124;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6125;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6126;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6127;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6128;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6129;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6130;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6131;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6132;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6133;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6134;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6135;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6136;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6137;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6138;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6139;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6140;
                }
            }
            Block::PurpleStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6141;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6142;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6143;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6144;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6145;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6146;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6147;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6148;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6149;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6150;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6151;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6152;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6153;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6154;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6155;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6156;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6157;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6158;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6159;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6160;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6161;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6162;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6163;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6164;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6165;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6166;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6167;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6168;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6169;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6170;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6171;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6172;
                }
            }
            Block::BlueStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6173;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6174;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6175;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6176;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6177;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6178;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6179;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6180;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6181;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6182;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6183;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6184;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6185;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6186;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6187;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6188;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6189;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6190;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6191;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6192;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6193;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6194;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6195;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6196;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6197;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6198;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6199;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6200;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6201;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6202;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6203;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6204;
                }
            }
            Block::BrownStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6205;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6206;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6207;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6208;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6209;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6210;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6211;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6212;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6213;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6214;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6215;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6216;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6217;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6218;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6219;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6220;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6221;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6222;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6223;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6224;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6225;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6226;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6227;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6228;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6229;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6230;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6231;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6232;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6233;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6234;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6235;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6236;
                }
            }
            Block::GreenStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6237;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6238;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6239;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6240;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6241;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6242;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6243;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6244;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6245;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6246;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6247;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6248;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6249;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6250;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6251;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6252;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6253;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6254;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6255;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6256;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6257;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6258;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6259;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6260;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6261;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6262;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6263;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6264;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6265;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6266;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6267;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6268;
                }
            }
            Block::RedStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6269;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6270;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6271;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6272;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6273;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6274;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6275;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6276;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6277;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6278;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6279;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6280;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6281;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6282;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6283;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6284;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6285;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6286;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6287;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6288;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6289;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6290;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6291;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6292;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6293;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6294;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6295;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6296;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6297;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6298;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6299;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6300;
                }
            }
            Block::BlackStainedGlassPane(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6301;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6302;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6303;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6304;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6305;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6306;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6307;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6308;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6309;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6310;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6311;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6312;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6313;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6314;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6315;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6316;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6317;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6318;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6319;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6320;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6321;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6322;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6323;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6324;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6325;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6326;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6327;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6328;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 6329;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 6330;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 6331;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 6332;
                }
            }
            Block::AcaciaStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6333;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6334;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6335;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6336;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6337;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6338;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6339;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6340;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6341;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6342;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6343;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6344;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6345;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6346;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6347;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6348;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6349;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6350;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6351;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6352;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6353;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6354;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6355;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6356;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6357;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6358;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6359;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6360;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6361;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6362;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6363;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6364;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6365;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6366;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6367;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6368;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6369;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6370;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6371;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6372;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6373;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6374;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6375;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6376;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6377;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6378;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6379;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6380;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6381;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6382;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6383;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6384;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6385;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6386;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6387;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6388;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6389;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6390;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6391;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6392;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6393;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6394;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6395;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6396;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6397;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6398;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6399;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6400;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6401;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6402;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6403;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6404;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6405;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6406;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6407;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6408;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6409;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6410;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6411;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6412;
                }
            }
            Block::DarkOakStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6413;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6414;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6415;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6416;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6417;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6418;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6419;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6420;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6421;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6422;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6423;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6424;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6425;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6426;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6427;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6428;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6429;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6430;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6431;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6432;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6433;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6434;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6435;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6436;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6437;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6438;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6439;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6440;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6441;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6442;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6443;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6444;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6445;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6446;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6447;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6448;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6449;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6450;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6451;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6452;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6453;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6454;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6455;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6456;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6457;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6458;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6459;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6460;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6461;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6462;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6463;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6464;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6465;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6466;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6467;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6468;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6469;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6470;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6471;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6472;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6473;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6474;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6475;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6476;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6477;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6478;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6479;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6480;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6481;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6482;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6483;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6484;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6485;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6486;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6487;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6488;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6489;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6490;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6491;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6492;
                }
            }
            Block::SlimeBlock => return 6493,
            Block::Barrier => return 6494,
            Block::IronTrapdoor(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6495;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6496;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6497;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6498;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6499;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6500;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6501;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6502;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6503;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6504;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6505;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6506;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6507;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6508;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6509;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6510;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6511;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6512;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6513;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6514;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6515;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6516;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6517;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6518;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6519;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6520;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6521;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6522;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6523;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6524;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6525;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6526;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6527;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6528;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6529;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6530;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6531;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6532;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6533;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6534;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6535;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6536;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6537;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6538;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6539;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6540;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6541;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6542;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6543;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6544;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6545;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6546;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6547;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6548;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6549;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6550;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6551;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6552;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6553;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == true
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6554;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == true
                {
                    return 6555;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == true
                    && data.waterlogged == false
                {
                    return 6556;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == true
                {
                    return 6557;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.open == false
                    && data.powered == false
                    && data.waterlogged == false
                {
                    return 6558;
                }
            }
            Block::Prismarine => return 6559,
            Block::PrismarineBricks => return 6560,
            Block::DarkPrismarine => return 6561,
            Block::PrismarineStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6562;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6563;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6564;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6565;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6566;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6567;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6568;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6569;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6570;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6571;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6572;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6573;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6574;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6575;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6576;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6577;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6578;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6579;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6580;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6581;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6582;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6583;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6584;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6585;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6586;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6587;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6588;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6589;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6590;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6591;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6592;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6593;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6594;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6595;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6596;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6597;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6598;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6599;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6600;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6601;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6602;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6603;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6604;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6605;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6606;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6607;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6608;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6609;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6610;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6611;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6612;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6613;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6614;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6615;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6616;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6617;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6618;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6619;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6620;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6621;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6622;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6623;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6624;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6625;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6626;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6627;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6628;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6629;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6630;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6631;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6632;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6633;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6634;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6635;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6636;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6637;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6638;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6639;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6640;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6641;
                }
            }
            Block::PrismarineBrickStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6642;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6643;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6644;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6645;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6646;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6647;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6648;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6649;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6650;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6651;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6652;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6653;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6654;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6655;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6656;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6657;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6658;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6659;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6660;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6661;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6662;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6663;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6664;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6665;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6666;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6667;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6668;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6669;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6670;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6671;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6672;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6673;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6674;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6675;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6676;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6677;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6678;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6679;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6680;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6681;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6682;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6683;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6684;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6685;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6686;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6687;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6688;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6689;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6690;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6691;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6692;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6693;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6694;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6695;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6696;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6697;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6698;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6699;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6700;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6701;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6702;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6703;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6704;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6705;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6706;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6707;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6708;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6709;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6710;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6711;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6712;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6713;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6714;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6715;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6716;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6717;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6718;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6719;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6720;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6721;
                }
            }
            Block::DarkPrismarineStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6722;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6723;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6724;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6725;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6726;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6727;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6728;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6729;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6730;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6731;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6732;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6733;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6734;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6735;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6736;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6737;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6738;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6739;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6740;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6741;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6742;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6743;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6744;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6745;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6746;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6747;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6748;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6749;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6750;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6751;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6752;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6753;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6754;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6755;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6756;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6757;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6758;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6759;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6760;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6761;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6762;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6763;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6764;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6765;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6766;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6767;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6768;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6769;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6770;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6771;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6772;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6773;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6774;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6775;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6776;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6777;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6778;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6779;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6780;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6781;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6782;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6783;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6784;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6785;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6786;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6787;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6788;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6789;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6790;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6791;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 6792;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 6793;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 6794;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 6795;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 6796;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 6797;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 6798;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 6799;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 6800;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 6801;
                }
            }
            Block::PrismarineSlab(data) => {
                if data.type_ == PrismarineSlabType::Top && data.waterlogged == true {
                    return 6802;
                }
                if data.type_ == PrismarineSlabType::Top && data.waterlogged == false {
                    return 6803;
                }
                if data.type_ == PrismarineSlabType::Bottom && data.waterlogged == true {
                    return 6804;
                }
                if data.type_ == PrismarineSlabType::Bottom && data.waterlogged == false {
                    return 6805;
                }
                if data.type_ == PrismarineSlabType::Double && data.waterlogged == true {
                    return 6806;
                }
                if data.type_ == PrismarineSlabType::Double && data.waterlogged == false {
                    return 6807;
                }
            }
            Block::PrismarineBrickSlab(data) => {
                if data.type_ == PrismarineBrickSlabType::Top && data.waterlogged == true {
                    return 6808;
                }
                if data.type_ == PrismarineBrickSlabType::Top && data.waterlogged == false {
                    return 6809;
                }
                if data.type_ == PrismarineBrickSlabType::Bottom && data.waterlogged == true {
                    return 6810;
                }
                if data.type_ == PrismarineBrickSlabType::Bottom && data.waterlogged == false {
                    return 6811;
                }
                if data.type_ == PrismarineBrickSlabType::Double && data.waterlogged == true {
                    return 6812;
                }
                if data.type_ == PrismarineBrickSlabType::Double && data.waterlogged == false {
                    return 6813;
                }
            }
            Block::DarkPrismarineSlab(data) => {
                if data.type_ == DarkPrismarineSlabType::Top && data.waterlogged == true {
                    return 6814;
                }
                if data.type_ == DarkPrismarineSlabType::Top && data.waterlogged == false {
                    return 6815;
                }
                if data.type_ == DarkPrismarineSlabType::Bottom && data.waterlogged == true {
                    return 6816;
                }
                if data.type_ == DarkPrismarineSlabType::Bottom && data.waterlogged == false {
                    return 6817;
                }
                if data.type_ == DarkPrismarineSlabType::Double && data.waterlogged == true {
                    return 6818;
                }
                if data.type_ == DarkPrismarineSlabType::Double && data.waterlogged == false {
                    return 6819;
                }
            }
            Block::SeaLantern => return 6820,
            Block::HayBlock(data) => {
                if data.axis == Axis::X {
                    return 6821;
                }
                if data.axis == Axis::Y {
                    return 6822;
                }
                if data.axis == Axis::Z {
                    return 6823;
                }
            }
            Block::WhiteCarpet => return 6824,
            Block::OrangeCarpet => return 6825,
            Block::MagentaCarpet => return 6826,
            Block::LightBlueCarpet => return 6827,
            Block::YellowCarpet => return 6828,
            Block::LimeCarpet => return 6829,
            Block::PinkCarpet => return 6830,
            Block::GrayCarpet => return 6831,
            Block::LightGrayCarpet => return 6832,
            Block::CyanCarpet => return 6833,
            Block::PurpleCarpet => return 6834,
            Block::BlueCarpet => return 6835,
            Block::BrownCarpet => return 6836,
            Block::GreenCarpet => return 6837,
            Block::RedCarpet => return 6838,
            Block::BlackCarpet => return 6839,
            Block::Terracotta => return 6840,
            Block::CoalBlock => return 6841,
            Block::PackedIce => return 6842,
            Block::Sunflower(data) => {
                if data.half == Half::Upper {
                    return 6843;
                }
                if data.half == Half::Lower {
                    return 6844;
                }
            }
            Block::Lilac(data) => {
                if data.half == Half::Upper {
                    return 6845;
                }
                if data.half == Half::Lower {
                    return 6846;
                }
            }
            Block::RoseBush(data) => {
                if data.half == Half::Upper {
                    return 6847;
                }
                if data.half == Half::Lower {
                    return 6848;
                }
            }
            Block::Peony(data) => {
                if data.half == Half::Upper {
                    return 6849;
                }
                if data.half == Half::Lower {
                    return 6850;
                }
            }
            Block::TallGrass(data) => {
                if data.half == Half::Upper {
                    return 6851;
                }
                if data.half == Half::Lower {
                    return 6852;
                }
            }
            Block::LargeFern(data) => {
                if data.half == Half::Upper {
                    return 6853;
                }
                if data.half == Half::Lower {
                    return 6854;
                }
            }
            Block::WhiteBanner(data) => {
                if data.rotation == 0 {
                    return 6855;
                }
                if data.rotation == 1 {
                    return 6856;
                }
                if data.rotation == 2 {
                    return 6857;
                }
                if data.rotation == 3 {
                    return 6858;
                }
                if data.rotation == 4 {
                    return 6859;
                }
                if data.rotation == 5 {
                    return 6860;
                }
                if data.rotation == 6 {
                    return 6861;
                }
                if data.rotation == 7 {
                    return 6862;
                }
                if data.rotation == 8 {
                    return 6863;
                }
                if data.rotation == 9 {
                    return 6864;
                }
                if data.rotation == 10 {
                    return 6865;
                }
                if data.rotation == 11 {
                    return 6866;
                }
                if data.rotation == 12 {
                    return 6867;
                }
                if data.rotation == 13 {
                    return 6868;
                }
                if data.rotation == 14 {
                    return 6869;
                }
                if data.rotation == 15 {
                    return 6870;
                }
            }
            Block::OrangeBanner(data) => {
                if data.rotation == 0 {
                    return 6871;
                }
                if data.rotation == 1 {
                    return 6872;
                }
                if data.rotation == 2 {
                    return 6873;
                }
                if data.rotation == 3 {
                    return 6874;
                }
                if data.rotation == 4 {
                    return 6875;
                }
                if data.rotation == 5 {
                    return 6876;
                }
                if data.rotation == 6 {
                    return 6877;
                }
                if data.rotation == 7 {
                    return 6878;
                }
                if data.rotation == 8 {
                    return 6879;
                }
                if data.rotation == 9 {
                    return 6880;
                }
                if data.rotation == 10 {
                    return 6881;
                }
                if data.rotation == 11 {
                    return 6882;
                }
                if data.rotation == 12 {
                    return 6883;
                }
                if data.rotation == 13 {
                    return 6884;
                }
                if data.rotation == 14 {
                    return 6885;
                }
                if data.rotation == 15 {
                    return 6886;
                }
            }
            Block::MagentaBanner(data) => {
                if data.rotation == 0 {
                    return 6887;
                }
                if data.rotation == 1 {
                    return 6888;
                }
                if data.rotation == 2 {
                    return 6889;
                }
                if data.rotation == 3 {
                    return 6890;
                }
                if data.rotation == 4 {
                    return 6891;
                }
                if data.rotation == 5 {
                    return 6892;
                }
                if data.rotation == 6 {
                    return 6893;
                }
                if data.rotation == 7 {
                    return 6894;
                }
                if data.rotation == 8 {
                    return 6895;
                }
                if data.rotation == 9 {
                    return 6896;
                }
                if data.rotation == 10 {
                    return 6897;
                }
                if data.rotation == 11 {
                    return 6898;
                }
                if data.rotation == 12 {
                    return 6899;
                }
                if data.rotation == 13 {
                    return 6900;
                }
                if data.rotation == 14 {
                    return 6901;
                }
                if data.rotation == 15 {
                    return 6902;
                }
            }
            Block::LightBlueBanner(data) => {
                if data.rotation == 0 {
                    return 6903;
                }
                if data.rotation == 1 {
                    return 6904;
                }
                if data.rotation == 2 {
                    return 6905;
                }
                if data.rotation == 3 {
                    return 6906;
                }
                if data.rotation == 4 {
                    return 6907;
                }
                if data.rotation == 5 {
                    return 6908;
                }
                if data.rotation == 6 {
                    return 6909;
                }
                if data.rotation == 7 {
                    return 6910;
                }
                if data.rotation == 8 {
                    return 6911;
                }
                if data.rotation == 9 {
                    return 6912;
                }
                if data.rotation == 10 {
                    return 6913;
                }
                if data.rotation == 11 {
                    return 6914;
                }
                if data.rotation == 12 {
                    return 6915;
                }
                if data.rotation == 13 {
                    return 6916;
                }
                if data.rotation == 14 {
                    return 6917;
                }
                if data.rotation == 15 {
                    return 6918;
                }
            }
            Block::YellowBanner(data) => {
                if data.rotation == 0 {
                    return 6919;
                }
                if data.rotation == 1 {
                    return 6920;
                }
                if data.rotation == 2 {
                    return 6921;
                }
                if data.rotation == 3 {
                    return 6922;
                }
                if data.rotation == 4 {
                    return 6923;
                }
                if data.rotation == 5 {
                    return 6924;
                }
                if data.rotation == 6 {
                    return 6925;
                }
                if data.rotation == 7 {
                    return 6926;
                }
                if data.rotation == 8 {
                    return 6927;
                }
                if data.rotation == 9 {
                    return 6928;
                }
                if data.rotation == 10 {
                    return 6929;
                }
                if data.rotation == 11 {
                    return 6930;
                }
                if data.rotation == 12 {
                    return 6931;
                }
                if data.rotation == 13 {
                    return 6932;
                }
                if data.rotation == 14 {
                    return 6933;
                }
                if data.rotation == 15 {
                    return 6934;
                }
            }
            Block::LimeBanner(data) => {
                if data.rotation == 0 {
                    return 6935;
                }
                if data.rotation == 1 {
                    return 6936;
                }
                if data.rotation == 2 {
                    return 6937;
                }
                if data.rotation == 3 {
                    return 6938;
                }
                if data.rotation == 4 {
                    return 6939;
                }
                if data.rotation == 5 {
                    return 6940;
                }
                if data.rotation == 6 {
                    return 6941;
                }
                if data.rotation == 7 {
                    return 6942;
                }
                if data.rotation == 8 {
                    return 6943;
                }
                if data.rotation == 9 {
                    return 6944;
                }
                if data.rotation == 10 {
                    return 6945;
                }
                if data.rotation == 11 {
                    return 6946;
                }
                if data.rotation == 12 {
                    return 6947;
                }
                if data.rotation == 13 {
                    return 6948;
                }
                if data.rotation == 14 {
                    return 6949;
                }
                if data.rotation == 15 {
                    return 6950;
                }
            }
            Block::PinkBanner(data) => {
                if data.rotation == 0 {
                    return 6951;
                }
                if data.rotation == 1 {
                    return 6952;
                }
                if data.rotation == 2 {
                    return 6953;
                }
                if data.rotation == 3 {
                    return 6954;
                }
                if data.rotation == 4 {
                    return 6955;
                }
                if data.rotation == 5 {
                    return 6956;
                }
                if data.rotation == 6 {
                    return 6957;
                }
                if data.rotation == 7 {
                    return 6958;
                }
                if data.rotation == 8 {
                    return 6959;
                }
                if data.rotation == 9 {
                    return 6960;
                }
                if data.rotation == 10 {
                    return 6961;
                }
                if data.rotation == 11 {
                    return 6962;
                }
                if data.rotation == 12 {
                    return 6963;
                }
                if data.rotation == 13 {
                    return 6964;
                }
                if data.rotation == 14 {
                    return 6965;
                }
                if data.rotation == 15 {
                    return 6966;
                }
            }
            Block::GrayBanner(data) => {
                if data.rotation == 0 {
                    return 6967;
                }
                if data.rotation == 1 {
                    return 6968;
                }
                if data.rotation == 2 {
                    return 6969;
                }
                if data.rotation == 3 {
                    return 6970;
                }
                if data.rotation == 4 {
                    return 6971;
                }
                if data.rotation == 5 {
                    return 6972;
                }
                if data.rotation == 6 {
                    return 6973;
                }
                if data.rotation == 7 {
                    return 6974;
                }
                if data.rotation == 8 {
                    return 6975;
                }
                if data.rotation == 9 {
                    return 6976;
                }
                if data.rotation == 10 {
                    return 6977;
                }
                if data.rotation == 11 {
                    return 6978;
                }
                if data.rotation == 12 {
                    return 6979;
                }
                if data.rotation == 13 {
                    return 6980;
                }
                if data.rotation == 14 {
                    return 6981;
                }
                if data.rotation == 15 {
                    return 6982;
                }
            }
            Block::LightGrayBanner(data) => {
                if data.rotation == 0 {
                    return 6983;
                }
                if data.rotation == 1 {
                    return 6984;
                }
                if data.rotation == 2 {
                    return 6985;
                }
                if data.rotation == 3 {
                    return 6986;
                }
                if data.rotation == 4 {
                    return 6987;
                }
                if data.rotation == 5 {
                    return 6988;
                }
                if data.rotation == 6 {
                    return 6989;
                }
                if data.rotation == 7 {
                    return 6990;
                }
                if data.rotation == 8 {
                    return 6991;
                }
                if data.rotation == 9 {
                    return 6992;
                }
                if data.rotation == 10 {
                    return 6993;
                }
                if data.rotation == 11 {
                    return 6994;
                }
                if data.rotation == 12 {
                    return 6995;
                }
                if data.rotation == 13 {
                    return 6996;
                }
                if data.rotation == 14 {
                    return 6997;
                }
                if data.rotation == 15 {
                    return 6998;
                }
            }
            Block::CyanBanner(data) => {
                if data.rotation == 0 {
                    return 6999;
                }
                if data.rotation == 1 {
                    return 7000;
                }
                if data.rotation == 2 {
                    return 7001;
                }
                if data.rotation == 3 {
                    return 7002;
                }
                if data.rotation == 4 {
                    return 7003;
                }
                if data.rotation == 5 {
                    return 7004;
                }
                if data.rotation == 6 {
                    return 7005;
                }
                if data.rotation == 7 {
                    return 7006;
                }
                if data.rotation == 8 {
                    return 7007;
                }
                if data.rotation == 9 {
                    return 7008;
                }
                if data.rotation == 10 {
                    return 7009;
                }
                if data.rotation == 11 {
                    return 7010;
                }
                if data.rotation == 12 {
                    return 7011;
                }
                if data.rotation == 13 {
                    return 7012;
                }
                if data.rotation == 14 {
                    return 7013;
                }
                if data.rotation == 15 {
                    return 7014;
                }
            }
            Block::PurpleBanner(data) => {
                if data.rotation == 0 {
                    return 7015;
                }
                if data.rotation == 1 {
                    return 7016;
                }
                if data.rotation == 2 {
                    return 7017;
                }
                if data.rotation == 3 {
                    return 7018;
                }
                if data.rotation == 4 {
                    return 7019;
                }
                if data.rotation == 5 {
                    return 7020;
                }
                if data.rotation == 6 {
                    return 7021;
                }
                if data.rotation == 7 {
                    return 7022;
                }
                if data.rotation == 8 {
                    return 7023;
                }
                if data.rotation == 9 {
                    return 7024;
                }
                if data.rotation == 10 {
                    return 7025;
                }
                if data.rotation == 11 {
                    return 7026;
                }
                if data.rotation == 12 {
                    return 7027;
                }
                if data.rotation == 13 {
                    return 7028;
                }
                if data.rotation == 14 {
                    return 7029;
                }
                if data.rotation == 15 {
                    return 7030;
                }
            }
            Block::BlueBanner(data) => {
                if data.rotation == 0 {
                    return 7031;
                }
                if data.rotation == 1 {
                    return 7032;
                }
                if data.rotation == 2 {
                    return 7033;
                }
                if data.rotation == 3 {
                    return 7034;
                }
                if data.rotation == 4 {
                    return 7035;
                }
                if data.rotation == 5 {
                    return 7036;
                }
                if data.rotation == 6 {
                    return 7037;
                }
                if data.rotation == 7 {
                    return 7038;
                }
                if data.rotation == 8 {
                    return 7039;
                }
                if data.rotation == 9 {
                    return 7040;
                }
                if data.rotation == 10 {
                    return 7041;
                }
                if data.rotation == 11 {
                    return 7042;
                }
                if data.rotation == 12 {
                    return 7043;
                }
                if data.rotation == 13 {
                    return 7044;
                }
                if data.rotation == 14 {
                    return 7045;
                }
                if data.rotation == 15 {
                    return 7046;
                }
            }
            Block::BrownBanner(data) => {
                if data.rotation == 0 {
                    return 7047;
                }
                if data.rotation == 1 {
                    return 7048;
                }
                if data.rotation == 2 {
                    return 7049;
                }
                if data.rotation == 3 {
                    return 7050;
                }
                if data.rotation == 4 {
                    return 7051;
                }
                if data.rotation == 5 {
                    return 7052;
                }
                if data.rotation == 6 {
                    return 7053;
                }
                if data.rotation == 7 {
                    return 7054;
                }
                if data.rotation == 8 {
                    return 7055;
                }
                if data.rotation == 9 {
                    return 7056;
                }
                if data.rotation == 10 {
                    return 7057;
                }
                if data.rotation == 11 {
                    return 7058;
                }
                if data.rotation == 12 {
                    return 7059;
                }
                if data.rotation == 13 {
                    return 7060;
                }
                if data.rotation == 14 {
                    return 7061;
                }
                if data.rotation == 15 {
                    return 7062;
                }
            }
            Block::GreenBanner(data) => {
                if data.rotation == 0 {
                    return 7063;
                }
                if data.rotation == 1 {
                    return 7064;
                }
                if data.rotation == 2 {
                    return 7065;
                }
                if data.rotation == 3 {
                    return 7066;
                }
                if data.rotation == 4 {
                    return 7067;
                }
                if data.rotation == 5 {
                    return 7068;
                }
                if data.rotation == 6 {
                    return 7069;
                }
                if data.rotation == 7 {
                    return 7070;
                }
                if data.rotation == 8 {
                    return 7071;
                }
                if data.rotation == 9 {
                    return 7072;
                }
                if data.rotation == 10 {
                    return 7073;
                }
                if data.rotation == 11 {
                    return 7074;
                }
                if data.rotation == 12 {
                    return 7075;
                }
                if data.rotation == 13 {
                    return 7076;
                }
                if data.rotation == 14 {
                    return 7077;
                }
                if data.rotation == 15 {
                    return 7078;
                }
            }
            Block::RedBanner(data) => {
                if data.rotation == 0 {
                    return 7079;
                }
                if data.rotation == 1 {
                    return 7080;
                }
                if data.rotation == 2 {
                    return 7081;
                }
                if data.rotation == 3 {
                    return 7082;
                }
                if data.rotation == 4 {
                    return 7083;
                }
                if data.rotation == 5 {
                    return 7084;
                }
                if data.rotation == 6 {
                    return 7085;
                }
                if data.rotation == 7 {
                    return 7086;
                }
                if data.rotation == 8 {
                    return 7087;
                }
                if data.rotation == 9 {
                    return 7088;
                }
                if data.rotation == 10 {
                    return 7089;
                }
                if data.rotation == 11 {
                    return 7090;
                }
                if data.rotation == 12 {
                    return 7091;
                }
                if data.rotation == 13 {
                    return 7092;
                }
                if data.rotation == 14 {
                    return 7093;
                }
                if data.rotation == 15 {
                    return 7094;
                }
            }
            Block::BlackBanner(data) => {
                if data.rotation == 0 {
                    return 7095;
                }
                if data.rotation == 1 {
                    return 7096;
                }
                if data.rotation == 2 {
                    return 7097;
                }
                if data.rotation == 3 {
                    return 7098;
                }
                if data.rotation == 4 {
                    return 7099;
                }
                if data.rotation == 5 {
                    return 7100;
                }
                if data.rotation == 6 {
                    return 7101;
                }
                if data.rotation == 7 {
                    return 7102;
                }
                if data.rotation == 8 {
                    return 7103;
                }
                if data.rotation == 9 {
                    return 7104;
                }
                if data.rotation == 10 {
                    return 7105;
                }
                if data.rotation == 11 {
                    return 7106;
                }
                if data.rotation == 12 {
                    return 7107;
                }
                if data.rotation == 13 {
                    return 7108;
                }
                if data.rotation == 14 {
                    return 7109;
                }
                if data.rotation == 15 {
                    return 7110;
                }
            }
            Block::WhiteWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7111;
                }
                if data.facing == Facing::South {
                    return 7112;
                }
                if data.facing == Facing::West {
                    return 7113;
                }
                if data.facing == Facing::East {
                    return 7114;
                }
            }
            Block::OrangeWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7115;
                }
                if data.facing == Facing::South {
                    return 7116;
                }
                if data.facing == Facing::West {
                    return 7117;
                }
                if data.facing == Facing::East {
                    return 7118;
                }
            }
            Block::MagentaWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7119;
                }
                if data.facing == Facing::South {
                    return 7120;
                }
                if data.facing == Facing::West {
                    return 7121;
                }
                if data.facing == Facing::East {
                    return 7122;
                }
            }
            Block::LightBlueWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7123;
                }
                if data.facing == Facing::South {
                    return 7124;
                }
                if data.facing == Facing::West {
                    return 7125;
                }
                if data.facing == Facing::East {
                    return 7126;
                }
            }
            Block::YellowWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7127;
                }
                if data.facing == Facing::South {
                    return 7128;
                }
                if data.facing == Facing::West {
                    return 7129;
                }
                if data.facing == Facing::East {
                    return 7130;
                }
            }
            Block::LimeWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7131;
                }
                if data.facing == Facing::South {
                    return 7132;
                }
                if data.facing == Facing::West {
                    return 7133;
                }
                if data.facing == Facing::East {
                    return 7134;
                }
            }
            Block::PinkWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7135;
                }
                if data.facing == Facing::South {
                    return 7136;
                }
                if data.facing == Facing::West {
                    return 7137;
                }
                if data.facing == Facing::East {
                    return 7138;
                }
            }
            Block::GrayWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7139;
                }
                if data.facing == Facing::South {
                    return 7140;
                }
                if data.facing == Facing::West {
                    return 7141;
                }
                if data.facing == Facing::East {
                    return 7142;
                }
            }
            Block::LightGrayWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7143;
                }
                if data.facing == Facing::South {
                    return 7144;
                }
                if data.facing == Facing::West {
                    return 7145;
                }
                if data.facing == Facing::East {
                    return 7146;
                }
            }
            Block::CyanWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7147;
                }
                if data.facing == Facing::South {
                    return 7148;
                }
                if data.facing == Facing::West {
                    return 7149;
                }
                if data.facing == Facing::East {
                    return 7150;
                }
            }
            Block::PurpleWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7151;
                }
                if data.facing == Facing::South {
                    return 7152;
                }
                if data.facing == Facing::West {
                    return 7153;
                }
                if data.facing == Facing::East {
                    return 7154;
                }
            }
            Block::BlueWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7155;
                }
                if data.facing == Facing::South {
                    return 7156;
                }
                if data.facing == Facing::West {
                    return 7157;
                }
                if data.facing == Facing::East {
                    return 7158;
                }
            }
            Block::BrownWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7159;
                }
                if data.facing == Facing::South {
                    return 7160;
                }
                if data.facing == Facing::West {
                    return 7161;
                }
                if data.facing == Facing::East {
                    return 7162;
                }
            }
            Block::GreenWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7163;
                }
                if data.facing == Facing::South {
                    return 7164;
                }
                if data.facing == Facing::West {
                    return 7165;
                }
                if data.facing == Facing::East {
                    return 7166;
                }
            }
            Block::RedWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7167;
                }
                if data.facing == Facing::South {
                    return 7168;
                }
                if data.facing == Facing::West {
                    return 7169;
                }
                if data.facing == Facing::East {
                    return 7170;
                }
            }
            Block::BlackWallBanner(data) => {
                if data.facing == Facing::North {
                    return 7171;
                }
                if data.facing == Facing::South {
                    return 7172;
                }
                if data.facing == Facing::West {
                    return 7173;
                }
                if data.facing == Facing::East {
                    return 7174;
                }
            }
            Block::RedSandstone => return 7175,
            Block::ChiseledRedSandstone => return 7176,
            Block::CutRedSandstone => return 7177,
            Block::RedSandstoneStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7178;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7179;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7180;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7181;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7182;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7183;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7184;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7185;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7186;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7187;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7188;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7189;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7190;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7191;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7192;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7193;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7194;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7195;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7196;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7197;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7198;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7199;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7200;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7201;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7202;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7203;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7204;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7205;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7206;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7207;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7208;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7209;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7210;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7211;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7212;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7213;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7214;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7215;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7216;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7217;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7218;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7219;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7220;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7221;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7222;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7223;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7224;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7225;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7226;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7227;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7228;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7229;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7230;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7231;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7232;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7233;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7234;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7235;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7236;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7237;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7238;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7239;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7240;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7241;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7242;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7243;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7244;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7245;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7246;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7247;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 7248;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 7249;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 7250;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 7251;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 7252;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 7253;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 7254;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 7255;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 7256;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 7257;
                }
            }
            Block::OakSlab(data) => {
                if data.type_ == OakSlabType::Top && data.waterlogged == true {
                    return 7258;
                }
                if data.type_ == OakSlabType::Top && data.waterlogged == false {
                    return 7259;
                }
                if data.type_ == OakSlabType::Bottom && data.waterlogged == true {
                    return 7260;
                }
                if data.type_ == OakSlabType::Bottom && data.waterlogged == false {
                    return 7261;
                }
                if data.type_ == OakSlabType::Double && data.waterlogged == true {
                    return 7262;
                }
                if data.type_ == OakSlabType::Double && data.waterlogged == false {
                    return 7263;
                }
            }
            Block::SpruceSlab(data) => {
                if data.type_ == SpruceSlabType::Top && data.waterlogged == true {
                    return 7264;
                }
                if data.type_ == SpruceSlabType::Top && data.waterlogged == false {
                    return 7265;
                }
                if data.type_ == SpruceSlabType::Bottom && data.waterlogged == true {
                    return 7266;
                }
                if data.type_ == SpruceSlabType::Bottom && data.waterlogged == false {
                    return 7267;
                }
                if data.type_ == SpruceSlabType::Double && data.waterlogged == true {
                    return 7268;
                }
                if data.type_ == SpruceSlabType::Double && data.waterlogged == false {
                    return 7269;
                }
            }
            Block::BirchSlab(data) => {
                if data.type_ == BirchSlabType::Top && data.waterlogged == true {
                    return 7270;
                }
                if data.type_ == BirchSlabType::Top && data.waterlogged == false {
                    return 7271;
                }
                if data.type_ == BirchSlabType::Bottom && data.waterlogged == true {
                    return 7272;
                }
                if data.type_ == BirchSlabType::Bottom && data.waterlogged == false {
                    return 7273;
                }
                if data.type_ == BirchSlabType::Double && data.waterlogged == true {
                    return 7274;
                }
                if data.type_ == BirchSlabType::Double && data.waterlogged == false {
                    return 7275;
                }
            }
            Block::JungleSlab(data) => {
                if data.type_ == JungleSlabType::Top && data.waterlogged == true {
                    return 7276;
                }
                if data.type_ == JungleSlabType::Top && data.waterlogged == false {
                    return 7277;
                }
                if data.type_ == JungleSlabType::Bottom && data.waterlogged == true {
                    return 7278;
                }
                if data.type_ == JungleSlabType::Bottom && data.waterlogged == false {
                    return 7279;
                }
                if data.type_ == JungleSlabType::Double && data.waterlogged == true {
                    return 7280;
                }
                if data.type_ == JungleSlabType::Double && data.waterlogged == false {
                    return 7281;
                }
            }
            Block::AcaciaSlab(data) => {
                if data.type_ == AcaciaSlabType::Top && data.waterlogged == true {
                    return 7282;
                }
                if data.type_ == AcaciaSlabType::Top && data.waterlogged == false {
                    return 7283;
                }
                if data.type_ == AcaciaSlabType::Bottom && data.waterlogged == true {
                    return 7284;
                }
                if data.type_ == AcaciaSlabType::Bottom && data.waterlogged == false {
                    return 7285;
                }
                if data.type_ == AcaciaSlabType::Double && data.waterlogged == true {
                    return 7286;
                }
                if data.type_ == AcaciaSlabType::Double && data.waterlogged == false {
                    return 7287;
                }
            }
            Block::DarkOakSlab(data) => {
                if data.type_ == DarkOakSlabType::Top && data.waterlogged == true {
                    return 7288;
                }
                if data.type_ == DarkOakSlabType::Top && data.waterlogged == false {
                    return 7289;
                }
                if data.type_ == DarkOakSlabType::Bottom && data.waterlogged == true {
                    return 7290;
                }
                if data.type_ == DarkOakSlabType::Bottom && data.waterlogged == false {
                    return 7291;
                }
                if data.type_ == DarkOakSlabType::Double && data.waterlogged == true {
                    return 7292;
                }
                if data.type_ == DarkOakSlabType::Double && data.waterlogged == false {
                    return 7293;
                }
            }
            Block::StoneSlab(data) => {
                if data.type_ == StoneSlabType::Top && data.waterlogged == true {
                    return 7294;
                }
                if data.type_ == StoneSlabType::Top && data.waterlogged == false {
                    return 7295;
                }
                if data.type_ == StoneSlabType::Bottom && data.waterlogged == true {
                    return 7296;
                }
                if data.type_ == StoneSlabType::Bottom && data.waterlogged == false {
                    return 7297;
                }
                if data.type_ == StoneSlabType::Double && data.waterlogged == true {
                    return 7298;
                }
                if data.type_ == StoneSlabType::Double && data.waterlogged == false {
                    return 7299;
                }
            }
            Block::SandstoneSlab(data) => {
                if data.type_ == SandstoneSlabType::Top && data.waterlogged == true {
                    return 7300;
                }
                if data.type_ == SandstoneSlabType::Top && data.waterlogged == false {
                    return 7301;
                }
                if data.type_ == SandstoneSlabType::Bottom && data.waterlogged == true {
                    return 7302;
                }
                if data.type_ == SandstoneSlabType::Bottom && data.waterlogged == false {
                    return 7303;
                }
                if data.type_ == SandstoneSlabType::Double && data.waterlogged == true {
                    return 7304;
                }
                if data.type_ == SandstoneSlabType::Double && data.waterlogged == false {
                    return 7305;
                }
            }
            Block::PetrifiedOakSlab(data) => {
                if data.type_ == PetrifiedOakSlabType::Top && data.waterlogged == true {
                    return 7306;
                }
                if data.type_ == PetrifiedOakSlabType::Top && data.waterlogged == false {
                    return 7307;
                }
                if data.type_ == PetrifiedOakSlabType::Bottom && data.waterlogged == true {
                    return 7308;
                }
                if data.type_ == PetrifiedOakSlabType::Bottom && data.waterlogged == false {
                    return 7309;
                }
                if data.type_ == PetrifiedOakSlabType::Double && data.waterlogged == true {
                    return 7310;
                }
                if data.type_ == PetrifiedOakSlabType::Double && data.waterlogged == false {
                    return 7311;
                }
            }
            Block::CobblestoneSlab(data) => {
                if data.type_ == CobblestoneSlabType::Top && data.waterlogged == true {
                    return 7312;
                }
                if data.type_ == CobblestoneSlabType::Top && data.waterlogged == false {
                    return 7313;
                }
                if data.type_ == CobblestoneSlabType::Bottom && data.waterlogged == true {
                    return 7314;
                }
                if data.type_ == CobblestoneSlabType::Bottom && data.waterlogged == false {
                    return 7315;
                }
                if data.type_ == CobblestoneSlabType::Double && data.waterlogged == true {
                    return 7316;
                }
                if data.type_ == CobblestoneSlabType::Double && data.waterlogged == false {
                    return 7317;
                }
            }
            Block::BrickSlab(data) => {
                if data.type_ == BrickSlabType::Top && data.waterlogged == true {
                    return 7318;
                }
                if data.type_ == BrickSlabType::Top && data.waterlogged == false {
                    return 7319;
                }
                if data.type_ == BrickSlabType::Bottom && data.waterlogged == true {
                    return 7320;
                }
                if data.type_ == BrickSlabType::Bottom && data.waterlogged == false {
                    return 7321;
                }
                if data.type_ == BrickSlabType::Double && data.waterlogged == true {
                    return 7322;
                }
                if data.type_ == BrickSlabType::Double && data.waterlogged == false {
                    return 7323;
                }
            }
            Block::StoneBrickSlab(data) => {
                if data.type_ == StoneBrickSlabType::Top && data.waterlogged == true {
                    return 7324;
                }
                if data.type_ == StoneBrickSlabType::Top && data.waterlogged == false {
                    return 7325;
                }
                if data.type_ == StoneBrickSlabType::Bottom && data.waterlogged == true {
                    return 7326;
                }
                if data.type_ == StoneBrickSlabType::Bottom && data.waterlogged == false {
                    return 7327;
                }
                if data.type_ == StoneBrickSlabType::Double && data.waterlogged == true {
                    return 7328;
                }
                if data.type_ == StoneBrickSlabType::Double && data.waterlogged == false {
                    return 7329;
                }
            }
            Block::NetherBrickSlab(data) => {
                if data.type_ == NetherBrickSlabType::Top && data.waterlogged == true {
                    return 7330;
                }
                if data.type_ == NetherBrickSlabType::Top && data.waterlogged == false {
                    return 7331;
                }
                if data.type_ == NetherBrickSlabType::Bottom && data.waterlogged == true {
                    return 7332;
                }
                if data.type_ == NetherBrickSlabType::Bottom && data.waterlogged == false {
                    return 7333;
                }
                if data.type_ == NetherBrickSlabType::Double && data.waterlogged == true {
                    return 7334;
                }
                if data.type_ == NetherBrickSlabType::Double && data.waterlogged == false {
                    return 7335;
                }
            }
            Block::QuartzSlab(data) => {
                if data.type_ == QuartzSlabType::Top && data.waterlogged == true {
                    return 7336;
                }
                if data.type_ == QuartzSlabType::Top && data.waterlogged == false {
                    return 7337;
                }
                if data.type_ == QuartzSlabType::Bottom && data.waterlogged == true {
                    return 7338;
                }
                if data.type_ == QuartzSlabType::Bottom && data.waterlogged == false {
                    return 7339;
                }
                if data.type_ == QuartzSlabType::Double && data.waterlogged == true {
                    return 7340;
                }
                if data.type_ == QuartzSlabType::Double && data.waterlogged == false {
                    return 7341;
                }
            }
            Block::RedSandstoneSlab(data) => {
                if data.type_ == RedSandstoneSlabType::Top && data.waterlogged == true {
                    return 7342;
                }
                if data.type_ == RedSandstoneSlabType::Top && data.waterlogged == false {
                    return 7343;
                }
                if data.type_ == RedSandstoneSlabType::Bottom && data.waterlogged == true {
                    return 7344;
                }
                if data.type_ == RedSandstoneSlabType::Bottom && data.waterlogged == false {
                    return 7345;
                }
                if data.type_ == RedSandstoneSlabType::Double && data.waterlogged == true {
                    return 7346;
                }
                if data.type_ == RedSandstoneSlabType::Double && data.waterlogged == false {
                    return 7347;
                }
            }
            Block::PurpurSlab(data) => {
                if data.type_ == PurpurSlabType::Top && data.waterlogged == true {
                    return 7348;
                }
                if data.type_ == PurpurSlabType::Top && data.waterlogged == false {
                    return 7349;
                }
                if data.type_ == PurpurSlabType::Bottom && data.waterlogged == true {
                    return 7350;
                }
                if data.type_ == PurpurSlabType::Bottom && data.waterlogged == false {
                    return 7351;
                }
                if data.type_ == PurpurSlabType::Double && data.waterlogged == true {
                    return 7352;
                }
                if data.type_ == PurpurSlabType::Double && data.waterlogged == false {
                    return 7353;
                }
            }
            Block::SmoothStone => return 7354,
            Block::SmoothSandstone => return 7355,
            Block::SmoothQuartz => return 7356,
            Block::SmoothRedSandstone => return 7357,
            Block::SpruceFenceGate(data) => {
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7358;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7359;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7360;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7361;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7362;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7363;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7364;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7365;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7366;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7367;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7368;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7369;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7370;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7371;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7372;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7373;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7374;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7375;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7376;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7377;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7378;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7379;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7380;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7381;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7382;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7383;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7384;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7385;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7386;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7387;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7388;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7389;
                }
            }
            Block::BirchFenceGate(data) => {
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7390;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7391;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7392;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7393;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7394;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7395;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7396;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7397;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7398;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7399;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7400;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7401;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7402;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7403;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7404;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7405;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7406;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7407;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7408;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7409;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7410;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7411;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7412;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7413;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7414;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7415;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7416;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7417;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7418;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7419;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7420;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7421;
                }
            }
            Block::JungleFenceGate(data) => {
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7422;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7423;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7424;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7425;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7426;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7427;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7428;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7429;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7430;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7431;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7432;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7433;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7434;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7435;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7436;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7437;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7438;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7439;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7440;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7441;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7442;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7443;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7444;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7445;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7446;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7447;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7448;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7449;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7450;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7451;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7452;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7453;
                }
            }
            Block::AcaciaFenceGate(data) => {
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7454;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7455;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7456;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7457;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7458;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7459;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7460;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7461;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7462;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7463;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7464;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7465;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7466;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7467;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7468;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7469;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7470;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7471;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7472;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7473;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7474;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7475;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7476;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7477;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7478;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7479;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7480;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7481;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7482;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7483;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7484;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7485;
                }
            }
            Block::DarkOakFenceGate(data) => {
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7486;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7487;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7488;
                }
                if data.facing == Facing::North
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7489;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7490;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7491;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7492;
                }
                if data.facing == Facing::North
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7493;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7494;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7495;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7496;
                }
                if data.facing == Facing::South
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7497;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7498;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7499;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7500;
                }
                if data.facing == Facing::South
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7501;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7502;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7503;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7504;
                }
                if data.facing == Facing::West
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7505;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7506;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7507;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7508;
                }
                if data.facing == Facing::West
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7509;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == true
                {
                    return 7510;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == true
                    && data.powered == false
                {
                    return 7511;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == true
                {
                    return 7512;
                }
                if data.facing == Facing::East
                    && data.in_wall == true
                    && data.open == false
                    && data.powered == false
                {
                    return 7513;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == true
                {
                    return 7514;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == true
                    && data.powered == false
                {
                    return 7515;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == true
                {
                    return 7516;
                }
                if data.facing == Facing::East
                    && data.in_wall == false
                    && data.open == false
                    && data.powered == false
                {
                    return 7517;
                }
            }
            Block::SpruceFence(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7518;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7519;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7520;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7521;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7522;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7523;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7524;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7525;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7526;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7527;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7528;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7529;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7530;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7531;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7532;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7533;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7534;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7535;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7536;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7537;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7538;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7539;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7540;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7541;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7542;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7543;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7544;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7545;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7546;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7547;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7548;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7549;
                }
            }
            Block::BirchFence(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7550;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7551;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7552;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7553;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7554;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7555;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7556;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7557;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7558;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7559;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7560;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7561;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7562;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7563;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7564;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7565;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7566;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7567;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7568;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7569;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7570;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7571;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7572;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7573;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7574;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7575;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7576;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7577;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7578;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7579;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7580;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7581;
                }
            }
            Block::JungleFence(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7582;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7583;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7584;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7585;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7586;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7587;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7588;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7589;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7590;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7591;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7592;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7593;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7594;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7595;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7596;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7597;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7598;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7599;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7600;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7601;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7602;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7603;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7604;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7605;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7606;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7607;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7608;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7609;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7610;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7611;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7612;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7613;
                }
            }
            Block::AcaciaFence(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7614;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7615;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7616;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7617;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7618;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7619;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7620;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7621;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7622;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7623;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7624;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7625;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7626;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7627;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7628;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7629;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7630;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7631;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7632;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7633;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7634;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7635;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7636;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7637;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7638;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7639;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7640;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7641;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7642;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7643;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7644;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7645;
                }
            }
            Block::DarkOakFence(data) => {
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7646;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7647;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7648;
                }
                if data.east == true
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7649;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7650;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7651;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7652;
                }
                if data.east == true
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7653;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7654;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7655;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7656;
                }
                if data.east == true
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7657;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7658;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7659;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7660;
                }
                if data.east == true
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7661;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7662;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7663;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7664;
                }
                if data.east == false
                    && data.north == true
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7665;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7666;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7667;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7668;
                }
                if data.east == false
                    && data.north == true
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7669;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7670;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7671;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7672;
                }
                if data.east == false
                    && data.north == false
                    && data.south == true
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7673;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == true
                {
                    return 7674;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == true
                    && data.west == false
                {
                    return 7675;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == true
                {
                    return 7676;
                }
                if data.east == false
                    && data.north == false
                    && data.south == false
                    && data.waterlogged == false
                    && data.west == false
                {
                    return 7677;
                }
            }
            Block::SpruceDoor(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7678;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7679;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7680;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7681;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7682;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7683;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7684;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7685;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7686;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7687;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7688;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7689;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7690;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7691;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7692;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7693;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7694;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7695;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7696;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7697;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7698;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7699;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7700;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7701;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7702;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7703;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7704;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7705;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7706;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7707;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7708;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7709;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7710;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7711;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7712;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7713;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7714;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7715;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7716;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7717;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7718;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7719;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7720;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7721;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7722;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7723;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7724;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7725;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7726;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7727;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7728;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7729;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7730;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7731;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7732;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7733;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7734;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7735;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7736;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7737;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7738;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7739;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7740;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7741;
                }
            }
            Block::BirchDoor(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7742;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7743;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7744;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7745;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7746;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7747;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7748;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7749;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7750;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7751;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7752;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7753;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7754;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7755;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7756;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7757;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7758;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7759;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7760;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7761;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7762;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7763;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7764;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7765;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7766;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7767;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7768;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7769;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7770;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7771;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7772;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7773;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7774;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7775;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7776;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7777;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7778;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7779;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7780;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7781;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7782;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7783;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7784;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7785;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7786;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7787;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7788;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7789;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7790;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7791;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7792;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7793;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7794;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7795;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7796;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7797;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7798;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7799;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7800;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7801;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7802;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7803;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7804;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7805;
                }
            }
            Block::JungleDoor(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7806;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7807;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7808;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7809;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7810;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7811;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7812;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7813;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7814;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7815;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7816;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7817;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7818;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7819;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7820;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7821;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7822;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7823;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7824;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7825;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7826;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7827;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7828;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7829;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7830;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7831;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7832;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7833;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7834;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7835;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7836;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7837;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7838;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7839;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7840;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7841;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7842;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7843;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7844;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7845;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7846;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7847;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7848;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7849;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7850;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7851;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7852;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7853;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7854;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7855;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7856;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7857;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7858;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7859;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7860;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7861;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7862;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7863;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7864;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7865;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7866;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7867;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7868;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7869;
                }
            }
            Block::AcaciaDoor(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7870;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7871;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7872;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7873;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7874;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7875;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7876;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7877;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7878;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7879;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7880;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7881;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7882;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7883;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7884;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7885;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7886;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7887;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7888;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7889;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7890;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7891;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7892;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7893;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7894;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7895;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7896;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7897;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7898;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7899;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7900;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7901;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7902;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7903;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7904;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7905;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7906;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7907;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7908;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7909;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7910;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7911;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7912;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7913;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7914;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7915;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7916;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7917;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7918;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7919;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7920;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7921;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7922;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7923;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7924;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7925;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7926;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7927;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7928;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7929;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7930;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7931;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7932;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7933;
                }
            }
            Block::DarkOakDoor(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7934;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7935;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7936;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7937;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7938;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7939;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7940;
                }
                if data.facing == Facing::North
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7941;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7942;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7943;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7944;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7945;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7946;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7947;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7948;
                }
                if data.facing == Facing::North
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7949;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7950;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7951;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7952;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7953;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7954;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7955;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7956;
                }
                if data.facing == Facing::South
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7957;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7958;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7959;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7960;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7961;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7962;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7963;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7964;
                }
                if data.facing == Facing::South
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7965;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7966;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7967;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7968;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7969;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7970;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7971;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7972;
                }
                if data.facing == Facing::West
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7973;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7974;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7975;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7976;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7977;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7978;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7979;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7980;
                }
                if data.facing == Facing::West
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7981;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7982;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7983;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7984;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7985;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7986;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7987;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7988;
                }
                if data.facing == Facing::East
                    && data.half == Half::Upper
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7989;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == true
                {
                    return 7990;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == true
                    && data.powered == false
                {
                    return 7991;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == true
                {
                    return 7992;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Left
                    && data.open == false
                    && data.powered == false
                {
                    return 7993;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == true
                {
                    return 7994;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == true
                    && data.powered == false
                {
                    return 7995;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == true
                {
                    return 7996;
                }
                if data.facing == Facing::East
                    && data.half == Half::Lower
                    && data.hinge == Hinge::Right
                    && data.open == false
                    && data.powered == false
                {
                    return 7997;
                }
            }
            Block::EndRod(data) => {
                if data.facing == Facing::North {
                    return 7998;
                }
                if data.facing == Facing::East {
                    return 7999;
                }
                if data.facing == Facing::South {
                    return 8000;
                }
                if data.facing == Facing::West {
                    return 8001;
                }
                if data.facing == Facing::Up {
                    return 8002;
                }
                if data.facing == Facing::Down {
                    return 8003;
                }
            }
            Block::ChorusPlant(data) => {
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8004;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8005;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8006;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8007;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8008;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8009;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8010;
                }
                if data.down == true
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8011;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8012;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8013;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8014;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8015;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8016;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8017;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8018;
                }
                if data.down == true
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8019;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8020;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8021;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8022;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8023;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8024;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8025;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8026;
                }
                if data.down == true
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8027;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8028;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8029;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8030;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8031;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8032;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8033;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8034;
                }
                if data.down == true
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8035;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8036;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8037;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8038;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8039;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8040;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8041;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8042;
                }
                if data.down == false
                    && data.east == true
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8043;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8044;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8045;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8046;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8047;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8048;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8049;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8050;
                }
                if data.down == false
                    && data.east == true
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8051;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8052;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8053;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8054;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8055;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8056;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8057;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8058;
                }
                if data.down == false
                    && data.east == false
                    && data.north == true
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8059;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == true
                {
                    return 8060;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == true
                    && data.west == false
                {
                    return 8061;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == true
                {
                    return 8062;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == true
                    && data.up == false
                    && data.west == false
                {
                    return 8063;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == true
                {
                    return 8064;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == true
                    && data.west == false
                {
                    return 8065;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == true
                {
                    return 8066;
                }
                if data.down == false
                    && data.east == false
                    && data.north == false
                    && data.south == false
                    && data.up == false
                    && data.west == false
                {
                    return 8067;
                }
            }
            Block::ChorusFlower(data) => {
                if data.age == 0 {
                    return 8068;
                }
                if data.age == 1 {
                    return 8069;
                }
                if data.age == 2 {
                    return 8070;
                }
                if data.age == 3 {
                    return 8071;
                }
                if data.age == 4 {
                    return 8072;
                }
                if data.age == 5 {
                    return 8073;
                }
            }
            Block::PurpurBlock => return 8074,
            Block::PurpurPillar(data) => {
                if data.axis == Axis::X {
                    return 8075;
                }
                if data.axis == Axis::Y {
                    return 8076;
                }
                if data.axis == Axis::Z {
                    return 8077;
                }
            }
            Block::PurpurStairs(data) => {
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8078;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8079;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8080;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8081;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8082;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8083;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8084;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8085;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8086;
                }
                if data.facing == Facing::North
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8087;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8088;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8089;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8090;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8091;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8092;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8093;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8094;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8095;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8096;
                }
                if data.facing == Facing::North
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8097;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8098;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8099;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8100;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8101;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8102;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8103;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8104;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8105;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8106;
                }
                if data.facing == Facing::South
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8107;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8108;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8109;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8110;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8111;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8112;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8113;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8114;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8115;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8116;
                }
                if data.facing == Facing::South
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8117;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8118;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8119;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8120;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8121;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8122;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8123;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8124;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8125;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8126;
                }
                if data.facing == Facing::West
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8127;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8128;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8129;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8130;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8131;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8132;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8133;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8134;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8135;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8136;
                }
                if data.facing == Facing::West
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8137;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8138;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8139;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8140;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8141;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8142;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8143;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8144;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8145;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8146;
                }
                if data.facing == Facing::East
                    && data.half == Half::Top
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8147;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == true
                {
                    return 8148;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::Straight
                    && data.waterlogged == false
                {
                    return 8149;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == true
                {
                    return 8150;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerLeft
                    && data.waterlogged == false
                {
                    return 8151;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == true
                {
                    return 8152;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::InnerRight
                    && data.waterlogged == false
                {
                    return 8153;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == true
                {
                    return 8154;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterLeft
                    && data.waterlogged == false
                {
                    return 8155;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == true
                {
                    return 8156;
                }
                if data.facing == Facing::East
                    && data.half == Half::Bottom
                    && data.shape == Shape::OuterRight
                    && data.waterlogged == false
                {
                    return 8157;
                }
            }
            Block::EndStoneBricks => return 8158,
            Block::Beetroots(data) => {
                if data.age == 0 {
                    return 8159;
                }
                if data.age == 1 {
                    return 8160;
                }
                if data.age == 2 {
                    return 8161;
                }
                if data.age == 3 {
                    return 8162;
                }
            }
            Block::GrassPath => return 8163,
            Block::EndGateway => return 8164,
            Block::RepeatingCommandBlock(data) => {
                if data.conditional == true && data.facing == Facing::North {
                    return 8165;
                }
                if data.conditional == true && data.facing == Facing::East {
                    return 8166;
                }
                if data.conditional == true && data.facing == Facing::South {
                    return 8167;
                }
                if data.conditional == true && data.facing == Facing::West {
                    return 8168;
                }
                if data.conditional == true && data.facing == Facing::Up {
                    return 8169;
                }
                if data.conditional == true && data.facing == Facing::Down {
                    return 8170;
                }
                if data.conditional == false && data.facing == Facing::North {
                    return 8171;
                }
                if data.conditional == false && data.facing == Facing::East {
                    return 8172;
                }
                if data.conditional == false && data.facing == Facing::South {
                    return 8173;
                }
                if data.conditional == false && data.facing == Facing::West {
                    return 8174;
                }
                if data.conditional == false && data.facing == Facing::Up {
                    return 8175;
                }
                if data.conditional == false && data.facing == Facing::Down {
                    return 8176;
                }
            }
            Block::ChainCommandBlock(data) => {
                if data.conditional == true && data.facing == Facing::North {
                    return 8177;
                }
                if data.conditional == true && data.facing == Facing::East {
                    return 8178;
                }
                if data.conditional == true && data.facing == Facing::South {
                    return 8179;
                }
                if data.conditional == true && data.facing == Facing::West {
                    return 8180;
                }
                if data.conditional == true && data.facing == Facing::Up {
                    return 8181;
                }
                if data.conditional == true && data.facing == Facing::Down {
                    return 8182;
                }
                if data.conditional == false && data.facing == Facing::North {
                    return 8183;
                }
                if data.conditional == false && data.facing == Facing::East {
                    return 8184;
                }
                if data.conditional == false && data.facing == Facing::South {
                    return 8185;
                }
                if data.conditional == false && data.facing == Facing::West {
                    return 8186;
                }
                if data.conditional == false && data.facing == Facing::Up {
                    return 8187;
                }
                if data.conditional == false && data.facing == Facing::Down {
                    return 8188;
                }
            }
            Block::FrostedIce(data) => {
                if data.age == 0 {
                    return 8189;
                }
                if data.age == 1 {
                    return 8190;
                }
                if data.age == 2 {
                    return 8191;
                }
                if data.age == 3 {
                    return 8192;
                }
            }
            Block::MagmaBlock => return 8193,
            Block::NetherWartBlock => return 8194,
            Block::RedNetherBricks => return 8195,
            Block::BoneBlock(data) => {
                if data.axis == Axis::X {
                    return 8196;
                }
                if data.axis == Axis::Y {
                    return 8197;
                }
                if data.axis == Axis::Z {
                    return 8198;
                }
            }
            Block::StructureVoid => return 8199,
            Block::Observer(data) => {
                if data.facing == Facing::North && data.powered == true {
                    return 8200;
                }
                if data.facing == Facing::North && data.powered == false {
                    return 8201;
                }
                if data.facing == Facing::East && data.powered == true {
                    return 8202;
                }
                if data.facing == Facing::East && data.powered == false {
                    return 8203;
                }
                if data.facing == Facing::South && data.powered == true {
                    return 8204;
                }
                if data.facing == Facing::South && data.powered == false {
                    return 8205;
                }
                if data.facing == Facing::West && data.powered == true {
                    return 8206;
                }
                if data.facing == Facing::West && data.powered == false {
                    return 8207;
                }
                if data.facing == Facing::Up && data.powered == true {
                    return 8208;
                }
                if data.facing == Facing::Up && data.powered == false {
                    return 8209;
                }
                if data.facing == Facing::Down && data.powered == true {
                    return 8210;
                }
                if data.facing == Facing::Down && data.powered == false {
                    return 8211;
                }
            }
            Block::ShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8212;
                }
                if data.facing == Facing::East {
                    return 8213;
                }
                if data.facing == Facing::South {
                    return 8214;
                }
                if data.facing == Facing::West {
                    return 8215;
                }
                if data.facing == Facing::Up {
                    return 8216;
                }
                if data.facing == Facing::Down {
                    return 8217;
                }
            }
            Block::WhiteShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8218;
                }
                if data.facing == Facing::East {
                    return 8219;
                }
                if data.facing == Facing::South {
                    return 8220;
                }
                if data.facing == Facing::West {
                    return 8221;
                }
                if data.facing == Facing::Up {
                    return 8222;
                }
                if data.facing == Facing::Down {
                    return 8223;
                }
            }
            Block::OrangeShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8224;
                }
                if data.facing == Facing::East {
                    return 8225;
                }
                if data.facing == Facing::South {
                    return 8226;
                }
                if data.facing == Facing::West {
                    return 8227;
                }
                if data.facing == Facing::Up {
                    return 8228;
                }
                if data.facing == Facing::Down {
                    return 8229;
                }
            }
            Block::MagentaShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8230;
                }
                if data.facing == Facing::East {
                    return 8231;
                }
                if data.facing == Facing::South {
                    return 8232;
                }
                if data.facing == Facing::West {
                    return 8233;
                }
                if data.facing == Facing::Up {
                    return 8234;
                }
                if data.facing == Facing::Down {
                    return 8235;
                }
            }
            Block::LightBlueShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8236;
                }
                if data.facing == Facing::East {
                    return 8237;
                }
                if data.facing == Facing::South {
                    return 8238;
                }
                if data.facing == Facing::West {
                    return 8239;
                }
                if data.facing == Facing::Up {
                    return 8240;
                }
                if data.facing == Facing::Down {
                    return 8241;
                }
            }
            Block::YellowShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8242;
                }
                if data.facing == Facing::East {
                    return 8243;
                }
                if data.facing == Facing::South {
                    return 8244;
                }
                if data.facing == Facing::West {
                    return 8245;
                }
                if data.facing == Facing::Up {
                    return 8246;
                }
                if data.facing == Facing::Down {
                    return 8247;
                }
            }
            Block::LimeShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8248;
                }
                if data.facing == Facing::East {
                    return 8249;
                }
                if data.facing == Facing::South {
                    return 8250;
                }
                if data.facing == Facing::West {
                    return 8251;
                }
                if data.facing == Facing::Up {
                    return 8252;
                }
                if data.facing == Facing::Down {
                    return 8253;
                }
            }
            Block::PinkShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8254;
                }
                if data.facing == Facing::East {
                    return 8255;
                }
                if data.facing == Facing::South {
                    return 8256;
                }
                if data.facing == Facing::West {
                    return 8257;
                }
                if data.facing == Facing::Up {
                    return 8258;
                }
                if data.facing == Facing::Down {
                    return 8259;
                }
            }
            Block::GrayShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8260;
                }
                if data.facing == Facing::East {
                    return 8261;
                }
                if data.facing == Facing::South {
                    return 8262;
                }
                if data.facing == Facing::West {
                    return 8263;
                }
                if data.facing == Facing::Up {
                    return 8264;
                }
                if data.facing == Facing::Down {
                    return 8265;
                }
            }
            Block::LightGrayShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8266;
                }
                if data.facing == Facing::East {
                    return 8267;
                }
                if data.facing == Facing::South {
                    return 8268;
                }
                if data.facing == Facing::West {
                    return 8269;
                }
                if data.facing == Facing::Up {
                    return 8270;
                }
                if data.facing == Facing::Down {
                    return 8271;
                }
            }
            Block::CyanShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8272;
                }
                if data.facing == Facing::East {
                    return 8273;
                }
                if data.facing == Facing::South {
                    return 8274;
                }
                if data.facing == Facing::West {
                    return 8275;
                }
                if data.facing == Facing::Up {
                    return 8276;
                }
                if data.facing == Facing::Down {
                    return 8277;
                }
            }
            Block::PurpleShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8278;
                }
                if data.facing == Facing::East {
                    return 8279;
                }
                if data.facing == Facing::South {
                    return 8280;
                }
                if data.facing == Facing::West {
                    return 8281;
                }
                if data.facing == Facing::Up {
                    return 8282;
                }
                if data.facing == Facing::Down {
                    return 8283;
                }
            }
            Block::BlueShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8284;
                }
                if data.facing == Facing::East {
                    return 8285;
                }
                if data.facing == Facing::South {
                    return 8286;
                }
                if data.facing == Facing::West {
                    return 8287;
                }
                if data.facing == Facing::Up {
                    return 8288;
                }
                if data.facing == Facing::Down {
                    return 8289;
                }
            }
            Block::BrownShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8290;
                }
                if data.facing == Facing::East {
                    return 8291;
                }
                if data.facing == Facing::South {
                    return 8292;
                }
                if data.facing == Facing::West {
                    return 8293;
                }
                if data.facing == Facing::Up {
                    return 8294;
                }
                if data.facing == Facing::Down {
                    return 8295;
                }
            }
            Block::GreenShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8296;
                }
                if data.facing == Facing::East {
                    return 8297;
                }
                if data.facing == Facing::South {
                    return 8298;
                }
                if data.facing == Facing::West {
                    return 8299;
                }
                if data.facing == Facing::Up {
                    return 8300;
                }
                if data.facing == Facing::Down {
                    return 8301;
                }
            }
            Block::RedShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8302;
                }
                if data.facing == Facing::East {
                    return 8303;
                }
                if data.facing == Facing::South {
                    return 8304;
                }
                if data.facing == Facing::West {
                    return 8305;
                }
                if data.facing == Facing::Up {
                    return 8306;
                }
                if data.facing == Facing::Down {
                    return 8307;
                }
            }
            Block::BlackShulkerBox(data) => {
                if data.facing == Facing::North {
                    return 8308;
                }
                if data.facing == Facing::East {
                    return 8309;
                }
                if data.facing == Facing::South {
                    return 8310;
                }
                if data.facing == Facing::West {
                    return 8311;
                }
                if data.facing == Facing::Up {
                    return 8312;
                }
                if data.facing == Facing::Down {
                    return 8313;
                }
            }
            Block::WhiteGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8314;
                }
                if data.facing == Facing::South {
                    return 8315;
                }
                if data.facing == Facing::West {
                    return 8316;
                }
                if data.facing == Facing::East {
                    return 8317;
                }
            }
            Block::OrangeGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8318;
                }
                if data.facing == Facing::South {
                    return 8319;
                }
                if data.facing == Facing::West {
                    return 8320;
                }
                if data.facing == Facing::East {
                    return 8321;
                }
            }
            Block::MagentaGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8322;
                }
                if data.facing == Facing::South {
                    return 8323;
                }
                if data.facing == Facing::West {
                    return 8324;
                }
                if data.facing == Facing::East {
                    return 8325;
                }
            }
            Block::LightBlueGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8326;
                }
                if data.facing == Facing::South {
                    return 8327;
                }
                if data.facing == Facing::West {
                    return 8328;
                }
                if data.facing == Facing::East {
                    return 8329;
                }
            }
            Block::YellowGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8330;
                }
                if data.facing == Facing::South {
                    return 8331;
                }
                if data.facing == Facing::West {
                    return 8332;
                }
                if data.facing == Facing::East {
                    return 8333;
                }
            }
            Block::LimeGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8334;
                }
                if data.facing == Facing::South {
                    return 8335;
                }
                if data.facing == Facing::West {
                    return 8336;
                }
                if data.facing == Facing::East {
                    return 8337;
                }
            }
            Block::PinkGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8338;
                }
                if data.facing == Facing::South {
                    return 8339;
                }
                if data.facing == Facing::West {
                    return 8340;
                }
                if data.facing == Facing::East {
                    return 8341;
                }
            }
            Block::GrayGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8342;
                }
                if data.facing == Facing::South {
                    return 8343;
                }
                if data.facing == Facing::West {
                    return 8344;
                }
                if data.facing == Facing::East {
                    return 8345;
                }
            }
            Block::LightGrayGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8346;
                }
                if data.facing == Facing::South {
                    return 8347;
                }
                if data.facing == Facing::West {
                    return 8348;
                }
                if data.facing == Facing::East {
                    return 8349;
                }
            }
            Block::CyanGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8350;
                }
                if data.facing == Facing::South {
                    return 8351;
                }
                if data.facing == Facing::West {
                    return 8352;
                }
                if data.facing == Facing::East {
                    return 8353;
                }
            }
            Block::PurpleGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8354;
                }
                if data.facing == Facing::South {
                    return 8355;
                }
                if data.facing == Facing::West {
                    return 8356;
                }
                if data.facing == Facing::East {
                    return 8357;
                }
            }
            Block::BlueGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8358;
                }
                if data.facing == Facing::South {
                    return 8359;
                }
                if data.facing == Facing::West {
                    return 8360;
                }
                if data.facing == Facing::East {
                    return 8361;
                }
            }
            Block::BrownGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8362;
                }
                if data.facing == Facing::South {
                    return 8363;
                }
                if data.facing == Facing::West {
                    return 8364;
                }
                if data.facing == Facing::East {
                    return 8365;
                }
            }
            Block::GreenGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8366;
                }
                if data.facing == Facing::South {
                    return 8367;
                }
                if data.facing == Facing::West {
                    return 8368;
                }
                if data.facing == Facing::East {
                    return 8369;
                }
            }
            Block::RedGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8370;
                }
                if data.facing == Facing::South {
                    return 8371;
                }
                if data.facing == Facing::West {
                    return 8372;
                }
                if data.facing == Facing::East {
                    return 8373;
                }
            }
            Block::BlackGlazedTerracotta(data) => {
                if data.facing == Facing::North {
                    return 8374;
                }
                if data.facing == Facing::South {
                    return 8375;
                }
                if data.facing == Facing::West {
                    return 8376;
                }
                if data.facing == Facing::East {
                    return 8377;
                }
            }
            Block::WhiteConcrete => return 8378,
            Block::OrangeConcrete => return 8379,
            Block::MagentaConcrete => return 8380,
            Block::LightBlueConcrete => return 8381,
            Block::YellowConcrete => return 8382,
            Block::LimeConcrete => return 8383,
            Block::PinkConcrete => return 8384,
            Block::GrayConcrete => return 8385,
            Block::LightGrayConcrete => return 8386,
            Block::CyanConcrete => return 8387,
            Block::PurpleConcrete => return 8388,
            Block::BlueConcrete => return 8389,
            Block::BrownConcrete => return 8390,
            Block::GreenConcrete => return 8391,
            Block::RedConcrete => return 8392,
            Block::BlackConcrete => return 8393,
            Block::WhiteConcretePowder => return 8394,
            Block::OrangeConcretePowder => return 8395,
            Block::MagentaConcretePowder => return 8396,
            Block::LightBlueConcretePowder => return 8397,
            Block::YellowConcretePowder => return 8398,
            Block::LimeConcretePowder => return 8399,
            Block::PinkConcretePowder => return 8400,
            Block::GrayConcretePowder => return 8401,
            Block::LightGrayConcretePowder => return 8402,
            Block::CyanConcretePowder => return 8403,
            Block::PurpleConcretePowder => return 8404,
            Block::BlueConcretePowder => return 8405,
            Block::BrownConcretePowder => return 8406,
            Block::GreenConcretePowder => return 8407,
            Block::RedConcretePowder => return 8408,
            Block::BlackConcretePowder => return 8409,
            Block::Kelp(data) => {
                if data.age == 0 {
                    return 8410;
                }
                if data.age == 1 {
                    return 8411;
                }
                if data.age == 2 {
                    return 8412;
                }
                if data.age == 3 {
                    return 8413;
                }
                if data.age == 4 {
                    return 8414;
                }
                if data.age == 5 {
                    return 8415;
                }
                if data.age == 6 {
                    return 8416;
                }
                if data.age == 7 {
                    return 8417;
                }
                if data.age == 8 {
                    return 8418;
                }
                if data.age == 9 {
                    return 8419;
                }
                if data.age == 10 {
                    return 8420;
                }
                if data.age == 11 {
                    return 8421;
                }
                if data.age == 12 {
                    return 8422;
                }
                if data.age == 13 {
                    return 8423;
                }
                if data.age == 14 {
                    return 8424;
                }
                if data.age == 15 {
                    return 8425;
                }
                if data.age == 16 {
                    return 8426;
                }
                if data.age == 17 {
                    return 8427;
                }
                if data.age == 18 {
                    return 8428;
                }
                if data.age == 19 {
                    return 8429;
                }
                if data.age == 20 {
                    return 8430;
                }
                if data.age == 21 {
                    return 8431;
                }
                if data.age == 22 {
                    return 8432;
                }
                if data.age == 23 {
                    return 8433;
                }
                if data.age == 24 {
                    return 8434;
                }
                if data.age == 25 {
                    return 8435;
                }
            }
            Block::KelpPlant => return 8436,
            Block::DriedKelpBlock => return 8437,
            Block::TurtleEgg(data) => {
                if data.eggs == 1 && data.hatch == 0 {
                    return 8438;
                }
                if data.eggs == 1 && data.hatch == 1 {
                    return 8439;
                }
                if data.eggs == 1 && data.hatch == 2 {
                    return 8440;
                }
                if data.eggs == 2 && data.hatch == 0 {
                    return 8441;
                }
                if data.eggs == 2 && data.hatch == 1 {
                    return 8442;
                }
                if data.eggs == 2 && data.hatch == 2 {
                    return 8443;
                }
                if data.eggs == 3 && data.hatch == 0 {
                    return 8444;
                }
                if data.eggs == 3 && data.hatch == 1 {
                    return 8445;
                }
                if data.eggs == 3 && data.hatch == 2 {
                    return 8446;
                }
                if data.eggs == 4 && data.hatch == 0 {
                    return 8447;
                }
                if data.eggs == 4 && data.hatch == 1 {
                    return 8448;
                }
                if data.eggs == 4 && data.hatch == 2 {
                    return 8449;
                }
            }
            Block::DeadTubeCoralBlock => return 8450,
            Block::DeadBrainCoralBlock => return 8451,
            Block::DeadBubbleCoralBlock => return 8452,
            Block::DeadFireCoralBlock => return 8453,
            Block::DeadHornCoralBlock => return 8454,
            Block::TubeCoralBlock => return 8455,
            Block::BrainCoralBlock => return 8456,
            Block::BubbleCoralBlock => return 8457,
            Block::FireCoralBlock => return 8458,
            Block::HornCoralBlock => return 8459,
            Block::DeadTubeCoral(data) => {
                if data.waterlogged == true {
                    return 8460;
                }
                if data.waterlogged == false {
                    return 8461;
                }
            }
            Block::DeadBrainCoral(data) => {
                if data.waterlogged == true {
                    return 8462;
                }
                if data.waterlogged == false {
                    return 8463;
                }
            }
            Block::DeadBubbleCoral(data) => {
                if data.waterlogged == true {
                    return 8464;
                }
                if data.waterlogged == false {
                    return 8465;
                }
            }
            Block::DeadFireCoral(data) => {
                if data.waterlogged == true {
                    return 8466;
                }
                if data.waterlogged == false {
                    return 8467;
                }
            }
            Block::DeadHornCoral(data) => {
                if data.waterlogged == true {
                    return 8468;
                }
                if data.waterlogged == false {
                    return 8469;
                }
            }
            Block::TubeCoral(data) => {
                if data.waterlogged == true {
                    return 8470;
                }
                if data.waterlogged == false {
                    return 8471;
                }
            }
            Block::BrainCoral(data) => {
                if data.waterlogged == true {
                    return 8472;
                }
                if data.waterlogged == false {
                    return 8473;
                }
            }
            Block::BubbleCoral(data) => {
                if data.waterlogged == true {
                    return 8474;
                }
                if data.waterlogged == false {
                    return 8475;
                }
            }
            Block::FireCoral(data) => {
                if data.waterlogged == true {
                    return 8476;
                }
                if data.waterlogged == false {
                    return 8477;
                }
            }
            Block::HornCoral(data) => {
                if data.waterlogged == true {
                    return 8478;
                }
                if data.waterlogged == false {
                    return 8479;
                }
            }
            Block::DeadTubeCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8480;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8481;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8482;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8483;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8484;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8485;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8486;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8487;
                }
            }
            Block::DeadBrainCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8488;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8489;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8490;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8491;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8492;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8493;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8494;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8495;
                }
            }
            Block::DeadBubbleCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8496;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8497;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8498;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8499;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8500;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8501;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8502;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8503;
                }
            }
            Block::DeadFireCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8504;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8505;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8506;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8507;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8508;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8509;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8510;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8511;
                }
            }
            Block::DeadHornCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8512;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8513;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8514;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8515;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8516;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8517;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8518;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8519;
                }
            }
            Block::TubeCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8520;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8521;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8522;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8523;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8524;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8525;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8526;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8527;
                }
            }
            Block::BrainCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8528;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8529;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8530;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8531;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8532;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8533;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8534;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8535;
                }
            }
            Block::BubbleCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8536;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8537;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8538;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8539;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8540;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8541;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8542;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8543;
                }
            }
            Block::FireCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8544;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8545;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8546;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8547;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8548;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8549;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8550;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8551;
                }
            }
            Block::HornCoralWallFan(data) => {
                if data.facing == Facing::North && data.waterlogged == true {
                    return 8552;
                }
                if data.facing == Facing::North && data.waterlogged == false {
                    return 8553;
                }
                if data.facing == Facing::South && data.waterlogged == true {
                    return 8554;
                }
                if data.facing == Facing::South && data.waterlogged == false {
                    return 8555;
                }
                if data.facing == Facing::West && data.waterlogged == true {
                    return 8556;
                }
                if data.facing == Facing::West && data.waterlogged == false {
                    return 8557;
                }
                if data.facing == Facing::East && data.waterlogged == true {
                    return 8558;
                }
                if data.facing == Facing::East && data.waterlogged == false {
                    return 8559;
                }
            }
            Block::DeadTubeCoralFan(data) => {
                if data.waterlogged == true {
                    return 8560;
                }
                if data.waterlogged == false {
                    return 8561;
                }
            }
            Block::DeadBrainCoralFan(data) => {
                if data.waterlogged == true {
                    return 8562;
                }
                if data.waterlogged == false {
                    return 8563;
                }
            }
            Block::DeadBubbleCoralFan(data) => {
                if data.waterlogged == true {
                    return 8564;
                }
                if data.waterlogged == false {
                    return 8565;
                }
            }
            Block::DeadFireCoralFan(data) => {
                if data.waterlogged == true {
                    return 8566;
                }
                if data.waterlogged == false {
                    return 8567;
                }
            }
            Block::DeadHornCoralFan(data) => {
                if data.waterlogged == true {
                    return 8568;
                }
                if data.waterlogged == false {
                    return 8569;
                }
            }
            Block::TubeCoralFan(data) => {
                if data.waterlogged == true {
                    return 8570;
                }
                if data.waterlogged == false {
                    return 8571;
                }
            }
            Block::BrainCoralFan(data) => {
                if data.waterlogged == true {
                    return 8572;
                }
                if data.waterlogged == false {
                    return 8573;
                }
            }
            Block::BubbleCoralFan(data) => {
                if data.waterlogged == true {
                    return 8574;
                }
                if data.waterlogged == false {
                    return 8575;
                }
            }
            Block::FireCoralFan(data) => {
                if data.waterlogged == true {
                    return 8576;
                }
                if data.waterlogged == false {
                    return 8577;
                }
            }
            Block::HornCoralFan(data) => {
                if data.waterlogged == true {
                    return 8578;
                }
                if data.waterlogged == false {
                    return 8579;
                }
            }
            Block::SeaPickle(data) => {
                if data.pickles == 1 && data.waterlogged == true {
                    return 8580;
                }
                if data.pickles == 1 && data.waterlogged == false {
                    return 8581;
                }
                if data.pickles == 2 && data.waterlogged == true {
                    return 8582;
                }
                if data.pickles == 2 && data.waterlogged == false {
                    return 8583;
                }
                if data.pickles == 3 && data.waterlogged == true {
                    return 8584;
                }
                if data.pickles == 3 && data.waterlogged == false {
                    return 8585;
                }
                if data.pickles == 4 && data.waterlogged == true {
                    return 8586;
                }
                if data.pickles == 4 && data.waterlogged == false {
                    return 8587;
                }
            }
            Block::BlueIce => return 8588,
            Block::Conduit(data) => {
                if data.waterlogged == true {
                    return 8589;
                }
                if data.waterlogged == false {
                    return 8590;
                }
            }
            Block::VoidAir => return 8591,
            Block::CaveAir => return 8592,
            Block::BubbleColumn(data) => {
                if data.drag == true {
                    return 8593;
                }
                if data.drag == false {
                    return 8594;
                }
            }
            Block::StructureBlock(data) => {
                if data.mode == StructureBlockMode::Save {
                    return 8595;
                }
                if data.mode == StructureBlockMode::Load {
                    return 8596;
                }
                if data.mode == StructureBlockMode::Corner {
                    return 8597;
                }
                if data.mode == StructureBlockMode::Data {
                    return 8598;
                }
            }
            _ => panic!("Invalid block state {:?}", self),
        }
        panic!("Invalid block state {:?}", self);
    }
}
