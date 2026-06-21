//! Dreamwinder — `{3}{U}` 4/3 Serpent.
//! "This creature can't attack unless defending player controls an
//! Island."
//! "{U}, Sacrifice an Island: Target land becomes an Island until end of
//! turn."
//!
//! The attack-restriction static has no expressible primitive, so it is
//! GAP'd. The activated ability's cost (mana {U} + sacrifice a chosen
//! Island via sacrifice_other) and its target (a land) are wired, but
//! "becomes an Island" is a subtype-grant for which the demonstrated
//! Effect catalog has no variant (AddType handles card TYPES, not
//! subtypes), so the resolver body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreamwinder");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let island = reg.interner_mut().intern("Island");
    let island_filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .with_subtype_sym(island);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "This creature can't attack unless defending player controls an
    // Island." — a conditional attack-restriction static with no expressible
    // primitive.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}, Sacrifice an Island: Target land becomes an Island until end of turn."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                sacrifice_other: Some(island_filter),
                ..ActivationCost::default()
            },
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
            effect: become_island,
        }),
    )
}

fn become_island(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Target land becomes an Island until end of turn." — granting a
    // subtype is not expressible (Effect::AddType adds card TYPES, not
    // subtypes; there is no add-subtype Effect variant).
    Vec::new()
}
