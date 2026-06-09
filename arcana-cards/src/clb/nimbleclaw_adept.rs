//! Nimbleclaw Adept — `{3}{U}` 2/3 Dragon Wizard.
//! Bigby's Hand — `{T}: Untap two other target permanents. Activate only as a sorcery and only once each turn.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nimbleclaw Adept");
    let dragon = reg.interner_mut().intern("Dragon");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Bigby's Hand — {T}: Untap two other target permanents. Activate only as a sorcery and only once each turn.".into(),
                cost: ActivationCost {
                    tap: true,
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
                    count: TargetCount::Exactly(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_two,
            }),
    )
}

fn untap_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets.targets.iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::Untap { target: *id })
            } else {
                None
            }
        })
        .collect()
}
