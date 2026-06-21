//! Gnarlbark Elm — `{2}{B}` 3/4 Treefolk Warlock.
//! "This creature enters with two -1/-1 counters on it.
//!  {2}{B}, Remove two counters from this creature: Target creature gets -2/-2
//!  until end of turn. Activate only as a sorcery."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnarlbark Elm");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: static — "This creature enters with two -1/-1 counters on
            //       it." (enters-with-counters spec not in the demonstrated
            //       API surface).
            // "{2}{B}, Remove two counters from this creature: Target creature
            //  gets -2/-2 until end of turn. Activate only as a sorcery."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, Remove two counters from this creature: Target creature gets -2/-2 until end of turn. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::MinusOneMinusOne, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_minus_two,
            }),
    )
}

fn minus_two_minus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: -2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
