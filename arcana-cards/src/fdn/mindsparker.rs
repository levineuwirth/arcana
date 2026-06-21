//! Mindsparker — `{1}{R}{R}` 3/2 Elemental with First strike.
//! Whenever an opponent casts a white or blue instant or sorcery spell, this
//! creature deals 2 damage to that player.

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
    let name = reg.interner_mut().intern("Mindsparker");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                ),
                caster: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: deal_2_to_caster,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn deal_2_to_caster(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the spell must be "white or blue" but ObjectFilter color matching is
    // AND-only (no color-any builder), so the white-OR-blue restriction can't be
    // expressed. Dealing 2 damage on every opponent instant/sorcery would be
    // materially over-broad, so the effect is omitted.
    Vec::new()
}
