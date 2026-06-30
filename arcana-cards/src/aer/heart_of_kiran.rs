//! Heart of Kiran — `{2}` Legendary Artifact — Vehicle, 4/4, Flying,
//! Vigilance, Crew 3. (GAP: "you may remove a loyalty counter from a
//! planeswalker you control rather than pay the crew cost" — the alternate
//! crew payment is not modeled; the keywords + Crew 3 are faithful.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heart of Kiran");
    let vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vehicle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
        text: "Crew 3.".into(),
        cost: ActivationCost { crew: Some(3), ..Default::default() },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: true,
        face_gate: None,
        effect: crew,
    }))
}

fn crew(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::add_type(
            ctx.source,
            ctx.source,
            TypeLine::CREATURE.into(),
            Duration::EndOfTurn,
        ),
    }]
}
