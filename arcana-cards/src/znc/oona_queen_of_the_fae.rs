//! Oona, Queen of the Fae — `{3}{U/B}{U/B}{U/B}` 5/5 Legendary Creature —
//! Faerie Wizard. Black/blue.
//! Flying.
//! "{X}{U/B}: Choose a color. Target opponent exiles the top X cards of
//! their library. For each card of the chosen color exiled this way,
//! create a 1/1 blue and black Faerie Rogue creature token with flying."
//!
//! The activated ability's effect is GAP'd: there is no Effect primitive
//! that exiles the top X cards of a target opponent's library, no
//! "choose a color" cost/effect, and no script helper that counts the
//! cards of a chosen color exiled this way — so the token count cannot
//! be computed. The ability is recorded with its X-bearing target slot;
//! the resolver returns no effects.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oona, Queen of the Fae");
    let faerie = reg.interner_mut().intern("Faerie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U/B}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{U/B}: Choose a color. Target opponent exiles the top X cards of their library. For each card of the chosen color exiled this way, create a 1/1 blue and black Faerie Rogue creature token with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{U/B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: oona_exile_and_make_tokens,
            }),
    )
}

fn oona_exile_and_make_tokens(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect exiles the top X of a target opponent's library;
    // no "choose a color" primitive; no script helper to count the
    // chosen-color cards exiled this way to scale the Faerie Rogue tokens.
    Vec::new()
}
