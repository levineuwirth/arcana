//! Hezrou // Demonic Stench — `{5}{B}{B}` 6/6 black Frog Demon.
//! "Whenever one or more creatures you control become blocked, each blocking
//! creature gets -1/-1 until end of turn."
//! Adventure face "Demonic Stench" (`{B}` Instant):
//! "Each creature that blocked this turn gets -1/-1 until end of turn."
//! GAP: creature trigger "one or more creatures become blocked" — using
//! SelfBecomesBlocked as best-effort (only covers self, not all creatures).
//! GAP: "each blocking creature" and "each creature that blocked this turn"
//! — no script helpers available for these.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hezrou");
    let frog = reg.interner_mut().intern("Frog");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Demonic Stench");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid adv cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Each creature that blocked this turn gets -1/-1 until end of turn.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: demonic_stench,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "whenever one or more creatures you control become blocked"
                // using SelfBecomesBlocked as best-effort
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: blocked_debuff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn blocked_debuff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each blocking creature gets -1/-1" — no script helper for blocking creatures
    Vec::new()
}

fn demonic_stench(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each creature that blocked this turn gets -1/-1" — no script helper
    Vec::new()
}
