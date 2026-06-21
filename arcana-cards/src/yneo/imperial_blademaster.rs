//! Imperial Blademaster — `{1}{R}{W}` 2/3 Human Samurai.
//!
//! Double strike.
//! Whenever a Samurai or Warrior you control attacks alone, draft a card from
//! Imperial Blademaster's spellbook.
//!
//! The attacks-alone trigger fires (filtered to Samurai/Warrior you control)
//! but its effect is GAP'd: "draft a card from a spellbook" is an Arena-only
//! Spellbook mechanic with no expressible primitive.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imperial Blademaster");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let samurai_sub = reg.interner_mut().intern("Samurai");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let attacker_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![samurai_sub, warrior_sub]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::AttacksAlone {
                filter: attacker_filter,
            },
            intervening_if: None,
            effect: draft_spellbook,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draft_spellbook(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draft a card from a spellbook" — Arena-only Spellbook mechanic,
    // no expressible primitive.
    Vec::new()
}
