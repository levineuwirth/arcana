//! Skittering Crustacean — `{2}{U}` 2/3 Crab.
//! "{6}{U}: Monstrosity 4." (If this isn't monstrous, put four +1/+1
//! counters on it and it becomes monstrous.)
//! "As long as this creature is monstrous, it has hexproof."
//!
//! There is no Monstrosity primitive (no monstrous flag) and the
//! Scryfall keyword `Monstrosity` is not a `KeywordAbility` variant.
//! The activated ability is emitted as a partial: it puts four +1/+1
//! counters on this creature (the visible payload), but the "becomes
//! monstrous" state and the "isn't monstrous" precondition are GAP'd.
//! The monstrous-gated static hexproof has no expressible primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skittering Crustacean");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // PARTIAL: "Monstrosity 4" — emits the four +1/+1 counters but
            // GAP: no monstrous-flag state, so the "if not monstrous"
            // precondition and "becomes monstrous" status are unmodeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{U}: Monstrosity 4.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: monstrosity_4,
            }),
            // GAP (static): "As long as this creature is monstrous, it has
            // hexproof." No monstrous-flag tracking and no monstrous-gated
            // static-ability primitive.
    )
}

fn monstrosity_4(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 4,
    }]
}
