//! The Rani — `{1}{U}{B}{R}` 3/4 Legendary Time Lord Scientist.
//!
//! Oracle:
//! * Whenever The Rani enters or attacks, create a red Aura enchantment
//!   token named Mark of the Rani attached to another target creature.
//!   That token has enchant creature and "Enchanted creature gets +2/+2 and
//!   is goaded."
//! * Whenever a goaded creature deals combat damage to one of your
//!   opponents, investigate.
//!
//! "Enters or attacks" decomposes into two triggers (SelfEntersBattlefield
//! + SelfAttacks). Each should mint an Aura token attached to the target;
//! the demonstrated TokenDefinition API cannot create an Aura token that
//! carries a continuous +2/+2 + goad static (no attach/static fields on
//! tokens), so the token itself is a GAP — but the salient game effect,
//! GOADING the target creature permanently, IS wired via Effect::Goad.
//! The "+2/+2" rider is GAP'd (no permanent pump primitive).
//!
//! The combat-damage trigger investigates (creates a Clue). The "goaded
//! creature" source restriction can't be expressed as an ObjectFilter
//! predicate, so it watches any creature dealing combat damage to a player
//! — a documented over-fire GAP.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("The Rani");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        // Investigate / Goad are mechanics expressed by the abilities
        // below, not creature keyword abilities — keywords vec stays empty.
        ..Default::default()
    };

    let mark_target = || TargetRequirement {
        filter: TargetFilter::Creature,
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Ability 1a: ETB — mark/goad another target creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mark_of_the_rani,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![mark_target()],
            })
            // Ability 1b: attacks — mark/goad another target creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: mark_of_the_rani,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![mark_target()],
            })
            // Ability 2: goaded creature deals combat damage to an opponent
            // → investigate.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::DamageDealt {
                    // GAP: cannot filter "goaded" creatures; watches any.
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Mark of the Rani: the printed effect attaches an Aura granting +2/+2 and
/// goad. The Aura token itself is unrepresentable (TokenDefinition has no
/// attach/static fields); the salient effect — goading the target — is
/// wired. The +2/+2 is a GAP.
fn mark_of_the_rani(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: Aura token "Mark of the Rani" + its +2/+2 static are unmodeled.
    vec![Effect::Goad {
        target: *id,
        goader: trig.controller,
        duration: Duration::Permanent,
    }]
}

fn investigate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
