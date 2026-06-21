//! Tsabo Tavoc — `{5}{B}{R}` 7/4 Legendary Phyrexian Horror with First
//! strike.
//! Protection from legendary creatures (keyword unsupported — GAP).
//! "{B}{B}, {T}: Destroy target legendary creature. It can't be
//! regenerated." (the can't-be-regenerated rider is a documented
//! partial — DestroyPermanent has no no-regen flag.)

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tsabo Tavoc");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        // GAP: keyword — Protection from legendary creatures (not in the
        // usable keyword surface).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}{B}, {T}: Destroy target legendary creature. It can't be regenerated."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}{B}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_legendary,
        }),
    )
}

fn destroy_legendary(
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
    // Partial: "It can't be regenerated" rider is unmodeled —
    // DestroyPermanent carries no no-regeneration flag.
    vec![Effect::DestroyPermanent { target: *id }]
}
