//! Elder Pine of Jukai — `{2}{G}` 2/1 Spirit with Soulshift 2.
//! "Whenever you cast a Spirit or Arcane spell, reveal the top three cards
//! of your library. Put all land cards revealed this way into your hand
//! and the rest on the bottom of your library in any order."
//!
//! The cast trigger is wired (SpellCast, you, Spirit-or-Arcane subtype),
//! but the reveal-three / put-ALL-lands-to-hand body is GAP'd: DigTopN is
//! single-take and RevealUntil takes only the first match, so neither
//! expresses "put every land among the top three into your hand".

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
    let name = reg.interner_mut().intern("Elder Pine of Jukai");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let spell_filter = ObjectFilter::new()
        .with_subtypes_any(vec![spirit, arcane])
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Soulshift(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(spell_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: reveal_top_three,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn reveal_top_three(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top three, put ALL land cards into hand, rest on
    // bottom" — DigTopN is single-take, RevealUntil takes only the first
    // match; no effect expresses taking every matching card of a fixed
    // reveal count.
    Vec::new()
}
