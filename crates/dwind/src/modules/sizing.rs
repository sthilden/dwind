use dwind_macros::dwgenerate_map;

include!(concat!(env!("OUT_DIR"), "/sizing.rs"));

#[macro_export]
macro_rules! width_generator {
    ($width:tt) => {
        const_format::formatcp!("width: {};", $width)
    };
}

#[macro_export]
macro_rules! height_generator {
    ($height:tt) => {
        const_format::formatcp!("height: {};", $height)
    };
}

#[macro_export]
macro_rules! max_width_generator {
    ($width:tt) => {
        const_format::formatcp!("max-width: {};", $width)
    };
}

#[macro_export]
macro_rules! max_height_generator {
    ($width:tt) => {
        const_format::formatcp!("max-height: {};", $width)
    };
}

#[macro_export]
macro_rules! min_width_generator {
    ($width:tt) => {
        const_format::formatcp!("min-width: {};", $width)
    };
}

#[macro_export]
macro_rules! min_height_generator {
    ($width:tt) => {
        const_format::formatcp!("min-height: {};", $width)
    };
}

dwgenerate_map!(
    "w",
    "width-",
    [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
        ("24", "96px"),
        ("28", "112px"),
        ("32", "128px"),
        ("36", "144px"),
        ("40", "160px"),
        ("44", "176px"),
        ("48", "192px"),
        ("52", "208px"),
        ("56", "224px"),
        ("60", "240px"),
        ("64", "256px"),
        ("72", "288px"),
        ("80", "320px"),
        ("96", "384px"),
        ("sm", "384px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
        ("2xl", "1536px"),
        ("3xl", "1792px"),
        ("4xl", "2048px"),
        ("5xl", "2304px"),
        ("6xl", "2560px"),
        ("7xl", "2816px"),
        ("8xl", "3072px"),
        ("9xl", "3328px"),
        ("10xl", "3584px"),
        ("px", "1px"),
        ("0", "0"),
        ("auto", "auto"),
        ("screen", "100vw"),
        ("full", "100%"),
        ("min", "min-content"),
        ("max", "max-content"),
    ]
);

dwgenerate_map!(
    "min-w",
    "min-width-",
    [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
        ("24", "96px"),
        ("28", "112px"),
        ("32", "128px"),
        ("36", "144px"),
        ("40", "160px"),
        ("44", "176px"),
        ("48", "192px"),
        ("52", "208px"),
        ("56", "224px"),
        ("60", "240px"),
        ("64", "256px"),
        ("72", "288px"),
        ("80", "320px"),
        ("96", "384px"),
        ("sm", "384px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
        ("2xl", "1536px"),
        ("3xl", "1792px"),
        ("4xl", "2048px"),
        ("5xl", "2304px"),
        ("6xl", "2560px"),
        ("7xl", "2816px"),
        ("8xl", "3072px"),
        ("9xl", "3328px"),
        ("10xl", "3584px"),
        ("px", "1px"),
        ("0", "0"),
    ]
);

dwgenerate_map!(
    "max-w",
    "max-width-",
    [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
        ("24", "96px"),
        ("28", "112px"),
        ("32", "128px"),
        ("36", "144px"),
        ("40", "160px"),
        ("44", "176px"),
        ("48", "192px"),
        ("52", "208px"),
        ("56", "224px"),
        ("60", "240px"),
        ("64", "256px"),
        ("72", "288px"),
        ("80", "320px"),
        ("96", "384px"),
        ("sm", "384px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
        ("2xl", "1536px"),
        ("3xl", "1792px"),
        ("4xl", "2048px"),
        ("5xl", "2304px"),
        ("6xl", "2560px"),
        ("7xl", "2816px"),
        ("8xl", "3072px"),
        ("9xl", "3328px"),
        ("10xl", "3584px"),
        ("px", "1px"),
        ("0", "0"),
        ("auto", "auto"),
        ("screen", "100vw"),
        ("full", "100%"),
        ("min", "min-content"),
        ("max", "max-content"),
    ]
);

dwgenerate_map!(
    "min-h",
    "min-height-",
    [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
        ("24", "96px"),
        ("28", "112px"),
        ("32", "128px"),
        ("36", "144px"),
        ("40", "160px"),
        ("44", "176px"),
        ("48", "192px"),
        ("52", "208px"),
        ("56", "224px"),
        ("60", "240px"),
        ("64", "256px"),
        ("72", "288px"),
        ("80", "320px"),
        ("96", "384px"),
        ("sm", "384px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
        ("2xl", "1536px"),
        ("3xl", "1792px"),
        ("4xl", "2048px"),
        ("5xl", "2304px"),
        ("6xl", "2560px"),
        ("7xl", "2816px"),
        ("8xl", "3072px"),
        ("9xl", "3328px"),
        ("10xl", "3584px"),
        ("px", "1px"),
        ("0", "0"),
    ]
);

dwgenerate_map!(
    "max-h",
    "max-height-",
    [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
        ("24", "96px"),
        ("28", "112px"),
        ("32", "128px"),
        ("36", "144px"),
        ("40", "160px"),
        ("44", "176px"),
        ("48", "192px"),
        ("52", "208px"),
        ("56", "224px"),
        ("60", "240px"),
        ("64", "256px"),
        ("72", "288px"),
        ("80", "320px"),
        ("96", "384px"),
        ("sm", "384px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
        ("2xl", "1536px"),
        ("3xl", "1792px"),
        ("4xl", "2048px"),
        ("5xl", "2304px"),
        ("6xl", "2560px"),
        ("7xl", "2816px"),
        ("8xl", "3072px"),
        ("9xl", "3328px"),
        ("10xl", "3584px"),
        ("px", "1px"),
        ("0", "0"),
        ("auto", "auto"),
        ("screen", "100vh"),
        ("full", "100%"),
        ("min", "min-content"),
        ("max", "max-content"),
    ]
);

dwgenerate_map!(
    "h",
    "height-",
    [
        ("p-5", "5%"),
        ("p-10", "10%"),
        ("p-15", "15%"),
        ("p-20", "20%"),
        ("p-25", "25%"),
        ("p-30", "30%"),
        ("p-35", "35%"),
        ("p-40", "40%"),
        ("p-45", "45%"),
        ("p-50", "50%"),
        ("p-55", "55%"),
        ("p-60", "60%"),
        ("p-65", "65%"),
        ("p-70", "70%"),
        ("p-75", "75%"),
        ("p-80", "80%"),
        ("p-85", "85%"),
        ("p-90", "90%"),
        ("p-95", "95%"),
        ("p-100", "100%"),
        ("1", "4px"),
        ("2", "8px"),
        ("3", "12px"),
        ("4", "16px"),
        ("5", "20px"),
        ("6", "24px"),
        ("7", "28px"),
        ("8", "32px"),
        ("9", "36px"),
        ("10", "40px"),
        ("11", "44px"),
        ("12", "48px"),
        ("13", "52px"),
        ("14", "56px"),
        ("15", "60px"),
        ("16", "64px"),
        ("17", "68px"),
        ("18", "72px"),
        ("19", "76px"),
        ("20", "80px"),
        ("24", "96px"),
        ("28", "112px"),
        ("32", "128px"),
        ("36", "144px"),
        ("40", "160px"),
        ("44", "176px"),
        ("48", "192px"),
        ("52", "208px"),
        ("56", "224px"),
        ("60", "240px"),
        ("64", "256px"),
        ("72", "288px"),
        ("80", "320px"),
        ("96", "384px"),
        ("sm", "384px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
        ("2xl", "1536px"),
        ("3xl", "1792px"),
        ("4xl", "2048px"),
        ("5xl", "2304px"),
        ("6xl", "2560px"),
        ("7xl", "2816px"),
        ("8xl", "3072px"),
        ("9xl", "3328px"),
        ("10xl", "3584px"),
        ("px", "1px"),
        ("0", "0"),
        ("auto", "auto"),
        ("screen", "100vh"),
        ("full", "100%"),
        ("min", "min-content"),
        ("max", "max-content"),
    ]
);
