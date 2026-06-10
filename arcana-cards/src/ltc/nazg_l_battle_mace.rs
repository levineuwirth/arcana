//! Nazgûl Battle-Mace — `{5}` artifact — Equipment.
//! "Equipped creature has menace, deathtouch, annihilator 1, and 'Whenever
//! an opponent sacrifices a nontoken permanent, put that card onto the
//! battlefield under your control unless that player pays 3 life.'
//! Equip {3}"
//!
//! Implementation: `.with_equip({3})` wires the canonical Equip ability.
//! The static grants no P/T, so the ETB install is empty with the gaps
//! noted — attached keyword grants, annihilator, and granted triggered
//! abilities are all outside the Wave-1 Equipment surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nazgûl Battle-Mace");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_static,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: nothing installable — the static grants no P/T.
fn etb_install_static(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature has menace, deathtouch, annihilator 1, and
    // a granted triggered ability" — attached_pt covers P/T only (no
    // attached keyword grant yet); annihilator is not a supported
    // KeywordAbility and granting triggered abilities to the equipped
    // creature is not expressible.
    Vec::new()
}
