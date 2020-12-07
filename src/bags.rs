enum BagColour {
    LightRed,
    DarkOrange,
    BrightWhite,
    MutedYellow,
    ShinyGold,
    DarkOlive,
    VibrantPlum,
    FadedBlue,
    DottedBlack,
}

pub struct BagRule {
    bag: BagColour,
    contents: Vec<(BagColour, u8)>,
}

pub fn bag_rules() -> Vec<BagRule> {
    let mut rules: Vec<BagRule> = Vec::new();
    let mut rule = BagRule {
        bag: BagColour::LightRed,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::BrightWhite, 1));
    rule.contents.push((BagColour::MutedYellow, 2));
    rules.push(rule);

    let mut rule = BagRule {
        bag: BagColour::DarkOrange,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::BrightWhite, 3));
    rule.contents.push((BagColour::MutedYellow, 4));
    rules.push(rule);

    let mut rule = BagRule {
        bag: BagColour::BrightWhite,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::ShinyGold, 1));
    rules.push(rule);

    rule = BagRule {
        bag: BagColour::MutedYellow,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::ShinyGold, 2));
    rule.contents.push((BagColour::FadedBlue, 9));
    rules.push(rule);

    rule = BagRule {
        bag: BagColour::ShinyGold,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::DarkOlive, 1));
    rule.contents.push((BagColour::VibrantPlum, 2));
    rules.push(rule);

    rule = BagRule {
        bag: BagColour::DarkOlive,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::FadedBlue, 3));
    rule.contents.push((BagColour::DottedBlack, 4));
    rules.push(rule);

    rule = BagRule {
        bag: BagColour::VibrantPlum,
        contents: Vec::new(),
    };
    rule.contents.push((BagColour::FadedBlue, 5));
    rule.contents.push((BagColour::DottedBlack, 6));
    rules.push(rule);

    rule = BagRule {
        bag: BagColour::FadedBlue,
        contents: Vec::new(),
    };
    rules.push(rule);

    rule = BagRule {
        bag: BagColour::DottedBlack,
        contents: Vec::new(),
    };
    rules.push(rule);

    rules
}
