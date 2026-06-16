//! Ashiok, Dream Render — `{1}{U/B}{U/B}` legendary planeswalker, starting
//! loyalty 5. Subtype Ashiok; blue-black. (The static "opponents can't
//! search" is a continuous ability, not a loyalty ability — not modeled
//! here.)
//!
//! Loyalty abilities:
//! * `−1`: Target player mills four cards. Then exile each opponent's
//!   graveyard. (The mill is functional; exiling each opponent's entire
//!   graveyard is omitted — no zone-wide exile primitive.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashiok, Dream Render");
    let ashiok = reg.interner_mut().intern("Ashiok");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ashiok);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "−1: Target player mills four cards. Then exile each \
                   opponent's graveyard."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Loyalty, 1)),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: true,
            activation_zone: arcana_core::registry::ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: minus_one_mill,
        }),
    )
}

fn minus_one_mill(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // The mill is functional; exiling each opponent's entire graveyard is
    // omitted (no zone-wide exile primitive in the demonstrated surface).
    vec![Effect::Mill { player: *p, count: 4 }]
}
