//! Streambed Aquitects — `{1}{U}{U}` 2/3 blue Merfolk Scout.
//! "{T}: Target Merfolk creature gets +1/+1 and gains islandwalk until end of
//! turn."
//! "{T}: Target land becomes an Island until end of turn." (the land-subtype
//! change is not expressible — GAP'd.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Streambed Aquitects");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let _island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);

    let merfolk_filter = script::subtype_filter(reg, "Merfolk");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
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
                text: "{T}: Target Merfolk creature gets +1/+1 and gains islandwalk until end of turn.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(merfolk_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_merfolk,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target land becomes an Island until end of turn.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
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
                effect: land_becomes_island,
            }),
    )
}

fn pump_merfolk(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut keywords = vec![];
    if let Some(island) = reg.interner().lookup("Island") {
        keywords.push(KeywordAbility::Landwalk(island));
    }
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords,
    }]
}

fn land_becomes_island(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target land becomes an Island" — adding/replacing a land's land
    // subtype is not expressible (AddType is TypeLine-only, not subtype).
    Vec::new()
}
