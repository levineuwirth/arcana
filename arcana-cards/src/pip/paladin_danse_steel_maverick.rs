//! Paladin Danse, Steel Maverick — `{2}{W}` 3/3 Legendary Artifact
//! Creature — Synth Knight.
//!
//! * Vigilance, lifelink.
//! * Exile Paladin Danse: Each creature you control that's an artifact or
//!   Human gains indestructible until end of turn.

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
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paladin Danse, Steel Maverick");
    let synth = reg.interner_mut().intern("Synth");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(synth);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Exile Paladin Danse: Each creature you control that's an artifact or Human gains indestructible until end of turn."
                .into(),
            cost: ActivationCost {
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_indestructible,
        }),
    )
}

/// Grant indestructible (EOT) to each of your artifact creatures and each
/// of your Human creatures.
fn grant_indestructible(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut ids: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let humans = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Human").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    for h in humans {
        if !ids.contains(&h) {
            ids.push(h);
        }
    }
    ids.into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
