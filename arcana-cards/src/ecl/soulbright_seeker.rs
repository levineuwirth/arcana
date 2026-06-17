//! Soulbright Seeker — `{R}` 2/1 Elemental Sorcerer.
//! Behold an Elemental or pay {2} as an additional cost (GAP — additional
//! cast cost not expressible).
//! {R}: Target creature you control gains trample until end of turn.
//! (The "third time this turn → add {R}{R}{R}{R}" rider is unexpressible.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soulbright Seeker");
    let elemental = reg.interner_mut().intern("Elemental");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(sorcerer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // Behold is not a usable keyword on this card class.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "As an additional cost to cast this spell, behold an Elemental or
    // pay {2}" — additional cast costs are not expressible here.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: Target creature you control gains trample until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_trample,
            }),
    )
}

fn grant_trample(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "If this is the third time this ability has resolved this turn,
    // add {R}{R}{R}{R}" — per-resolution counting is not expressible.
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}
