//! Kairi, the Swirling Sky — `{4}{U}{U}` 6/6 Legendary Dragon Spirit.
//!
//! Flying, ward {3}
//! When Kairi dies, choose one —
//! • Return any number of target nonland permanents with total mana value
//!   6 or less to their owners' hands.
//! • Mill six cards, then return up to two instant and/or sorcery cards
//!   from your graveyard to your hand.
//!
//! The Scryfall "Mill" keyword is reminder for the second mode's "Mill
//! six"; it is not a `KeywordAbility` variant.
//!
//! GAP: the engine has no modal dispatch for TRIGGERED abilities (modal
//! support is spell-ability-only). The dies trigger is therefore wired to
//! the second mode (mill six, then return up to two instant/sorcery cards
//! from your graveyard — fully expressible and targeted). The first mode
//! (return any number of target nonland permanents with total mv ≤ 6) and
//! the "choose one" selection are GAP'd for a human to route once trigger
//! modal dispatch exists.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kairi, the Swirling Sky");
    let dragon = reg.interner_mut().intern("Dragon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_mill_and_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                },
                count: TargetCount::UpTo(2),
                controller: None,
            }],
        }),
    )
}

/// Dies trigger (second mode) — mill six, then return up to two chosen
/// instant/sorcery cards from your graveyard to hand.
fn dies_mill_and_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::Mill {
        player: trig.controller,
        count: 6,
    }];
    for t in trig.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    vec![Effect::Sequence(effects)]
}
