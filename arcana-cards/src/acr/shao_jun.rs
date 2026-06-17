//! Shao Jun — `{1}{U}{R}` 3/3 Legendary Human Assassin.
//! Leap Strike ("During your turn, Shao Jun has flying and first
//! strike.") and Rope Dart ("Tap two untapped artifacts you control:
//! Shao Jun deals 1 damage to each opponent.").
//!
//! Leap Strike is a conditional static (your-turn keyword grant) and is
//! GAP'd. Rope Dart is wired as an activated ability whose cost taps two
//! untapped artifacts you control.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: "Leap Strike — During your turn, Shao Jun has flying and first
//      strike." — a conditional (your-turn-only) static keyword grant;
//      not expressible as a triggered/activated ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shao Jun");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped artifacts you control: Shao Jun deals 1 damage to each opponent.".into(),
                cost: ActivationCost {
                    tap_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: rope_dart,
            }),
    )
}

fn rope_dart(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
