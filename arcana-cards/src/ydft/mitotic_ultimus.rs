//! Mitotic Ultimus — `{7}{G}{G}` 8/8 Legendary Creature — Ooze Giant.
//! "This spell costs {X} less to cast, where X is the greatest power among
//! creatures you control." Trample. "When Mitotic Ultimus dies, conjure two
//! cards named Mitotic Slime onto the battlefield."
//!
//! Trample is a base keyword. The cost-reduction static and the Conjure death
//! trigger are both GAP material (no cost-reduction static is expressible here,
//! and Conjure is an Arena-only mechanic with no `Effect` variant).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mitotic Ultimus");
    let ooze = reg.interner_mut().intern("Ooze");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    subtypes.0.insert(giant);

    // GAP: static cost reduction "This spell costs {X} less to cast, where X is
    // the greatest power among creatures you control" — no cost-reduction
    // primitive is available for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_conjure,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_conjure(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "conjure two cards named Mitotic Slime onto the battlefield" —
    // Conjure is an Arena-only mechanic with no Effect::Conjure variant
    // (would need a registry-by-name lookup in Effect::execute).
    Vec::new()
}
