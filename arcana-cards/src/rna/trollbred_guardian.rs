//! Trollbred Guardian — `{4}{G}` 5/5 Troll Frog Warrior.
//!
//! Oracle:
//! * {2}{G}: Adapt 2. (If this creature has no +1/+1 counters on it, put two
//!   +1/+1 counters on it.)
//! * Each creature you control with a +1/+1 counter on it has trample.
//!
//! The activated ability is emitted with its `{2}{G}` cost, but its effect is
//! GAP'd: Adapt ("if it has no +1/+1 counters, put two on it") is a
//! conditional-on-source-counters mechanic with no expressible Effect (no
//! Adapt primitive, and `Effect::Conditional`'s `Condition` constructors are
//! not part of the demonstrated API). The Adapt keyword is likewise not in the
//! usable evergreen keyword surface, so `keywords` is empty.
//! The trample-granting line is a pure continuous STATIC — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trollbred Guardian");
    let troll = reg.interner_mut().intern("Troll");
    let frog = reg.interner_mut().intern("Frog");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(frog);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Adapt keyword not in the usable evergreen surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Adapt 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_two,
            }),
    )
    // GAP (static): "Each creature you control with a +1/+1 counter on it has
    // trample." — a pure continuous static, not a triggered/activated ability.
}

fn adapt_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Adapt 2 ("if this creature has no +1/+1 counters on it, put two
    // +1/+1 counters on it") is conditional on the source's own counters; no
    // Adapt primitive and no expressible source-counter Condition.
    Vec::new()
}
