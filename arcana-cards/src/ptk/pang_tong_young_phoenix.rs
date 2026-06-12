//! Pang Tong, "Young Phoenix" — `{1}{W}{W}` 1/2 legendary white Human Advisor.
//! "{T}: Target creature gets +0/+2 until end of turn. Activate only during
//! your turn, before attackers are declared."
//!
//! The "during your turn, before attackers are declared" window is
//! enforced via `ActivationCost.activation_condition`
//! (`conditions::your_turn_before_attackers`).
//! timing restriction not expressible.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pang Tong, \"Young Phoenix\"");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target creature gets +0/+2 until end of turn. Activate only during your turn, before attackers are declared.".into(),
                cost: ActivationCost {
                    // "Activate only during your turn, before attackers
                    // are declared" (CR 602.5e window).
                    activation_condition: Some(|s, _src, you, _reg| {
                        arcana_core::conditions::your_turn_before_attackers(s, you)
                    }),
                    ..ActivationCost::tap_only()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_zero_two,
            }),
    )
}

fn pump_zero_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 0,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
