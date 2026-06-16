//! Golgari Grave-Troll — `{4}{G}` 0/0 Troll Skeleton.
//! "This creature enters with a +1/+1 counter on it for each creature
//! card in your graveyard. {1}, Remove a +1/+1 counter from this
//! creature: Regenerate this creature. Dredge 6."
//!
//! Dredge and Mill have no `KeywordAbility` variant (keywords empty).
//! The enters-with-counters clause needs a filtered graveyard count
//! that no demonstrated script helper provides, so it is GAP'd. The
//! remove-counter regenerate activation is fully expressible.

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
    let name = reg.interner_mut().intern("Golgari Grave-Troll");
    let troll = reg.interner_mut().intern("Troll");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: Dredge 6 / Mill have no KeywordAbility variant.
        ..Default::default()
    };

    // GAP: "enters with a +1/+1 counter for each creature card in your
    // graveyard" needs a filtered-graveyard count not in the demonstrated
    // script helpers (graveyard_size is unfiltered; ids_matching is
    // battlefield-only).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, Remove a +1/+1 counter from this creature: Regenerate this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: regenerate_self,
        }),
    )
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
