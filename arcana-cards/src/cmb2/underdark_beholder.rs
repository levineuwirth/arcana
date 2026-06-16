//! Underdark Beholder — `{4}{B}{B}` 6/6 Beholder.
//! Enters with ten eyestalk counters; a damage-replacement that removes
//! eyestalk counters instead; and an attack trigger that reveals/casts a
//! cheap instant/sorcery/enchantment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Underdark Beholder");
    let beholder = reg.interner_mut().intern("Beholder");
    let _eyestalk = reg.interner_mut().intern("eyestalk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beholder);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Scryfall "Convert" is reminder-flavor, not a usable keyword.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_eyestalk_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_reveal_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "If Underdark Beholder would be dealt damage, remove that many
    // eyestalk counters from it instead. If you can't, sacrifice it." is a
    // damage replacement effect keyed to a named counter; no expressible
    // InstallReplacementEffect form is available in this surface.
}

fn etb_eyestalk_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kind = match reg.interner().lookup("eyestalk") {
        Some(sym) => CounterKind::Named(sym),
        None => return Vec::new(),
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 10,
    }]
}

fn attack_reveal_cast(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "reveal until an instant/sorcery/enchantment with CMC less than
    // the number of eyestalk counters; cast it for free" combines a
    // dynamic-CMC reveal filter with a cast-for-free, which RevealUntil
    // (hand/battlefield destinations only) cannot express.
    Vec::new()
}
