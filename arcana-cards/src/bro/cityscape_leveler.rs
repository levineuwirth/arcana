//! Cityscape Leveler — `{8}` 8/8 Artifact Creature — Construct.
//! Trample.
//! "When you cast this spell and whenever this creature attacks, destroy
//!  up to one target nonland permanent. Its controller creates a tapped
//!  Powerstone token."
//! Unearth {8}.
//!
//! Trample is a base keyword. The "when you cast this spell" half of the
//! shared trigger has no cast-this-spell trigger condition (GAP); the
//! "whenever this creature attacks" half is wired. Unearth is not in the
//! usable keyword surface (GAP). The Powerstone "tapped" rider is a
//! fidelity gap in CreateCommodityToken.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cityscape Leveler");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    // GAP: Unearth {8} — graveyard-recursion keyword not in the usable
    // keyword surface for this card class.
    // GAP: "When you cast this spell, destroy up to one target nonland
    // permanent. Its controller creates a tapped Powerstone token." — no
    // cast-this-spell trigger condition; the attacks half below covers the
    // same effect on attack.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: destroy_and_powerstone,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn destroy_and_powerstone(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "Its controller creates a tapped Powerstone token." (tapped rider is
    // a fidelity gap.) Capture the controller before destroying.
    let owner = script::target_controller(state, *id, trig.controller);
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateCommodityToken {
            controller: owner,
            kind: CommodityToken::Powerstone,
            count: 1,
        },
    ]
}
