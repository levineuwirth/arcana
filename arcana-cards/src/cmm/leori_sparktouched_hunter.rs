//! Leori, Sparktouched Hunter — `{U}{R}{W}` 3/3 Legendary Elemental Cat.
//! Flying, vigilance.
//! Whenever Leori deals combat damage to a player, choose a planeswalker type.
//! Until end of turn, whenever you activate an ability of a planeswalker of
//! that type, copy that ability. You may choose new targets for the copies.
//!
//! Flying + Vigilance are base characteristics. The combat-damage trigger sets
//! up a delayed type-conditioned ability-copy replacement that is not
//! expressible (no "choose a planeswalker type" effect, no granted
//! activate-ability watcher keyed to a chosen subtype, no ability-copy
//! primitive for activated abilities), so the trigger's effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leori, Sparktouched Hunter");
    let elemental = reg.interner_mut().intern("Elemental");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: gap_planeswalker_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gap_planeswalker_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a planeswalker type; until end of turn, whenever you
    // activate an ability of a planeswalker of that type, copy that ability."
    // No effect chooses a planeswalker subtype, no granted watcher keys on a
    // player-chosen subtype, and there is no activated-ability copy primitive.
    Vec::new()
}
