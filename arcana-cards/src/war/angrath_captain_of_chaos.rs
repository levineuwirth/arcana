//! Angrath, Captain of Chaos — `{2}{B/R}{B/R}` Legendary Planeswalker —
//! Angrath, starting loyalty 5. Colors B, R.
//!
//! Static (GAP): Creatures you control have menace. (Not a loyalty ability;
//! a team-wide keyword-granting static not in the demonstrated surface —
//! omitted. The Scryfall "Amass" keyword is the −2 ability text, not a
//! static keyword, so `keywords` stays empty.)
//! −2: Amass Zombies 2.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angrath, Captain of Chaos");
    let angrath = reg.interner_mut().intern("Angrath");
    let _army = reg.interner_mut().intern("Army");
    let _zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angrath);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Amass Zombies 2.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_amass,
            }),
    )
}

fn minus_two_amass(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Zombie").unwrap_or_default();
    vec![Effect::Amass {
        controller: ctx.controller,
        count: 2,
        army_subtype,
        race_subtype,
    }]
}
