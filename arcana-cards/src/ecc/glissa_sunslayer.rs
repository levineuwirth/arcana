//! Glissa Sunslayer — `{1}{B}{G}` 3/3 Legendary Phyrexian Zombie Elf with
//! First strike and Deathtouch.
//! "Whenever Glissa Sunslayer deals combat damage to a player, choose one —
//!  • You draw a card and lose 1 life.
//!  • Destroy target enchantment.
//!  • Remove up to three counters from target permanent."
//!
//! The combat-damage trigger is a MODAL ("choose one —") triggered ability.
//! The demonstrated modal machinery (`ModalSpec` / `dispatch_modal_effect`)
//! exists only for SPELL abilities (`SpellAbilityDef`), not for
//! `TriggeredAbilityDef`; a triggered effect fn returns a flat `Vec<Effect>`
//! with no facility to post the mode choice (and modes 2/3 carry their own
//! targets, which a non-targeting trigger can't declare per-mode). So the
//! trigger condition is recorded faithfully but its modal payload is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glissa Sunslayer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_choose_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_choose_one(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal ("choose one —") payload on a TRIGGERED ability is not
    // expressible — ModalSpec/dispatch_modal_effect are SpellAbilityDef-only,
    // and modes 2/3 declare their own targets which a flat trigger effect fn
    // cannot post or select among.
    Vec::new()
}
