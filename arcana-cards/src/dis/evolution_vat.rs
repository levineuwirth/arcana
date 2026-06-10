//! Evolution Vat — `{3}` artifact.
//! "{3}, {T}: Tap target creature and put a +1/+1 counter on it. Until
//! end of turn, that creature gains \"{2}{G}{U}: Double the number of
//! +1/+1 counters on this creature.\""
//! The tap + counter halves are wired; the granted ACTIVATED ability
//! is not expressible (GrantTriggeredAbility covers triggered
//! abilities only — there is no grant-activated-ability effect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evolution Vat");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, {T}: Tap target creature and put a +1/+1 counter on it. Until end of turn, that creature gains \"{2}{G}{U}: Double the number of +1/+1 counters on this creature.\"".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tap_and_grow,
        }),
    )
}

fn tap_and_grow(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: 'until end of turn, that creature gains "{2}{G}{U}: Double
    // the number of +1/+1 counters on this creature."' — granting an
    // ACTIVATED ability is not expressible (GrantTriggeredAbility is
    // triggered-only).
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
