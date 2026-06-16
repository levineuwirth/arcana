//! Glissa, Herald of Predation — `{3}{B}{G}` 3/5 Legendary Phyrexian Zombie Elf.
//!
//! Oracle text:
//! At the beginning of combat on your turn, choose one —
//! • Incubate 2 twice.
//! • Transform all Incubator tokens you control.
//! • Phyrexians you control gain first strike and deathtouch until end of turn.
//!
//! The Incubate/Transform Scryfall "keywords" are mechanic markers, not
//! keyword-line abilities, so `keywords` is empty.
//!
//! Modal dispatch is only supported on SPELL abilities (ModalSpec /
//! dispatch_modal_effect on SpellAbilityDef), not on triggered abilities, so
//! the "choose one —" structure cannot be wired on this beginning-of-combat
//! trigger. The trigger condition itself is faithful; the modal choice is
//! GAP'd (emitting any single mode unconditionally would be incorrect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glissa, Herald of Predation");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Incubate / Transform Scryfall markers are not usable KeywordAbility variants.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: glissa_choose_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn glissa_choose_one(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one —" modal trigger. Modal dispatch (ModalSpec /
    // dispatch_modal_effect) is only supported on SpellAbilityDef, not on
    // triggered abilities. The three modes (Incubate 2 twice / Transform all
    // Incubator tokens / Phyrexians gain first strike + deathtouch) cannot be
    // offered as a choice here, and emitting one mode unconditionally would be
    // incorrect.
    Vec::new()
}
