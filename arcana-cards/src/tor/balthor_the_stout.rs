//! Balthor the Stout — `{1}{R}{R}` 2/2 Legendary Dwarf Barbarian.
//! "Other Barbarian creatures get +1/+1."
//! "{R}: Another target Barbarian creature gets +1/+0 until end of turn."
//!
//! The first line is a pure static anthem (no trigger/cost) and is GAP'd.
//! The activated pump is faithful; "another" (excluding Balthor itself) is
//! not expressible as a target filter (no self-exclusion), a minor gap.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Balthor the Stout");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(barbarian);

    // GAP: static "Other Barbarian creatures get +1/+1" — a pure continuous
    // anthem, not a triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let barbarian_filter = ObjectFilter::creature().with_subtype_sym(barbarian);

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}: Another target Barbarian creature gets +1/+0 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            // GAP: "Another" (exclude Balthor) not expressible as a filter.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(barbarian_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_barbarian,
        }),
    )
}

fn pump_barbarian(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
