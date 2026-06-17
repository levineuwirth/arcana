//! Myojin of Life's Web — `{6}{G}{G}{G}` 8/8 Legendary Spirit.
//! "Enters with a divinity counter on it if you cast it from your hand.
//! Has indestructible as long as it has a divinity counter on it.
//! Remove a divinity counter from Myojin of Life's Web: Put any number of
//! creature cards from your hand onto the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Life's Web");
    let spirit = reg.interner_mut().intern("Spirit");
    let divinity = reg.interner_mut().intern("divinity");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    // GAP: "enters with a divinity counter on it if you cast it from your
    // hand" — no cast-from-hand-conditional ETB-counter primitive.
    // GAP: static "has indestructible as long as it has a divinity counter"
    // — conditional static keyword grant is not a triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove a divinity counter from Myojin of Life's Web: Put any number of creature cards from your hand onto the battlefield.".into(),
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
            effect: put_creatures,
        }),
    )
}

fn put_creatures(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // PARTIAL: "put ANY NUMBER of creature cards from your hand onto the
    // battlefield" — only a single-pick PutFromHandOntoBattlefield exists,
    // so this posts one creature-card put rather than the full any-number set.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
