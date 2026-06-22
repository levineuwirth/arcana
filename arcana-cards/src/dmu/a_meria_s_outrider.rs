//! A-Meria's Outrider — `{4}{R}` 4/4 Elf Archer with Reach.
//!
//! Oracle:
//! * Reach (keyword). ("Domain" in the Scryfall keyword line is an ability-word
//!   flavor marker, not a `KeywordAbility` variant.)
//! * Domain — When Meria's Outrider enters, it deals damage equal to the number
//!   of basic land types among lands you control to any target.
//!
//! The ETB damage count is "domain" (number of basic land types among your
//! lands). There is no `script::` helper to compute the count of distinct basic
//! land types, so the dynamic damage amount is not computable and the effect is
//! GAP'd. The any-target requirement is still declared for catalog fidelity.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Meria's Outrider");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_domain_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::any_target()],
        }),
    )
}

fn etb_domain_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage "equal to the number of basic land types among lands you
    // control" (domain) — no script helper computes the count of distinct basic
    // land types, so the dynamic amount is not computable.
    Vec::new()
}
