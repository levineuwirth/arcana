//! Zombie Trailblazer — `{B}{B}{B}` 2/2 Zombie Scout.
//! "Tap an untapped Zombie you control: Target land becomes a Swamp until
//! end of turn." and "Tap an untapped Zombie you control: Target creature
//! gains swampwalk until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Zombie Trailblazer");
    let zombie = reg.interner_mut().intern("Zombie");
    let scout = reg.interner_mut().intern("Scout");
    // Interned at register so the swampwalk resolver can recover it.
    let _swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let zombie_cost = || ActivationCost {
        tap_other: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            subtypes: Some(vec![zombie]),
            tapped: Some(false),
            ..ObjectFilter::default()
        }),
        ..ActivationCost::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped Zombie you control: Target land becomes a Swamp until end of turn.".into(),
                cost: zombie_cost(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: land_becomes_swamp,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped Zombie you control: Target creature gains swampwalk until end of turn.".into(),
                cost: zombie_cost(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_swampwalk,
            }),
    )
}

fn land_becomes_swamp(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Target land becomes a Swamp until end of turn." There is no
    // effect that adds a land subtype (and no mana-type rewrite); the type
    // change is unexpressible.
    Vec::new()
}

fn grant_swampwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(swamp) = reg.interner().lookup("Swamp") else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Landwalk(swamp),
        duration: Duration::EndOfTurn,
    }]
}
