//! Thassa's Emissary — `{3}{U}` 3/3 blue Enchantment Creature — Crab.
//!
//! GAP (keyword): Bestow {5}{U} — Bestow is not in the usable KeywordAbility
//!   surface (no alternative aura-cast cost), so the card is modeled as the
//!   creature face only.
//! Whenever this creature or enchanted creature deals combat damage to a player,
//!   draw a card.
//! GAP (static): "Enchanted creature gets +3/+3." — only relevant when cast for
//!   bestow (an Aura); not expressible without Bestow support.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thassa's Emissary");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "this creature or enchanted creature deals combat damage to a player"
            // — modeled as the creature-face self combat-damage draw (the
            // enchanted-creature branch needs Bestow, which is GAP'd above).
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: on_combat_damage_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_combat_damage_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
