//! The Odd Acorn Gang — `{3}{B}{G}` 5/5 Legendary Squirrel Warrior with
//! Menace, Trample, Reach.
//! "Squirrels you control have '{T}: …'" (static ability-grant — GAP).
//! "Whenever one or more Squirrels you control deal combat damage to a player,
//! draw a card."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Odd Acorn Gang");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warrior);

    let squirrel_filter = arcana_core::script::subtype_filter(reg, "Squirrel")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Menace,
            KeywordAbility::Trample,
            KeywordAbility::Reach,
        ],
        ..Default::default()
    };

    // GAP: static "Squirrels you control have '{T}: Target Squirrel gets +2/+2 …'" —
    // grants an activated ability to a whole creature class (continuous ability-grant),
    // not expressible as a triggered/activated ability on this card.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: squirrel_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
