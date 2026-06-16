//! Cloudhoof Kirin — `{3}{U}{U}` 4/4 Legendary Kirin Spirit.
//! Flying.
//! Whenever you cast a Spirit or Arcane spell, you may have target player
//! mill X cards, where X is that spell's mana value.
//!
//! Mill is not a usable `KeywordAbility` variant — keyword line is Flying
//! only. The trigger fires on casting a Spirit-or-Arcane spell; the mill
//! amount X = the triggering spell's mana value, but no pending-trigger
//! accessor exposes the cast spell's mana value (only damage / dying /
//! entering / combatant accessors exist), so the amount can't be computed
//! — GAP the effect body.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloudhoof Kirin");
    let kirin = reg.interner_mut().intern("Kirin");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);
    subtypes.0.insert(spirit);
    // "a Spirit or Arcane spell" — subtype-OR filter built with the interner.
    let spirit_sym = reg.interner_mut().intern("Spirit");
    let arcane_sym = reg.interner_mut().intern("Arcane");
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![spirit_sym, arcane_sym]);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
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
            effect: mill_x,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn mill_x(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: mill X where X = the triggering spell's mana value — no
    // pending-trigger accessor exposes the cast spell's mana value.
    Vec::new()
}
