//! Patron of the Orochi — `{6}{G}{G}` 7/7 Legendary Spirit.
//! "Snake offering (...)"
//! "{T}: Untap all Forests and all green creatures. Activate only once
//! each turn."
//!
//! The Offering alternative-cost keyword is GAP'd (no
//! `KeywordAbility::Offering`). The activated ability uses a tap cost
//! with `once_per_turn: true` and untaps every Forest and every green
//! creature (one ForEach Untap over the union of the two id sets).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patron of the Orochi");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Snake offering — no KeywordAbility::Offering variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Untap all Forests and all green creatures. Activate only once each turn.".into(),
            cost: ActivationCost {
                tap: true,
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: untap_forests_and_green,
        }),
    )
}

fn untap_forests_and_green(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut ids = script::ids_matching(state, &script::subtype_filter(reg, "Forest"), ctx.controller);
    let green = script::ids_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::green()),
        ctx.controller,
    );
    ids.extend(green);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap { target: NULL_OBJECT_ID }),
    }]
}
