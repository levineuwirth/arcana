//! Myojin of Infinite Rage — `{7}{R}{R}{R}` 7/4 Legendary Spirit.
//! "Enters with a divinity counter on it if you cast it from your
//! hand." — the cast-from-hand condition has no ETB accessor → GAP'd.
//! "Has indestructible as long as it has a divinity counter on it." —
//! a counter-gated static, not expressible as a triggered/activated
//! ability → GAP'd.
//! "Remove a divinity counter from Myojin of Infinite Rage: Destroy
//! all lands." — fully expressed (counter-removal cost + board-wide
//! land destruction).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Infinite Rage");
    let spirit = reg.interner_mut().intern("Spirit");
    let divinity = reg.interner_mut().intern("divinity");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove a divinity counter from Myojin of Infinite Rage: \
                   Destroy all lands."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Named(divinity), 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_all_lands,
        }),
    )
}

fn destroy_all_lands(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let lands = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: lands,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}
