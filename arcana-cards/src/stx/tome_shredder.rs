//! Tome Shredder — `{2}{R}` 2/2 red Wolf with Haste.
//!
//! * Haste — keyword.
//! * "{T}, Exile an instant or sorcery card from your graveyard: Put a +1/+1
//!   counter on this creature." — the {T} cost and the +1/+1 effect are
//!   modeled. GAP (cost): there is no "exile a card from your graveyard"
//!   field on `ActivationCost`, so the additional exile cost is omitted; the
//!   ability is registered as tap-only with that cost component as a gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tome Shredder");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Exile an instant or sorcery card from your graveyard: Put a +1/+1 counter on this creature.".into(),
            // GAP: "Exile an instant or sorcery card from your graveyard" cost
            // — no exile-from-graveyard field on ActivationCost.
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter_self,
        }),
    )
}

fn add_counter_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
