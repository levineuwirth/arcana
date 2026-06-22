//! Orcus, Prince of Undeath — `{X}{2}{B}{R}` 5/3 Legendary Demon.
//! Flying, trample.
//! When Orcus enters, choose one —
//! • Each other creature gets -X/-X until end of turn. You lose X life.
//! • Return up to X target creature cards with total mana value X or less from
//!   your graveyard to the battlefield. They gain haste until end of turn.
//!
//! Flying + Trample are base characteristics. The ETB is a MODAL choice on a
//! TRIGGERED ability — triggered abilities carry no modal mechanism (only spell
//! abilities do), and both modes scale on the cast {X} (an X not exposed to a
//! trigger effect), so the modal ETB effect is GAP'd.

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
    let name = reg.interner_mut().intern("Orcus, Prince of Undeath");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: gap_modal_etb,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gap_modal_etb(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: ETB "choose one —" modal on a triggered ability. Triggered abilities
    // have no modal mechanism (only spell abilities carry `modal`), and both
    // modes scale on the cast {X} (mass -X/-X + lose X life; reanimate up to X
    // creatures with total mv X or less) — the cast X is not exposed to a
    // trigger effect.
    Vec::new()
}
