//! Tyranid Harridan — `{4}{G}{U}` 4/4 Tyranid.
//!
//! * Flying, Ward {4} — base keywords.
//! * "Shrieking Gargoyles — Whenever this creature or another Tyranid you
//!   control deals combat damage to a player, create a 1/1 blue Tyranid
//!   Gargoyle creature token with flying." — a combat-damage-to-a-player
//!   trigger whose source is any Tyranid you control; creates one token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyranid Harridan");
    let tyranid = reg.interner_mut().intern("Tyranid");
    // Intern the token's subtype so the resolver's lookup succeeds.
    let _gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{4}").expect("valid cost")),
        ],
        ..Default::default()
    };

    let tyranid_source = script::subtype_filter(reg, "Tyranid")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: tyranid_source,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_gargoyle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_gargoyle(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let gargoyle = reg.interner().lookup("Gargoyle").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(t) = reg.interner().lookup("Tyranid") {
        subtypes.0.insert(t);
    }
    subtypes.0.insert(gargoyle);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: gargoyle,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
