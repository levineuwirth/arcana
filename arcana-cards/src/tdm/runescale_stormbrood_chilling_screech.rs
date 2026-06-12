//! Runescale Stormbrood // Chilling Screech — `{3}{R}` Dragon creature 2/4
//! with Flying. Whenever you cast a noncreature spell or a Dragon spell, this
//! creature gets +2/+0 until end of turn.
//! Adventure face "Chilling Screech" (`{1}{U}` Instant — Omen): Counter target
//! spell with mana value 2 or less.
//!
//! GAP: "Whenever you cast a noncreature spell or a Dragon spell" trigger —
//! TriggerCondition::SpellCast is not in the demonstrated API; omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Runescale Stormbrood");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: "Whenever you cast a noncreature spell or a Dragon spell, this
        // creature gets +2/+0 until end of turn" — TriggerCondition::SpellCast
        // not in demonstrated API; omitted.
        ..Default::default()
    };

    // Adventure face: "Chilling Screech" — {1}{U} Instant
    let adv_name = reg.interner_mut().intern("Chilling Screech");
    let omen_sub = reg.interner_mut().intern("Omen");
    let mut adv_subtypes = SubtypeSet::default();
    adv_subtypes.0.insert(omen_sub);

    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid adv cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        subtypes: adv_subtypes,
        ..Default::default()
    };

    let adv_ability = SpellAbilityDef {
        text: "Counter target spell with mana value 2 or less.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Spell(ObjectFilter::new().with_max_cmc(2)),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: chilling_screech_resolve,
    };

    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure),
    )
}

fn chilling_screech_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Counter the targeted spell (mana value 2 or less, enforced by the filter).
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Counter { target: *id }]
}
