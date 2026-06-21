//! Hydromorph Gull — `{3}{U}{U}` 3/3 blue Elemental Bird.
//!
//! Oracle:
//! * Flying.
//! * "{U}, Sacrifice this creature: Counter target spell that targets a
//!   creature you control." — a mana + sacrifice-self activation that
//!   counters a target spell. FIDELITY GAP: the "that targets a creature
//!   you control" restriction is not expressible via the spell
//!   `ObjectFilter`, so the requirement is approximated as any target
//!   spell.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hydromorph Gull");
    let elemental = reg.interner_mut().intern("Elemental");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}, Sacrifice this creature: Counter target spell that targets a creature you control."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(ObjectFilter::new()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: true,
            face_gate: None,
            effect: counter_target_spell,
        }),
    )
}

fn counter_target_spell(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Counter { target: *id }]
}
