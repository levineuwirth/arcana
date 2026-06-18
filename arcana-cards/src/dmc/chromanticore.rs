//! Chromanticore — `{W}{U}{B}{R}{G}` 4/4 Enchantment Creature — Manticore.
//! Flying, first strike, vigilance, trample, lifelink. Bestow {2}{W}{U}{B}{R}{G}
//! and the "Enchanted creature gets +4/+4 and has ..." aura static are GAP'd —
//! Bestow is not in the usable keyword surface and the bestow aura grant is a
//! static continuous ability with no triggered/activated expression.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chromanticore");
    let manticore = reg.interner_mut().intern("Manticore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(manticore);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Lifelink,
        ],
        // GAP: Bestow {2}{W}{U}{B}{R}{G} (not in usable keyword surface).
        ..Default::default()
    };

    // GAP: static "Enchanted creature gets +4/+4 and has flying, first strike,
    // vigilance, trample, and lifelink" (bestow aura grant — continuous static).
    reg.register(CardDefinition::new(name, chars))
}
