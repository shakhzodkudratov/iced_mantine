// https://github.com/mantinedev/mantine/blob/master/packages/%40mantine/core/src/core/MantineProvider/default-colors.ts
// https://mantine.dev/theming/colors/#default-colors

use iced::{color, Color};

pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0};

/// #FFFFFF
pub const WHITE: Color = color!(0xffffff);
/// #000000
pub const BLACK: Color = color!(0x000000);

/// #C9C9C9
pub const DARK_0: Color = color!(0xC9C9C9);
/// #b8b8b8
pub const DARK_1: Color = color!(0xb8b8b8);
/// #828282
pub const DARK_2: Color = color!(0x828282);
/// #696969
pub const DARK_3: Color = color!(0x696969);
/// #424242
pub const DARK_4: Color = color!(0x424242);
/// #3b3b3b
pub const DARK_5: Color = color!(0x3b3b3b);
/// #2e2e2e
pub const DARK_6: Color = color!(0x2e2e2e);
/// #242424
pub const DARK_7: Color = color!(0x242424);
/// #1f1f1f
pub const DARK_8: Color = color!(0x1f1f1f);
/// #141414
pub const DARK_9: Color = color!(0x141414);

/// #f8f9fa
pub const GRAY_0: Color = color!(0xf8f9fa);
/// #f1f3f5
pub const GRAY_1: Color = color!(0xf1f3f5);
/// #e9ecef
pub const GRAY_2: Color = color!(0xe9ecef);
/// #dee2e6
pub const GRAY_3: Color = color!(0xdee2e6);
/// #ced4da
pub const GRAY_4: Color = color!(0xced4da);
/// #adb5bd
pub const GRAY_5: Color = color!(0xadb5bd);
/// #868e96
pub const GRAY_6: Color = color!(0x868e96);
/// #495057
pub const GRAY_7: Color = color!(0x495057);
/// #343a40
pub const GRAY_8: Color = color!(0x343a40);
/// #212529
pub const GRAY_9: Color = color!(0x212529);

/// #fff5f5
pub const RED_0: Color = color!(0xfff5f5);
/// #ffe3e3
pub const RED_1: Color = color!(0xffe3e3);
/// #ffc9c9
pub const RED_2: Color = color!(0xffc9c9);
/// #ffa8a8
pub const RED_3: Color = color!(0xffa8a8);
/// #ff8787
pub const RED_4: Color = color!(0xff8787);
/// #ff6b6b
pub const RED_5: Color = color!(0xff6b6b);
/// #fa5252
pub const RED_6: Color = color!(0xfa5252);
/// #f03e3e
pub const RED_7: Color = color!(0xf03e3e);
/// #e03131
pub const RED_8: Color = color!(0xe03131);
/// #c92a2a
pub const RED_9: Color = color!(0xc92a2a);

/// #fff0f6
pub const PINK_0: Color = color!(0xfff0f6);
/// #ffdeeb
pub const PINK_1: Color = color!(0xffdeeb);
/// #fcc2d7
pub const PINK_2: Color = color!(0xfcc2d7);
/// #faa2c1
pub const PINK_3: Color = color!(0xfaa2c1);
/// #f783ac
pub const PINK_4: Color = color!(0xf783ac);
/// #f06595
pub const PINK_5: Color = color!(0xf06595);
/// #e64980
pub const PINK_6: Color = color!(0xe64980);
/// #d6336c
pub const PINK_7: Color = color!(0xd6336c);
/// #c2255c
pub const PINK_8: Color = color!(0xc2255c);
/// #a61e4d
pub const PINK_9: Color = color!(0xa61e4d);

/// #f8f0fc
pub const GRAPE_0: Color = color!(0xf8f0fc);
/// #f3d9fa
pub const GRAPE_1: Color = color!(0xf3d9fa);
/// #eebefa
pub const GRAPE_2: Color = color!(0xeebefa);
/// #e599f7
pub const GRAPE_3: Color = color!(0xe599f7);
/// #da77f2
pub const GRAPE_4: Color = color!(0xda77f2);
/// #cc5de8
pub const GRAPE_5: Color = color!(0xcc5de8);
/// #be4bdb
pub const GRAPE_6: Color = color!(0xbe4bdb);
/// #ae3ec9
pub const GRAPE_7: Color = color!(0xae3ec9);
/// #9c36b5
pub const GRAPE_8: Color = color!(0x9c36b5);
/// #862e9c
pub const GRAPE_9: Color = color!(0x862e9c);

/// #f3f0ff
pub const VIOLET_0: Color = color!(0xf3f0ff);
/// #e5dbff
pub const VIOLET_1: Color = color!(0xe5dbff);
/// #d0bfff
pub const VIOLET_2: Color = color!(0xd0bfff);
/// #b197fc
pub const VIOLET_3: Color = color!(0xb197fc);
/// #9775fa
pub const VIOLET_4: Color = color!(0x9775fa);
/// #845ef7
pub const VIOLET_5: Color = color!(0x845ef7);
/// #7950f2
pub const VIOLET_6: Color = color!(0x7950f2);
/// #7048e8
pub const VIOLET_7: Color = color!(0x7048e8);
/// #6741d9
pub const VIOLET_8: Color = color!(0x6741d9);
/// #5f3dc4
pub const VIOLET_9: Color = color!(0x5f3dc4);

/// #edf2ff
pub const INDIGO_0: Color = color!(0xedf2ff);
/// #dbe4ff
pub const INDIGO_1: Color = color!(0xdbe4ff);
/// #bac8ff
pub const INDIGO_2: Color = color!(0xbac8ff);
/// #91a7ff
pub const INDIGO_3: Color = color!(0x91a7ff);
/// #748ffc
pub const INDIGO_4: Color = color!(0x748ffc);
/// #5c7cfa
pub const INDIGO_5: Color = color!(0x5c7cfa);
/// #4c6ef5
pub const INDIGO_6: Color = color!(0x4c6ef5);
/// #4263eb
pub const INDIGO_7: Color = color!(0x4263eb);
/// #3b5bdb
pub const INDIGO_8: Color = color!(0x3b5bdb);
/// #364fc7
pub const INDIGO_9: Color = color!(0x364fc7);

/// #e7f5ff
pub const BLUE_0: Color = color!(0xe7f5ff);
/// #d0ebff
pub const BLUE_1: Color = color!(0xd0ebff);
/// #a5d8ff
pub const BLUE_2: Color = color!(0xa5d8ff);
/// #74c0fc
pub const BLUE_3: Color = color!(0x74c0fc);
/// #4dabf7
pub const BLUE_4: Color = color!(0x4dabf7);
/// #339af0
pub const BLUE_5: Color = color!(0x339af0);
/// #228be6
pub const BLUE_6: Color = color!(0x228be6);
/// #1c7ed6
pub const BLUE_7: Color = color!(0x1c7ed6);
/// #1971c2
pub const BLUE_8: Color = color!(0x1971c2);
/// #1864ab
pub const BLUE_9: Color = color!(0x1864ab);

/// #e3fafc
pub const CYAN_0: Color = color!(0xe3fafc);
/// #c5f6fa
pub const CYAN_1: Color = color!(0xc5f6fa);
/// #99e9f2
pub const CYAN_2: Color = color!(0x99e9f2);
/// #66d9e8
pub const CYAN_3: Color = color!(0x66d9e8);
/// #3bc9db
pub const CYAN_4: Color = color!(0x3bc9db);
/// #22b8cf
pub const CYAN_5: Color = color!(0x22b8cf);
/// #15aabf
pub const CYAN_6: Color = color!(0x15aabf);
/// #1098ad
pub const CYAN_7: Color = color!(0x1098ad);
/// #0c8599
pub const CYAN_8: Color = color!(0x0c8599);
/// #0b7285
pub const CYAN_9: Color = color!(0x0b7285);

/// #e6fcf5
pub const TEAL_0: Color = color!(0xe6fcf5);
/// #c3fae8
pub const TEAL_1: Color = color!(0xc3fae8);
/// #96f2d7
pub const TEAL_2: Color = color!(0x96f2d7);
/// #63e6be
pub const TEAL_3: Color = color!(0x63e6be);
/// #38d9a9
pub const TEAL_4: Color = color!(0x38d9a9);
/// #20c997
pub const TEAL_5: Color = color!(0x20c997);
/// #12b886
pub const TEAL_6: Color = color!(0x12b886);
/// #0ca678
pub const TEAL_7: Color = color!(0x0ca678);
/// #099268
pub const TEAL_8: Color = color!(0x099268);
/// #087f5b
pub const TEAL_9: Color = color!(0x087f5b);

/// #ebfbee
pub const GREEN_0: Color = color!(0xebfbee);
/// #d3f9d8
pub const GREEN_1: Color = color!(0xd3f9d8);
/// #b2f2bb
pub const GREEN_2: Color = color!(0xb2f2bb);
/// #8ce99a
pub const GREEN_3: Color = color!(0x8ce99a);
/// #69db7c
pub const GREEN_4: Color = color!(0x69db7c);
/// #51cf66
pub const GREEN_5: Color = color!(0x51cf66);
/// #40c057
pub const GREEN_6: Color = color!(0x40c057);
/// #37b24d
pub const GREEN_7: Color = color!(0x37b24d);
/// #2f9e44
pub const GREEN_8: Color = color!(0x2f9e44);
/// #2b8a3e
pub const GREEN_9: Color = color!(0x2b8a3e);

/// #f4fce3
pub const LIME_0: Color = color!(0xf4fce3);
/// #e9fac8
pub const LIME_1: Color = color!(0xe9fac8);
/// #d8f5a2
pub const LIME_2: Color = color!(0xd8f5a2);
/// #c0eb75
pub const LIME_3: Color = color!(0xc0eb75);
/// #a9e34b
pub const LIME_4: Color = color!(0xa9e34b);
/// #94d82d
pub const LIME_5: Color = color!(0x94d82d);
/// #82c91e
pub const LIME_6: Color = color!(0x82c91e);
/// #74b816
pub const LIME_7: Color = color!(0x74b816);
/// #66a80f
pub const LIME_8: Color = color!(0x66a80f);
/// #5c940d
pub const LIME_9: Color = color!(0x5c940d);

/// #fff9db
pub const YELLOW_0: Color = color!(0xfff9db);
/// #fff3bf
pub const YELLOW_1: Color = color!(0xfff3bf);
/// #ffec99
pub const YELLOW_2: Color = color!(0xffec99);
/// #ffe066
pub const YELLOW_3: Color = color!(0xffe066);
/// #ffd43b
pub const YELLOW_4: Color = color!(0xffd43b);
/// #fcc419
pub const YELLOW_5: Color = color!(0xfcc419);
/// #fab005
pub const YELLOW_6: Color = color!(0xfab005);
/// #f59f00
pub const YELLOW_7: Color = color!(0xf59f00);
/// #f08c00
pub const YELLOW_8: Color = color!(0xf08c00);
/// #e67700
pub const YELLOW_9: Color = color!(0xe67700);

/// #fff4e6
pub const ORANGE_0: Color = color!(0xfff4e6);
/// #ffe8cc
pub const ORANGE_1: Color = color!(0xffe8cc);
/// #ffd8a8
pub const ORANGE_2: Color = color!(0xffd8a8);
/// #ffc078
pub const ORANGE_3: Color = color!(0xffc078);
/// #ffa94d
pub const ORANGE_4: Color = color!(0xffa94d);
/// #ff922b
pub const ORANGE_5: Color = color!(0xff922b);
/// #fd7e14
pub const ORANGE_6: Color = color!(0xfd7e14);
/// #f76707
pub const ORANGE_7: Color = color!(0xf76707);
/// #e8590c
pub const ORANGE_8: Color = color!(0xe8590c);
/// #d9480f
pub const ORANGE_9: Color = color!(0xd9480f);
