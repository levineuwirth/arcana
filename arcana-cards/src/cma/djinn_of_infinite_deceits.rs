//! Djinn of Infinite Deceits — `{4}{U}{U}` 2/7 Djinn with Flying.
//! "{T}: Exchange control of two target nonlegendary creatures. You
//! can't activate this ability during combat."
//!
//! Flying is a base keyword. The activated ability is wired with its
//! tap cost and two nonlegendary-creature targets, but its effect is
//! GAP'd: there is no `Effect` variant that EXCHANGES control of two
//! permanents (only one-directional `ChangeControl`). The "can't
//! activate during combat" rider also has no expressible field.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Djinn of Infinite Deceits");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let nonlegendary = ObjectFilter::creature()
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Exchange control of two target nonlegendary creatures.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(nonlegendary),
                count: TargetCount::Exactly(2),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exchange_control,
        }),
    )
}

fn exchange_control(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant EXCHANGES control of two permanents
    // (only one-directional ChangeControl). Effect omitted.
    Vec::new()
}
