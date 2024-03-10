// Copyright 2022 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::download::BuiltinLottieProps;

// This content cannot be formatted by rustfmt because of the long strings, so it's in its own file
use super::LottieDownload;

pub(super) fn default_downloads() -> Vec<LottieDownload> {
    vec![
        LottieDownload {
            name: "Smile".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f600/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 37328,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Smile-with-big-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f603/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 48097,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Grin".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f604/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 48154,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Grinning".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f601/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 70876,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Laughing".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f606/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 59130,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Grin-sweat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f605/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 67751,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Joy".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f602/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 59124,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rofl".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f923/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 78237,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Loudly-crying".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 109511,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wink".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f609/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 27737,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kissing".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f617/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 29240,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kissing-smiling-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f619/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 13200,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kissing-closed-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f61a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 20588,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kissing-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f618/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 64904,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Heart-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f970/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58353,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Heart-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f60d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 42336,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Star-struck".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f929/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 46757,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Partying-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f973/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 67877,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Melting".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae0/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 155514,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Upside-down-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f643/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 12177,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Slightly-happy".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f642/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 24463,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Happy-cry".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f972/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 30188,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Holding-back-tears".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f979/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 75974,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Blush".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f60a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 30013,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Warm-smile".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/263a_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 25877,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Relieved".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f60c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21131,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Smirk".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f60f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 31466,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sleep".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f634/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 24420,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sleepy".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33013,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Drool".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f924/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 39367,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Yum".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f60b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38167,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Stuck-out-tongue".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f61b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 40520,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Squinting-tongue".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f61d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 51166,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Winky-tongue".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f61c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 74487,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Zany-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f92a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 69242,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Woozy".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f974/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 32633,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Pensive".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f614/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18003,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Pleading".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f97a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 45677,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Grimacing".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 60270,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Expressionless".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f611/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15581,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Neutral-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f610/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 22574,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Mouth-none".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f636/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21638,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Face-in-clouds".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f636_200d_1f32b_fe0f/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 135831,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dotted-line-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae5/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15473,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Zipper-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f910/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 79614,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Salute".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae1/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 50214,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thinking-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f914/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 68027,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Shushing-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f92b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 60687,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hand-over-mouth".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae2/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 30548,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Smiling-eyes-with-hand-over-mouth".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f92d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58552,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Yawn".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f971/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 40591,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hug-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f917/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 80181,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Peeking".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae3/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 117202,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Screaming".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f631/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 90226,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raised-eyebrow".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f928/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 31912,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Monocle".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9d0/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 67304,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Unamused".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f612/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 30060,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rolling-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f644/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 29512,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Exhale".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62e_200d_1f4a8/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 57245,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Triumph".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f624/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 128406,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Angry".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f620/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33672,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rage".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f621/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 46289,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cursing".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f92c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 235182,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sad".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f61e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 30944,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sweat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f613/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 35718,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Worried".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f61f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 19906,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Concerned".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f625/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 39143,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cry".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f622/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 47018,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Big-frown".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2639_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 17600,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Frown".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f641/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15949,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Diagonal-mouth".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae4/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 27493,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Slightly-frowning".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f615/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 25699,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Anxious-with-sweat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f630/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 31920,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Scared".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f628/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23708,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Anguished".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f627/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 22195,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Gasp".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f626/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15110,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Mouth-open".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18749,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Surprised".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 27156,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Astonished".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f632/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21560,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Flushed".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f633/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 39206,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Mind-blown".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f92f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 143616,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Scrunched-mouth".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f616/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 25776,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Scrunched-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f623/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 35178,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Weary".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f629/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 43145,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Distraught".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f62b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 41299,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "X-eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f635/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 27858,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dizzy-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f635_200d_1f4ab/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 37987,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Shaking-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae8/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 62846,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cold-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f976/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 105845,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hot-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f975/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 66152,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sick".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f922/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38007,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Vomit".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f92e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 113083,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sneeze".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f927/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 62632,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thermometer-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f912/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 35011,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bandage-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f915/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 24897,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Mask".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f637/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 26028,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Liar".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f925/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 110222,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Halo".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f607/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 47770,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cowboy".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f920/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 68918,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Money-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f911/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 62491,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Nerd-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f913/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 34888,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sunglasses-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f60e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 25407,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Disguise".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f978/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 49569,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clown".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f921/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 46553,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Imp-smile".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f608/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 43402,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Imp-frown".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f47f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 39216,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Ghost".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f47b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 200747,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Jack-o-lantern".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f383/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 62686,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Poop".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4a9/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 257521,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Robot".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f916/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 187153,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Alien".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f47d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 113419,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Moon-face-first-quarter".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f31b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 36591,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Moon-face-last-quarter".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f31c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15486,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sun-with-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f31e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18283,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Fire".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f525/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 29358,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "100".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4af/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 66142,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Glowing-star".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f31f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 73203,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sparkles".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2728/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15584,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Collision".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4a5/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 87831,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Party-popper".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f389/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 67875,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "See-no-evil-monkey".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f648/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 52125,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hear-no-evil-monkey".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f649/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 66440,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Speak-no-evil-monkey".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 76392,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Smiley-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f63a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 43360,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Smile-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f638/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 66070,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Joy-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f639/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 86244,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Heart-eyes-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f63b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 54362,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Smirk-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f63c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 46333,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kissing-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f63d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 34296,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Scream-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f640/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 90268,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crying-cat-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f63f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 105822,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Pouting-cat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f63e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 47069,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Red-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2764_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8439,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Orange-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9e1/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8378,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Yellow-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f49b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8382,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Green-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f49a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8458,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Light-blue-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fa75/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8456,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Blue-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f499/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8458,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Purple-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f49c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8458,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Brown-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f90e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8458,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Black-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f5a4/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8458,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Grey-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fa76/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8470,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "White-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f90d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8380,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Pink-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fa77/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 8431,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cupid".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f498/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 79374,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Gift-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f49d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 152189,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sparkling-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f496/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 34772,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Heart-grow".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f497/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 12631,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Beating-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f493/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 26352,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Revolving-hearts".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f49e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 31191,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Two-hearts".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f495/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 12457,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Love-letter".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f48c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 66873,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Heart-exclamation-point".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2763_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 19995,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bandaged-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2764_fe0f_200d_1fa79/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 100250,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Broken-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f494/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 87260,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Fire-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2764_fe0f_200d_1f525/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 64140,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kiss".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f48b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 11896,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Footprints".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f463/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21917,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Anatomical-heart".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fac0/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 67731,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Blood".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fa78/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 13119,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Microbe".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9a0/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 65924,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Skull".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f480/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 226657,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Eyes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f440/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28242,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Eye".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f441_fe0f/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 44364,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Biting-lip".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fae6/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18180,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Leg-mechanical".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9bf/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 42593,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Arm-mechanical".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9be/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 51190,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Muscle".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4aa/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23340,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Muscle-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4aa_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23476,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Muscle-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4aa_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23516,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Muscle-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4aa_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23518,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Muscle-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4aa_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23521,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Muscle-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4aa_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23517,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clap".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38150,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clap-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44f_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38819,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clap-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44f_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38824,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clap-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44f_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38855,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clap-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44f_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38745,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clap-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44f_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 38848,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-up".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 57963,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-up-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44d_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58034,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-up-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44d_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58034,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-up-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44d_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58044,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-up-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44d_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58035,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-up-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44d_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 58044,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-down".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28607,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-down-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44e_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28664,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-down-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44e_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28665,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-down-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44e_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28673,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-down-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44e_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28665,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Thumbs-down-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44e_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 28673,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raising-hands".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81851,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raising-hands-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64c_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81925,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raising-hands-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64c_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81929,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raising-hands-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64c_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81931,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raising-hands-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64c_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81927,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Raising-hands-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64c_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81925,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wave".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14645,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wave-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14734,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wave-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14739,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wave-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14739,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wave-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14740,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wave-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14733,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Victory".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/270c_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 68888,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Victory-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/270c_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 69086,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Victory-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/270c_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 69069,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Victory-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/270c_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 69077,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Victory-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/270c_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 69061,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Victory-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/270c_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 69054,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crossed-fingers".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f91e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33723,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crossed-fingers-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f91e_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33930,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crossed-fingers-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f91e_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33916,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crossed-fingers-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f91e_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33916,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crossed-fingers-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f91e_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33909,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crossed-fingers-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f91e_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33903,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Index-finger".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/261d_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21113,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Index-finger-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/261d_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21277,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Index-finger-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/261d_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21267,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Index-finger-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/261d_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21271,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Index-finger-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/261d_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21262,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Index-finger-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/261d_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 21256,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Folded-hands".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18592,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Folded-hands-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64f_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18658,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Folded-hands-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64f_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18676,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Folded-hands-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64f_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18682,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Folded-hands-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64f_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18682,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Folded-hands-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f64f_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 18670,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dancer-woman".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f483/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 364759,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dancer-woman-1".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f483_1f3fb/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 364030,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dancer-woman-2".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f483_1f3fc/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 364036,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dancer-woman-3".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f483_1f3fd/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 365020,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dancer-woman-4".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f483_1f3fe/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 364037,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dancer-woman-5".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f483_1f3ff/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 364035,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rose".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f339/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 45174,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wilted-flower".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f940/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 37881,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Fallen-leaf".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f342/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 45536,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Plant".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f331/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 53145,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Luck".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f340/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 70266,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Snowflake".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2744_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 138664,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Volcano".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f30b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 116692,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sunrise".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f305/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 51088,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sunrise-over-mountains".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f304/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 29561,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rainbow".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f308/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 13837,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wind-face".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f32c_fe0f/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 135084,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Electricity".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/26a1/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 89096,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dizzy".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4ab/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 80643,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Comet".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2604_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 170252,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Globe-showing-europe-africa".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f30d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 188658,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Unicorn".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f984/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 278449,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Lizard".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f98e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 104721,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dragon".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f409/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 258884,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "T-rex".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f996/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 131948,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Turtle".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f422/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 55887,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Snake".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f40d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 97973,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Frog".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f438/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 108606,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rabbit".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f407/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81000,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f400/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 124290,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dog".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f415/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 98235,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Pig".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f416/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 143547,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Racehorse".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f40e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 85914,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Donkey".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1facf/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 105494,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Ox".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f402/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 76491,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Goat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f410/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 248917,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Kangaroo".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f998/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 124870,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Tiger".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f405/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 428543,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Monkey".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f412/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 136109,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Chipmunk".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f43f_fe0f/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 144799,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Otter".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9a6/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 94172,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bat".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f987/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33394,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rooster".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f413/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 88968,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hatching-chick".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f423/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 53445,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Baby-chick".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f424/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 30711,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hatched-chick".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f425/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 50066,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Eagle".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f985/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 72897,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Peace".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f54a_fe0f/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 176763,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Goose".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fabf/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 185187,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Peacock".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f99a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 167783,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Seal".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f9ad/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 357487,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Dolphin".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f42c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81544,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Whale".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f433/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 91653,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Blowfish".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f421/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 124208,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Crab".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f980/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 160764,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Octopus".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f419/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 234509,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Jellyfish".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fabc/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 97671,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Snail".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f40c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 49014,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Ant".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f41c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 100806,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Mosquito".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f99f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 88334,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bee".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f41d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 482250,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Butterfly".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f98b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 249980,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Paw Prints".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f43e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 17179,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Tomato".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f345/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 142308,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Popcorn".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f37f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 135307,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Hot-beverage".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2615/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 36778,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clinking-beer-mugs".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f37b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 112052,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Clinking-glasses".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f942/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 94364,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bottle-with-popping-cork".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f37e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 37645,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Wine-glass".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f377/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 60942,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Tropical-drink".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f379/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 85325,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Police-car-light".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f6a8/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 65174,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Flying-saucer".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f6f8/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 153261,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Rocket".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f680/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81575,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Airplane-departure".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f6eb/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 31939,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Airplane-arrival".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f6ec/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 40894,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Roller-coaster".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f3a2/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 66382,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Confetti-ball".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f38a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 122718,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Balloon".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f388/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 14095,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Birthday-cake".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f382/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 93106,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Fireworks".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f386/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 110210,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Mirror-ball".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1faa9/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 172207,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Soccer-ball".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/26bd/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 41860,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Direct-hit".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f3af/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 45947,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Violin".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f3bb/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 36626,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Drum".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f941/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 37285,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Maracas".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1fa87/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 53870,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Battery-full".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f50b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 64179,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Battery-low".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1faab/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 145891,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Money-with-wings".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4b8/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 97230,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Light-bulb".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f4a1/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 23436,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Graduation-cap".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f393/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 229903,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Umbrella".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2602_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 27242,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Gem-stone".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f48e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 62646,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Alarm-clock".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/23f0/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 36409,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bellhop-bell".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f6ce_fe0f/lottie.json"
                .to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 26841,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Bell".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f514/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 36717,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Aries".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2648/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 82667,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Taurus".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2649/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 82769,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Gemini".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/264a/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 106146,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cancer".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/264b/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 89628,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Leo".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/264c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 88502,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Virgo".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/264d/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 101944,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Libra".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/264e/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 81824,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Scorpio".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/264f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 101492,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Sagittarius".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2650/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 94578,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Capricorn".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2651/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 101304,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Aquarius".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2652/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 111335,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Pisces".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2653/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 106574,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Ophiuchus".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/26ce/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 100303,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Exclamation-double".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/203c_fe0f/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 26663,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cross-mark".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/274c/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 7292,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Musical-notes".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f3b6/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 12828,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Check-mark".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2705/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 15925,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Cool".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f192/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 19755,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Plus-sign".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/2795/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 33464,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
        LottieDownload {
            name: "Chequered-flag".to_string(),
            url: "https://fonts.gstatic.com/s/e/notoemoji/latest/1f3c1/lottie.json".to_string(),
            builtin: Some(BuiltinLottieProps {
                expected_size: 328109,
                info: "https://googlefonts.github.io/noto-emoji-animation/",
                license: "CC BY 4.0",
            }),
        },
    ]
}
